use leetcode_cli::code_signature::*;
use leetcoderustapi::ProgrammingLanguage::*;

#[test]
fn test_code_signature_resolve_declaration_rust_number() {
    let result = CodeSignature::resolve_declaration(&Rust, "42");
    assert_eq!(result, "42");
}

#[test]
fn test_code_signature_resolve_declaration_rust_string() {
    let result = CodeSignature::resolve_declaration(&Rust, "\"Hello, World!\"");
    assert_eq!(result, "\"Hello, World!\".to_string()");
}

#[test]
fn test_code_signature_resolve_declaration_rust_vector() {
    let result = CodeSignature::resolve_declaration(&Rust, "[1, 2, 3]");
    assert_eq!(result, "vec![1, 2, 3]");
}

#[test]
fn test_code_signature_resolve_declaration_rust_vec_of_strings() {
    let result = CodeSignature::resolve_declaration(
        &Rust,
        "[\"abc\", \"bde\", \"cfg\"]",
    );
    assert_eq!(
        result,
        "vec![\"abc\".to_string(), \"bde\".to_string(), \"cfg\".to_string()]"
    );
}

#[test]
fn test_parse_rust_signature_parameters() {
    let starter = "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, \
                   target: i32) -> Vec<i32> {\n        \n    }\n}";
    let sig = CodeSignature::parse_code_signature(&Rust, starter).unwrap();
    assert_eq!(sig.function_name, "two_sum");
    assert_eq!(sig.parameters, vec!["nums: Vec<i32>", "target: i32"]);
}

#[test]
fn test_parse_rust_signature_generic_parameters() {
    let starter = "impl Solution {\n    pub fn f(m: HashMap<i32, i32>, g: \
                   impl Fn(i32) -> i32) -> i32 {\n    }\n}";
    let sig = CodeSignature::parse_code_signature(&Rust, starter).unwrap();
    assert_eq!(sig.parameters, vec![
        "m: HashMap<i32, i32>",
        "g: impl Fn(i32) -> i32"
    ]);
}

#[test]
fn test_parse_rust_signature_without_parameters_does_not_panic() {
    let starter = "impl Solution {\n    pub fn f() -> i32 {\n    }\n}";
    let sig = CodeSignature::parse_code_signature(&Rust, starter).unwrap();
    assert_eq!(sig.function_name, "f");
    assert!(sig.parameters.is_empty());
}

#[test]
fn test_parse_python_signature_without_parameters_does_not_panic() {
    let sig =
        CodeSignature::parse_code_signature(&Python, "def f():\n    pass")
            .unwrap();
    assert_eq!(sig.function_name, "f");
    assert!(sig.parameters.is_empty());
}

#[test]
fn test_parse_signature_without_parenthesis_is_an_error() {
    assert!(CodeSignature::parse_code_signature(
        &Rust,
        "impl Solution { fn broken }"
    )
    .is_err());
    assert!(CodeSignature::parse_code_signature(&Python, "def broken").is_err());
}

#[test]
fn test_parse_rust_signature_skips_commented_list_node() {
    let starter = "// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: \
                   Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    }
}";
    let sig = CodeSignature::parse_code_signature(&Rust, starter).unwrap();
    assert_eq!(sig.function_name, "add_two_numbers");
    assert_eq!(sig.parameters, vec![
        "l1: Option<Box<ListNode>>",
        "l2: Option<Box<ListNode>>"
    ]);
}

#[test]
fn test_parse_rust_signature_design_problem_is_an_error() {
    let starter = "struct MinStack {\n\n}\n\n/**\n * `&self` means the method \
                   takes an immutable reference.\n */\nimpl MinStack {\n    \
                   fn new() -> Self {\n        \n    }\n}";
    assert_eq!(
        CodeSignature::parse_code_signature(&Rust, starter).unwrap_err(),
        CodeSignatureError::NoSolution
    );
}

#[test]
fn test_parse_rust_signature_return_type() {
    let starter = "impl Solution {\n    pub fn f(nums: Vec<i32>) -> \
                   Vec<Vec<char>> {\n    }\n}";
    let sig = CodeSignature::parse_code_signature(&Rust, starter).unwrap();
    assert_eq!(sig.return_type.as_deref(), Some("Vec<Vec<char>>"));
    assert_eq!(sig.parameter_types(), vec![Some("Vec<i32>".to_string())]);

    let starter =
        "impl Solution {\n    pub fn f(s: &mut Vec<char>) {\n    }\n}";
    let sig = CodeSignature::parse_code_signature(&Rust, starter).unwrap();
    assert_eq!(sig.return_type, None);
}

#[test]
fn test_resolve_rust_typed_declaration_char() {
    assert_eq!(
        CodeSignature::resolve_rust_typed_declaration(
            "[[\"1\",\"0\"],[\"'\",\"\\\\\"]]",
            Some("Vec<Vec<char>>")
        ),
        "vec![vec!['1', '0'], vec!['\\'', '\\\\']]"
    );
    assert_eq!(
        CodeSignature::resolve_rust_typed_declaration(
            "[\"h\",\"e\"]",
            Some("&mut Vec<char>")
        ),
        "vec!['h', 'e']"
    );
    assert_eq!(
        CodeSignature::resolve_rust_typed_declaration(
            "[\"h\",\"e\"]",
            Some("Vec<String>")
        ),
        "vec![\"h\".to_string(), \"e\".to_string()]"
    );
}

#[test]
fn test_resolve_rust_typed_declaration_float() {
    assert_eq!(
        CodeSignature::resolve_rust_typed_declaration("2", Some("f64")),
        "2.0"
    );
    assert_eq!(
        CodeSignature::resolve_rust_typed_declaration("2.50000", Some("f64")),
        "2.50000"
    );
    assert_eq!(
        CodeSignature::resolve_rust_typed_declaration(
            "[1,2]",
            Some("Vec<f64>")
        ),
        "vec![1.0, 2.0]"
    );
    assert_eq!(
        CodeSignature::resolve_rust_typed_declaration("2", Some("i32")),
        "2"
    );
}

#[test]
fn test_resolve_rust_typed_declaration_non_ascii_string() {
    assert_eq!(
        CodeSignature::resolve_rust_typed_declaration("\"héllo\"", None),
        "\"héllo\".to_string()"
    );
}

#[test]
fn test_resolve_rust_typed_declaration_list_and_tree() {
    assert_eq!(
        CodeSignature::resolve_rust_typed_declaration(
            "[1,2,3]",
            Some("Option<Box<ListNode>>")
        ),
        "to_list(vec![1, 2, 3])"
    );
    assert_eq!(
        CodeSignature::resolve_rust_typed_declaration(
            "[[1,4],[]]",
            Some("Vec<Option<Box<ListNode>>>")
        ),
        "vec![to_list(vec![1, 4]), to_list(vec![])]"
    );
    assert_eq!(
        CodeSignature::resolve_rust_typed_declaration(
            "[1,null,2]",
            Some("Option<Rc<RefCell<TreeNode>>>")
        ),
        "to_tree(vec![Some(1), None, Some(2)])"
    );
    assert_eq!(
        CodeSignature::resolve_rust_typed_declaration(
            "[]",
            Some("Option<Rc<RefCell<TreeNode>>>")
        ),
        "to_tree(vec![])"
    );
}

#[test]
fn test_parse_rust_signature_design_problem_named_solution_is_an_error() {
    let starter = "struct Solution {\n\n}\n\nimpl Solution {\n    fn \
                   new(nums: Vec<i32>) -> Self {\n        \n    }\n}";
    assert_eq!(
        CodeSignature::parse_code_signature(&Rust, starter).unwrap_err(),
        CodeSignatureError::NoSolution
    );
}

#[test]
fn test_resolve_rust_nested_integer() {
    assert_eq!(
        CodeSignature::resolve_rust_typed_declaration(
            "324",
            Some("NestedInteger")
        ),
        "NestedInteger::Int(324)"
    );
    assert_eq!(
        CodeSignature::resolve_rust_typed_declaration(
            "[123,[456]]",
            Some("NestedInteger")
        ),
        "NestedInteger::List(vec![NestedInteger::Int(123), \
         NestedInteger::List(vec![NestedInteger::Int(456)])])"
    );
}

#[test]
fn test_parse_rust_design_skips_node_definitions() {
    let starter = "pub struct TreeNode {}\nimpl TreeNode {\n    pub fn \
                   new(val: i32) -> Self {}\n}\nstruct BSTIterator {}\nimpl \
                   BSTIterator {\n    fn new(root: \
                   Option<Rc<RefCell<TreeNode>>>) -> Self {}\n    fn \
                   has_next(&self) -> bool {}\n}";
    let methods = CodeSignature::parse_rust_design(starter).unwrap();
    assert_eq!(methods.len(), 2);
    assert_eq!(methods[0].class_name.as_deref(), Some("BSTIterator"));
    assert_eq!(methods[1].function_name, "has_next");
    assert_eq!(methods[1].return_type.as_deref(), Some("bool"));
}

#[test]
fn test_parse_rust_design_random_solution_is_an_error() {
    let starter = "struct Solution {}\nimpl Solution {\n    fn new(nums: \
                   Vec<i32>) -> Self {}\n}";
    assert_eq!(
        CodeSignature::parse_rust_design(starter).unwrap_err(),
        CodeSignatureError::NoSolution
    );
}
