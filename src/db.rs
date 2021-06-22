use crate::error::{Error::MongoQueryError, Result};
use mongodb::bson::{doc, Document};

pub(crate) type Client = mongodb::Client;

pub async fn ping(client: &Client) -> Result<Document> {
    client
        .database("admin")
        .run_command(doc! {"ping":1}, None)
        .await
        .map_err(MongoQueryError)
}
