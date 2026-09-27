use leetcode_cli::utils;
use leetcoderustapi::ProgrammingLanguage;

fn assert_language(actual: ProgrammingLanguage, expected: ProgrammingLanguage) {
    let actual_str = format!("{:?}", actual);
    let expected_str = format!("{:?}", expected);
    assert_eq!(
        actual_str, expected_str,
        "Languages don't match: got {:?}, expected {:?}",
        actual, expected
    );
}

#[test]
fn test_parse_programming_language() {
    // Test valid programming languages
    assert_language(
        utils::parse_programming_language("rust").unwrap(),
        ProgrammingLanguage::Rust,
    );
    assert_language(
        utils::parse_programming_language("python3").unwrap(),
        ProgrammingLanguage::Python3,
    );
    assert_language(
        utils::parse_programming_language("javascript").unwrap(),
        ProgrammingLanguage::JavaScript,
    );
}

#[test]
fn test_get_language_from_extension() {
    // Test valid extensions to programming languages
    assert_language(
        utils::get_language_from_extension("rs"),
        ProgrammingLanguage::Rust,
    );
    assert_language(
        utils::get_language_from_extension("py"),
        ProgrammingLanguage::Python3,
    );
    assert_language(
        utils::get_language_from_extension("js"),
        ProgrammingLanguage::JavaScript,
    );
}

#[test]
fn test_get_file_name() {
    // Test language to filename mapping
    assert_eq!(utils::get_file_name(&ProgrammingLanguage::Rust), "main.rs");
    assert_eq!(utils::get_file_name(&ProgrammingLanguage::Python3), "main.py");
    assert_eq!(
        utils::get_file_name(&ProgrammingLanguage::JavaScript),
        "main.js"
    );
}

#[test]
fn test_language_to_string() {
    // Test language enum to string conversion
    assert_eq!(utils::language_to_string(&ProgrammingLanguage::Rust), "rust");
    assert_eq!(
        utils::language_to_string(&ProgrammingLanguage::Python3),
        "python3"
    );
}

#[test]
fn test_every_language_has_a_string_and_extension() {
    use ProgrammingLanguage::*;
    for lang in [
        CPP, Java, Python, Python3, C, CSharp, JavaScript, TypeScript, Ruby,
        Swift, Go, Bash, Scala, Kotlin, Rust, PHP, Racket, Erlang, Elixir,
        Dart, Pandas, React,
    ] {
        assert!(!utils::language_to_string(&lang).is_empty());
        assert!(!utils::get_extension_from_language(&lang).is_empty());
    }
    assert_eq!(utils::language_to_string(&Pandas), "pythondata");
    assert_eq!(utils::get_file_name(&Racket), "main.rkt");
    assert_eq!(utils::get_file_name(&Elixir), "main.ex");
}

#[test]
fn test_inject_default_return_value_rust() {
    let starter = "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, \
                   target: i32) -> Vec<i32> {\n        \n    }\n}";
    let result =
        utils::inject_default_return_value(starter, &ProgrammingLanguage::Rust);
    assert_eq!(
        result,
        "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> \
         Vec<i32> {\n        todo!()\n    }\n}"
    );
}

#[test]
fn test_inject_default_return_value_keeps_non_empty_bodies() {
    let starter =
        "// impl ListNode {\n//   fn new(val: i32) -> Self {\n//     ListNode \
         { next: None, val }\n//   }\n// }\nimpl Solution {\n    pub fn f() \
         -> i32 {\n        1\n    }\n}";
    let result =
        utils::inject_default_return_value(starter, &ProgrammingLanguage::Rust);
    assert_eq!(result, starter);
}

#[cfg(test)]
mod file_operations {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_ensure_directory_exists() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().join("test_dir");

        let result = utils::ensure_directory_exists(&test_path);
        assert!(result.is_ok());
        assert!(test_path.exists());
    }

    #[test]
    fn test_write_to_file() {
        let temp_dir = TempDir::new().unwrap();
        let content = "Hello, World!";
        let filename = "test.txt";

        let result = utils::write_to_file(temp_dir.path(), filename, content);
        assert!(result.is_ok());

        let file_path = temp_dir.path().join(filename);
        assert!(file_path.exists());

        let read_content = fs::read_to_string(file_path).unwrap();
        assert_eq!(read_content, content);
    }
}

const LIST_STARTER: &str = "// Definition for singly-linked list.
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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}";

#[test]
fn test_uncomment_local_definitions_rust() {
    let result = utils::uncomment_local_definitions(
        LIST_STARTER,
        &ProgrammingLanguage::Rust,
    );
    assert!(result.contains("\npub struct ListNode {\n"));
    assert!(result.contains("\n  fn new(val: i32) -> Self {\n"));
    assert!(!result.contains("// pub struct ListNode"));
    assert!(!result.contains("Definition for"));
    assert!(result.contains("impl Solution {"));
}

#[test]
fn test_uncomment_local_definitions_list_and_tree() {
    let starter = "// Definition for singly-linked list.\n// pub struct \
                   ListNode {}\n// Definition for a binary tree node.\n// pub \
                   struct TreeNode {}\nimpl Solution {}";
    let result =
        utils::uncomment_local_definitions(starter, &ProgrammingLanguage::Rust);
    assert!(
        result.contains("\npub struct ListNode {}\npub struct TreeNode {}\n")
    );
    assert!(!result.contains("Definition for"));
}

#[test]
fn test_uncomment_local_definitions_without_definitions() {
    let starter =
        "impl Solution {\n    pub fn f() -> i32 {\n        todo!()\n    }\n}";
    assert_eq!(
        utils::uncomment_local_definitions(starter, &ProgrammingLanguage::Rust),
        starter
    );
    assert_eq!(
        utils::uncomment_local_definitions(
            LIST_STARTER,
            &ProgrammingLanguage::Python3
        ),
        LIST_STARTER
    );
}

#[test]
fn test_preprocess_code_removes_local_definitions() {
    let local = utils::uncomment_local_definitions(
        LIST_STARTER,
        &ProgrammingLanguage::Rust,
    );
    let file = format!("pub struct Solution;\n\n{local}\n\nfn main() {{}}\n");
    let sent = utils::preprocess_code(&file, &ProgrammingLanguage::Rust);
    assert!(!sent.contains("struct ListNode"));
    assert!(!sent.contains("leetcode-cli"));
    assert!(!sent.contains("pub struct Solution;"));
    assert!(!sent.contains("fn main"));
    assert!(sent.contains("pub fn reverse_list"));
}
