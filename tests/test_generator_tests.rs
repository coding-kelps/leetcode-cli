use std::vec;

use leetcode_cli::{
    code_signature::CodeSignature,
    readme_parser::ProblemTestData,
    test_generator::TestGenerator,
};
use leetcoderustapi::ProgrammingLanguage;

#[test]
fn test_code_signature_function() {
    let sig = CodeSignature::new_function("two_sum".to_string(), vec![]);
    assert_eq!(sig.function_name, "two_sum");
    assert!(sig.class_name.is_none());
}

#[test]
fn test_code_signature_class() {
    let sig =
        CodeSignature::new_class("Solution".to_string(), "two_sum".to_string());
    assert_eq!(sig.function_name, "two_sum");
    assert_eq!(sig.class_name, Some("Solution".to_string()));
}

#[test]
fn test_python_function_parsing() {
    let starter_code = "def two_sum(nums, target):\n    pass".to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[2,7,11,15], 9".to_string()],
        outputs:       vec!["[0,1]".to_string()],
    };

    let _generator = TestGenerator::new(&starter_code, test_data);
    let signature = CodeSignature::parse_code_signature(
        &ProgrammingLanguage::Python,
        &starter_code,
    )
    .unwrap();

    assert_eq!(signature.function_name, "two_sum");
}

#[test]
fn test_python_class_parsing() {
    let starter_code = r#"class Solution:
    def two_sum(self, nums, target):
        pass"#
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[2,7,11,15], 9".to_string()],
        outputs:       vec!["[0,1]".to_string()],
    };

    let _generator = TestGenerator::new(&starter_code, test_data);
    let signature = CodeSignature::parse_code_signature(
        &ProgrammingLanguage::Python,
        &starter_code,
    )
    .unwrap();

    assert_eq!(signature.function_name, "two_sum");
    assert_eq!(signature.class_name, Some("Solution".to_string()));
}

#[test]
fn test_rust_function_parsing() {
    let starter_code = "fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> \
                        {\n    vec![]\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["vec![2,7,11,15], 9".to_string()],
        outputs:       vec!["vec![0,1]".to_string()],
    };

    let _generator = TestGenerator::new(&starter_code, test_data);
    let signature = CodeSignature::parse_code_signature(
        &ProgrammingLanguage::Rust,
        &starter_code,
    )
    .unwrap();

    assert_eq!(signature.function_name, "two_sum");
}

#[test]
fn test_python_test_generation_function() {
    let starter_code = "def two_sum(nums, target):\n    pass".to_string();
    let test_data = ProblemTestData {
        example_count: 2,
        inputs:        vec![
            "[2,7,11,15], 9".to_string(),
            "[3,2,4], 6".to_string(),
        ],
        outputs:       vec!["[0,1]".to_string(), "[1,2]".to_string()],
    };

    let mut generator = TestGenerator::new(&starter_code, test_data);
    let result = generator.run(&ProgrammingLanguage::Python).unwrap();

    assert!(result.contains("def test_case_0():"));
    assert!(result.contains("def test_case_1():"));
    assert!(result.contains("assert two_sum([2,7,11,15], 9) == [0,1]"));
    assert!(result.contains("assert two_sum([3,2,4], 6) == [1,2]"));
}

#[test]
fn test_python_test_generation_class() {
    let starter_code = r#"class Solution:
    def two_sum(self, nums, target):
        pass"#
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[2,7,11,15], 9".to_string()],
        outputs:       vec!["[0,1]".to_string()],
    };

    let mut generator = TestGenerator::new(&starter_code, test_data);
    let result = generator.run(&ProgrammingLanguage::Python).unwrap();

    assert!(result.contains("def test_case_0():"));
    assert!(
        result.contains("assert Solution().two_sum([2,7,11,15], 9) == [0,1]")
    );
}
