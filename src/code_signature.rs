use core::panic;

use leetcoderustapi::ProgrammingLanguage;
use warp::test;

#[derive(Debug, Clone)]
pub struct CodeSignature {
    pub function_name:  String,
    pub class_name:     Option<String>,
    pub is_class_based: bool,
    pub parameters:     Vec<String>,
    pub return_type:    Option<String>,
    pub methods:        Vec<String>,
}

impl CodeSignature {
    pub fn new_function(name: String) -> Self {
        Self {
            function_name:  name,
            class_name:     None,
            is_class_based: false,
            parameters:     Vec::new(),
            return_type:    None,
            methods:        Vec::new(),
        }
    }

    pub fn new_class(class_name: String, method_name: String) -> Self {
        Self {
            function_name:  method_name,
            class_name:     Some(class_name),
            is_class_based: true,
            parameters:     Vec::new(),
            return_type:    None,
            methods:        Vec::new(),
        }
    }

    pub fn parse_code_signature(
        lang: &ProgrammingLanguage, starter_code: &str,
    ) -> Result<CodeSignature, String> {
        match lang {
            ProgrammingLanguage::Python => {
                Self::parse_python_signature(&starter_code)
            },
            ProgrammingLanguage::Rust => {
                Self::parse_rust_signature(&starter_code)
            },
            _ => Err("Unsupported language".to_string()),
        }
    }

    fn parse_python_signature(
        starter_code: &str,
    ) -> Result<CodeSignature, String> {
        if let Some(class_start) = starter_code.find("class ") {
            let class_end = starter_code[class_start..].find(':').unwrap_or(0)
                + class_start;
            let class_line = &starter_code[class_start..class_end];

            if let Some(class_name) =
                class_line.strip_prefix("class ").map(|s| s.trim())
            {
                // Look for the main method (usually not __init__)
                if let Some(def_start) = starter_code.find("def ") {
                    let def_end =
                        starter_code[def_start..].find('(').unwrap_or(0)
                            + def_start;
                    let method_name =
                        starter_code[def_start + 4..def_end].trim();

                    if method_name != "__init__" {
                        return Ok(CodeSignature::new_class(
                            class_name.to_string(),
                            method_name.to_string(),
                        ));
                    }
                }
            }
        }
        if let Some(start) = starter_code.find("def ") {
            let end = starter_code[start..].find('(').unwrap_or(0) + start;
            let fn_name = starter_code[start + 4..end].trim().to_string();
            return Ok(CodeSignature::new_function(fn_name));
        }

        Err("No function or class definition found in Python code".to_string())
    }

    fn parse_rust_signature(
        starter_code: &str,
    ) -> Result<CodeSignature, String> {
        if let Some(impl_start) = starter_code.find("impl ") {
            let impl_line_end =
                starter_code[impl_start..].find('{').unwrap_or(0) + impl_start;
            let impl_line = &starter_code[impl_start..impl_line_end];

            if let Some(struct_name) =
                impl_line.strip_prefix("impl ").map(|s| s.trim())
            {
                if let Some(fn_start) = starter_code.find("pub fn ") {
                    let fn_end =
                        starter_code[fn_start..].find('(').unwrap_or(0)
                            + fn_start;
                    let method_name = starter_code[fn_start + 7..fn_end].trim();
                    return Ok(CodeSignature::new_class(
                        struct_name.to_string(),
                        method_name.to_string(),
                    ));
                }
            }
        }

        if let Some(start) = starter_code.find("fn ") {
            let end = starter_code[start..].find('(').unwrap_or(0) + start;
            let fn_name = starter_code[start + 3..end].trim().to_string();
            return Ok(CodeSignature::new_function(fn_name));
        }

        pub fn resolve_declaration(
            lang: &ProgrammingLanguage, variable_name: &str, test_data: &str,
        ) -> String {
            match lang {
                ProgrammingLanguage::Rust => {
                    resolve_rust_declaration(variable_name, test_data)
                },
                ProgrammingLanguage::C => {
                    resolve_c_declaration(variable_name, test_data)
                },
                _ => panic!("Unsupported language for declaration resolution"),
            }
        }

        fn resolve_rust_declaration(
            variable_name: &str, test_data: &str,
        ) -> String {
            return format!("let {} = {};", variable_name, test_data);
        }
        fn resolve_c_declaration(
            variable_name: &str, test_data: &str,
        ) -> String {
            return format!("{} {} = {};", "int", variable_name, test_data);
        }

        Err("No function or impl block found in Rust code".to_string())
    }
}
