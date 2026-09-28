use std::{
    fs,
    io::{
        self,
    },
    path::{
        Path,
        PathBuf,
    },
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

impl Default for RuntimeConfigSetup {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeConfigSetup {
    /// Sets up a fresh runtime config rooted at the user home directory.
    ///
    /// # Panics
    ///
    /// Panics when no home directory can be determined — the whole config
    /// layout depends on it.
    #[must_use]
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().expect("no home directory");
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
    /// Creates a config file in ~/.config/leetcode-cli/config.toml with
    /// default values.
    ///
    /// # Panics
    ///
    /// Panics when the config directory cannot be created or the file
    /// cannot be written — setup cannot proceed without them.
    fn create_config_file(&self) {
        std::fs::create_dir_all(&self.config_dir)
            .expect("Unable to create directory");
        std::fs::write(
            &self.config_file,
            "leetcode_token = ''\ndefault_language = \
             'Rust'\nleetcode_dir_path = '~/leetcode'\n",
        )
        .expect("Unable to create file");
    }

    /// Saves the leetcode token into the config file, in the `leetcode_token`
    /// entry, preserving the rest of the file (other keys, comments) as is.
    ///
    /// # Errors
    ///
    /// `io::Error` when the file cannot be created/read/written or contains
    /// invalid toml (kind `InvalidData`).
    pub fn write_token_to_file(
        config_file: &PathBuf, token: &str,
    ) -> io::Result<()> {
        if !config_file.is_file() {
            std::fs::create_dir_all(
                config_file.parent().unwrap_or(Path::new(".")),
            )?;
            std::fs::File::create(config_file)?;
        }
        let raw = std::fs::read_to_string(config_file)?;
        let mut doc = raw
            .parse::<toml_edit::DocumentMut>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mut new_value = toml_edit::Value::from(token);
        if let Some(old_value) =
            doc.get("leetcode_token").and_then(|item| item.as_value())
        {
            *new_value.decor_mut() = old_value.decor().clone();
        }
        doc["leetcode_token"] = toml_edit::Item::Value(new_value);

        std::fs::write(config_file, doc.to_string())
    }
    /// check for a config file in ~/.config/leetcode-cli/config.toml
    /// read it or create it with default values if it doesn't exist
    /// load the config in Config struct and check if the token is valid
    ///
    /// # Errors
    ///
    /// `io::Error` when the config file cannot be read or the directory
    /// cannot be created.
    ///
    /// # Panics
    ///
    /// Panics when the config file exists but cannot be read or contains
    /// invalid toml — requires manual fixing.
    pub fn status(&mut self) -> Result<(), io::Error> {
        if self.config_file.is_file() {
            let config_file = std::fs::read_to_string(&self.config_file)
                .expect("Unable to read file");
            if config_file.trim().is_empty() {
                return Ok(());
            }
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

    /// Resolve the configured `LeetCode` directory, expand ~, canonicalize, and
    /// create if missing.
    ///
    /// # Errors
    ///
    /// `io::Error` (kind `NotFound`) when no `leetcode_dir_path` is set in
    /// the config, or when the directory cannot be created/canonicalized.
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
