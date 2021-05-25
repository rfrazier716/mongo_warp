use mongodb::bson::doc;
use mongodb::options::UpdateModifications::Document;
use mongodb::options::{ClientOptions, Credential, ServerAddress};
use mongodb::Client;

pub struct DatabaseConfig {
    pub address: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub struct Database {
    pub client: mongodb::Client,
}

impl Database {
    pub fn new(config: DatabaseConfig) -> Self {
        // Replace optional config arguments with defaults
        let username = config.username.unwrap_or_else(|| String::from("root"));
        let password = config.password.unwrap_or_else(|| String::from("password"));

        // Create the Connection Credentials
        let credentials = Credential::builder()
            .username(username)
            .password(password)
            .build();

        // Fill out the rest of the connection options
        let db_options = ClientOptions::builder()
            .hosts(vec![ServerAddress::Tcp {
                host: config.address.into(),
                port: Some(config.port),
            }])
            .credential(credentials)
            .build();

        // Get a handle to the deployment.
        let client = Client::with_options(db_options).expect("Could not Connect to Client");
        Self { client } // Return the Client
    }

    pub async fn ping(&self) -> Result<(), String> {
        match self
            .client
            .database("admin")
            .run_command(doc! {"ping":1}, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err("Bad".to_string()),
        }
    }
}