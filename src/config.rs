#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config
{
    leetcode_token: Option<String>,
}

impl Config
{
    pub fn new() -> Self
    {
        Config {
            leetcode_token: None,
        }
    }

    fn create_config_file(&self)
    {
        let home_dir =
            dirs::home_dir().expect("Unable to determine home directory");
        let config_dir = home_dir.join(".config/leetcode-cli");

        std::fs::create_dir_all(&config_dir)
            .expect("Unable to create directory");
        let config_file = config_dir.join("config.toml");
        std::fs::File::create(&config_file).expect("Unable to create file");
        println!("Config file created at {:?}", config_file);
    }
    /// check for a config file in
    /// - ~/.config/leetcode-cli/config.toml
    pub async fn status(&mut self) -> Result<(), reqwest::Error>
    {
        println!("Checking for config file...");
        if std::path::Path::new("~/.config/leetcode-cli/config.toml").exists() {
            println!("Config file exists.");
            println!("Reading config file...");
            let config_file =
                std::fs::read_to_string("~/.config/leetcode-cli/config.toml")
                    .expect("Unable to read file");
            println!("Parsing config file...");
            let config: Config =
                toml::from_str(&config_file).expect("Unable to parse file");
            println!("Config file parsed.");
            println!("Config file: {:?}", config);
            self.leetcode_token = config.leetcode_token;
            self.check_token().await?;
        } else {
            println!("Config file does not exist");
            println!("Creating default config file...");
            self.create_config_file();
            self.generate_token();
        }
        Ok(())
    }

    async fn check_token(&self) -> Result<bool, reqwest::Error>
    {
        unimplemented!();
        let token = self.leetcode_token.as_ref();
        if token.is_none() {
            println!("Leetcode token not found");
            println!("Let's set it together.");
            self.generate_token();
            return Ok(true);
        }
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

    fn generate_token(&mut self)
    {
        unimplemented!();
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
