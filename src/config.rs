use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::{env, fmt, fs, io};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ServiceConfig {
    pub api_token: String,
    pub model: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub default_service: String,
    pub services: HashMap<String, ServiceConfig>,
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Default Service: {}", self.default_service)?;

        for (service_name, service_config) in &self.services {
            writeln!(f, "\nService: {}", service_name)?;
            writeln!(f, "{}", service_config)?;
        }

        Ok(())
    }
}

impl Display for ServiceConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let masked_token = if self.api_token.len() > 5 {
            format!("{}***", &self.api_token[..5])
        } else {
            format!("{}***", self.api_token)
        };

        writeln!(f, "API Token: {}", masked_token)?;
        writeln!(f, "Model: {}", self.model)
    }
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
        base_dir.join(CONFIG_DIRECTORY)
    }

    pub fn save(&self) -> Result<(), io::Error> {
        let config_dir = Self::get_config_dir();
        let config_file = config_dir.join("config.json");

        let config_json = serde_json::to_string_pretty(&self)?;
        fs::create_dir_all(config_dir)?;
        fs::write(config_file, config_json)?;

        Ok(())
    }

    pub fn load() -> Result<Self, io::Error> {
        let config_dir = Self::get_config_dir();
        let config_file = config_dir.join("config.json");

        let config_data = fs::read_to_string(config_file)?;
        let config: Config = serde_json::from_str(&config_data)?;

        Ok(config)
    }

    pub fn get_default_service_config(&self) -> Option<&ServiceConfig> {
        self.services.get(self.default_service.as_str())
    }

    pub fn set_default_service(&mut self, service_name: &str) -> Result<(), String> {
        if self.services.contains_key(service_name) {
            self.default_service = service_name.to_string();
            self.save().unwrap();
            Ok(())
        } else {
            Err(format!("Service '{}' not found", service_name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use uuid::Uuid;

    // Utility function to set up a unique temporary environment for each test
    fn setup_temp_env() -> (PathBuf, PathBuf) {
        let unique_id = Uuid::new_v4().to_string();
        let temp_dir = env::temp_dir().join(format!("config_test_env_{}", unique_id));
        let xdg_config_home = temp_dir.join("xdg_config");
        let home_dir = temp_dir.join("home");

        fs::create_dir_all(&xdg_config_home).unwrap();
        fs::create_dir_all(&home_dir).unwrap();

        env::set_var("XDG_CONFIG_HOME", &xdg_config_home);
        env::set_var("HOME", &home_dir);

        (xdg_config_home, home_dir)
    }

    #[test]
    #[serial]
    fn test_save_creates_config_file() {
        let (xdg_config_home, _) = setup_temp_env();
        let config_dir = xdg_config_home.join(CONFIG_DIRECTORY);
        let config_file = config_dir.join("config.json");

        // Ensure the directory exists before proceeding
        fs::create_dir_all(&config_dir).unwrap();

        // Ensure the test directory is clean before running the test
        if config_file.exists() {
            fs::remove_file(&config_file).unwrap();
        }

        let service_config = ServiceConfig {
            api_token: "test_token".to_string(),
            model: "gpt-4o-mini".to_string(),
        };

        let config = Config {
            default_service: "OpenAI".to_string(),
            services: [("OpenAI".to_string(), service_config)]
                .iter()
                .cloned()
                .collect(),
        };

        config.save().unwrap();

        // Check if the file was created
        assert!(config_file.exists());

        // Clean up the test file
        fs::remove_file(config_file).unwrap();
    }

    #[test]
    #[serial]
    fn test_load_reads_correct_config() {
        let (xdg_config_home, _) = setup_temp_env();
        let config_dir = xdg_config_home.join(CONFIG_DIRECTORY);
        let config_file = config_dir.join("config.json");

        // Ensure the directory exists before proceeding
        fs::create_dir_all(&config_dir).unwrap();

        let service_config = ServiceConfig {
            api_token: "test_token".to_string(),
            model: "gpt-4o-mini".to_string(),
        };

        let config = Config {
            default_service: "OpenAI".to_string(),
            services: [("OpenAI".to_string(), service_config.clone())]
                .iter()
                .cloned()
                .collect(),
        };

        config.save().unwrap();

        // Load the configuration from the file
        let loaded_config = Config::load().unwrap();
        assert_eq!(loaded_config.default_service, "OpenAI");
        assert_eq!(
            loaded_config.services.get("OpenAI").unwrap(),
            &service_config
        );

        // Clean up the test file
        fs::remove_file(config_file).unwrap();
    }

    #[test]
    #[serial]
    fn test_load_returns_error_when_file_missing() {
        let (xdg_config_home, _) = setup_temp_env();
        let config_dir = xdg_config_home.join(CONFIG_DIRECTORY);
        let config_file = config_dir.join("config.json");

        // Ensure the directory exists before proceeding
        fs::create_dir_all(&config_dir).unwrap();

        // Ensure the file does not exist
        if config_file.exists() {
            fs::remove_file(&config_file).unwrap();
        }

        // Try to load a config from a non-existent file
        let result = Config::load();
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn test_set_default_service() {
        let (xdg_config_home, _) = setup_temp_env();
        let config_dir = xdg_config_home.join(CONFIG_DIRECTORY);
        let config_file = config_dir.join("config.json");

        let service_config_openai = ServiceConfig {
            api_token: "test_token_openai".to_string(),
            model: "gpt-4o-mini".to_string(),
        };

        let service_config_anthropic = ServiceConfig {
            api_token: "test_token_anthropic".to_string(),
            model: "claude-v1".to_string(),
        };

        let mut config = Config {
            default_service: "OpenAI".to_string(),
            services: [
                ("OpenAI".to_string(), service_config_openai.clone()),
                ("Anthropic".to_string(), service_config_anthropic.clone()),
            ]
            .iter()
            .cloned()
            .collect(),
        };

        config.save().unwrap();

        // Set the default service to "Anthropic"
        config.set_default_service("Anthropic").unwrap();

        // Load the configuration from the file
        let loaded_config = Config::load().unwrap();
        assert_eq!(loaded_config.default_service, "Anthropic");

        // Clean up the test file
        fs::remove_file(config_file).unwrap();
    }

    #[test]
    #[serial]
    fn test_get_config_dir_respects_xdg_config_home() {
        let (xdg_config_home, _) = setup_temp_env();

        let config_dir = Config::get_config_dir();
        assert_eq!(config_dir, xdg_config_home.join(CONFIG_DIRECTORY));
    }

    #[test]
    #[serial]
    fn test_get_config_dir_falls_back_to_home_config() {
        let (_, home_dir) = setup_temp_env();

        env::remove_var("XDG_CONFIG_HOME");

        let expected_dir = home_dir.join(".config").join(CONFIG_DIRECTORY);

        let config_dir = Config::get_config_dir();
        assert_eq!(config_dir, expected_dir);
    }
}
