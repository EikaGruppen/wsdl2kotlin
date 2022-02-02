use tree_sitter::{Language, Node, Parser, Query, QueryCursor, QueryMatch, Tree};

struct JavaFile {
    content: String,
    tree: Tree,
}

struct JavaQuery<'a> {
    cursor: QueryCursor,
    file: &'a JavaFile,
    query: Query,
}

impl<'a> JavaQuery<'a> {
    fn matches(&'a mut self) -> impl Iterator<Item = QueryMatch<'a>> + 'a {
        self.cursor.matches(
            &self.query,
            self.file.tree.root_node(),
            to_callback(&self.file.content),
        )
    }

    fn matches_node(&'a mut self, node: Node<'a>) -> impl Iterator<Item = QueryMatch<'a>> + 'a {
        self.cursor
            .matches(&self.query, node, to_callback(&self.file.content))
    }
}

impl<'a> JavaFile {
    fn new(content: &str) -> JavaFile {
        let mut parser = Parser::new();
        match parser.set_language(lang()) {
            Ok(_) => (),
            Err(why) => panic!("{:?}", why),
        }
        let tree = parser.parse(content, None).unwrap();

        JavaFile {
            content: content.to_string(),
            tree,
        }
    }

    fn new_query(&'a self, query: &'a str) -> JavaQuery {
        JavaQuery {
            cursor: QueryCursor::new(),
            file: self,
            query: Query::new(lang(), query).unwrap(),
        }
    }

    fn get_functions(&'a self) -> Vec<Function> {
        let mut query =
            self.new_query("((method_declaration type: *@type name: *@name parameters: *@pars))");
        let matches = query.matches();

        matches
            .map(|mat| Function {
                name: mat.second_capture(self),
                return_type: mat.first_capture(self),
                arguments: self.parse_arguments(mat.captures[2].node),
            })
            .collect()
    }

    fn get_superclass(&'a self) -> Option<String> {
        let mut query = self
            .new_query("((class_declaration superclass: (superclass (type_identifier) @type)))");
        let matches = query.matches().next();

        matches.map(|mat| mat.first_capture(self))
    }

    fn get_subclasses(&'a self, imports: &Vec<Import>) -> Vec<Subclass> {
        let mut query = self.new_query("((class_declaration (modifiers (annotation name: (identifier) @name arguments: (annotation_argument_list) @args))))");
        let mut matches = query.matches();

        let xml_see_also_node = matches
            .find(|mat| mat.first_capture(self) == "XmlSeeAlso")
            .map(|mat| mat.captures[1].node);

        if xml_see_also_node.is_some() {
            let mut query = self.new_query(
                "((element_value_array_initializer (field_access) @subclass))",
                // "((element_value_array_initializer (class_literal (identifier) @subclass )))",
            );

            // scoped_identifier
            let matches = query.matches_node(xml_see_also_node.unwrap());

            let subclasses: Vec<Subclass> = matches
                .map(|mat| {
                    let first_capture = mat.first_capture(self);
                    let subclass: Vec<&str> = first_capture
                        .strip_suffix(".class")
                        .unwrap_or(&first_capture)
                        .rsplitn(2, ".")
                        .collect();
                    if subclass.len() > 1 {
                        Subclass {
                            name: format!("{}.{}", subclass[1], subclass[0]),
                            package: None,
                        }
                    } else {
                        let subclass_name = subclass[0];
                        Subclass {
                            name: subclass_name.to_owned(),
                            package: package_import(subclass_name, imports),
                        }
                    }
                })
                .collect();
            // Needed only in tree-sitter 0.16
            if subclasses.len() == 0 {
                let mut query = self.new_query(
                    "((element_value_array_initializer (class_literal (scoped_identifier) @subclass )))",
                );
                let matches = query.matches_node(xml_see_also_node.unwrap());

                matches
                    .map(|mat| {
                        let first_capture = mat.first_capture(self);
                        let subclass: Vec<&str> = first_capture
                            .strip_suffix(".class")
                            .unwrap_or(&first_capture)
                            .rsplitn(2, ".")
                            .collect();
                        if subclass.len() > 1 {
                            Subclass {
                                name: format!("{}.{}", subclass[1], subclass[0]),
                                package: None,
                            }
                        } else {
                            let subclass_name = subclass[0];
                            Subclass {
                                name: subclass_name.to_owned(),
                                package: package_import(subclass_name, imports),
                            }
                        }
                    })
                    .collect()
            } else {
                subclasses
            }
        } else {
            vec![]
        }
    }

    fn parse_arguments(&'a self, arguments_node: Node) -> Vec<Argument> {
        let mut query =
            self.new_query("((formal_parameter (modifiers) @anno type: *@type name: *@name ))");
        let matches = query.matches_node(arguments_node);

        matches
            .map(|mat| Argument {
                r#type: mat.second_capture(self),
                name: mat.third_capture(self),
                nullable: self.is_header(mat.captures[0].node),
            })
            .collect()
    }

    fn get_imports(&'a self) -> Vec<Import> {
        let mut query = self.new_query("((import_declaration) @import)");
        let matches = query.matches();

        matches
            .map(|mat| {
                mat.captures[0]
                    .node
                    .utf8_text(self.content.as_bytes())
                    .unwrap()
            })
            .filter(|import_declaration| {
                !import_declaration.starts_with("import javax.xml.ws")
                    && !import_declaration.starts_with("import java.util")
                    && !import_declaration.starts_with("import javax.xml.bind")
                    && !import_declaration.starts_with("import javax.jws")
            })
            .map(|import_declaration| import_declaration.trim_end_matches(";"))
            .map(|import_declaration| import_declaration.trim_start_matches("import "))
            .map(|import_declaration| {
                let parts: Vec<&str> = import_declaration.rsplitn(2, ".").collect();
                Import {
                    package: parts[1].to_string(),
                    class: parts[0].to_string(),
                }
            })
            .collect()
    }

    fn is_abstract(&'a self) -> bool {
        let mut query = self.new_query("((class_declaration (modifiers) @modifiers))");
        let mut matches = query.matches();

        let is_abstract = matches
            .find(|mat| mat.first_capture(self).ends_with("public abstract"))
            .is_some();

        is_abstract
    }

    fn is_header(&'a self, annotations_node: Node) -> bool {
        let mut query = self.new_query("((element_value_pair key: *@key value: *@value))");
        let mut matches = query.matches_node(annotations_node);

        matches.any(|mat| {
            let key = mat.first_capture(self);
            let value = mat.second_capture(self);
            key == "header" && value == "true"
        })
    }

    fn get_enum_constants(&'a self) -> Vec<String> {
        let mut query = self.new_query("((enum_constant name: *@const))");
        let matches = query.matches();

        matches.map(|mat| mat.first_capture(self)).collect()
    }

    fn get_class_name(&'a self, class_node: Node) -> String {
        let mut class_name_query = self.new_query("((class_declaration name: * @class))");
        let mut matches = class_name_query.matches_node(class_node);
        matches.next().unwrap().first_capture(self)
    }

    fn get_inner_classes(&'a self, class_package: &str, imports: &Vec<Import>) -> Vec<InnerClass> {
        let mut query = self.new_query(
            "(program (class_declaration body: (class_body (class_declaration) @inner_class)))",
        );
        let matches = query.matches();
        matches
            .map(|mat| mat.captures[0].node)
            .map(|inner_class_node| InnerClass {
                name: self.get_class_name(inner_class_node),
                fields: self.get_fields(Some(inner_class_node), class_package, imports),
            })
            .collect()
    }

    fn get_fields(
        &'a self,
        inner_class_node: Option<Node>,
        class_package: &str,
        imports: &Vec<Import>,
    ) -> Vec<Field> {
        let mut query: JavaQuery;
        let matches = if inner_class_node.is_some() {
            query = self
                .new_query("((class_declaration body: (class_body (field_declaration) @field)))");
            query.matches_node(inner_class_node.unwrap())
        } else {
            query = self.new_query(
                "(program (class_declaration body: (class_body (field_declaration) @field)))",
            );
            query.matches_node(self.tree.root_node())
        };

        let field_nodes: Vec<Node> = matches.map(|mat| mat.captures[0].node).collect();

        field_nodes
        .iter()
        .map(|node| {
            let mut query =
                self.new_query("((field_declaration type: * @imp declarator: (variable_declarator name: * @dec)))");
            let captures = query
                .matches_node(*node)
                .next()
                .unwrap()
                .captures;

            let type_node = captures[0].node;
            let variable_name = captures[1]
                .node
                .utf8_text(self.content.as_bytes())
                .unwrap()
                .to_string();

            let inner_type: &str;
            let generic_type: Option<String>;

            if type_node.kind() == "generic_type" {
                inner_type = type_node
                    .child(1)
                    .unwrap()
                    .child(1)
                    .unwrap()
                    .utf8_text(self.content.as_bytes())
                    .unwrap();
                generic_type = Some(
                    type_node
                        .child(0)
                        .unwrap()
                        .utf8_text(self.content.as_bytes())
                        .unwrap()
                        .to_string(),
                );
            } else {
                inner_type = type_node.utf8_text(self.content.as_bytes()).unwrap();
                generic_type = None;
            }

            let primitive = inner_type.chars().next().unwrap().is_lowercase();

            let is_builtin = [
                "Long", "long", "Double", "double", "Float", "float", "String", "Integer", "int",
                "Short", "short", "Boolean", "boolean", "byte[]", "Object",
            ]
            .iter()
            .any(|builtin| builtin == &inner_type);

            let package = type_package(is_builtin, class_package, inner_type, imports);
            let is_stdlib = package.as_ref().map_or(true, |package| package.starts_with("java"));

            Field {
                name: variable_name,
                xml_name: self.xml_name(*node),
                package: class_package.to_owned(),
                r#type: Type {
                    package,
                    class: inner_type.to_string(),
                    stdlib: is_stdlib,
                },
                generic_type,
                builtin: is_builtin,
                nullable: !primitive && self.is_nullable(*node),
            }
        })
        .collect()
    }

    fn is_nullable(&'a self, field_node: Node) -> bool {
        let mut query = self.new_query(
            "((field_declaration (modifiers (annotation name: * @ann arguments: * @arg))))",
        );
        let matches = query.matches_node(field_node);

        let xml_element_arguments: Option<Node> = matches
            .filter(|mat| mat.first_capture(self) == "XmlElement")
            .map(|mat| mat.captures[1].node)
            .next();

        match xml_element_arguments {
            Some(xml_element_arguments) => {
                let mut arg_query =
                    self.new_query("((element_value_pair key: * @key value: * @value))");
                let mut arg_matches = arg_query.matches_node(xml_element_arguments);

                let required = arg_matches.any(|mat| {
                    mat.first_capture(self) == "required" && mat.second_capture(self) == "true"
                });
                !required
            }
            None => return true,
        }
    }

    fn xml_name(&'a self, field_node: Node) -> Option<String> {
        let mut query = self.new_query(
            "((field_declaration (modifiers (annotation name: * @ann arguments: * @arg))))",
        );
        let matches = query.matches_node(field_node);

        let xml_element_arguments: Option<Node> = matches
            .filter(|mat| mat.first_capture(self) == "XmlElement" || mat.first_capture(self) == "XmlElementRef")
            .map(|mat| mat.captures[1].node)
            .next();

        match xml_element_arguments {
            Some(xml_element_arguments) => {
                let mut arg_query =
                    self.new_query("((element_value_pair key: * @key value: * @value))");
                let mut arg_matches = arg_query.matches_node(xml_element_arguments);

                arg_matches
                    .find(|mat| mat.first_capture(self) == "name")
                    .map(|mat| mat.second_capture(self).replace("\"", ""))
            }
            None => return None,
        }
    }
}

trait CapureContent {
    fn first_capture(&self, java_file: &JavaFile) -> String;
    fn second_capture(&self, java_file: &JavaFile) -> String;
    fn third_capture(&self, java_file: &JavaFile) -> String;
    fn capture_at(&self, index: usize, java_file: &JavaFile) -> String;
}

impl<'a> CapureContent for QueryMatch<'a> {
    fn first_capture(&self, java_file: &JavaFile) -> String {
        self.capture_at(0, java_file)
    }

    fn second_capture(&self, java_file: &JavaFile) -> String {
        self.capture_at(1, java_file)
    }

    fn third_capture(&self, java_file: &JavaFile) -> String {
        self.capture_at(2, java_file)
    }

    fn capture_at(&self, index: usize, java_file: &JavaFile) -> String {
        self.captures[index]
            .node
            .utf8_text(java_file.content.as_bytes())
            .unwrap()
            .to_string()
    }
}

pub fn parse_class(source_code: &str) -> Class {
    let java_file = JavaFile::new(source_code);

    let mut package_query =
        java_file.new_query("((package_declaration (scoped_identifier) @package))");

    let package_match = package_query.matches().next().unwrap();
    let package_name = package_match.first_capture(&java_file);

    let mut class_name_query = java_file.new_query("((class_declaration name: * @class))");
    let class_matches = class_name_query.matches().next();

    let imports = java_file.get_imports();
    let subclasses = java_file.get_subclasses(&imports);

    if class_matches.is_some() {
        let class_name = class_matches.unwrap().first_capture(&java_file);
        let fields = java_file.get_fields(None, &package_name, &imports);
        let inner_classes = java_file.get_inner_classes(&package_name, &imports);

        Class {
            package: package_name,
            imports,
            name: class_name,
            fields,
            is_abstract: java_file.is_abstract(),
            subclasses,
            superclass: java_file.get_superclass().map_or(None, |name| {
                Some(Superclass {
                    name,
                    ..Default::default()
                })
            }),
            inner_classes,
            skip: false,
            ..Default::default()
        }
    } else {
        let mut enum_name_query = java_file.new_query("((enum_declaration name: * @enum))");
        let enum_match = enum_name_query.matches().next();
        if enum_match.is_some() {
            let enum_name = enum_match.unwrap().first_capture(&java_file);
            Class {
                package: package_name.to_string(),
                imports,
                name: enum_name,
                enum_constants: java_file.get_enum_constants(),
                ..Default::default()
            }
        } else {
            let mut interface_name_query =
                java_file.new_query("((interface_declaration name: * @interface))");
            let interface_match = interface_name_query.matches().next().unwrap();
            let interface_name = interface_match.first_capture(&java_file);

            Class {
                package: package_name.to_string(),
                imports,
                name: interface_name,
                functions: java_file.get_functions(),
                ..Default::default()
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<Argument>,
    pub return_type: String,
}

#[derive(PartialEq, Debug)]
pub struct Argument {
    pub name: String,
    pub r#type: String,
    pub nullable: bool,
}

#[derive(PartialEq, Debug, Default)]
pub struct Class {
    pub package: String,
    pub imports: Vec<Import>,
    pub name: String,
    pub fields: Vec<Field>,
    pub is_abstract: bool,
    pub superclass: Option<Superclass>,
    pub subclasses: Vec<Subclass>,
    pub enum_constants: Vec<String>,
    pub functions: Vec<Function>,
    pub inner_classes: Vec<InnerClass>,
    pub skip: bool,
}

#[derive(PartialEq, Debug, Default)]
pub struct InnerClass {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(PartialEq, Debug, Default)]
pub struct Subclass {
    pub name: String,
    pub package: Option<String>,
}

#[derive(PartialEq, Debug, Default)]
pub struct Superclass {
    pub name: String,
    pub package: String,
    pub imports: Vec<Import>,
    pub fields: Vec<Field>,
    pub is_abstract: bool,
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Type {
    pub package: Option<String>,
    pub class: String,
    pub stdlib: bool,
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Field {
    pub name: String,
    pub xml_name: Option<String>,
    pub package: String,
    pub r#type: Type,
    pub generic_type: Option<String>,
    pub builtin: bool,
    pub nullable: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Import {
    pub package: String,
    pub class: String,
}

// impl Clone for Field {
//     fn clone(&self) -> Self {
//         Field {
//             name: self.name,
//             r#type: self.r#type,
//             generic_type: self.generic_type,

//         }
//     }

// }

fn lang() -> Language {
    extern "C" {
        fn tree_sitter_java() -> Language;
    }
    unsafe { tree_sitter_java() }
}

fn to_callback<'a>(source: &'a str) -> impl Fn(Node) -> &'a [u8] {
    move |n| &source.as_bytes()[n.byte_range()]
}

fn type_package(
    builtin: bool,
    class_package: &str,
    inner_type: &str,
    imports: &Vec<Import>,
) -> Option<String> {
    if builtin {
        None
    } else {
        package_import(inner_type, imports).or(Some(class_package.to_string()))
    }
}

fn package_import(class_name: &str, imports: &Vec<Import>) -> Option<String> {
    imports
        .iter()
        .find(|import| import.class == class_name)
        .map(|import| import.package.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;

    #[test]
    fn test_parse_class() {
        let source_code =
            fs::read_to_string("test-resources/src/main/kotlin/misc/Types.java").unwrap();

        let parsed_class = parse_class(&source_code);

        let expected = Class {
            package: "misc.a".to_string(),
            name: "Types".to_string(),
            fields: vec![
                Field {
                    name: "integer".to_string(),
                    xml_name: Some("Integer".to_string()),
                    package: "misc.a".to_string(),
                    r#type: Type {
                        package: None,
                        class: "Integer".to_string(),
                        stdlib: true,
                    },
                    generic_type: None,
                    builtin: true,
                    nullable: true,
                },
                Field {
                    name: "cars".to_string(),
                    xml_name: Some("Cars".to_string()),
                    package: "misc.a".to_string(),
                    r#type: Type {
                        package: Some("misc.a".to_string()),
                        class: "Car".to_string(),
                        stdlib: false,
                    },
                    generic_type: Some("List".to_string()),
                    builtin: false,
                    nullable: true,
                },
                Field {
                    name: "bool".to_string(),
                    xml_name: Some("Bool".to_string()),
                    package: "misc.a".to_string(),
                    r#type: Type {
                        package: None,
                        class: "Boolean".to_string(),
                        stdlib: true,
                    },
                    generic_type: None,
                    builtin: true,
                    nullable: true,
                },
                Field {
                    name: "nillableShort".to_string(),
                    xml_name: Some("NillableShort".to_string()),
                    package: "misc.a".to_string(),
                    r#type: Type {
                        package: None,
                        class: "Short".to_string(),
                        stdlib: true,
                    },
                    generic_type: None,
                    builtin: true,
                    nullable: true,
                },
                Field {
                    name: "car".to_string(),
                    xml_name: None,
                    package: "misc.a".to_string(),
                    r#type: Type {
                        package: Some("misc.a".to_string()),
                        class: "Car".to_string(),
                        stdlib: false,
                    },
                    generic_type: None,
                    builtin: false,
                    nullable: true,
                },
                Field {
                    name: "xmlElementString".to_string(),
                    xml_name: Some("XmlElementString".to_string()),
                    package: "misc.a".to_string(),
                    r#type: Type {
                        package: None,
                        class: "String".to_string(),
                        stdlib: true,
                    },
                    generic_type: Some("JAXBElement".to_string()),
                    builtin: true,
                    nullable: true,
                },
                Field {
                    name: "primInt".to_string(),
                    xml_name: None,
                    package: "misc.a".to_string(),
                    r#type: Type {
                        package: None,
                        class: "int".to_string(),
                        stdlib: true,
                    },
                    generic_type: None,
                    builtin: true,
                    nullable: false,
                },
                Field {
                    name: "primBool".to_string(),
                    xml_name: None,
                    package: "misc.a".to_string(),
                    r#type: Type {
                        package: None,
                        class: "boolean".to_string(),
                        stdlib: true,
                    },
                    generic_type: None,
                    builtin: true,
                    nullable: false,
                },
                Field {
                    name: "primLong".to_string(),
                    xml_name: None,
                    package: "misc.a".to_string(),
                    r#type: Type {
                        package: None,
                        class: "long".to_string(),
                        stdlib: true,
                    },
                    generic_type: None,
                    builtin: true,
                    nullable: false,
                },
                Field {
                    name: "primByteArray".to_string(),
                    xml_name: None,
                    package: "misc.a".to_string(),
                    r#type: Type {
                        package: None,
                        class: "byte[]".to_string(),
                        stdlib: true,
                    },
                    generic_type: None,
                    builtin: true,
                    nullable: false,
                },
                Field {
                    name: "primShort".to_string(),
                    xml_name: None,
                    package: "misc.a".to_string(),
                    r#type: Type {
                        package: None,
                        class: "short".to_string(),
                        stdlib: true,
                    },
                    generic_type: None,
                    builtin: true,
                    nullable: false,
                },
                Field {
                    name: "primDouble".to_string(),
                    xml_name: None,
                    package: "misc.a".to_string(),
                    r#type: Type {
                        package: None,
                        class: "double".to_string(),
                        stdlib: true,
                    },
                    generic_type: None,
                    builtin: true,
                    nullable: false,
                },
                Field {
                    name: "primFloat".to_string(),
                    xml_name: None,
                    package: "misc.a".to_string(),
                    r#type: Type {
                        package: None,
                        class: "float".to_string(),
                        stdlib: true,
                    },
                    generic_type: None,
                    builtin: true,
                    nullable: false,
                },
            ],
            ..Default::default()
        };

        assert_eq!(parsed_class, expected)
    }

    #[test]
    fn test_superclass() {
        let source_code =
            fs::read_to_string("test-resources/src/main/kotlin/misc/VehicleType.java")
                .unwrap();
        let parsed_class = parse_class(&source_code);
        let expected = Class {
            package: "misc.a".to_string(),
            name: "VehicleType".to_string(),
            imports: vec![Import {
                package: "misc.b".to_string(),
                class: "Car".to_string(),
            }],
            fields: vec![Field {
                name: "doors".to_string(),
                xml_name: None,
                package: "misc.a".to_string(),
                r#type: Type {
                    package: None,
                    class: "int".to_string(),
                    stdlib: true,
                },
                generic_type: None,
                builtin: true,
                nullable: false,
            }],
            subclasses: vec![
                Subclass {
                    package: Some("misc.b".to_string()),
                    name: "Car".to_string(),
                },
                Subclass {
                    package: None,
                    name: "Bus".to_string(),
                },
            ],
            ..Default::default()
        };
        assert_eq!(parsed_class, expected)
    }

    #[test]
    fn test_superclass_with_subclasses_with_same_name() {
        let source_code = fs::read_to_string(
            "test-resources/src/main/kotlin/misc/SuperclassWithSubclassesWithSameName.java",
        )
        .unwrap();
        let parsed_class = parse_class(&source_code);
        let expected = Class {
            package: "misc.a".to_string(),
            name: "SuperclassWithSubclassesWithSameName".to_string(),
            subclasses: vec![
                Subclass {
                    package: None,
                    name: "misc.b.Subclass".to_string(),
                },
                Subclass {
                    package: None,
                    name: "misc.c.Subclass".to_string(),
                },
            ],
            ..Default::default()
        };
        assert_eq!(parsed_class, expected)
    }

    #[test]
    fn test_abstract_class() {
        let source_code =
            fs::read_to_string("test-resources/src/main/kotlin/misc/BaseResponseType.java")
                .unwrap();
        let parsed_class = parse_class(&source_code);
        let expected = Class {
            package: "misc.a".to_string(),
            imports: vec![
                Import {
                    class: "ResponseStateType".to_string(),
                    package: "misc.b".to_string(),
                },
                Import {
                    class: "CarCreateResponseType".to_string(),
                    package: "misc.c".to_string(),
                },
            ],
            name: "BaseResponseType".to_string(),
            fields: vec![Field {
                name: "responseState".to_string(),
                xml_name: Some("ResponseState".to_string()),
                package: "misc.a".to_string(),
                r#type: Type {
                    package: Some("misc.b".to_string()),
                    class: "ResponseStateType".to_string(),
                    stdlib: false,
                },
                generic_type: None,
                builtin: false,
                nullable: false,
            }],
            is_abstract: true,
            subclasses: vec![Subclass {
                package: Some("misc.c".to_string()),
                name: "CarCreateResponseType".to_string(),
            }],
            ..Default::default()
        };
        assert_eq!(parsed_class, expected)
    }

    #[test]
    fn test_parse_interface() {
        let source_code = fs::read_to_string(
            "test-resources/src/main/kotlin/misc/CarsService.java",
        )
        .unwrap();

        let parsed_class = parse_class(&source_code);
        let expected = Class {
            package: "misc.a".to_string(),
            name: "CarsService".to_string(),
            functions: vec![Function {
                name: "getCars".to_string(),
                arguments: vec![
                    Argument {
                        name: "soapHeader".to_string(),
                        r#type: "misc.b.HeaderType".to_string(),
                        nullable: true,
                    },
                    Argument {
                        name: "getCars".to_string(),
                        r#type: "GetCars".to_string(),
                        nullable: false,
                    },
                ],
                return_type: "GetCarsResponse".to_string(),
            }],
            ..Default::default()
        };
        assert_eq!(parsed_class, expected)
    }

    #[test]
    fn test_parse_class_with_innerclass() {
        let source_code = fs::read_to_string(
            "test-resources/src/main/kotlin/inheritance/abstractclasses/ALevelWithInner.java",
        )
        .unwrap();

        let parsed_class = parse_class(&source_code);
        let expected = Class {
            package: "inheritance.abstractclasses".to_string(),
            imports: vec![],
            name: "ALevelWithInner".to_string(),
            fields: vec![
                Field {
                    name: "a".to_string(),
                    xml_name: None,
                    package: "inheritance.abstractclasses".to_string(),
                    r#type: Type {
                        package: None,
                        class: "int".to_string(),
                        stdlib: true,
                    },
                    generic_type: None,
                    builtin: true,
                    nullable: false,
                },
                Field {
                    name: "inner".to_string(),
                    xml_name: None,
                    package: "inheritance.abstractclasses".to_string(),
                    r#type: Type {
                        package: Some("inheritance.abstractclasses".to_string()),
                        class: "ALevelWithInner.Inner".to_string(),
                        stdlib: false,
                    },
                    generic_type: None,
                    builtin: false,
                    nullable: true,
                },
            ],
            is_abstract: true,
            inner_classes: vec![InnerClass {
                name: "Inner".to_string(),
                fields: vec![
                    Field {
                        name: "ia".to_string(),
                        xml_name: None,
                        package: "inheritance.abstractclasses"
                            .to_string(),
                        r#type: Type {
                            package: None,
                            class: "String".to_string(),
                            stdlib: true,
                        },
                        generic_type: None,
                        builtin: true,
                        nullable: true,
                    },
                ],
            }],
            ..Default::default()
        };

        assert_eq!(parsed_class, expected)
    }
}
