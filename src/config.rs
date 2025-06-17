use std::{
    fs,
    io::{self},
    path::PathBuf,
};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigFile {
    pub leetcode_token: String,
    pub default_language: Option<String>,
    pub leetcode_dir_path: Option<PathBuf>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RuntimeConfigSetup {
    pub home_dir: PathBuf,
    pub config_dir: PathBuf,
    pub config_file: PathBuf,
    pub config: ConfigFile,
}

impl Default for RuntimeConfigSetup {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeConfigSetup {
    pub fn new() -> Self {
        let home_dir =
            dirs::home_dir().expect("Unable to determine home directory");
        let config_dir = home_dir.join(".config/leetcode-cli");
        let config_file = config_dir.join("config.toml");
        let default_leetcode_dir = home_dir.join("leetcode");

        RuntimeConfigSetup {
            home_dir,
            config_dir,
            config_file,
            config: ConfigFile {
                leetcode_token: String::new(),
                default_language: None,
                leetcode_dir_path: Some(default_leetcode_dir),
            },
        }
    }
    /// create a config file in ~/.config/leetcode-cli/config.toml
    fn create_config_file(&self) {
        std::fs::create_dir_all(&self.config_dir)
            .expect("Unable to create directory");
        let config_file = self.config_dir.join("config.toml");
        std::fs::File::create(&config_file).expect("Unable to create file");
    }
    /// check for a config file in ~/.config/leetcode-cli/config.toml
    /// read it or create it with default values if it doesn't exist
    /// load the config in Config struct and check if the token is valid
    pub fn status(&mut self) -> Result<(), io::Error> {
        if self.config_file.is_file() {
            let config_file = std::fs::read_to_string(&self.config_file)
                .expect("Unable to read file");
            let parsed_config: ConfigFile =
                toml::from_str(&config_file).expect("Unable to parse config");
            if !parsed_config.leetcode_token.is_empty() {
                self.config.leetcode_token = parsed_config.leetcode_token;
            }
            if parsed_config.default_language.is_some() {
                self.config.default_language = parsed_config.default_language;
            }
            if parsed_config.leetcode_dir_path.is_some() {
                self.config.leetcode_dir_path = parsed_config.leetcode_dir_path;
            }
        } else {
            self.create_config_file();
        }
        Ok(())
    }

    /// Resolve the configured LeetCode directory, expand ~, canonicalize, and
    /// create if missing.
    pub fn resolve_leetcode_dir(&self) -> io::Result<PathBuf> {
        let raw = if let Some(ref custom) = self.config.leetcode_dir_path {
            custom.to_string_lossy()
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No leetcode_dir_path set",
            ));
        };
        let raw_str = raw.as_ref();
        let mut path = if raw_str == "~" {
            self.home_dir.clone()
        } else if let Some(stripped) = raw_str.strip_prefix("~/") {
            let home = self.home_dir.clone();
            home.join(stripped)
        } else {
            PathBuf::from(raw_str)
        };
        path = fs::canonicalize(&path).unwrap_or(path);
        fs::create_dir_all(&path)?;
        Ok(path)
    }
}
