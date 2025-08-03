use std::{
    fs,
    io,
    path::Path,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LocalConfig {
    pub problem_id:   u32,
    pub problem_name: String,
    pub language:     String,
}

impl LocalConfig {
    pub fn new(
        problem_id: u32, problem_name: String, language: String,
    ) -> Self {
        Self { problem_id, problem_name, language }
    }

    /// Find and read local config from current directory or parent directories
    pub fn find_and_read() -> io::Result<Option<Self>> {
        let mut current_dir = std::env::current_dir()?;

        loop {
            let config_path = current_dir.join(".leetcode-cli");
            if config_path.exists() {
                let content = fs::read_to_string(&config_path)?;
                let config: LocalConfig =
                    toml::from_str(&content).map_err(|e| {
                        io::Error::new(io::ErrorKind::InvalidData, e)
                    })?;
                return Ok(Some(config));
            }

            if !current_dir.pop() {
                break;
            }
        }

        Ok(None)
    }

    /// Write local config to specified directory
    pub fn write_to_dir(&self, dir: &Path) -> io::Result<()> {
        let config_path = dir.join(".leetcode-cli");
        let content = toml::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(config_path, content)
    }

    /// Read local config from specified file path
    pub fn read_from_path(path: &Path) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        toml::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Get the main source file name based on language
    pub fn get_main_file(&self) -> String {
        match self.language.to_lowercase().as_str() {
            "rust" => "main.rs".to_string(),
            "python" | "python3" => "main.py".to_string(),
            "javascript" => "main.js".to_string(),
            "typescript" => "main.ts".to_string(),
            "go" => "main.go".to_string(),
            "java" => "Main.java".to_string(),
            "c++" => "main.cpp".to_string(),
            "c" => "main.c".to_string(),
            _ => "main.txt".to_string(),
        }
    }

    /// Resolve problem ID and file path from CLI args or local config
    pub fn resolve_problem_params(
        id: Option<u32>, path_to_file: Option<String>,
    ) -> io::Result<(u32, String)> {
        match (id, &path_to_file) {
            (Some(id), Some(path)) => Ok((id, path.clone())),
            _ => {
                // Try to find local config
                match Self::find_and_read()? {
                    Some(config) => {
                        let problem_id = id.unwrap_or(config.problem_id);
                        let file_path = path_to_file.unwrap_or_else(|| {
                            format!("src/{}", config.get_main_file())
                        });
                        Ok((problem_id, file_path))
                    },
                    None => {
                        if id.is_none() {
                            return Err(io::Error::new(
                                io::ErrorKind::NotFound,
                                "No problem ID provided and no .leetcode-cli \
                                 config found. Either provide --id or run \
                                 from a problem directory",
                            ));
                        }
                        if path_to_file.is_none() {
                            return Err(io::Error::new(
                                io::ErrorKind::NotFound,
                                "No file path provided",
                            ));
                        }
                        Ok((id.unwrap(), path_to_file.unwrap()))
                    },
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_local_config_creation() {
        let config =
            LocalConfig::new(1, "two_sum".to_string(), "Rust".to_string());

        assert_eq!(config.problem_id, 1);
        assert_eq!(config.problem_name, "two_sum");
        assert_eq!(config.language, "Rust");
    }

    #[test]
    fn test_write_and_read_config() {
        let temp_dir = TempDir::new().unwrap();
        let config =
            LocalConfig::new(1, "two_sum".to_string(), "Rust".to_string());

        config.write_to_dir(temp_dir.path()).unwrap();

        let config_path = temp_dir.path().join(".leetcode-cli");
        assert!(config_path.exists());

        let read_config = LocalConfig::read_from_path(&config_path).unwrap();
        assert_eq!(read_config.problem_id, 1);
        assert_eq!(read_config.problem_name, "two_sum");
        assert_eq!(read_config.language, "Rust");
    }

    #[test]
    fn test_get_main_file() {
        let config =
            LocalConfig::new(1, "two_sum".to_string(), "Rust".to_string());
        assert_eq!(config.get_main_file(), "main.rs");

        let config =
            LocalConfig::new(1, "two_sum".to_string(), "Python".to_string());
        assert_eq!(config.get_main_file(), "main.py");
    }
}
