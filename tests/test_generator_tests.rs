use std::vec;

use leetcode_cli::{
    code_signature::CodeSignature,
    readme_parser::ProblemTestData,
    test_generator::{
        TestGenerator,
        TestGeneratorError,
    },
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
    let starter_code = "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, \
                        target: i32) -> Vec<i32> {\n        vec![]\n    }\n}"
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
fn test_rust_multiple_arrays_parsing() {
    let starter_code = "impl Solution {\n    pub fn \
                        find_median_sorted_arrays(nums1: Vec<i32>, nums2: \
                        Vec<i32>) -> f64 {\n        0.0\n    }\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[1, 3],[2]".to_string()],
        outputs:       vec!["2.0".to_string()],
    };

    let mut generator = TestGenerator::new(&starter_code, test_data);
    let result = generator.run(&ProgrammingLanguage::Rust).unwrap();

    // Check that both arrays have vec! macro
    assert!(result.contains("vec![1, 3], vec![2]"));
    assert!(!result.contains("vec![1, 3],[2]")); // Should not have this invalid
                                                 // syntax
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

#[test]
fn test_python3_test_generation_class() {
    let starter_code = r#"class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        "#
    .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[2,7,11,15], 9".to_string()],
        outputs:       vec!["[0,1]".to_string()],
    };

    let mut generator = TestGenerator::new(&starter_code, test_data);
    let result = generator.run(&ProgrammingLanguage::Python3).unwrap();

    assert!(result.contains("def test_case_0():"));
    assert!(
        result.contains("assert Solution().twoSum([2,7,11,15], 9) == [0,1]")
    );
}

#[test]
fn test_unsupported_language_returns_unsupported_error() {
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[2,7,11,15], 9".to_string()],
        outputs:       vec!["[0,1]".to_string()],
    };

    let mut generator = TestGenerator::new("class Solution {\n};", test_data);
    let result = generator.run(&ProgrammingLanguage::CPP);

    assert_eq!(result.unwrap_err(), TestGeneratorError::UnsupportedLanguage);
}

#[test]
fn test_more_examples_than_inputs_does_not_panic() {
    let starter_code = "def two_sum(nums, target):\n    pass".to_string();
    let test_data = ProblemTestData {
        example_count: 3,
        inputs:        vec!["[2,7,11,15], 9".to_string()],
        outputs:       vec!["[0,1]".to_string(), "[1,2]".to_string()],
    };

    let mut generator = TestGenerator::new(&starter_code, test_data);
    let result = generator.run(&ProgrammingLanguage::Python).unwrap();

    assert!(result.contains("def test_case_0():"));
    assert!(!result.contains("def test_case_1():"));
}

#[test]
fn test_rust_in_place_problem_checks_mutated_parameter() {
    let starter_code = "impl Solution {\n    pub fn reverse_string(s: &mut \
                        Vec<char>) {\n        todo!()\n    }\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[\"h\",\"e\"]".to_string()],
        outputs:       vec!["[\"e\",\"h\"]".to_string()],
    };

    let mut generator = TestGenerator::new(&starter_code, test_data);
    let result = generator.run(&ProgrammingLanguage::Rust).unwrap();

    assert!(result.contains("let expected: Vec<char> = vec!['e', 'h'];"));
    assert!(result.contains("let mut arg0 = vec!['h', 'e'];"));
    assert!(result.contains("Solution::reverse_string(&mut arg0);"));
    assert!(result.contains("assert_eq!(arg0, expected);"));
    assert!(!result.contains("let result"));
}

#[test]
fn test_rust_mut_parameter_with_return_value_checks_result() {
    let starter_code = "impl Solution {\n    pub fn remove_element(nums: &mut \
                        Vec<i32>, val: i32) -> i32 {\n        todo!()\n    \
                        }\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[3,2,2,3],3".to_string()],
        outputs:       vec!["2".to_string()],
    };

    let mut generator = TestGenerator::new(&starter_code, test_data);
    let result = generator.run(&ProgrammingLanguage::Rust).unwrap();

    assert!(result.contains("let mut arg0 = vec![3, 2, 2, 3];"));
    assert!(
        result.contains("let result = Solution::remove_element(&mut arg0, 3);")
    );
    assert!(result.contains("assert_eq!(result, expected);"));
}

#[test]
fn test_rust_tree_problem_adds_tree_helper_only() {
    let starter_code =
        "use std::rc::Rc;\nuse std::cell::RefCell;\nimpl Solution {\n    pub \
         fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> \
         Option<Rc<RefCell<TreeNode>>> {\n        todo!()\n    }\n}"
            .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[2,1,3]".to_string()],
        outputs:       vec!["[2,3,1]".to_string()],
    };

    let mut generator = TestGenerator::new(&starter_code, test_data);
    let result = generator.run(&ProgrammingLanguage::Rust).unwrap();

    assert!(result.contains("fn to_tree("));
    assert!(!result.contains("fn to_list("));
    assert!(result.contains(
        "let result = Solution::invert_tree(to_tree(vec![Some(2), Some(1), \
         Some(3)]));"
    ));
}

#[test]
fn test_rust_plain_problem_adds_no_helper() {
    let starter_code = "impl Solution {\n    pub fn f(nums: Vec<i32>) -> i32 \
                        {\n        todo!()\n    }\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[1]".to_string()],
        outputs:       vec!["1".to_string()],
    };

    let mut generator = TestGenerator::new(&starter_code, test_data);
    let result = generator.run(&ProgrammingLanguage::Rust).unwrap();

    assert!(!result.contains("fn to_tree("));
    assert!(!result.contains("fn to_list("));
}

#[test]
fn test_rust_expected_is_annotated_with_return_type() {
    let starter_code = "impl Solution {\n    pub fn three_sum(nums: Vec<i32>) \
                        -> Vec<Vec<i32>> {\n        todo!()\n    }\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[0,1,1]".to_string()],
        outputs:       vec!["[]".to_string()],
    };

    let mut generator = TestGenerator::new(&starter_code, test_data);
    let result = generator.run(&ProgrammingLanguage::Rust).unwrap();

    assert!(result.contains("let expected: Vec<Vec<i32>> = vec![];"));
}

#[test]
fn test_rust_inputs_not_matching_parameters_is_an_error() {
    // guess number higher or lower: `pick` is hidden behind the guess api
    let starter_code = "impl Solution {\n    unsafe fn guessNumber(n: i32) -> \
                        i32 {\n        todo!()\n    }\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["10,6".to_string()],
        outputs:       vec!["6".to_string()],
    };

    let mut generator = TestGenerator::new(&starter_code, test_data);
    assert_eq!(
        generator.run(&ProgrammingLanguage::Rust).unwrap_err(),
        TestGeneratorError::InputMismatch
    );
}

#[test]
fn test_rust_custom_judge_output_checks_length_and_head() {
    let starter_code = "impl Solution {\n    pub fn remove_element(nums: &mut \
                        Vec<i32>, val: i32) -> i32 {\n        todo!()\n    \
                        }\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[3,2,2,3],3".to_string()],
        outputs:       vec!["2, nums = [2,2,_,_]".to_string()],
    };

    let mut generator = TestGenerator::new(&starter_code, test_data);
    let result = generator.run(&ProgrammingLanguage::Rust).unwrap();

    assert!(result.contains("let expected: i32 = 2;"));
    assert!(result.contains("assert_eq!(result, expected);"));
    assert!(result.contains("let mut expected_arg0: Vec<i32> = vec![2, 2];"));
    assert!(
        result.contains("let mut head = arg0[..expected_arg0.len()].to_vec();")
    );
    assert!(result.contains("assert_eq!(head, expected_arg0);"));
}

#[test]
fn test_rust_float_outputs_are_compared_with_tolerance() {
    let starter_code = "impl Solution {\n    pub fn my_pow(x: f64, n: i32) -> \
                        f64 {\n        todo!()\n    }\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["2.10000,3".to_string()],
        outputs:       vec!["9.26100".to_string()],
    };

    let mut generator = TestGenerator::new(&starter_code, test_data);
    let result = generator.run(&ProgrammingLanguage::Rust).unwrap();

    assert!(result.contains("let expected: f64 = 9.26100;"));
    assert!(result.contains("assert!((result - expected).abs() < 1e-5"));
    assert!(!result.contains("assert_eq!"));
}

#[test]
fn test_rust_float_vec_outputs_are_compared_with_tolerance() {
    let starter_code = "impl Solution {\n    pub fn f(nums: Vec<i32>) -> \
                        Vec<f64> {\n        todo!()\n    }\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[1]".to_string()],
        outputs:       vec!["[0.50000,-1.00000]".to_string()],
    };

    let mut generator = TestGenerator::new(&starter_code, test_data);
    let result = generator.run(&ProgrammingLanguage::Rust).unwrap();

    assert!(result.contains("assert_eq!(result.len(), expected.len());"));
    assert!(result.contains("assert!((a - e).abs() < 1e-5"));
}

#[test]
fn test_rust_any_order_outputs_are_sorted() {
    let starter_code = "impl Solution {\n    pub fn three_sum(nums: Vec<i32>) \
                        -> Vec<Vec<i32>> {\n        todo!()\n    }\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[-1,0,1,2,-1,-4]".to_string()],
        outputs:       vec!["[[-1,-1,2],[-1,0,1]]".to_string()],
    };

    let result = TestGenerator::new(&starter_code, test_data)
        .any_order(true)
        .run(&ProgrammingLanguage::Rust)
        .unwrap();

    assert!(result.contains("let mut result = result;"));
    assert!(result.contains("result.iter_mut().for_each(|v| v.sort());"));
    assert!(result.contains("expected.sort();"));
}

#[test]
fn test_rust_any_order_skips_floats_and_scalars() {
    for (ret, output) in [("Vec<f64>", "[1.0]"), ("i32", "1")] {
        let starter_code = format!(
            "impl Solution {{\n    pub fn f(n: i32) -> {ret} {{\n        \
             todo!()\n    }}\n}}"
        );
        let test_data = ProblemTestData {
            example_count: 1,
            inputs:        vec!["1".to_string()],
            outputs:       vec![output.to_string()],
        };
        let result = TestGenerator::new(&starter_code, test_data)
            .any_order(true)
            .run(&ProgrammingLanguage::Rust)
            .unwrap();
        assert!(!result.contains(".sort()"));
    }
}

#[test]
fn test_rust_self_methods_are_not_generated() {
    let starter_code = "impl Solution {\n    pub fn first_bad_version(&self, \
                        n: i32) -> i32 {\n        todo!()\n    }\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["5,4".to_string()],
        outputs:       vec!["4".to_string()],
    };
    let result = TestGenerator::new(&starter_code, test_data)
        .run(&ProgrammingLanguage::Rust);
    assert_eq!(result, Err(TestGeneratorError::InputMismatch));
}

#[test]
fn test_rust_design_problem() {
    let starter_code = "struct MinStack {\n\n}\n\nimpl MinStack {\n    fn \
                        new() -> Self {\n        todo!()\n    }\n    fn \
                        push(&self, val: i32) {\n        todo!()\n    }\n    \
                        fn get_min(&self) -> i32 {\n        todo!()\n    }\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec![
            "[\"MinStack\",\"push\",\"getMin\"] [[],[-2],[]]".to_string()
        ],
        outputs:       vec!["[null,null,-2]".to_string()],
    };

    let result = TestGenerator::new(&starter_code, test_data)
        .run(&ProgrammingLanguage::Rust)
        .unwrap();

    assert!(result.contains("let mut obj = MinStack::new();"));
    assert!(result.contains("obj.push(-2);"));
    assert!(result.contains("let expected: i32 = -2;"));
    assert!(result.contains("let result = obj.get_min();"));
}

#[test]
fn test_rust_design_problem_unknown_method_is_an_error() {
    let starter_code =
        "struct A {}\nimpl A {\n    fn new() -> Self {\n        todo!()\n    \
         }\n}"
            .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[\"A\",\"missing\"] [[],[]]".to_string()],
        outputs:       vec!["[null,1]".to_string()],
    };
    let result = TestGenerator::new(&starter_code, test_data)
        .run(&ProgrammingLanguage::Rust);
    assert_eq!(result, Err(TestGeneratorError::InputMismatch));
}

#[test]
fn test_rust_judge_interface_parameters_are_not_generated() {
    let starter_code = "impl Solution {\n    pub fn \
                        find_in_mountain_array(target: i32, mountainArr: \
                        &MountainArray) -> i32 {\n        todo!()\n    }\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["3,[1,2,3,4,5,3,1]".to_string()],
        outputs:       vec!["2".to_string()],
    };
    let result = TestGenerator::new(&starter_code, test_data)
        .run(&ProgrammingLanguage::Rust);
    assert_eq!(result, Err(TestGeneratorError::InputMismatch));
}

#[test]
fn test_rust_design_null_arguments() {
    let starter_code = "struct A {}\nimpl A {\n    fn new(name: String) -> \
                        Self {\n        todo!()\n    }\n    fn order(&self) \
                        -> Vec<String> {\n        todo!()\n    }\n}"
        .to_string();
    let test_data = ProblemTestData {
        example_count: 1,
        inputs:        vec!["[\"A\",\"order\"] [[\"king\"],[null]]".to_string()],
        outputs:       vec!["[null,[\"king\"]]".to_string()],
    };
    let result = TestGenerator::new(&starter_code, test_data)
        .run(&ProgrammingLanguage::Rust)
        .unwrap();
    assert!(result.contains("let result = obj.order();"));
}

#[test]
fn test_rust_many_answers_note() {
    let starter_code =
        "impl Solution {\n    pub fn f(n: i32) -> i32 {\n        todo!()\n    \
         }\n}"
            .to_string();
    let test_data = || ProblemTestData {
        example_count: 1,
        inputs:        vec!["1".to_string()],
        outputs:       vec!["1".to_string()],
    };
    let note = "// several answers are accepted";
    let result = TestGenerator::new(&starter_code, test_data())
        .many_answers(true)
        .run(&ProgrammingLanguage::Rust)
        .unwrap();
    assert!(result.contains(note));
    let result = TestGenerator::new(&starter_code, test_data())
        .run(&ProgrammingLanguage::Rust)
        .unwrap();
    assert!(!result.contains(note));
}
