use crate::config::DatabaseSettings;
use mongodb::bson::{doc, Document};
use mongodb::error::Result;
use mongodb::options::{ClientOptions, Credential, ServerAddress};

pub(crate) type Client = mongodb::Client;

#[derive(Clone)]
pub struct Database {
    pub client: mongodb::Client,
}

pub async fn create_client(uri: &str) -> Result<Client> {
    mongodb::Client::with_uri_str(uri).await
}

pub async fn ping(client: &Client) -> Result<Document> {
    client
        .database("admin")
        .run_command(doc! {"ping":1}, None)
        .await
}

impl Database {
    pub fn new(settings: DatabaseSettings) -> Result<Self> {
        // Create the Connection Credentials
        let credentials = Credential::builder()
            .username(settings.username)
            .password(settings.password)
            .build();

        // Fill out the rest of the connection options
        let db_options = ClientOptions::builder()
            .hosts(vec![ServerAddress::Tcp {
                host: settings.host,
                port: Some(settings.port),
            }])
            .credential(credentials)
            .build();

        // Get a handle to the deployment.
        let client = Client::with_options(db_options)?;
        Ok(Self { client }) // Return the Client
    }

    pub async fn from_uri(uri: String) -> Result<Self> {
        let client = Client::with_uri_str(uri).await?;
        Ok(Self { client })
    }

    pub async fn ping(&self) -> Result<Document> {
        self.client
            .database("admin")
            .run_command(doc! {"ping":1}, None)
            .await
    }
}
