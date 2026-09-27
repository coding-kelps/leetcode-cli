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
    #[error(
        "no `impl Solution` found, tests are not generated for design problems"
    )]
    NoSolution,
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
            ProgrammingLanguage::Python | ProgrammingLanguage::Python3 => {
                Self::parse_python_signature(starter_code)
            },
            ProgrammingLanguage::Rust => {
                Self::parse_rust_signature(starter_code)
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
                    let (method_name, _) = Self::parse_name_and_parameters(
                        &starter_code[def_start + 4..],
                    )?;

                    if method_name != "__init__" {
                        return Ok(CodeSignature::new_class(
                            class_name.to_string(),
                            method_name,
                        ));
                    }
                }
            }
        }
        if let Some(start) = starter_code.find("def ") {
            let (fn_name, parameters) =
                Self::parse_name_and_parameters(&starter_code[start + 4..])?;
            return Ok(CodeSignature::new_function(fn_name, parameters));
        }

        Err(CodeSignatureError::ParseError)
    }

    fn parse_rust_signature(
        starter_code: &str,
    ) -> Result<CodeSignature, CodeSignatureError> {
        // skip the commented ListNode / TreeNode definitions and their
        // `fn new`, the tests call the method of `impl Solution`
        let code = Self::strip_rust_comments(starter_code);
        let solution =
            code.find("impl Solution").ok_or(CodeSignatureError::NoSolution)?;
        let start = code[solution..]
            .find("fn ")
            .ok_or(CodeSignatureError::ParseError)?
            + solution;
        let (fn_name, parameters) =
            Self::parse_name_and_parameters(&code[start + 3..])?;
        Ok(CodeSignature::new_function(fn_name, parameters))
    }

    fn strip_rust_comments(code: &str) -> String {
        let block = regex::Regex::new(r"(?s)/\*.*?\*/").expect("valid regex");
        let line = regex::Regex::new(r"(?m)//.*$").expect("valid regex");
        line.replace_all(&block.replace_all(code, ""), "").into_owned()
    }

    /// Parses `name(param, param)` at the start of `code` into the function
    /// name and the list of its parameters, splitting on top level commas
    /// only (`HashMap<i32, i32>` stays one parameter).
    fn parse_name_and_parameters(
        code: &str,
    ) -> Result<(String, Vec<String>), CodeSignatureError> {
        let open = code.find('(').ok_or(CodeSignatureError::ParseError)?;
        let fn_name = code[..open].trim().to_string();
        if fn_name.is_empty() {
            return Err(CodeSignatureError::ParseError);
        }

        let mut parameters = Vec::new();
        let mut current = String::new();
        let mut depth = 0;
        let mut previous = ' ';
        for ch in code[open + 1..].chars() {
            match ch {
                '(' | '<' | '[' => depth += 1,
                ')' if depth == 0 => {
                    if !current.trim().is_empty() {
                        parameters.push(current.trim().to_string());
                    }
                    return Ok((fn_name, parameters));
                },
                // `->` is not a closing bracket
                '>' if previous == '-' => {},
                ')' | '>' | ']' => depth -= 1,
                ',' if depth == 0 => {
                    parameters.push(current.trim().to_string());
                    current.clear();
                    continue;
                },
                _ => {},
            }
            previous = ch;
            current.push(ch);
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

        if trimmed.starts_with('"') {
            let mut end_quote = 1;
            let chars: Vec<char> = trimmed.chars().collect();

            while end_quote < chars.len() {
                if chars[end_quote] == '"'
                    && (end_quote == 1 || chars[end_quote - 1] != '\\')
                {
                    break;
                }
                end_quote += 1;
            }

            if end_quote >= chars.len() {
                end_quote = chars.len();
            } else {
                end_quote += 1;
            }

            let string_content = &trimmed[1..end_quote - 1];
            return format!(
                "\"{}\".to_string()",
                string_content.replace('\\', "\\\\")
            );
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let inner = &trimmed[1..trimmed.len() - 1];
            let elements = Self::parse_array_elements(inner);
            let converted_elements: Vec<String> = elements
                .into_iter()
                .map(|elem| Self::resolve_rust_declaration(&elem))
                .collect();
            return format!("vec![{}]", converted_elements.join(", "));
        }
        if trimmed.parse::<i64>().is_ok()
            || trimmed == "true"
            || trimmed == "false"
        {
            return trimmed.to_string();
        }
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
        test_data.to_string()
    }
}
