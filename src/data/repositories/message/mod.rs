use mongodb::{Client, options::ClientOptions, Collection};
use std::error::Error;
use crate::entities::message::Message;

pub struct MessageRepository {
    client: Client,
}

impl MessageRepository {
    pub async fn new(db_url: &str) -> Result<Self, Box<dyn Error>> {
        let client_options = ClientOptions::parse(db_url).await?;
        let client = Client::with_options(client_options)?;
        Ok(MessageRepository { client })
    }

    pub async fn save(&self, message: &Message) -> Result<(), Box<dyn Error>> {
        let collection: Collection<Message> = self.client.database("mqtt").collection("messages");
        collection.insert_one(message, None).await?;
        Ok(())
    }
}
