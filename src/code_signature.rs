use core::panic;

use leetcoderustapi::ProgrammingLanguage;
use thiserror;

#[derive(Debug, Clone)]
pub struct CodeSignature {
    pub function_name: String,
    pub class_name:    Option<String>,
    pub parameters:    Vec<String>,
    pub return_type:   Option<String>,
}

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeSignatureError {
    #[error("Error parsing code signature")]
    ParseError,
}

impl CodeSignature {
    pub fn new_function(name: String, params: Vec<String>) -> Self {
        Self {
            function_name: name,
            class_name:    None,
            parameters:    params,
            return_type:   None,
        }
    }

    pub fn new_class(class_name: String, method_name: String) -> Self {
        Self {
            function_name: method_name,
            class_name:    Some(class_name),
            parameters:    Vec::new(),
            return_type:   None,
        }
    }

    pub fn parse_code_signature(
        lang: &ProgrammingLanguage, starter_code: &str,
    ) -> Result<CodeSignature, CodeSignatureError> {
        match lang {
            ProgrammingLanguage::Python => {
                Self::parse_python_signature(&starter_code)
            },
            ProgrammingLanguage::Rust => {
                Self::parse_rust_signature(&starter_code)
            },
            _ => Err(CodeSignatureError::ParseError),
        }
    }

    fn parse_python_signature(
        starter_code: &str,
    ) -> Result<CodeSignature, CodeSignatureError> {
        if let Some(class_start) = starter_code.find("class ") {
            let class_end = starter_code[class_start..].find(':').unwrap_or(0)
                + class_start;
            let class_line = &starter_code[class_start..class_end];

            if let Some(class_name) =
                class_line.strip_prefix("class ").map(|s| s.trim())
            {
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
            let parameters = starter_code[end + 1..]
                .find(')')
                .map(|p| &starter_code[end + 1..end + p])
                .unwrap_or("")
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();
            return Ok(CodeSignature::new_function(fn_name, parameters));
        }

        Err(CodeSignatureError::ParseError)
    }

    fn parse_rust_signature(
        starter_code: &str,
    ) -> Result<CodeSignature, CodeSignatureError> {
        if let Some(start) = starter_code.find("fn ") {
            let end = starter_code[start..].find('(').unwrap_or(0) + start;
            let fn_name = starter_code[start + 3..end].trim().to_string();
            let parameters = starter_code[end + 1..]
                .find(')')
                .map(|p| &starter_code[end + 1..end + p])
                .unwrap_or("")
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();
            return Ok(CodeSignature::new_function(fn_name, parameters));
        }
        Err(CodeSignatureError::ParseError)
    }

    pub fn resolve_declaration(
        lang: &ProgrammingLanguage, test_data: &str,
    ) -> String {
        match lang {
            ProgrammingLanguage::Rust => {
                Self::resolve_rust_declaration(test_data)
            },
            ProgrammingLanguage::C => Self::resolve_c_declaration(test_data),
            _ => panic!("Unsupported language for declaration resolution"),
        }
    }

    fn resolve_rust_declaration(test_data: &str) -> String {
        let trimmed = test_data.trim();

        // Handle string literals
        if trimmed.starts_with('"') && trimmed.ends_with('"') {
            return format!("{}.to_string()", trimmed);
        }

        // Handle arrays/vectors
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let inner = &trimmed[1..trimmed.len() - 1];
            let elements = Self::parse_array_elements(inner);
            let converted_elements: Vec<String> = elements
                .into_iter()
                .map(|elem| Self::resolve_rust_declaration(&elem))
                .collect();
            return format!("vec![{}]", converted_elements.join(", "));
        }

        // Handle primitives (numbers, booleans)
        if trimmed.parse::<i64>().is_ok()
            || trimmed.parse::<f64>().is_ok()
            || trimmed == "true"
            || trimmed == "false"
        {
            return trimmed.to_string();
        }

        // Default case
        trimmed.to_string()
    }

    fn parse_array_elements(inner: &str) -> Vec<String> {
        let mut elements = Vec::new();
        let mut current = String::new();
        let mut bracket_depth = 0;
        let mut in_quotes = false;

        for ch in inner.chars() {
            match ch {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(ch);
                },
                '[' if !in_quotes => {
                    bracket_depth += 1;
                    current.push(ch);
                },
                ']' if !in_quotes => {
                    bracket_depth -= 1;
                    current.push(ch);
                },
                ',' if !in_quotes && bracket_depth == 0 => {
                    if !current.trim().is_empty() {
                        elements.push(current.trim().to_string());
                    }
                    current.clear();
                },
                _ => current.push(ch),
            }
        }

        if !current.trim().is_empty() {
            elements.push(current.trim().to_string());
        }

        elements
    }

    fn resolve_c_declaration(test_data: &str) -> String {
        return format!("{}", test_data);
    }
}
