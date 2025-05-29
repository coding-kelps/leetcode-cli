use std::path::PathBuf;

use leetcode_cli::config::Config;
use tempfile::TempDir;

#[test]
fn test_config_new() {
    let config = Config::new();

    // Check that home_dir is set
    assert!(config.leetcode_dir_path.is_some());

    let expected_path = dirs::home_dir()
        .expect("Unable to determine home directory")
        .join("leetcode");
    assert_eq!(config.leetcode_dir_path.unwrap(), expected_path);
}

#[test]
fn test_resolve_leetcode_dir_with_tilde() {
    let mut config = Config::new();
    config.leetcode_dir_path = Some(PathBuf::from("~/leetcode"));

    let result = config.resolve_leetcode_dir();
    assert!(result.is_ok());

    let resolved_path = result.unwrap();
    assert!(resolved_path.is_absolute());
    assert!(resolved_path.ends_with("leetcode"));
}

#[test]
fn test_resolve_leetcode_dir_absolute_path() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = Config::new();
    config.leetcode_dir_path = Some(temp_dir.path().to_path_buf());

    let result = config.resolve_leetcode_dir();
    assert!(result.is_ok());

    let resolved_path = result.unwrap();
    assert!(resolved_path.exists());
}

#[test]
fn test_resolve_leetcode_dir_no_path() {
    let mut config = Config::new();
    config.leetcode_dir_path = None;

    let result = config.resolve_leetcode_dir();
    assert!(result.is_err());
}

#[test]
fn test_config_file_creation() {
    let home_dir =
        dirs::home_dir().expect("Unable to determine home directory");
    let config_file = home_dir.join(".config/leetcode-cli/config.toml");
    let mut config = Config::new();
    match config.status() {
        Ok(_) => {
            // If the status check passes, the config file should exist
            assert!(
                config_file.exists(),
                "Config file should exist at: {:?}",
                config_file
            );
        },
        Err(e) => {
            panic!("Failed to check config status: {}", e);
        },
    }
}
