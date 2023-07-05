use std::collections::HashMap;
use std::any::TypeId;
use rumqttc::{Incoming, Publish};
use crate::data::repositories::message::MessageRepository;
use crate::entities::message::Message;
use async_trait::async_trait;
use std::sync::Arc;

pub struct EventHandler {
    handlers: HashMap<TypeId, Arc<dyn Handler + Send + Sync>>,
}

#[async_trait]
pub trait Handler {
    async fn handle(&self, repository: &MessageRepository, incoming: Incoming);
}

pub struct PublishHandler;

#[async_trait]
impl Handler for PublishHandler {
    async fn handle(&self, repository: &MessageRepository, incoming: Incoming) {
        if let rumqttc::Incoming::Publish(packet) = incoming {
            let payload = String::from_utf8_lossy(&packet.payload).into_owned();
            println!("Received message: {}", payload);
            let message = Message::new(payload);
            repository.save(&message).await.unwrap();
        }
    }
}

impl EventHandler {
    pub fn new() -> Self {
        let mut handlers = HashMap::new();
        handlers.insert(TypeId::of::<Publish>(), Arc::new(PublishHandler) as Arc<dyn Handler + Send + Sync>);
        EventHandler { handlers }
    }

    pub async fn handle_event(&self, event: Incoming, repository: &MessageRepository) {
        let type_id = match &event {
            Incoming::Publish(_) => TypeId::of::<Publish>(),
            _ => return,
        };

        if let Some(handler) = self.handlers.get(&type_id) {
            handler.handle(repository, event).await;
        }
    }
}
