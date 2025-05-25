use std::{
    fs,
    io,
    path::PathBuf,
};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    home_dir:              Option<PathBuf>,
    config_dir:            Option<PathBuf>,
    config_file:           Option<PathBuf>,
    pub leetcode_token:    Option<String>,
    pub default_language:  Option<String>,
    pub leetcode_dir_path: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        let home_dir =
            dirs::home_dir().expect("Unable to determine home directory");
        let config_dir = home_dir.join(".config/leetcode-cli");
        let config_file = config_dir.join("config.toml");
        let default_leetcode_dir = home_dir.join("leetcode");

        Config {
            home_dir:          Some(home_dir),
            config_dir:        Some(config_dir),
            config_file:       Some(config_file),
            leetcode_token:    None,
            default_language:  None,
            leetcode_dir_path: Some(default_leetcode_dir),
        }
    }
    /// create a config file in ~/.config/leetcode-cli/config.toml
    fn create_config_file(&self) {
        let config_dir =
            self.config_dir.as_ref().expect("Config directory not set");

        std::fs::create_dir_all(&config_dir)
            .expect("Unable to create directory");
        let config_file = config_dir.join("config.toml");
        std::fs::File::create(&config_file).expect("Unable to create file");
    }
    /// check for a config file in ~/.config/leetcode-cli/config.toml
    /// read it or create it if it doesn't exist with default values
    /// load the config in Config struct and check if the token is valid
    /// if the token is not valid, generate a new one
    pub async fn status(&mut self) -> Result<(), reqwest::Error> {
        if self.config_file.is_some() {
            let config_file =
                std::fs::read_to_string(self.config_file.as_ref().unwrap())
                    .expect("Unable to read file");
            let config: Config =
                toml::from_str(&config_file).expect("Unable to parse file");
            self.leetcode_token = config.leetcode_token;
            self.default_language = config.default_language;
            if let Some(custom) = config.leetcode_dir_path {
                self.leetcode_dir_path = Some(PathBuf::from(custom));
            }
            self.check_token().await?;
        } else {
            self.create_config_file();
            self.generate_token();
        }
        Ok(())
    }

    async fn test_token_validity(&self) -> Result<bool, reqwest::Error> {
        let token = self.leetcode_token.as_ref().unwrap();
        let url = format!(
            "https://leetcode.com/api/user/check_token/?token={}",
            token
        );
        let response = reqwest::get(&url).await.unwrap();
        if response.status().is_success() {
            return Ok(true);
        }
        Ok(false)
    }

    async fn check_token(&mut self) -> Result<bool, reqwest::Error> {
        let token = self.leetcode_token.as_ref();
        if token.is_none() {
            println!("Leetcode token not found");
            println!("Let's set it together.");
            self.generate_token();
            return Ok(true);
        }
        self.test_token_validity().await
    }

    /// Generate a new LeetCode token by prompting the user to visit the
    /// LeetCode login page and copy the token
    fn generate_token(&mut self) {
        unimplemented!(
            "generating token is not implemented yet. Refer to the README for \
             instructions."
        );
        println!("Generating token...");
        println!("Please visit https://leetcode.com/submissions/#/1");
        println!("and copy the token here:");
        let mut token = String::new();
        std::io::stdin()
            .read_line(&mut token)
            .expect("Failed to read line");
        let token = token.trim();
        if token.is_empty() {
            println!("Token cannot be empty");
            return;
        }
        self.leetcode_token = Some(token.to_string());
        println!("Token set to: {}", token);
    }

    /// Resolve the configured LeetCode directory, expand ~, canonicalize, and
    /// create if missing.
    pub fn resolve_leetcode_dir(&self) -> io::Result<PathBuf> {
        let raw = if let Some(ref custom) = self.leetcode_dir_path {
            custom.to_string_lossy()
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No leetcode_dir_path set",
            ));
        };
        let raw_str = raw.as_ref();
        let mut path = if raw_str == "~" {
            self.home_dir.clone().ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "Unable to find home directory",
            ))?
        } else if raw_str.starts_with("~/") {
            let home = dirs::home_dir().ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "Unable to find home directory",
            ))?;
            home.join(&raw_str[2..])
        } else {
            PathBuf::from(raw_str)
        };
        // Canonicalize parent to normalize .. and . components
        path = fs::canonicalize(&path).unwrap_or(path);
        // Ensure directory exists
        fs::create_dir_all(&path)?;
        Ok(path)
    }
}
