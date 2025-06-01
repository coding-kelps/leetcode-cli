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
        utils::parse_programming_language("rust"),
        ProgrammingLanguage::Rust,
    );
    assert_language(
        utils::parse_programming_language("python3"),
        ProgrammingLanguage::Python3,
    );
    assert_language(
        utils::parse_programming_language("javascript"),
        ProgrammingLanguage::JavaScript,
    );
}

#[test]
fn test_extension_programming_language() {
    // Test valid extensions to programming languages
    assert_language(
        utils::extension_programming_language("rs"),
        ProgrammingLanguage::Rust,
    );
    assert_language(
        utils::extension_programming_language("py"),
        ProgrammingLanguage::Python3,
    );
    assert_language(
        utils::extension_programming_language("js"),
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
