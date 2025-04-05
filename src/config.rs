#![allow(dead_code)]
#![allow(unreachable_code)]

use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config
{
    home_dir: Option<PathBuf>,
    config_dir: Option<PathBuf>,
    config_file: Option<PathBuf>,
    pub leetcode_token: Option<String>,
}

impl Config
{
    pub fn new() -> Self
    {
        let home_dir = dirs::home_dir().expect("Unable to determine home directory");
        let config_dir = home_dir.join(".config/leetcode-cli");
        let config_file = config_dir.join("config.toml");

        Config {
            home_dir: Some(home_dir),
            config_dir: Some(config_dir),
            config_file: Some(config_file),
            leetcode_token: None,
        }
    }
    /// create a config file in ~/.config/leetcode-cli/config.toml
    fn create_config_file(&self)
    {
        let home_dir =
            dirs::home_dir().expect("Unable to determine home directory");
        let config_dir = home_dir.join(".config/leetcode-cli");

        std::fs::create_dir_all(&config_dir)
            .expect("Unable to create directory");
        let config_file = config_dir.join("config.toml");
        std::fs::File::create(&config_file).expect("Unable to create file");
    }
    /// check for a config file in ~/.config/leetcode-cli/config.toml
    pub async fn status(&mut self) -> Result<(), reqwest::Error>
    {
        if self.config_file.is_some() {
            let config_file =
                std::fs::read_to_string(self.config_file.as_ref().unwrap())
                    .expect("Unable to read file");
            let config: Config =
                toml::from_str(&config_file).expect("Unable to parse file");
            self.leetcode_token = config.leetcode_token;
            self.check_token().await?;
        } else {
            self.create_config_file();
            self.generate_token();
        }
        Ok(())
    }

    async fn test_token_validity(&self) -> Result<bool, reqwest::Error>
    {
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

    async fn check_token(&mut self) -> Result<bool, reqwest::Error>
    {
        let token = self.leetcode_token.as_ref();
        if token.is_none() {
            println!("Leetcode token not found");
            println!("Let's set it together.");
            self.generate_token();
            return Ok(true);
        }
        self.test_token_validity().await
    }

    fn generate_token(&mut self)
    {
        unimplemented!("generating token is not implemented yet");
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
}
