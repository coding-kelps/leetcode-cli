use std::path::PathBuf;

use leetcode_cli::config::RuntimeConfigSetup;
use tempfile::TempDir;

#[test]
fn test_config_new() {
    let rcs = RuntimeConfigSetup::new();

    assert!(rcs.home_dir.exists());

    let expected_path = dirs::home_dir()
        .expect("Unable to determine home directory")
        .join("leetcode");
    assert_eq!(rcs.config.leetcode_dir_path.unwrap(), expected_path);
}

#[test]
fn test_resolve_leetcode_dir_with_tilde() {
    let mut rcs = RuntimeConfigSetup::new();
    rcs.config.leetcode_dir_path = Some(PathBuf::from("~/leetcode"));

    let result = rcs.resolve_leetcode_dir();
    assert!(result.is_ok());

    let resolved_path = result.unwrap();
    assert!(resolved_path.is_absolute());
    assert!(resolved_path.ends_with("leetcode"));
}

#[test]
fn test_resolve_leetcode_dir_absolute_path() {
    let temp_dir = TempDir::new().unwrap();
    let mut rcs = RuntimeConfigSetup::new();
    rcs.config.leetcode_dir_path = Some(temp_dir.path().to_path_buf());

    let result = rcs.resolve_leetcode_dir();
    assert!(result.is_ok());

    let resolved_path = result.unwrap();
    assert!(resolved_path.exists());
}

#[test]
fn test_resolve_leetcode_dir_no_path() {
    let mut rcs = RuntimeConfigSetup::new();
    rcs.config.leetcode_dir_path = None;

    let result = rcs.resolve_leetcode_dir();
    assert!(result.is_err());
}

#[test]
fn test_config_file_creation() {
    let home_dir =
        dirs::home_dir().expect("Unable to determine home directory");
    let config_file = home_dir.join(".config/leetcode-cli/config.toml");
    let mut config = RuntimeConfigSetup::new();
    match config.status() {
        Ok(_) => {
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
