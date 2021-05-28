use confy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

impl std::default::Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            username: String::from("root"),
            password: String::from("example"),
            host: String::from("localhost"),
            port: 27017,
        }
    }
}

impl DatabaseSettings {
    pub fn connection_str(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

impl std::default::Default for Settings {
    fn default() -> Self {
        Self {
            application_port: 3030,
            database: DatabaseSettings::default(),
        }
    }
}

pub fn get_config() -> Result<Settings, confy::ConfyError> {
    confy::load("server.yaml")
}
