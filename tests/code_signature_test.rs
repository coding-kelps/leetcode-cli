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
