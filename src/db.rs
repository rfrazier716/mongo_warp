use crate::config::DatabaseSettings;
use mongodb::bson::{doc, Document};
use mongodb::error::Result;
use mongodb::options::{ClientOptions, Credential, ServerAddress};

pub(crate) type Client = mongodb::Client;

pub async fn ping(client: &Client) -> Result<Document> {
    client
        .database("admin")
        .run_command(doc! {"ping":1}, None)
        .await
}
