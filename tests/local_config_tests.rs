use leetcode_cli::local_config::LocalConfig;
use tempfile::TempDir;

#[test]
fn test_local_config_creation() {
    let config = LocalConfig::new(
        42,
        "valid_parentheses".to_string(),
        "Rust".to_string(),
    );

    assert_eq!(config.problem_id, 42);
    assert_eq!(config.problem_name, "valid_parentheses");
    assert_eq!(config.language, "Rust");
}

#[test]
fn test_write_and_read_config() {
    let temp_dir = TempDir::new().unwrap();
    let config =
        LocalConfig::new(1, "two_sum".to_string(), "Python".to_string());

    // Write config
    config.write_to_dir(temp_dir.path()).unwrap();

    // Verify file exists
    let config_path = temp_dir.path().join(".leetcode-cli");
    assert!(config_path.exists());

    // Read config back
    let read_config = LocalConfig::read_from_path(&config_path).unwrap();
    assert_eq!(read_config.problem_id, 1);
    assert_eq!(read_config.problem_name, "two_sum");
    assert_eq!(read_config.language, "Python");
}

#[test]
fn test_get_main_file_rust() {
    let config = LocalConfig::new(1, "two_sum".to_string(), "Rust".to_string());
    assert_eq!(config.get_main_file(), "main.rs");
}

#[test]
fn test_get_main_file_python() {
    let config =
        LocalConfig::new(1, "two_sum".to_string(), "Python".to_string());
    assert_eq!(config.get_main_file(), "main.py");
}

#[test]
fn test_get_main_file_javascript() {
    let config =
        LocalConfig::new(1, "two_sum".to_string(), "JavaScript".to_string());
    assert_eq!(config.get_main_file(), "main.js");
}

#[test]
fn test_find_config_not_found() {
    // Change to temp dir where no config exists
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();

    std::env::set_current_dir(temp_dir.path()).unwrap();

    let result = LocalConfig::find_and_read().unwrap();
    assert!(result.is_none());

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_find_config_in_current_dir() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();

    // Create config in temp dir
    let config =
        LocalConfig::new(123, "test_problem".to_string(), "Go".to_string());
    config.write_to_dir(temp_dir.path()).unwrap();

    // Change to temp dir
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Find config
    let found_config = LocalConfig::find_and_read().unwrap().unwrap();
    assert_eq!(found_config.problem_id, 123);
    assert_eq!(found_config.problem_name, "test_problem");
    assert_eq!(found_config.language, "Go");

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_find_config_in_parent_dir() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();

    // Create config in temp dir
    let config = LocalConfig::new(
        456,
        "parent_test".to_string(),
        "TypeScript".to_string(),
    );
    config.write_to_dir(temp_dir.path()).unwrap();

    // Create subdirectory
    let sub_dir = temp_dir.path().join("subdir");
    std::fs::create_dir(&sub_dir).unwrap();

    // Change to subdirectory
    std::env::set_current_dir(&sub_dir).unwrap();

    // Find config (should find it in parent)
    let found_config = LocalConfig::find_and_read().unwrap().unwrap();
    assert_eq!(found_config.problem_id, 456);
    assert_eq!(found_config.problem_name, "parent_test");
    assert_eq!(found_config.language, "TypeScript");

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_get_main_file_case_insensitive() {
    // Test uppercase
    let config = LocalConfig::new(1, "test".to_string(), "RUST".to_string());
    assert_eq!(config.get_main_file(), "main.rs");

    // Test mixed case
    let config = LocalConfig::new(1, "test".to_string(), "PyThOn".to_string());
    assert_eq!(config.get_main_file(), "main.py");

    // Test lowercase
    let config =
        LocalConfig::new(1, "test".to_string(), "typescript".to_string());
    assert_eq!(config.get_main_file(), "main.ts");

    // Test C++ case insensitive (special case with symbols)
    let config = LocalConfig::new(1, "test".to_string(), "C++".to_string());
    assert_eq!(config.get_main_file(), "main.cpp");
}

#[test]
fn test_resolve_problem_params_with_both_args() {
    let result = LocalConfig::resolve_problem_params(
        Some(42),
        Some("custom.rs".to_string()),
    );
    assert!(result.is_ok());
    let (id, path) = result.unwrap();
    assert_eq!(id, 42);
    assert_eq!(path, "custom.rs");
}

#[test]
fn test_resolve_problem_params_no_args_no_config() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();

    // Change to temp dir where no config exists
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let result = LocalConfig::resolve_problem_params(None, None);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("No problem ID provided"));

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_resolve_problem_params_with_config() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();

    // Create config in temp dir
    let config =
        LocalConfig::new(123, "test_problem".to_string(), "Rust".to_string());
    config.write_to_dir(temp_dir.path()).unwrap();

    // Change to temp dir
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Test resolution without args (should use config)
    let result = LocalConfig::resolve_problem_params(None, None);
    assert!(result.is_ok());
    let (id, path) = result.unwrap();
    assert_eq!(id, 123);
    assert_eq!(path, "src/main.rs");

    // Test resolution with partial args (should mix CLI and config)
    let result = LocalConfig::resolve_problem_params(Some(456), None);
    assert!(result.is_ok());
    let (id, path) = result.unwrap();
    assert_eq!(id, 456); // From CLI
    assert_eq!(path, "src/main.rs"); // From config

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}
