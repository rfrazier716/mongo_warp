use crate::error::{Result, ServerError};
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG_PATH: &str = "./config/default.yml";
const CONFIG_FILE_PREFIX: &str = "./config/";

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggerSettings {
    pub rules: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub log: Vec<String>,
    pub application_port: u16,
}

impl Settings {
    pub fn new() -> Result<Self> {
        // Figure out what config to load based on environment Variables
        // Use Development by Default
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| String::from("development"));
        let mut settings = Config::new(); // Create a new config

        settings
            .merge(File::with_name(DEFAULT_CONFIG_PATH))
            .map_err(|source| ServerError::ConfigurationError { source })?; // Merge Default Settings
        settings
            .merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))
            .map_err(|source| ServerError::ConfigurationError { source })?; //merge the specific environment settings

        // Get database login information from the Environment
        // These Env Variables should be EA_DATABASE__URI
        settings
            .merge(Environment::with_prefix("ea").separator("__"))
            .map_err(|source| ServerError::ConfigurationError { source })?;

        settings
            .try_into()
            .map_err(|source| ServerError::ConfigurationError { source })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    const TEST_CONFIG: &str = r#"
application_port: 3030
database:
    uri: mongodb://root:example@localhost:27017
log:
    - info
"#;

    #[test]
    fn test_loading_config_string() {
        let mut s = Config::new();
        s.merge(File::from_str(TEST_CONFIG, config::FileFormat::Yaml))
            .unwrap();
        let config = s.try_into::<Settings>().unwrap(); //panic if we cannot convert it
        assert_eq!(
            config.database.uri,
            "mongodb://root:example@localhost:27017"
        )
    }

    #[test]
    fn test_overwriting_nested_values() {
        // set the environment variable for the database username
        env::set_var("EA_DATABASE__URI", "changed");
        let mut s = Config::new();
        s.merge(File::from_str(TEST_CONFIG, config::FileFormat::Yaml))
            .unwrap();
        s.merge(Environment::with_prefix("ea").separator("__"))
            .unwrap();
        let config: Settings = s.try_into().unwrap();
        assert_eq!(config.database.uri, "changed");
    }

    // #[test]
    // fn test_creating_with_missing_fields() {
    //     // set the environment variable for the database username
    //     env::set_var("EA_DATABASE__USERNAME", "changed");
    //     let mut s = Config::new();
    //     s.merge(File::from_str(
    //         TEST_CONFIG_NO_USERNAME,
    //         config::FileFormat::Yaml,
    //     ))
    //     .unwrap();
    //     s.merge(Environment::with_prefix("ea").separator("__"))
    //         .unwrap();
    //     let config: Settings = s.try_into().unwrap();
    //     assert_eq!(config.database.username, "changed");
    // }
}
