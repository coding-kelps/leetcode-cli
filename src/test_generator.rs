use leetcoderustapi::ProgrammingLanguage;

use crate::readme_parser::ProblemTestData;

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

    pub fn run(&self, lang: &ProgrammingLanguage) -> Result<String, String> {
        unimplemented!();
    }
}
