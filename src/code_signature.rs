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
        "no `impl Solution` nor design class found, or a random design \
         problem, tests are not generated"
    )]
    NoSolution,
}

impl CodeSignature {
    #[must_use]
    pub fn new_function(name: String, params: Vec<String>) -> Self {
        Self {
            function_name: name,
            class_name:    None,
            parameters:    params,
            return_type:   None,
        }
    }

    #[must_use]
    pub fn new_class(class_name: String, method_name: String) -> Self {
        Self {
            function_name: method_name,
            class_name:    Some(class_name),
            parameters:    Vec::new(),
            return_type:   None,
        }
    }

    /// Parses starter code for the given language.
    ///
    /// # Errors
    ///
    /// `ParseError` when the signature cannot be read or the language is
    /// unsupported for signature parsing.
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
                class_line.strip_prefix("class ").map(str::trim)
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
        // a design problem can name its own class `Solution` (384, 398)
        if code.contains("struct Solution") {
            return Err(CodeSignatureError::NoSolution);
        }
        let solution =
            code.find("impl Solution").ok_or(CodeSignatureError::NoSolution)?;
        let start = code[solution..]
            .find("fn ")
            .ok_or(CodeSignatureError::ParseError)?
            + solution;
        Self::parse_rust_function(&code[start + 3..])
    }

    /// `name(params) -> Type {` right after a `fn `
    fn parse_rust_function(
        code: &str,
    ) -> Result<CodeSignature, CodeSignatureError> {
        let (fn_name, parameters, rest) = Self::parse_signature_parts(code)?;
        let return_type = rest
            .split('{')
            .next()
            .and_then(|head| head.trim().strip_prefix("->"))
            .map(|ty| ty.trim().to_string())
            .filter(|ty| !ty.is_empty());
        Ok(CodeSignature {
            return_type,
            ..CodeSignature::new_function(fn_name, parameters)
        })
    }

    /// Methods of a rust design problem class (`struct LRUCache` and its
    /// `impl`), `new` included. Classes named `Solution` are random problems
    /// (384 shuffle, 528 pick index) and are not supported.
    ///
    /// # Errors
    ///
    /// `NoSolution` when the code contains no supported design class, only
    /// a `Solution` class, or the `impl` block has no `new` method.
    /// `ParseError` propagates from `parse_rust_function` on malformed
    /// signatures.
    ///
    /// # Panics
    ///
    /// Panics only if the hardcoded regexes are invalid — compile-time
    /// constants, cannot happen in practice.
    pub fn parse_rust_design(
        starter_code: &str,
    ) -> Result<Vec<CodeSignature>, CodeSignatureError> {
        let code = Self::strip_rust_comments(starter_code);
        // skip the uncommented ListNode / TreeNode definitions
        let classes: Vec<String> = regex::Regex::new(r"\bstruct\s+(\w+)")
            .expect("valid regex")
            .captures_iter(&code)
            .map(|caps| caps[1].to_string())
            .filter(|class| !["ListNode", "TreeNode"].contains(&class.as_str()))
            .collect();
        let class = classes
            .first()
            .filter(|_| !classes.iter().any(|c| c == "Solution"))
            .cloned()
            .ok_or(CodeSignatureError::NoSolution)?;
        let impl_start = code
            .find(&format!("impl {class}"))
            .ok_or(CodeSignatureError::NoSolution)?;
        let fn_re = regex::Regex::new(r"\bfn\s+").expect("valid regex");
        let methods = fn_re
            .find_iter(&code[impl_start..])
            .map(|m| {
                Self::parse_rust_function(&code[impl_start + m.end()..]).map(
                    |signature| CodeSignature {
                        class_name: Some(class.clone()),
                        ..signature
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        if !methods.iter().any(|method| method.function_name == "new") {
            return Err(CodeSignatureError::NoSolution);
        }
        Ok(methods)
    }

    /// Type of each parameter (`nums: Vec<i32>` -> `Vec<i32>`), `None` when
    /// it cannot be read.
    #[must_use]
    pub fn parameter_types(&self) -> Vec<Option<String>> {
        self.parameters
            .iter()
            .map(|p| p.split_once(':').map(|(_, ty)| ty.trim().to_string()))
            .collect()
    }

    /// Removes `//` line comments and `/* */` block comments from rust code.
    ///
    /// # Panics
    ///
    /// Panics only if the hardcoded comment regexes are invalid — they are
    /// compile-time constants, so this cannot happen in practice.
    #[must_use]
    pub fn strip_rust_comments(code: &str) -> String {
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
        Self::parse_signature_parts(code)
            .map(|(fn_name, parameters, _)| (fn_name, parameters))
    }

    /// Same as `parse_name_and_parameters`, also returns what follows the
    /// closing parenthesis (return type, body...).
    fn parse_signature_parts(
        code: &str,
    ) -> Result<(String, Vec<String>, &str), CodeSignatureError> {
        let open = code.find('(').ok_or(CodeSignatureError::ParseError)?;
        let fn_name = code[..open].trim().to_string();
        if fn_name.is_empty() {
            return Err(CodeSignatureError::ParseError);
        }

        let mut parameters = Vec::new();
        let mut current = String::new();
        let mut depth = 0;
        let mut previous = ' ';
        for (i, ch) in code[open + 1..].char_indices() {
            match ch {
                '(' | '<' | '[' => depth += 1,
                ')' if depth == 0 => {
                    if !current.trim().is_empty() {
                        parameters.push(current.trim().to_string());
                    }
                    return Ok((fn_name, parameters, &code[open + 2 + i..]));
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

    /// Builds the source code declaration of the test subject for the given
    /// language (`struct` for rust design classes, plain signature for C).
    ///
    /// # Panics
    ///
    /// Panics when the language is neither `Rust` nor `C` — only those two
    /// declaration styles exist.
    #[must_use]
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
        Self::resolve_rust_typed_declaration(test_data, None)
    }

    /// Converts a leetcode example value into a rust expression, using the
    /// rust type when known: `"a"` is a `char` for `char`, `2` is `2.0` for
    /// `f64`, `[..]` elements follow the `Vec` element type.
    /// `[1,[2]]` -> `NestedInteger::List(vec![NestedInteger::Int(1), ..])`
    fn nested_integer(value: &str) -> String {
        match value
            .trim()
            .strip_prefix('[')
            .and_then(|inner| inner.strip_suffix(']'))
        {
            Some(inner) => format!(
                "NestedInteger::List(vec![{}])",
                Self::parse_array_elements(inner)
                    .iter()
                    .map(|element| Self::nested_integer(element))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            None => format!("NestedInteger::Int({})", value.trim()),
        }
    }

    pub fn resolve_rust_typed_declaration(
        test_data: &str, rust_type: Option<&str>,
    ) -> String {
        let trimmed = test_data.trim();
        let rust_type = rust_type.map(|ty| {
            ty.trim().trim_start_matches("&mut ").trim_start_matches('&').trim()
        });

        if rust_type == Some("NestedInteger") {
            return Self::nested_integer(trimmed);
        }
        if let Some(helper) = rust_type.and_then(Self::rust_node_helper) {
            let values = trimmed
                .strip_prefix('[')
                .and_then(|values| values.strip_suffix(']'))
                .unwrap_or(trimmed);
            let values: Vec<String> = Self::parse_array_elements(values)
                .into_iter()
                .filter(|value| helper == "to_tree" || value != "null")
                .map(|value| match (helper, value.as_str()) {
                    ("to_tree", "null") => "None".to_string(),
                    ("to_tree", _) => format!("Some({value})"),
                    _ => value,
                })
                .collect();
            return format!("{helper}(vec![{}])", values.join(", "));
        }
        if let Some(content) = trimmed
            .strip_prefix('"')
            .map(|rest| rest.strip_suffix('"').unwrap_or(rest))
        {
            if rust_type == Some("char") {
                let escaped = match content {
                    "'" => "\\'".to_string(),
                    "\\" => "\\\\".to_string(),
                    _ => content.to_string(),
                };
                return format!("'{escaped}'");
            }
            return format!(
                "\"{}\".to_string()",
                content.replace('\\', "\\\\")
            );
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let element_type = rust_type
                .and_then(|ty| ty.strip_prefix("Vec<"))
                .and_then(|ty| ty.strip_suffix('>'));
            let inner = &trimmed[1..trimmed.len() - 1];
            let converted_elements: Vec<String> =
                Self::parse_array_elements(inner)
                    .into_iter()
                    .map(|elem| {
                        Self::resolve_rust_typed_declaration(
                            &elem,
                            element_type,
                        )
                    })
                    .collect();
            return format!("vec![{}]", converted_elements.join(", "));
        }
        if matches!(rust_type, Some("f64" | "f32"))
            && trimmed.parse::<i64>().is_ok()
        {
            return format!("{trimmed}.0");
        }
        trimmed.to_string()
    }

    /// Test helper building a leetcode linked list / binary tree from its
    /// array representation, for the rust types using them.
    fn rust_node_helper(rust_type: &str) -> Option<&'static str> {
        match rust_type.replace(' ', "").as_str() {
            "Option<Box<ListNode>>" => Some("to_list"),
            "Option<Rc<RefCell<TreeNode>>>" => Some("to_tree"),
            _ => None,
        }
    }

    #[must_use]
    pub fn parse_array_elements(inner: &str) -> Vec<String> {
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
