use clap::Parser;
use leetcode_cli::cli::{Cli, Commands};

#[test]
fn test_cli_info_command() {
    let args = vec!["leetcode_cli", "info", "--id", "1"];
    let cli = Cli::try_parse_from(args).unwrap();
    
    match cli.command {
        Commands::Info { id } => assert_eq!(id, 1),
        _ => panic!("Expected Info command"),
    }
}

#[test]
fn test_cli_start_command() {
    let args = vec!["leetcode_cli", "start", "--id", "1", "--lang", "rust"];
    let cli = Cli::try_parse_from(args).unwrap();
    
    match cli.command {
        Commands::Start { id, language } => {
            assert_eq!(id, 1);
            assert_eq!(language, "rust");
        },
        _ => panic!("Expected Start command"),
    }
}

#[test]
fn test_cli_test_command() {
    let args = vec!["leetcode_cli", "test", "--id", "1", "--file", "main.rs"];
    let cli = Cli::try_parse_from(args).unwrap();
    
    match cli.command {
        Commands::Test { id, path_to_file } => {
            assert_eq!(id, 1);
            assert_eq!(path_to_file, "main.rs");
        },
        _ => panic!("Expected Test command"),
    }
}

#[test]
fn test_cli_submit_command() {
    let args = vec!["leetcode_cli", "submit", "--id", "1", "--file", "solution.py"];
    let cli = Cli::try_parse_from(args).unwrap();
    
    match cli.command {
        Commands::Submit { id, path_to_file } => {
            assert_eq!(id, 1);
            assert_eq!(path_to_file, "solution.py");
        },
        _ => panic!("Expected Submit command"),
    }
}

#[test]
fn test_cli_debug_command() {
    let args = vec!["leetcode_cli", "debug"];
    let cli = Cli::try_parse_from(args).unwrap();
    
    match cli.command {
        Commands::Debug {} => (),
        _ => panic!("Expected Debug command"),
    }
}

#[test]
fn test_cli_invalid_command() {
    let args = vec!["leetcode_cli", "invalid"];
    let result = Cli::try_parse_from(args);
    assert!(result.is_err());
}