use leetcoderustapi::ProgrammingLanguage::{
    self,
    *,
};
use thiserror;

use crate::{
    code_signature::*,
    readme_parser::ProblemTestData,
};
const RUST_TO_LIST: &str = "    fn to_list(values: Vec<i32>) -> \
                            Option<Box<ListNode>> {
        let mut head = None;
        for val in values.into_iter().rev() {
            head = Some(Box::new(ListNode { val, next: head }));
        }
        head
    }

";

const RUST_TO_TREE: &str = "    fn to_tree(
        values: Vec<Option<i32>>,
    ) -> Option<std::rc::Rc<std::cell::RefCell<TreeNode>>> {
        let nodes: Vec<Option<std::rc::Rc<std::cell::RefCell<TreeNode>>>> =
            values
                .into_iter()
                .map(|value| {
                    value.map(|val| {
                        std::rc::Rc::new(std::cell::RefCell::new(
                            TreeNode::new(val),
                        ))
                    })
                })
                .collect();
        let mut children = nodes.iter().skip(1);
        for node in nodes.iter().flatten() {
            let mut node = node.borrow_mut();
            node.left = children.next().cloned().flatten();
            node.right = children.next().cloned().flatten();
        }
        nodes.first().cloned().flatten()
    }

";

pub struct TestGenerator {
    starter_code: String,
    test_data:    ProblemTestData,
}

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestGeneratorError {
    #[error("Error creating tests")]
    ProblemTestDataError,
    #[error(transparent)]
    Signature(#[from] CodeSignatureError),
    #[error("test generation is not supported for this language yet")]
    UnsupportedLanguage,
    #[error(
        "example inputs do not match the function parameters (interactive \
         problem?)"
    )]
    InputMismatch,
}

impl From<TestGeneratorError> for std::io::Error {
    fn from(e: TestGeneratorError) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    }
}

impl TestGenerator {
    pub fn new(starter_code: &str, test_data: ProblemTestData) -> Self {
        TestGenerator { starter_code: starter_code.to_owned(), test_data }
    }

    /// Number of examples that have both an input and an output, so a readme
    /// with a missing Input / Output line does not index out of bounds.
    fn test_case_count(&self) -> usize {
        self.test_data
            .example_count
            .min(self.test_data.inputs.len())
            .min(self.test_data.outputs.len())
    }

    /// `2, nums = [1,2,_]` -> (`2`, `nums`, `[1,2]`), the `_` being the
    /// elements the judge ignores.
    fn parse_custom_judge_output(
        output: &str,
    ) -> Option<(String, String, String)> {
        let re = regex::Regex::new(
            r"^\s*([^,\[\]]+?)\s*,\s*([A-Za-z_]\w*)\s*=\s*\[(.*)\]\s*$",
        )
        .expect("valid regex");
        let caps = re.captures(output)?;
        let array = caps[3]
            .split(',')
            .map(str::trim)
            .filter(|value| *value != "_" && !value.is_empty())
            .collect::<Vec<_>>()
            .join(",");
        Some((caps[1].to_string(), caps[2].to_string(), format!("[{array}]")))
    }

    fn split_input_parameters(&self, input: &str) -> Vec<String> {
        let mut parameters = Vec::new();
        let mut current = String::new();
        let mut bracket_depth = 0;
        let mut in_quotes = false;

        for ch in input.chars() {
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
                        parameters.push(current.trim().to_string());
                    }
                    current.clear();
                },
                _ => current.push(ch),
            }
        }

        if !current.trim().is_empty() {
            parameters.push(current.trim().to_string());
        }

        parameters
    }

    fn generate_python_tests(
        &self, signature: &CodeSignature,
    ) -> Result<String, TestGeneratorError> {
        let mut tests = String::new();
        for i in 0..self.test_case_count() {
            let test_call = if signature.class_name.is_some() {
                if let Some(class_name) = &signature.class_name {
                    format!(
                        "{}().{}({})",
                        class_name,
                        signature.function_name,
                        self.test_data.inputs[i]
                    )
                } else {
                    format!(
                        "Solution().{}({})",
                        signature.function_name, self.test_data.inputs[i]
                    )
                }
            } else {
                format!(
                    "{}({})",
                    signature.function_name, self.test_data.inputs[i]
                )
            };

            tests.push_str(&format!(
                "def test_case_{}():\n    assert {} == {}\n\n",
                i, test_call, self.test_data.outputs[i]
            ));
        }
        Ok(tests)
    }

    fn generate_rust_tests(
        &self, signature: &CodeSignature,
    ) -> Result<String, TestGeneratorError> {
        let mut tests = Vec::new();
        let parameter_types = signature.parameter_types();

        // in-place problems (`&mut` parameter, no return value): the
        // expected output is the value of the first `&mut` parameter
        let mutated = signature
            .return_type
            .is_none()
            .then(|| {
                parameter_types.iter().position(|ty| {
                    ty.as_deref().is_some_and(|ty| ty.starts_with("&mut "))
                })
            })
            .flatten();
        let expected_type = match mutated {
            Some(j) => parameter_types[j].as_deref(),
            None => signature.return_type.as_deref(),
        };

        // annotated so that empty outputs (`vec![]`) still type check
        let expected_annotation = expected_type
            .map(|ty| {
                format!(
                    ": {}",
                    ty.trim_start_matches("&mut ").trim_start_matches('&')
                )
            })
            .unwrap_or_default();

        for i in 0..self.test_case_count() {
            let mut lines = Vec::new();
            let output = &self.test_data.outputs[i];

            // custom judge outputs, eg remove duplicates: `2, nums = [1,2,_]`
            // is the returned length and the first elements of `nums`
            let custom_judge = Self::parse_custom_judge_output(output)
                .and_then(|(value, name, array)| {
                    let j = signature.parameters.iter().position(|p| {
                        p.split_once(':').is_some_and(|(n, ty)| {
                            n.trim() == name && ty.trim().starts_with("&mut ")
                        })
                    })?;
                    Some((value, j, array))
                });
            let expected_output =
                custom_judge.as_ref().map_or(output.as_str(), |(v, ..)| v);

            lines.push(format!(
                "let expected{expected_annotation} = {};",
                CodeSignature::resolve_rust_typed_declaration(
                    expected_output,
                    expected_type,
                )
            ));

            // Split input parameters and convert each one with its type
            let input_params =
                self.split_input_parameters(&self.test_data.inputs[i]);
            if input_params.len() != parameter_types.len() {
                return Err(TestGeneratorError::InputMismatch);
            }
            let mut arguments = Vec::new();
            for (j, param) in input_params.iter().enumerate() {
                let rust_type =
                    parameter_types.get(j).and_then(|ty| ty.as_deref());
                let value = CodeSignature::resolve_rust_typed_declaration(
                    param, rust_type,
                );
                match rust_type {
                    Some(ty) if ty.starts_with("&mut ") => {
                        lines.push(format!("let mut arg{j} = {value};"));
                        arguments.push(format!("&mut arg{j}"));
                    },
                    Some(ty) if ty.starts_with('&') => {
                        arguments.push(format!("&{value}"))
                    },
                    _ => arguments.push(value),
                }
            }

            let call = format!(
                "Solution::{}({})",
                signature.function_name,
                arguments.join(", ")
            );
            match mutated {
                Some(j) => {
                    lines.push(format!("{call};"));
                    lines.push(format!("assert_eq!(arg{j}, expected);"));
                },
                None => {
                    lines.push(format!("let result = {call};"));
                    lines.push("assert_eq!(result, expected);".to_string());
                },
            }

            if let Some((_, j, array)) = &custom_judge {
                let element_type = parameter_types[*j]
                    .as_deref()
                    .map(|ty| ty.trim_start_matches("&mut ").trim());
                lines.push(format!(
                    "let mut expected_arg{j}{} = {};",
                    element_type
                        .map(|ty| format!(": {ty}"))
                        .unwrap_or_default(),
                    CodeSignature::resolve_rust_typed_declaration(
                        array,
                        element_type,
                    )
                ));
                // the judge sorts the first elements before comparing them
                lines.push(format!(
                    "let mut head = arg{j}[..expected_arg{j}.len()].to_vec();"
                ));
                lines.push("head.sort();".to_string());
                lines.push(format!("expected_arg{j}.sort();"));
                lines.push(format!("assert_eq!(head, expected_arg{j});"));
            }

            let body = lines
                .iter()
                .map(|line| format!("        {line}\n"))
                .collect::<String>();
            tests.push(format!(
                "    #[test]\n    fn test_case_{i}() {{\n{body}    }}\n"
            ));
        }
        let tests = tests.join("\n");
        let mut helpers = String::new();
        if tests.contains("to_list(") {
            helpers.push_str(RUST_TO_LIST);
        }
        if tests.contains("to_tree(") {
            helpers.push_str(RUST_TO_TREE);
        }
        Ok(format!(
            "#[cfg(test)]\nmod tests {{\n    use \
             super::*;\n\n{helpers}{tests}}}\n"
        ))
    }

    pub fn run(
        &mut self, lang: &ProgrammingLanguage,
    ) -> Result<String, TestGeneratorError> {
        if !matches!(lang, Rust | Python | Python3) {
            return Err(TestGeneratorError::UnsupportedLanguage);
        }
        let signature =
            CodeSignature::parse_code_signature(lang, &self.starter_code)?;

        match lang {
            Rust => self.generate_rust_tests(&signature),
            _ => self.generate_python_tests(&signature),
        }
    }
}
