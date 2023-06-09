use std::io::Write;

use gtmpl::{Func, FuncError};
use gtmpl_derive::Gtmpl;
use gtmpl_value::Value;

#[derive(Gtmpl, Default, Debug, PartialEq)]
pub struct KotlinClass {
    pub kotlin_name: String,
    pub java_name: String,
    pub package: String,
    pub fields: Vec<Field>,
    pub parts: Vec<Part>,
    pub java_superclass: Option<Superclass>, // TODO change field to 'superclass'
    pub functions: Vec<Function>,
    pub is_abstract: bool,
    pub subclasses: Vec<String>,
    pub imports: Vec<Import>,
    pub enum_constants: Vec<String>,
    pub inner_classes: Vec<InnerClass>,
}

#[derive(Gtmpl, Default, Debug, PartialEq, Clone)]
pub struct Part {
    pub fields: Vec<Field>,
}

#[derive(Gtmpl, Default, Debug, PartialEq)]
pub struct InnerClass {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Gtmpl, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Import {
    pub package: String,
    pub class: String,
}

#[derive(Gtmpl, Default, Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<Arg>,
    pub return_type: String,
}

#[derive(Gtmpl, Default, Debug, PartialEq)]
pub struct Arg {
    pub name: String,
    pub t: String,
    pub nullable: bool,
}

#[derive(Gtmpl, Default, Debug, PartialEq)]
pub struct Superclass {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Gtmpl)]
pub struct Field {
    pub name: String,
    pub r#type: String,
    pub generic_type: String, // TODO Option<String>, but gtmpl does not seem to support it
    pub factory_func: String,
    pub object_factory_class: String,
    pub object_factory_package: String, // TODO set correctly
    pub convert: bool,
    pub nullable: bool,
    pub use_getter: bool,
    get_type: Func,
    get_type_with_default: Func,
    to_java: Func,
    from_java: Func,
}

impl Field {
    pub fn new(
        name: &str,
        r#type: String,
        generic_type: &str,
        factory_func: String,
        object_factory_class: &str,
        object_factory_package: &str,
        convert: bool,
        nullable: bool,
        use_getter: bool,
    ) -> Field {
        Field {
            name: name.to_owned(),
            r#type,
            generic_type: generic_type.to_owned(),
            factory_func,
            object_factory_class: object_factory_class.to_owned(),
            object_factory_package: object_factory_package.to_owned(),
            convert,
            nullable,
            use_getter,
            get_type,
            get_type_with_default,
            to_java,
            from_java,
        }
    }
}

impl Clone for Field {
    fn clone(&self) -> Self {
        Field {
            name: self.name.clone(),
            r#type: self.r#type.clone(),
            generic_type: self.generic_type.clone(),
            factory_func: self.factory_func.clone(),
            object_factory_class: self.object_factory_class.clone(),
            object_factory_package: self.object_factory_package.clone(),
            convert: self.convert,
            nullable: self.nullable,
            use_getter: self.use_getter,
            get_type: self.get_type,
            get_type_with_default: self.get_type_with_default,
            to_java: self.to_java,
            from_java: self.from_java,
        }
    }
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.r#type == other.r#type
            && self.generic_type == other.generic_type
            && self.factory_func == other.factory_func
            && self.convert == other.convert
            && self.nullable == other.nullable
            && self.use_getter == other.use_getter
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Field")
            .field("name", &self.name)
            .field("type", &self.r#type)
            .field("generic_type", &self.generic_type)
            .field("factory_func", &self.factory_func)
            .field("convert", &self.convert)
            .field("nullable", &self.nullable)
            .field("use_getter", &self.use_getter)
            .finish()
    }
}

impl Default for Field {
    fn default() -> Self {
        Field {
            name: "".to_string(),
            r#type: "".to_string(),
            generic_type: "".to_string(),
            nullable: false,
            convert: false,
            factory_func: "".to_string(),
            object_factory_class: "".to_string(),
            object_factory_package: "".to_string(),
            use_getter: false,
            get_type,
            get_type_with_default,
            to_java,
            from_java,
        }
    }
}

impl Clone for Import {
    fn clone(&self) -> Self {
        Import {
            package: self.package.clone(),
            class: self.class.clone(),
        }
    }
}

impl Clone for Superclass {
    fn clone(&self) -> Self {
        Superclass {
            name: self.name.clone(),
            fields: self.fields.clone(),
        }
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Function {
            name: self.name.clone(),
            arguments: self.arguments.clone(),
            return_type: self.return_type.clone(),
        }
    }
}

impl Clone for InnerClass {
    fn clone(&self) -> Self {
        InnerClass {
            name: self.name.clone(),
            fields: self.fields.clone(),
        }
    }
}

impl Clone for Arg {
    fn clone(&self) -> Self {
        Arg {
            name: self.name.clone(),
            t: self.t.clone(),
            nullable: self.nullable.clone(),
        }
    }
}

pub fn write_class<W: Write>(kotlin_class: KotlinClass, writer: &mut W) {
    if kotlin_class.is_abstract {
        write_abstract_class(kotlin_class, writer)
    } else if !kotlin_class.subclasses.is_empty() {
        write_open_superclass(kotlin_class, writer)
    } else if kotlin_class.java_superclass.is_some() {
        write_subclass(kotlin_class, writer)
    } else if !kotlin_class.enum_constants.is_empty() {
        write_enum(kotlin_class, writer)
    } else if !kotlin_class.functions.is_empty() {
        write_interface(kotlin_class, writer)
    } else if !kotlin_class.parts.is_empty() {
        write_large_data_class(kotlin_class, writer)
    } else {
        write_data_class(kotlin_class, writer)
    }
}

fn write_data_class<W: Write>(interface: KotlinClass, writer: &mut W) {
    let output = gtmpl::template(
        "package {{.package}}
{{range .imports}}
import {{.package}}.{{.class}}
{{- end}}

/**
 * This file is GENERATED. Please don't change
 */
@Suppress(\"unused\", \"useless_cast\")
{{if .fields}}data {{end}}class {{.kotlin_name}}(
	{{- range .fields}}
    val {{.name}}: {{ .get_type_with_default }},
	{{- end}}
) {

    fun toJava(): {{.java_name}} = {{.java_name}}().also {
			{{- range .fields}}
        it.{{ .to_java }}
			{{- end}}
    }

    companion object {
        internal val factory = ObjectFactory()

        fun fromJava(javaClass: {{.java_name}}): {{.kotlin_name}} = {{.kotlin_name}}(
			{{- range .fields}}
            {{.name}} = javaClass.{{ .from_java }},
			{{- end}}
        )

    }
{{- $main_class_java_name:=.java_name}}
{{- range .inner_classes}}

    data class {{.name}}Kt(
        {{- range .fields}}
        val {{.name}}: {{ .get_type_with_default }},
        {{- end}}
    ) {

        fun toJava(): {{$main_class_java_name}}.{{.name}} = {{$main_class_java_name}}.{{.name}}().also {
                {{- range .fields}}
            it.{{ .to_java }}
                {{- end}}
        }

        companion object {
            internal val factory = ObjectFactory()

            fun fromJava(javaClass: {{$main_class_java_name}}.{{.name}}): {{.name}}Kt = {{.name}}Kt(
                {{- range .fields}}
                {{.name}} = javaClass.{{ .from_java }},
                {{- end}}
            )

        }
    }
    {{- end}}
}",
        interface,
    );
    writer.write_all(output.unwrap().as_bytes()).unwrap();
}

fn write_large_data_class<W: Write>(interface: KotlinClass, writer: &mut W) {
    let output = gtmpl::template(
        "package {{.package}}
{{range .imports}}
import {{.package}}.{{.class}}
{{- end}}

{{- $kotlin_name := .kotlin_name }}
{{ range $index, $part := .parts }}
private sealed interface {{$kotlin_name}}Part{{$index}} {
    {{- range .fields}}
    val {{.name}}: {{ .get_type }}
    {{- end}}
}
{{- end}}

/**
 * This file is GENERATED. Please don't change
 */
@Suppress(\"unused\", \"useless_cast\")
data class {{.kotlin_name}} private constructor(
{{- range $index, $part := .parts}}
    private val part{{$index}}: Part{{$index}},
{{- end}}
): {{ range $index, $part := .parts}}{{ if $index}}, {{end}}{{$kotlin_name}}Part{{$index}} by part{{$index}}{{end}} {

    constructor(
        {{- range .parts}}
        {{- range .fields}}
        {{.name}}: {{ .get_type_with_default }},
        {{- end}}
        {{- end}}
    ) : this(
{{- range $index, $part := .parts}}
        Part{{$index}}(
        {{- range $part.fields}}
            {{.name}},
        {{- end}}
        ),
{{- end}}
    )

    fun toJava(): {{.java_name}} = {{.java_name}}().apply {
    {{- range $index, $part := .parts}}
        part{{$index}}.toJava(this)
    {{- end}}
    }

    fun copy(
        {{- range .parts}}
        {{- range .fields}}
        {{.name}}: {{ .get_type }} = this.{{.name}},
        {{- end}}
        {{- end}}
    ) = {{.kotlin_name}}(
        {{- range .parts}}
        {{- range .fields}}
        {{.name}},
        {{- end}}
        {{- end}}
    )

    companion object {
        internal val factory = ObjectFactory()

        fun fromJava(javaClass: {{.java_name}}): {{.kotlin_name}} = {{.kotlin_name}}(
            {{- range $index, $part := .parts}}
            Part{{$index}}.fromJava(javaClass),
            {{- end}}
        )
    }

{{- $java_name := .java_name}}
{{- $kotlin_name := .kotlin_name}}
{{- range $index, $part := .parts }}

    private data class Part{{$index}}(
        {{- range $part.fields}}
        override val {{.name}}: {{ .get_type_with_default }},
        {{- end}}
    ): {{$kotlin_name}}Part{{$index}} {

        fun toJava(javaClass: {{$java_name}}): {{$java_name}} = javaClass.also {
                {{- range $part.fields}}
            it.{{ .to_java }}
                {{- end}}
        }

        companion object {
            internal val factory = ObjectFactory()

            fun fromJava(javaClass: {{$java_name}}): Part{{$index}} = Part{{$index}}(
                {{- range $part.fields}}
                {{.name}} = javaClass.{{ .from_java }},
                {{- end}}
            )
        }
    }
{{- end}}
}",
        interface,
    );
    writer.write_all(output.unwrap().as_bytes()).unwrap();
}

fn write_interface<W: Write>(interface: KotlinClass, writer: &mut W) {
    let output = gtmpl::template(
        "package {{.package}}
{{range .imports}}
import {{.package}}.{{.class}}
{{- end}}

/**
 * This file is GENERATED. Please don't change
 */
 class {{.kotlin_name}}(val portType: {{.java_name}}) {
		{{- range .functions}}
    fun {{.name}}(
		{{- range .arguments}}
		{{ .name}}: {{.t}}{{if .nullable}}? = null{{end}},
		{{- end }}
	) = {{.return_type}}.fromJava(portType.{{.name}}(
		{{- range .arguments}}
		{{ .name}}{{if .nullable}}?{{end}}.toJava(),
		{{- end }}
	))
		{{end}}
}
",
        interface,
    );
    writer.write_all(output.unwrap().as_bytes()).unwrap();
}

fn write_enum<W: Write>(kotlin_enum: KotlinClass, writer: &mut W) {
    let output = gtmpl::template(
        "package {{.package}}

{{range .imports}}
import {{.package}}.{{.class}}
{{- end}}
/**
 * This file is GENERATED. Please don't change
 */
enum class {{.kotlin_name}}(val value: {{.java_name}}) {
	{{- $javaname := .java_name}}
		{{- range .enum_constants}}
    {{.}}({{$javaname}}.{{.}}),
		{{- end}}
    ;

	fun toJava(): {{.java_name}} = {{.java_name}}.valueOf(this.name)

    companion object {
        fun fromJava(value: {{.java_name}}) = values().find { it.value == value }!!
    }
}",
        kotlin_enum,
    );

    writer.write_all(output.unwrap().as_bytes()).unwrap();
}

fn write_abstract_class<W: Write>(kotlin_enum: KotlinClass, writer: &mut W) {
    // gtmpl_fn!(
    // fn add(a: u64, b: u64) -> Result<u64, FuncError> {
    //     Ok(a + b)
    // });

    let output = gtmpl::template(
        "package {{.package}}
{{ range .imports}}
import {{.package}}.{{.class}}
{{- end}}

/**
 * This file is GENERATED. Please don't change
 */
interface {{.java_name}}Kt {{if .java_superclass}}: {{.java_superclass.name}} {{end}}{

	{{- range .fields}}
    val {{.name}}: {{ .get_type }}
	{{- end}}

    {{if .java_superclass}}override {{end}}fun toJava(): {{.java_name}}

    companion object {

        internal val factory = ObjectFactory()

        fun fromJava(javaClass: {{.java_name}}): {{.java_name}}Kt = when (javaClass) {
			{{- range .subclasses}}
            is {{.}} -> {{.}}Kt.fromJava(javaClass)
			{{- end}}
            else -> throw IllegalStateException(\"Not able to find implementation for class '${javaClass.javaClass.name}'\")
        }

    }
{{- $main_class_java_name:=.java_name}}
{{- range .inner_classes}}

    data class {{.name}}Kt(
        {{- range .fields}}
        val {{.name}}: {{ .get_type_with_default }},
        {{- end}}
    ) {

        fun toJava(): {{$main_class_java_name}}.{{.name}} = {{$main_class_java_name}}.{{.name}}().also {
                {{- range .fields}}
            it.{{ .to_java }}
                {{- end}}
        }

        companion object {
            internal val factory = ObjectFactory()

            fun fromJava(javaClass: {{$main_class_java_name}}.{{.name}}): {{.name}}Kt = {{.name}}Kt(
                {{- range .fields}}
                {{.name}} = javaClass.{{ .from_java }},
                {{- end}}
            )

        }
    }
    {{- end}}
}",
        kotlin_enum,
    );

    writer.write_all(output.unwrap().as_bytes()).unwrap();
}

fn write_open_superclass<W: Write>(kotlin_enum: KotlinClass, writer: &mut W) {
    let output = gtmpl::template(
        "package {{.package}}
{{ range .imports}}
import {{.package}}.{{.class}}
{{- end}}

/**
 * This file is GENERATED. Please don't change
 */
interface {{.java_name}}Kt {{if .java_superclass}}: {{.java_superclass.name}} {{end}}{

	{{- range .fields}}
    val {{.name}}: {{ .get_type }}
	{{- end}}

    {{if .java_superclass}}override {{end}}fun toJava(): {{.java_name}} = {{.java_name}}()
        .also {
			{{- if .java_superclass}}
			{{- range .java_superclass.fields}}
            it.{{ .to_java }}
			{{- end}}
			{{- end}}
			{{- range .fields}}
            it.{{ .to_java }}
			{{- end}}
        }

    companion object {

        internal val factory = ObjectFactory()

        fun fromJava(javaClass: {{.java_name}}): {{.java_name}}Kt = when (javaClass) {
			{{- range .subclasses}}
            is {{.}} -> {{.}}Kt.fromJava(javaClass)
			{{- end}}
            else -> {{.java_name}}ImplKt(
			    {{- if .java_superclass}}
				{{- range .java_superclass.fields}}
                {{.name}} = javaClass.{{ .from_java }},
				{{- end}}
				{{- end}}
				{{- range .fields}}
                {{.name}} = javaClass.{{ .from_java }},
				{{- end}}
            )
        }

    }
}

@Suppress(\"unused\", \"useless_cast\")
{{if .fields}}data {{end}}class {{.java_name}}ImplKt(
	{{- if .java_superclass}}
	{{- range .java_superclass.fields}}
    override val {{.name}}: {{ .get_type_with_default }},
	{{- end}}
	{{- end}}
	{{- range .fields}}
    override val {{.name}}: {{ .get_type_with_default }},
	{{- end}}
) : {{.java_name}}Kt",
        kotlin_enum,
    );

    writer.write_all(output.unwrap().as_bytes()).unwrap();
}

fn write_subclass<W: Write>(kotlin_enum: KotlinClass, writer: &mut W) {
    let output = gtmpl::template(
        "package {{.package}}
{{ range .imports}}
import {{.package}}.{{.class}}
{{- end}}

/**
 * This file is GENERATED. Please don't change
 */
@Suppress(\"unused\", \"useless_cast\")
data class {{.kotlin_name}}(
	{{- range .java_superclass.fields}}
    override val {{.name}}: {{ .get_type_with_default }},
	{{- end}}
	{{- range .fields}}
    val {{.name}}: {{ .get_type_with_default }},
	{{- end}}
) : {{.java_superclass.name}} {

    override fun toJava(): {{.java_name}} = {{.java_name}}()
        .also {
			{{- range .java_superclass.fields}}
            it.{{ .to_java }}
			{{- end}}
			{{- range .fields}}
            it.{{ .to_java }}
			{{- end}}
        }

    companion object {

        internal val factory = ObjectFactory()

        fun fromJava(javaClass: {{.java_name}}): {{.kotlin_name}} = {{.kotlin_name}}(
			{{- range .java_superclass.fields}}
            {{.name}} = javaClass.{{ .from_java }},
			{{- end}}
			{{- range .fields}}
            {{.name}} = javaClass.{{ .from_java }},
			{{- end}}
        )

    }
}",
        kotlin_enum,
    );

    writer.write_all(output.unwrap().as_bytes()).unwrap();
}

fn to_java(args: &[Value]) -> Result<Value, FuncError> {
    let field = to_field(args);

    if field.generic_type == "JAXBElement" {
        if field.convert {
            return Ok(format!(
                "{n} = {n}?.let {{ elem -> {}.factory.{}(elem.toJava()) }}",
                field.object_factory_class,
                field.factory_func,
                n = field.name
            )
            .into());
        } else {
            return Ok(format!(
                "{n} = {}.factory.{}({n})",
                field.object_factory_class,
                field.factory_func,
                n = field.name
            )
            .into());
        }
    } else if field.convert {
        if field.generic_type != "" {
            if field.use_getter {
                return Ok(format!(
                    "{n}.addAll({n}.map {{ elem -> elem.toJava() }})",
                    n = field.name
                )
                .into());
            } else {
                return Ok(
                    format!("{n} = {n}.map {{ elem -> elem.toJava() }}", n = field.name).into(),
                );
            }
        } else {
            if field.nullable {
                return Ok(format!("{n} = {n}?.toJava()", n = field.name).into());
            } else {
                return Ok(format!("{n} = {n}.toJava()", n = field.name).into());
            }
        }
    } else {
        if field.use_getter && field.generic_type != "" {
            return Ok(format!("{n}.addAll({n})", n = field.name).into());
        } else if field.use_getter && field.generic_type == "" && field.r#type == "Boolean" {
            return Ok(format!("{} = {n}", boolean_getter(&field.name), n = field.name).into());
        } else {
            return Ok(format!("{n} = {}", n = field.name).into());
        }
    }
}

fn boolean_getter(variable_name: &str) -> String {
    format!(
        "is{}{}",
        (&variable_name[..1].to_string()).to_uppercase(),
        &variable_name[1..]
    )
}

fn from_java(args: &[Value]) -> Result<Value, FuncError> {
    let field = to_field(args);

    if field.generic_type == "JAXBElement" {
        if field.convert {
            return Ok(format!(
                "{}?.value?.let {{ {t}.fromJava(it) as {t} }}",
                field.name,
                t = field.r#type
            )
            .into());
        } else {
            return Ok(format!("{}?.value", field.name).into());
        }
    } else if field.convert {
        if field.generic_type != "" {
            return Ok(format!(
                "{}?.map {{ {t}.fromJava(it) as {t} }} ?: empty{}()",
                field.name,
                field.generic_type,
                t = field.r#type
            )
            .into());
        } else {
            if field.nullable {
                return Ok(format!(
                    "{}?.let {{ {t}.fromJava(it) as {t} }}",
                    field.name,
                    t = field.r#type
                )
                .into());
            } else {
                return Ok(format!(
                    "{}.let {{ {t}.fromJava(it) as {t} }}",
                    field.name,
                    t = field.r#type
                )
                .into());
            }
        }
    } else {
        if field.generic_type != "" && field.nullable && !field.use_getter {
            return Ok(format!("{} ?: empty{}()", field.name, field.generic_type,).into());
        } else if field.use_getter && field.generic_type == "" && field.r#type == "Boolean" {
            return Ok(format!("{}", boolean_getter(&field.name)).into());
        } else {
            return Ok(field.name.into());
        }
    }
}

fn get_type(args: &[Value]) -> Result<Value, FuncError> {
    let field = to_field(args);
    if field.generic_type == "JAXBElement" || (field.nullable && field.generic_type == "") {
        return Ok(format!("{}?", field.r#type).into());
    } else if field.generic_type != "" {
        return Ok(format!("{}<{}>", field.generic_type, field.r#type).into());
    }
    return Ok(field.r#type.into());
}

fn get_type_with_default(args: &[Value]) -> Result<Value, FuncError> {
    let field = to_field(args);
    if field.generic_type == "JAXBElement" || (field.nullable && field.generic_type == "") {
        return Ok(format!("{}? = null", field.r#type).into());
    } else if field.generic_type != "" {
        return Ok(format!(
            "{generic}<{}> = empty{generic}()",
            field.r#type,
            generic = field.generic_type
        )
        .into());
    }
    return Ok(field.r#type.into());
}

fn to_field(args: &[Value]) -> Field {
    if let Value::Object(ref field) = &args[0] {
        if let Some(Value::String(ref name)) = field.get("name") {
            if let Some(Value::String(ref t)) = field.get("r#type") {
                if let Some(Value::Bool(ref nullable)) = field.get("nullable") {
                    if let Some(Value::String(ref generic_type)) = field.get("generic_type") {
                        if let Some(Value::Bool(ref convert)) = field.get("convert") {
                            if let Some(Value::Bool(ref use_getter)) = field.get("use_getter") {
                                if let Some(Value::String(ref object_factory_class)) =
                                    field.get("object_factory_class")
                                {
                                    if let Some(Value::String(ref object_factory_package)) =
                                        field.get("object_factory_package")
                                    {
                                        if let Some(Value::String(ref factory_func)) =
                                            field.get("factory_func")
                                        {
                                            return Field {
                                                name: name.to_string(),
                                                r#type: t.to_string(),
                                                generic_type: generic_type.to_string(),
                                                convert: *convert,
                                                nullable: *nullable,
                                                factory_func: factory_func.to_string(),
                                                object_factory_class: object_factory_class
                                                    .to_string(),
                                                object_factory_package: object_factory_package
                                                    .to_string(),
                                                use_getter: *use_getter,
                                                get_type,
                                                get_type_with_default,
                                                to_java,
                                                from_java,
                                            };
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // TODO
    Field {
        name: "ERRRR".to_string(),
        r#type: "ERRRR".to_string(),
        generic_type: "ERR".to_string(),
        convert: false,
        factory_func: "".to_string(),
        object_factory_class: "".to_string(),
        object_factory_package: "".to_string(),
        nullable: false,
        use_getter: false,
        get_type,
        get_type_with_default,
        to_java,
        from_java,
    }
    // Err(anyhow!("integer required, got: {:?}", args))
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::Cursor,
        path::{Path, PathBuf},
    };

    use pretty_assertions::assert_eq;

    use super::*;

    fn read_file(path: &Path) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test-resources/src/main/kotlin/");
        d.push(path);

        fs::read_to_string(d).unwrap()
        // println!("{}", d.display());
    }

    #[test]
    fn test_write_interface() {
        let class = KotlinClass {
            package: "mypack".to_string(),
            kotlin_name: "MyClass.kt".to_string(),
            java_name: "MyClass".to_string(),
            functions: vec![Function {
                name: "getCar".to_string(),
                arguments: vec![
                    Arg {
                        name: "carBrand".to_string(),
                        t: "CarBrandKt".to_string(),
                        nullable: false,
                    },
                    Arg {
                        name: "carBrand".to_string(),
                        t: "CarBrandKt".to_string(),
                        nullable: false,
                    },
                ],
                return_type: "CarKt".to_string(),
            }],
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_interface(class, &mut buffer);

        assert_eq!(
            "package mypack\n\n\n/**\n * This file is GENERATED. Please don't change\n */\n class MyClass.kt(val portType: MyClass) {\n    fun getCar(\n\t\tcarBrand: CarBrandKt,\n\t\tcarBrand: CarBrandKt,\n\t) = CarKt.fromJava(portType.getCar(\n\t\tcarBrand.toJava(),\n\t\tcarBrand.toJava(),\n\t))\n\t\t\n}\n",
            String::from_utf8_lossy(&buffer.get_ref())
        )
    }

    #[test]
    fn test_write_enum() {
        let class = KotlinClass {
            kotlin_name: "CarBrandKt".to_string(),
            java_name: "CarBrand".to_string(),
            enum_constants: vec!["VW".to_string(), "BMW".to_string()],
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_enum(class, &mut buffer);

        assert_eq!(
            "package \n\n\n/**\n * This file is GENERATED. Please don't change\n */\nenum class CarBrandKt(val value: CarBrand) {\n    VW(CarBrand.VW),\n    BMW(CarBrand.BMW),\n    ;\n\n\tfun toJava(): CarBrand = CarBrand.valueOf(this.name)\n\n    companion object {\n        fun fromJava(value: CarBrand) = values().find { it.value == value }!!\n    }\n}",
            String::from_utf8_lossy(&buffer.get_ref())
        )
    }

    #[test]
    fn test_abstract_class_a_level() {
        let expected = read_file(Path::new("inheritance/abstractclasses/ALevelKt.kt"));
        // let expected = read_file(Path::new("testfile"));

        let class = KotlinClass {
            package: "inheritance.abstractclasses".to_string(),
            is_abstract: true,
            fields: vec![a_field()],
            kotlin_name: "ALevelKt".to_string(),
            java_name: "ALevel".to_string(),
            subclasses: vec!["BLevel".to_string()],
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_abstract_class(class, &mut buffer);

        assert_eq!(String::from_utf8_lossy(&buffer.get_ref()), expected)
    }

    fn a_field() -> Field {
        Field {
            name: "a".to_string(),
            r#type: "Int".to_string(),
            ..Default::default()
        }
    }

    fn aa_field() -> Field {
        Field {
            name: "aa".to_string(),
            r#type: "WindowKt".to_string(),
            generic_type: "JAXBElement".to_string(),
            nullable: false,
            convert: true,
            factory_func: "createAa".to_string(),
            object_factory_class: "ALevelKt".to_string(),
            ..Default::default()
        }
    }

    fn b_field() -> Field {
        Field {
            name: "b".to_string(),
            r#type: "Int".to_string(),
            ..Default::default()
        }
    }

    fn bb_field() -> Field {
        Field {
            name: "bb".to_string(),
            r#type: "BigDecimal".to_string(),
            nullable: true,
            ..Default::default()
        }
    }

    fn bbb_field() -> Field {
        Field {
            name: "bbb".to_string(),
            r#type: "DoorKt".to_string(),
            generic_type: "".to_string(),
            convert: true,
            ..Default::default()
        }
    }

    fn bbbb_field(use_getter: bool) -> Field {
        Field {
            name: "bbbb".to_string(),
            r#type: "String".to_string(),
            generic_type: "List".to_string(),
            nullable: true,
            use_getter,
            ..Default::default()
        }
    }

    fn bbbbb_field(use_getter: bool) -> Field {
        Field {
            name: "bbbbb".to_string(),
            r#type: "Boolean".to_string(),
            nullable: false,
            use_getter,
            ..Default::default()
        }
    }

    fn c_field() -> Field {
        Field {
            name: "c".to_string(),
            r#type: "Int".to_string(),
            generic_type: "".to_string(),
            ..Default::default()
        }
    }

    fn bd_import() -> Import {
        Import {
            package: "java.math".to_string(),
            class: "BigDecimal".to_string(),
        }
    }

    fn doorkt_import() -> Import {
        Import {
            package: "dataclasses".to_string(),
            class: "DoorKt".to_string(),
        }
    }

    fn windowkt_import() -> Import {
        Import {
            package: "inheritance.openclasses".to_string(),
            class: "WindowKt".to_string(),
        }
    }

    fn alevelkt_import() -> Import {
        Import {
            package: "inheritance.openclasses".to_string(),
            class: "ALevelKt".to_string(),
        }
    }

    fn blevelkt_import() -> Import {
        Import {
            package: "inheritance.openclasses".to_string(),
            class: "BLevelKt".to_string(),
        }
    }

    fn clevel2_import() -> Import {
        Import {
            package: "inheritance.otherpackage".to_string(),
            class: "CLevel2".to_string(),
        }
    }

    fn clevel2kt_import() -> Import {
        Import {
            package: "inheritance.otherpackage".to_string(),
            class: "CLevel2Kt".to_string(),
        }
    }

    #[test]
    fn test_abstract_class_b_level() {
        let expected = read_file(Path::new("inheritance/abstractclasses/BLevelKt.kt"));

        let class = KotlinClass {
            package: "inheritance.abstractclasses".to_string(),
            kotlin_name: "BLevelKt".to_string(),
            java_name: "BLevel".to_string(),
            fields: vec![b_field(), bb_field()],
            imports: vec![bd_import()],
            java_superclass: Some(Superclass {
                name: "ALevelKt".to_string(),
                fields: vec![a_field()],
            }),
            is_abstract: true,
            subclasses: vec!["CLevel".to_string()],
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_class(class, &mut buffer);

        assert_eq!(String::from_utf8_lossy(&buffer.get_ref()), expected)
    }

    #[test]
    fn test_subclass_implementing_abstract_class() {
        let expected = read_file(Path::new("inheritance/abstractclasses/CLevelKt.kt"));

        let class = KotlinClass {
            package: "inheritance.abstractclasses".to_string(),
            kotlin_name: "CLevelKt".to_string(),
            java_name: "CLevel".to_string(),
            fields: vec![c_field()],
            imports: vec![bd_import()],
            java_superclass: Some(Superclass {
                name: "BLevelKt".to_string(),
                fields: vec![a_field(), b_field(), bb_field()],
            }),
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_class(class, &mut buffer);

        assert_eq!(String::from_utf8_lossy(&buffer.get_ref()), expected)
    }

    #[test]
    fn test_open_superclass_a_level() {
        let expected = read_file(Path::new("inheritance/openclasses/ALevel.kt"));

        let class = KotlinClass {
            package: "inheritance.openclasses".to_string(),
            kotlin_name: "ALevelKt".to_string(),
            java_name: "ALevel".to_string(),
            fields: vec![a_field(), aa_field()],
            subclasses: vec!["BLevel".to_string()],
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_class(class, &mut buffer);

        assert_eq!(String::from_utf8_lossy(&buffer.get_ref()), expected)
    }

    #[test]
    fn test_open_empty_superclass_a_level() {
        let expected = read_file(Path::new("inheritance/openclasses/ALevelEmpty.kt"));

        let class = KotlinClass {
            package: "inheritance.openclasses".to_string(),
            kotlin_name: "ALevelEmptyKt".to_string(),
            java_name: "ALevelEmpty".to_string(),
            fields: vec![],
            subclasses: vec!["BLevelEmpty".to_string()],
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_class(class, &mut buffer);

        assert_eq!(String::from_utf8_lossy(&buffer.get_ref()), expected)
    }

    #[test]
    fn test_open_superclass_b_level() {
        let expected = read_file(Path::new("inheritance/openclasses/BLevel.kt"));

        let class = KotlinClass {
            package: "inheritance.openclasses".to_string(),
            kotlin_name: "BLevelKt".to_string(),
            java_name: "BLevel".to_string(),
            fields: vec![
                b_field(),
                bb_field(),
                bbb_field(),
                bbbb_field(false),
                bbbbb_field(false),
            ],
            java_superclass: Some(Superclass {
                name: "ALevelKt".to_string(),
                fields: vec![a_field(), aa_field()],
            }),
            imports: vec![
                doorkt_import(),
                clevel2_import(),
                clevel2kt_import(),
                bd_import(),
            ],
            subclasses: vec!["CLevel".to_string(), "CLevel2".to_string()],
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_class(class, &mut buffer);

        assert_eq!(String::from_utf8_lossy(&buffer.get_ref()), expected)
    }

    #[test]
    fn test_subclass_implementing_open_class() {
        let expected = read_file(Path::new("inheritance/openclasses/CLevelKt.kt"));

        let class = KotlinClass {
            package: "inheritance.openclasses".to_string(),
            kotlin_name: "CLevelKt".to_string(),
            java_name: "CLevel".to_string(),
            fields: vec![c_field()],
            java_superclass: Some(Superclass {
                name: "BLevelKt".to_string(),
                fields: vec![
                    a_field(),
                    aa_field(),
                    b_field(),
                    bb_field(),
                    bbb_field(),
                    bbbb_field(false),
                    bbbbb_field(false),
                ],
            }),
            imports: vec![doorkt_import(), bd_import()],
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_class(class, &mut buffer);

        assert_eq!(String::from_utf8_lossy(&buffer.get_ref()), expected)
    }

    #[test]
    fn test_subclass_implementing_open_class_in_another_package() {
        let expected = read_file(Path::new("inheritance/otherpackage/CLevel2Kt.kt"));

        let class = KotlinClass {
            package: "inheritance.otherpackage".to_string(),
            kotlin_name: "CLevel2Kt".to_string(),
            java_name: "CLevel2".to_string(),
            fields: vec![c_field()],
            java_superclass: Some(Superclass {
                name: "BLevelKt".to_string(),
                fields: vec![
                    a_field(),
                    aa_field(),
                    b_field(),
                    bb_field(),
                    bbb_field(),
                    bbbb_field(true),
                    bbbbb_field(true),
                ],
            }),
            imports: vec![
                doorkt_import(),
                blevelkt_import(),
                windowkt_import(),
                bd_import(),
                alevelkt_import(),
            ],
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_class(class, &mut buffer);

        assert_eq!(String::from_utf8_lossy(&buffer.get_ref()), expected)
    }

    #[test]
    fn test_class_with_b_level() {
        let expected = read_file(Path::new("inheritance/openclasses/ClassWithBLevelKt.kt"));

        let class = KotlinClass {
            package: "inheritance.openclasses".to_string(),
            kotlin_name: "ClassWithBLevelKt".to_string(),
            java_name: "ClassWithBLevel".to_string(),
            fields: vec![Field {
                name: "bLevel".to_string(),
                r#type: "BLevelKt".to_string(),
                generic_type: "List".to_string(),
                convert: true,
                ..Default::default()
            }],
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_class(class, &mut buffer);

        assert_eq!(String::from_utf8_lossy(&buffer.get_ref()), expected)
    }

    #[test]
    fn test_data_class() {
        let expected = read_file(Path::new("dataclasses/CarKt.kt"));

        let class = KotlinClass {
            package: "dataclasses".to_string(),
            kotlin_name: "CarKt".to_string(),
            java_name: "Car".to_string(),
            fields: vec![
                Field {
                    name: "requiredInteger".to_string(),
                    r#type: "Int".to_string(),
                    ..Default::default()
                },
                Field {
                    name: "listOfInternalClasses".to_string(),
                    r#type: "DoorKt".to_string(),
                    generic_type: "List".to_string(),
                    convert: true,
                    ..Default::default()
                },
                Field {
                    name: "nullableListOfInternalClasses".to_string(),
                    r#type: "DoorKt".to_string(),
                    generic_type: "List".to_string(),
                    convert: true,
                    nullable: true,
                    ..Default::default()
                },
                Field {
                    name: "stringJAXBElement".to_string(),
                    r#type: "String".to_string(),
                    generic_type: "JAXBElement".to_string(),
                    factory_func: "createStringJAXBElement".to_string(),
                    object_factory_class: "CarKt".to_string(),
                    convert: false,
                    nullable: true,
                    ..Default::default()
                },
                Field {
                    name: "internalClassJAXBElement".to_string(),
                    r#type: "DoorKt".to_string(),
                    generic_type: "JAXBElement".to_string(),
                    factory_func: "createInternalClassJAXBElement".to_string(),
                    object_factory_class: "CarKt".to_string(),
                    convert: true,
                    nullable: true,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_class(class, &mut buffer);

        assert_eq!(String::from_utf8_lossy(&buffer.get_ref()), expected)
    }

    #[test]
    fn test_large_data_class() {
        let expected = read_file(Path::new("dataclasses/CarKtParted.kt"));

        let class = KotlinClass {
            package: "dataclasses".to_string(),
            kotlin_name: "CarKtParted".to_string(),
            java_name: "Car".to_string(),
            parts: vec![
                Part {
                    fields: vec![
                        Field {
                            name: "requiredInteger".to_string(),
                            r#type: "Int".to_string(),
                            ..Default::default()
                        },
                        Field {
                            name: "listOfInternalClasses".to_string(),
                            r#type: "DoorKt".to_string(),
                            generic_type: "List".to_string(),
                            convert: true,
                            ..Default::default()
                        },
                    ],
                },
                Part {
                    fields: vec![
                        Field {
                            name: "nullableListOfInternalClasses".to_string(),
                            r#type: "DoorKt".to_string(),
                            generic_type: "List".to_string(),
                            convert: true,
                            nullable: true,
                            ..Default::default()
                        },
                        Field {
                            name: "stringJAXBElement".to_string(),
                            r#type: "String".to_string(),
                            generic_type: "JAXBElement".to_string(),
                            factory_func: "createStringJAXBElement".to_string(),
                            object_factory_class: "CarKt".to_string(),
                            convert: false,
                            nullable: true,
                            ..Default::default()
                        },
                    ],
                },
                Part {
                    fields: vec![Field {
                        name: "internalClassJAXBElement".to_string(),
                        r#type: "DoorKt".to_string(),
                        generic_type: "JAXBElement".to_string(),
                        factory_func: "createInternalClassJAXBElement".to_string(),
                        object_factory_class: "CarKt".to_string(),
                        convert: true,
                        nullable: true,
                        ..Default::default()
                    }],
                },
            ],
            fields: vec![],
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_class(class, &mut buffer);

        let res = String::from_utf8_lossy(&buffer.get_ref());
        println!("{}", res);

        assert_eq!(res, expected)
    }

    #[test]
    fn test_class_with_inner_class() {
        let expected = read_file(Path::new("dataclasses/CxfMapKt.kt"));

        let class = KotlinClass {
            package: "dataclasses".to_string(),
            kotlin_name: "CxfMapKt".to_string(),
            java_name: "CxfMap".to_string(),
            fields: vec![Field {
                name: "entry".to_string(),
                r#type: "EntryKt".to_string(),
                generic_type: "List".to_string(),
                convert: true,
                ..Default::default()
            }],
            inner_classes: vec![InnerClass {
                name: "Entry".to_string(),
                fields: vec![
                    Field {
                        name: "key".to_string(),
                        r#type: "String".to_string(),
                        convert: false,
                        nullable: true,
                        ..Default::default()
                    },
                    Field {
                        name: "value".to_string(),
                        r#type: "DoorKt".to_string(),
                        convert: true,
                        nullable: true,
                        ..Default::default()
                    },
                ],
            }],
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_class(class, &mut buffer);

        assert_eq!(String::from_utf8_lossy(&buffer.get_ref()), expected)
    }

    #[test]
    fn test_superclass_with_inner_class() {
        let expected = read_file(Path::new(
            "inheritance/abstractclasses/ALevelWithInnerKt.kt",
        ));

        let class = KotlinClass {
            package: "inheritance.abstractclasses".to_string(),
            kotlin_name: "ALevelWithInnerKt".to_string(),
            java_name: "ALevelWithInner".to_string(),
            is_abstract: true,
            fields: vec![
                Field {
                    name: "a".to_string(),
                    r#type: "Int".to_string(),
                    generic_type: "".to_string(),
                    convert: false,
                    ..Default::default()
                },
                Field {
                    name: "inner".to_string(),
                    r#type: "InnerKt".to_string(),
                    generic_type: "".to_string(),
                    convert: true,
                    ..Default::default()
                },
            ],
            subclasses: vec!["BLevelWithInner".to_string()],
            inner_classes: vec![InnerClass {
                name: "Inner".to_string(),
                fields: vec![Field {
                    name: "ia".to_string(),
                    r#type: "String".to_string(),
                    convert: false,
                    nullable: true,
                    ..Default::default()
                }],
            }],
            ..Default::default()
        };

        let mut buffer = Cursor::new(Vec::new());

        write_class(class, &mut buffer);

        assert_eq!(String::from_utf8_lossy(&buffer.get_ref()), expected)
    }
}
