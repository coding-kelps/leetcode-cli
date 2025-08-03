use leetcoderustapi::ProgrammingLanguage::{
    self,
    *,
};
use thiserror;

use crate::{
    code_signature::*,
    readme_parser::ProblemTestData,
};
pub struct TestGenerator {
    starter_code: String,
    test_data:    ProblemTestData,
}

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestGeneratorError {
    #[error("Error creating tests")]
    ProblemTestDataError,
}

impl From<TestGeneratorError> for std::io::Error {
    fn from(e: TestGeneratorError) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    }
}

impl From<CodeSignatureError> for TestGeneratorError {
    fn from(_: CodeSignatureError) -> Self {
        TestGeneratorError::ProblemTestDataError
    }
}

impl TestGenerator {
    pub fn new(starter_code: &str, test_data: ProblemTestData) -> Self {
        TestGenerator { starter_code: starter_code.to_owned(), test_data }
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
        for i in 0..self.test_data.example_count {
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
        let mut tests =
            "#[cfg(test)]\nmod tests {\n\n\tuse super::*;\n\n".to_string();

        for i in 0..self.test_data.example_count {
            let expect = format!(
                "let expected = {};\n",
                CodeSignature::resolve_declaration(
                    &Rust,
                    &self.test_data.outputs[i]
                )
            );

            // Split input parameters and convert each one
            let input_params =
                self.split_input_parameters(&self.test_data.inputs[i]);
            let converted_params: Vec<String> = input_params
                .iter()
                .map(|param| CodeSignature::resolve_declaration(&Rust, param))
                .collect();

            let test_call = format!(
                "\t\tlet result = Solution::{}({});\n",
                signature.function_name,
                converted_params.join(", ")
            );
            tests.push_str(&format!(
                "\t#[test]\n\tfn test_case_{i}() {{\n\t    \
                 {expect}{test_call}\t\tassert_eq!(result, \
                 expected);\n\t}}\n\n"
            ));
        }
        tests.push_str("}\n");
        Ok(tests)
    }

    pub fn run(
        &mut self, lang: &ProgrammingLanguage,
    ) -> Result<String, TestGeneratorError> {
        let signature =
            CodeSignature::parse_code_signature(lang, &self.starter_code)?;

        match lang {
            Rust => self.generate_rust_tests(&signature),
            Python => self.generate_python_tests(&signature),
            _ => Err(TestGeneratorError::ProblemTestDataError),
        }
    }
}
