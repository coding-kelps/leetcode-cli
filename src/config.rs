use std::{
    fs,
    io::{
        self,
        Error,
    },
    path::PathBuf,
};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigFile {
    pub leetcode_token:    String,
    pub default_language:  Option<String>,
    pub leetcode_dir_path: Option<PathBuf>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RuntimeConfigSetup {
    pub home_dir:    PathBuf,
    pub config_dir:  PathBuf,
    pub config_file: PathBuf,
    pub config:      ConfigFile,
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
                leetcode_token:    String::new(),
                default_language:  None,
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
        // check if the config file exists and is readable
        if self.config_file.is_file() {
            let config_file = std::fs::read_to_string(&self.config_file)
                .expect("Unable to read file");
            let _: ConfigFile =
                toml::from_str(&config_file).expect("Unable to parse config");
            self.check_token()?;
        } else {
            self.create_config_file();
        }
        Ok(())
    }

    fn check_token(&mut self) -> Result<bool, io::Error> {
        if !self.config.leetcode_token.is_empty() {
            return Ok(true);
        }
        println!("No Leetcode token found.");
        Err(Error::new(
            io::ErrorKind::NotFound,
            "Leetcode token not found. Please set it in the config file \
             following readme instructions.",
        ))
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
        } else if raw_str.starts_with("~/") {
            let home = self.home_dir.clone();
            home.join(&raw_str[2..])
        } else {
            PathBuf::from(raw_str)
        };
        path = fs::canonicalize(&path).unwrap_or(path);
        fs::create_dir_all(&path)?;
        Ok(path)
    }
}
