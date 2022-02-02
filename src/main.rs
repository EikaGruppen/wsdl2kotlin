use regex::Regex;
use std::{
    fs::{self, File},
    io::BufWriter,
    path::{Path, PathBuf},
};

use javaparser::Class;

use javaparser::Class as JavaClass;
use javaparser::Field as JavaField;
use javaparser::Import as JavaImport;
use javaparser::Superclass as JavaSuperclass;

use kotlinprinter::Arg as KotlinArgument;
use kotlinprinter::Field as KotlinField;
use kotlinprinter::Function as KotlinFunction;
use kotlinprinter::Import as KotlinImport;
use kotlinprinter::InnerClass as KotlinInnerClass;
use kotlinprinter::KotlinClass;
use kotlinprinter::Part;
use kotlinprinter::Superclass as KotlinSuperclass;

use std::collections::HashMap;

mod files;
mod javaparser;
mod kotlinprinter;

const FIELDS_CHUNKING_LIMIT: usize = 50;

fn generate(generated_path: &Path) {
    let file_paths = files::find_classes(generated_path);
    file_paths
        .iter()
        .for_each(|path| write_kotlin_file(generated_path, path));
}

fn title(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn convert_fields(
    java_class_name: &str,
    java_class_package: &str,
    java_class_fields: &Vec<JavaField>,
    subclass_package: Option<&str>,
) -> Vec<KotlinField> {
    java_class_fields
        .iter()
        .map(|java_field| {
            let mut convert = false;
            let mut kotlin_type = java_field.r#type.class.clone();

            if java_field.builtin {
                let builin_types: HashMap<&str, &str> = [
                    ("String", "String"),
                    ("Integer", "Int"),
                    ("int", "Int"),
                    ("Double", "Double"),
                    ("double", "Double"),
                    ("Float", "Float"),
                    ("float", "Float"),
                    ("Short", "Short"),
                    ("short", "Short"),
                    ("Boolean", "Boolean"),
                    ("boolean", "Boolean"),
                    ("Long", "Long"),
                    ("long", "Long"),
                    ("byte[]", "ByteArray"),
                    ("Object", "Any"),
                ]
                .iter()
                .cloned()
                .collect();

                kotlin_type = builin_types
                    .get(java_field.r#type.class.as_str())
                    .unwrap()
                    .to_owned()
                    .to_owned();
            } else {
                //Assume this is an project internal package-level class
                if java_field.r#type.package.is_some() {
                    if java_field.r#type.stdlib {
                        convert = false
                    } else {
                        kotlin_type = kotlin_type
                            .strip_prefix(format!("{}.", java_class_name).as_str())
                            .unwrap_or(kotlin_type.as_str())
                            .to_string();

                        kotlin_type = kotlin_class_name(kotlin_type);
                        convert = true
                    }
                } else {
                    convert = true
                }
            }
            let mut factory_func = String::from("");
            let mut object_factory_class = "";
            let mut object_factory_package = "";

            // TODO not pretty
            let kotlin_class_name = kotlin_class_name(java_class_name.to_string());

            if java_field.generic_type == Some(String::from("JAXBElement")) {
                factory_func = format!(
                    "create{}{}",
                    java_class_name,
                    title(java_field.xml_name.as_ref().unwrap_or(&java_field.name))
                );
                object_factory_class = kotlin_class_name.as_str();
                object_factory_package = java_class_package;
            }

            KotlinField::new(
                java_field.name.as_str(),
                kotlin_type,
                java_field
                    .generic_type
                    .as_ref()
                    .map(|gt| gt.as_str())
                    .unwrap_or(""),
                factory_func,
                object_factory_class,
                object_factory_package,
                convert,
                java_field.nullable,
                subclass_package.map_or(false, |sub_pack| java_field.package != sub_pack),
            )
        })
        .collect()
}

fn convert(java_class: JavaClass) -> KotlinClass {
    let own_fields: Vec<KotlinField> = convert_fields(
        java_class.name.as_str(),
        java_class.package.as_str(),
        &java_class.fields,
        None,
    );

    let inner_classes: Vec<KotlinInnerClass> = java_class
        .inner_classes
        .iter()
        .map(|inner_class| {
            let name = inner_class.name.as_str();
            KotlinInnerClass {
                name: name.to_string(),
                fields: convert_fields(
                    name,
                    java_class.package.as_str(),
                    &inner_class.fields,
                    None,
                ),
            }
        })
        .collect();

    let mut inherited_imports: Vec<KotlinImport> = java_class
        .superclass
        .as_ref()
        .map(|superclass| {
            superclass
                .fields
                .iter()
                .filter(|field| field.r#type.package.is_some())
                .map(|field| {
                    let class_name = if field.r#type.stdlib {
                        field.r#type.class.clone()
                    } else {
                        kotlin_class_name(field.r#type.class.clone())
                    };
                    KotlinImport {
                        package: field.r#type.package.as_ref().unwrap().to_owned(),
                        class: class_name,
                    }
                })
                .collect()
        })
        .unwrap_or(vec![]);

    java_class
        .superclass
        .as_ref()
        .filter(|superclass| superclass.package != java_class.package)
        .map(|superclass| KotlinImport {
            package: superclass.package.clone(),
            class: kotlin_class_name(superclass.name.clone()),
        })
        .map(|superclass_as_import| inherited_imports.push(superclass_as_import));

    let subclass_imports: Vec<KotlinImport> = java_class
        .subclasses
        .iter()
        .filter(|subclass| subclass.package.is_some())
        .map(|subclass| KotlinImport {
            package: subclass.package.as_ref().unwrap().to_owned(),
            class: subclass.name.clone(),
        })
        .collect();

    let subclass_imports_kotlin: Vec<KotlinImport> = subclass_imports
        .iter()
        .map(|import| KotlinImport {
            package: import.package.clone(),
            class: kotlin_class_name(import.class.clone()),
        })
        .collect();

    let inner_class_imports: Vec<KotlinImport> = java_class
        .inner_classes
        .iter()
        .flat_map(|inner_class| &inner_class.fields)
        .filter(|field| field.r#type.package.is_some())
        .map(|field| {
            let class_name = if field.r#type.stdlib {
                field.r#type.class.clone()
            } else {
                kotlin_class_name(field.r#type.class.clone())
            };
            KotlinImport {
                package: field.r#type.package.as_ref().unwrap().to_owned(),
                class: class_name,
            }
        })
        .collect();

    let java_class_package = java_class.package.as_str();
    // add inherited fields here?
    let own_imports: Vec<KotlinImport> = java_class
        .fields
        .iter()
        .filter(|field| field.r#type.package.is_some())
        .filter(|field| field.r#type.package != Some(java_class_package.to_string()))
        .map(|field| {
            let class_name = if field.r#type.stdlib {
                field.r#type.class.clone()
            } else {
                kotlin_class_name(field.r#type.class.clone())
            };
            KotlinImport {
                package: field.r#type.package.as_ref().unwrap().to_owned(),
                class: class_name,
            }
        })
        .collect();

    let mut imports = [
        inherited_imports,
        subclass_imports,
        subclass_imports_kotlin,
        own_imports,
        inner_class_imports,
    ]
    .concat();
    imports.sort();
    imports.dedup();

    let functions = java_class
        .functions
        .iter()
        .map(|func| KotlinFunction {
            name: func.name.to_owned(),
            arguments: func
                .arguments
                .iter()
                .map(|arg| KotlinArgument {
                    name: arg.name.to_owned(),
                    t: kotlin_class_name(arg.r#type.to_owned()),
                    nullable: arg.nullable,
                })
                .collect(),
            return_type: kotlin_class_name(func.return_type.to_owned()),
        })
        .collect();

    let class_package = java_class.package.clone();

    let (fields, parts) = if own_fields.len() > FIELDS_CHUNKING_LIMIT {
        let parts: Vec<Part> = own_fields
            .chunks(FIELDS_CHUNKING_LIMIT)
            .map(|chunk| Part {
                fields: chunk.to_vec(),
            })
            .collect();
        (vec![], parts)
    } else {
        (own_fields, vec![])
    };

    KotlinClass {
        kotlin_name: kotlin_class_name(java_class.name.to_owned()),
        java_name: java_class.name.to_owned(),
        package: class_package,
        imports,
        fields,
        parts,
        functions,
        enum_constants: java_class.enum_constants.clone(),
        subclasses: java_class
            .subclasses
            .iter()
            .map(|subclass| subclass.name.clone())
            .collect(),
        java_superclass: java_class
            .superclass
            .as_ref()
            .map(|superclass| KotlinSuperclass {
                name: kotlin_class_name(superclass.name.to_owned()),
                fields: convert_fields(
                    superclass.name.as_str(),
                    superclass.package.as_str(),
                    &superclass.fields,
                    Some(&java_class.package),
                ),
            }),
        inner_classes,
        is_abstract: java_class.is_abstract,
    }
}

fn kotlin_class_name(java_class_name: String) -> String {
    let re = Regex::new(r"([A-Z]+[^\.]*)").unwrap(); // Match inner class names, but not packages
    re.replace_all(&java_class_name, "${1}Kt").to_string()
}

fn write_kotlin_file(base_path: &Path, file_path: &Path) {
    trace!("Will parse {:?}\n", file_path);

    let java_class = parse_class_with_inheritance(base_path, file_path);
    if java_class.skip {
        return;
    }
    debug!("Converting {}.{}\n", java_class.package, java_class.name);

    let kotlin_class = convert(java_class);
    trace!("Convert done\n");

    let mut absolute_path = base_path.to_owned();
    absolute_path.push(file_path);
    let filename = file_path.file_stem().unwrap();
    absolute_path.set_file_name(kotlin_class_name(filename.to_str().unwrap().to_owned()));
    absolute_path.set_extension("kt");

    let file = match File::create(&absolute_path) {
        Err(why) => panic!("couldn't create {}: {}", absolute_path.display(), why),
        Ok(file) => file,
    };

    let mut f = BufWriter::new(file);

    trace!("Will write file\n");
    kotlinprinter::write_class(kotlin_class, f.get_mut());
    trace!("File written\n");
}

fn parse_class_with_inheritance(base_path: &Path, file_path: &Path) -> Class {
    let content = fs::read_to_string(file_path).unwrap();

    let mut java_class = javaparser::parse_class(&content);

    let superclass_name: Option<&str> = java_class.superclass.as_ref().map(|s| s.name.as_ref());

    if superclass_name == Some("Exception") || superclass_name == Some("Service") {
        java_class.skip = true;
    } else if superclass_name.is_some() {
        warn!("Parsing superclass {}", superclass_name.unwrap());
        let superclass = fields_from_superclass(base_path, file_path, &java_class);
        debug!("Done parsing superclass");
        let mut imports = java_class.imports;
        imports.extend(superclass.imports.clone());
        java_class.superclass = Some(superclass);
        java_class.imports = imports;
    }
    java_class
}

fn to_path(import: &JavaImport) -> PathBuf {
    let path: PathBuf = import
        .package
        .split(".")
        .map(|part| Path::new(part))
        .collect();
    path
}

fn fields_from_superclass(
    base_path: &Path,
    subclass_file_path: &Path,
    subclass: &Class,
) -> JavaSuperclass {
    let superclass_name: &str = subclass
        .superclass
        .as_ref()
        .map(|s| s.name.as_str())
        .unwrap();

    let mut superclass_path = subclass
        .imports
        .iter()
        .find(|i| i.class == superclass_name)
        .map_or(subclass_file_path.parent().unwrap().to_owned(), |i| {
            to_path(i)
        });
    superclass_path.push(superclass_name);
    superclass_path.set_extension(subclass_file_path.extension().unwrap());

    let mut full_superclass_path = base_path.to_path_buf();
    full_superclass_path.push(superclass_path);

    trace!("Superclass path: {:?}", full_superclass_path);

    let superclass = parse_class_with_inheritance(base_path, full_superclass_path.as_path());

    let superclass_superclass = superclass.superclass;

    let mut superclass_own_fields: Vec<JavaField> = superclass_superclass
        .as_ref()
        .map_or(vec![], |ss| ss.fields.clone());
    superclass_own_fields.extend(superclass.fields.clone());

    let mut superclass_own_imports = superclass_superclass
        .as_ref()
        .map_or(vec![], |ss| ss.imports.clone());
    superclass_own_imports.extend(superclass.imports);

    return JavaSuperclass {
        name: superclass.name,
        package: superclass.package,
        imports: superclass_own_imports,
        fields: superclass_own_fields,
        is_abstract: superclass.is_abstract,
    };
}

#[macro_use]
extern crate log;

fn main() {
    env_logger::init();

    let path_from_arg = std::env::args().nth(1).expect("no path given");
    info!("Starting...");

    let generated_path = Path::new(&path_from_arg);
    generate(generated_path);
    println!("Done!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use javaparser::InnerClass as JavaInnerClass;
    use javaparser::Type as JavaType;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let java_class = JavaClass {
            package: "somepackage".to_string(),
            name: "TheClass".to_string(),
            imports: vec![
                JavaImport {
                    package: "java.math".to_string(),
                    class: "BigDecimal".to_string(),
                },
                JavaImport {
                    package: "dataclasses".to_string(),
                    class: "Door".to_string(),
                },
            ],
            fields: vec![
                JavaField {
                    name: "value".to_string(),
                    r#type: JavaType {
                        package: None,
                        class: "Object".to_string(),
                        stdlib: true,
                    },
                    builtin: true,
                    nullable: true,
                    ..Default::default()
                },
                JavaField {
                    package: "somepackage".to_string(),
                    name: "entry".to_string(),
                    xml_name: None,
                    r#type: JavaType {
                        package: Some("somepackage".to_string()),
                        class: "TheClass.InnerClass".to_string(),
                        stdlib: false,
                    },
                    generic_type: Some("List".to_string()),
                    builtin: false,
                    nullable: true,
                },
            ],
            superclass: Some(JavaSuperclass {
                name: "BLevel".to_string(),
                package: "inheritance.openclasses".to_string(),
                is_abstract: false,
                ..Default::default()
            }),
            inner_classes: vec![JavaInnerClass {
                name: "InnerClass".to_string(),
                fields: vec![
                    JavaField {
                        name: "innerclassfield".to_string(),
                        ..Default::default()
                    },
                    JavaField {
                        r#type: JavaType {
                            package: Some("javax.xml.datatype".to_string()),
                            class: "XMLGregorianCalendar".to_string(),
                            stdlib: true,
                        },
                        name: "date".to_string(),
                        ..Default::default()
                    },
                ],
            }],
            ..Default::default()
        };

        let converted = convert(java_class);

        // let expected_imports = vec![KotlinImport{package: "java.math".to_string(), class: "BigDecimal".to_string() }];
        let expected = KotlinClass {
            package: "somepackage".to_string(),
            imports: vec![
                KotlinImport {
                    package: "inheritance.openclasses".to_string(),
                    class: "BLevelKt".to_string(),
                },
                KotlinImport {
                    package: "javax.xml.datatype".to_string(),
                    class: "XMLGregorianCalendar".to_string(),
                },
            ],
            java_name: "TheClass".to_string(),
            kotlin_name: "TheClassKt".to_string(),
            fields: vec![
                KotlinField::new(
                    "value",
                    "Any".to_string(),
                    "",
                    "".to_string(),
                    "",
                    "",
                    false,
                    true,
                    false,
                ),
                KotlinField::new(
                    "entry",
                    "InnerClassKt".to_string(),
                    "List",
                    "".to_string(),
                    "",
                    "",
                    true,
                    true,
                    false,
                ),
            ],
            java_superclass: Some(KotlinSuperclass {
                name: "BLevelKt".to_string(),
                fields: vec![],
            }),
            inner_classes: vec![KotlinInnerClass {
                name: "InnerClass".to_string(),
                fields: vec![
                    KotlinField::new(
                        "innerclassfield",
                        "".to_string(),
                        "",
                        "".to_string(),
                        "",
                        "",
                        true,
                        false,
                        false,
                    ),
                    KotlinField::new(
                        "date",
                        "XMLGregorianCalendar".to_string(),
                        "",
                        "".to_string(),
                        "",
                        "",
                        false,
                        false,
                        false,
                    ),
                ],
            }],
            ..Default::default()
        };

        assert_eq!(expected, converted)
    }
}
