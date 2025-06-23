use leetcoderustapi::ProgrammingLanguage;

use crate::{
    code_signature::CodeSignature,
    readme_parser::ProblemTestData,
};

pub struct TestGenerator {
    starter_code: String,
    test_data:    ProblemTestData,
}

impl TestGenerator {
    pub fn new(starter_code: &String, test_data: ProblemTestData) -> Self {
        TestGenerator {
            starter_code: starter_code.clone(),
            test_data,
        }
    }

    fn generate_python_tests(
        &self, signature: &CodeSignature,
    ) -> Result<String, String> {
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
    ) -> Result<String, String> {
        let lang = ProgrammingLanguage::Rust;
        let test_data = &self.test_data;
        let mut tests = format!("#[cfg(test)]\nmod tests {{\n\n");
        tests.push_str("\tuse super::*;\n\n");
        for i in 0..test_data.example_count {
            let expect = format!(
                "let expected = {};\n",
                CodeSignature::resolve_declaration(
                    &lang,
                    &test_data.outputs[i]
                )
            );
            let test_call = format!(
                "\t\tlet result = Solution::{}({});\n",
                signature.function_name,
                CodeSignature::resolve_declaration(&lang, &test_data.inputs[i])
            );

            tests.push_str(&format!(
                "\t#[test]\n\tfn test_case_{}() {{\n\t    \
                 {}{}\t\tassert_eq!(result, expected);\n\t}}\n\n",
                i, expect, test_call
            ));
        }
        tests.push_str("}\n");
        Ok(tests)
    }

    pub fn run(
        &mut self, lang: &ProgrammingLanguage,
    ) -> Result<String, String> {
        let signature =
            CodeSignature::parse_code_signature(lang, &self.starter_code)?;

        match lang {
            ProgrammingLanguage::Rust => self.generate_rust_tests(&signature),
            ProgrammingLanguage::Python => {
                self.generate_python_tests(&signature)
            },
            _ => Err("Unsupported language".to_string()),
        }
    }
}
