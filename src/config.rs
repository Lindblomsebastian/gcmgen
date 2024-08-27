use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{env, fs};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_token: String,
}

static CONFIG_DIRECTORY: &str = "gcmgen";

impl Config {
    fn get_config_dir() -> PathBuf {
        let base_dir = if let Some(xdg_config_home) = env::var_os("XDG_CONFIG_HOME") {
            PathBuf::from(xdg_config_home)
        } else {
            let home_dir = env::var_os("HOME").expect("HOME environment variable not set");
            PathBuf::from(home_dir).join(".config")
        };
        base_dir.join(&CONFIG_DIRECTORY)
    }

    pub fn save_token(token: &str) -> std::io::Result<()> {
        let config_dir = Config::get_config_dir();
        let config_file = config_dir.join("config.json");

        let config = Config {
            api_token: token.to_string(),
        };

        let config_json = serde_json::to_string_pretty(&config)?;
        fs::create_dir_all(config_dir)?;
        fs::write(config_file, config_json)?;

        Ok(())
    }

    pub fn load_token() -> Result<String, std::io::Error> {
        let config_dir = Config::get_config_dir();
        let config_file = config_dir.join("config.json");

        let config_data = fs::read_to_string(config_file)?;
        let config: Config = serde_json::from_str(&config_data)?;

        Ok(config.api_token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    // Utility function to set up a temporary environment
    fn setup_temp_env() -> (PathBuf, PathBuf) {
        let temp_dir = env::temp_dir().join("config_test_env");
        let xdg_config_home = temp_dir.join("xdg_config");
        let home_dir = temp_dir.join("home");

        fs::create_dir_all(&xdg_config_home).unwrap();
        fs::create_dir_all(&home_dir).unwrap();

        env::set_var("XDG_CONFIG_HOME", &xdg_config_home);
        env::set_var("HOME", &home_dir);

        (xdg_config_home, home_dir)
    }

    #[test]
    fn test_save_token_creates_config_file() {
        let (xdg_config_home, _) = setup_temp_env();
        let config_file = xdg_config_home.join(&CONFIG_DIRECTORY).join("config.json");

        // Ensure the test directory is clean before running the test
        if config_file.exists() {
            fs::remove_file(&config_file).unwrap();
        }

        let token = "test_token";
        Config::save_token(token).unwrap();

        // Check if the file was created
        assert!(config_file.exists());

        // Clean up the test file
        fs::remove_file(config_file).unwrap();
    }

    #[test]
    fn test_load_token_reads_correct_value() {
        let (xdg_config_home, _) = setup_temp_env();
        let config_file = xdg_config_home.join(&CONFIG_DIRECTORY).join("config.json");

        let token = "test_token";
        Config::save_token(token).unwrap();

        // Load the token from the config file
        let loaded_token = Config::load_token().unwrap();
        assert_eq!(loaded_token, token);

        // Clean up the test file
        fs::remove_file(config_file).unwrap();
    }

    #[test]
    fn test_load_token_returns_error_when_file_missing() {
        let (xdg_config_home, _) = setup_temp_env();
        let config_file = xdg_config_home.join(&CONFIG_DIRECTORY).join("config.json");

        // Ensure the file does not exist
        if config_file.exists() {
            fs::remove_file(&config_file).unwrap();
        }

        // Try to load a token from a non-existent file
        let result = Config::load_token();
        assert!(result.is_err());
    }

    #[test]
    fn test_get_config_dir_respects_xdg_config_home() {
        let (xdg_config_home, _) = setup_temp_env();

        let config_dir = Config::get_config_dir();
        assert_eq!(config_dir, xdg_config_home.join(&CONFIG_DIRECTORY));
    }

    #[test]
    fn test_get_config_dir_falls_back_to_home_config() {
        let (_, home_dir) = setup_temp_env();

        env::remove_var("XDG_CONFIG_HOME");

        let expected_dir = home_dir.join(".config").join(&CONFIG_DIRECTORY);

        let config_dir = Config::get_config_dir();
        assert_eq!(config_dir, expected_dir);
    }
}
