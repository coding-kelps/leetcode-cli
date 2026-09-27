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
    let starter = "impl MinStack {\n    fn new() -> Self {\n    }\n}";
    let sig = CodeSignature::parse_code_signature(&Rust, starter).unwrap();
    assert_eq!(sig.function_name, "new");
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
    assert!(CodeSignature::parse_code_signature(&Rust, "fn broken").is_err());
    assert!(CodeSignature::parse_code_signature(&Python, "def broken").is_err());
}
