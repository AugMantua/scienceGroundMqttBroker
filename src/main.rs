mod events;
mod data;
mod entities;
mod configurations;

use configurations::Config;
use rumqttc::{MqttOptions, AsyncClient, QoS};
use std::{error::Error, time::Duration};
use tokio::task;
use crate::events::EventHandler;
use data::repositories::message::MessageRepository;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    dotenv::dotenv().ok();
    let config = Config::from_env();

    let repository = MessageRepository::new(&config.db_url).await?;
    let event_handler = EventHandler::new();
    let mut mqttoptions = MqttOptions::new(
        "rust_client",
        config.mqtt_host, 
        config.mqtt_port
    );
    let duration: Duration = Duration::from_secs(5);
    mqttoptions.set_keep_alive(duration);

    let (mut client, mut eventLoop) = AsyncClient::new(mqttoptions, 10);
    task::spawn(async move {
        match client.subscribe("hello/world", QoS::AtMostOnce).await {
            Ok(_) => println!("Subscribed successfully!"),
            Err(e) => println!("Failed to subscribe: {}", e),
        }
    });

    
    loop {
        let poll = eventLoop.poll().await;
        if let Ok(event) = poll {
            match event {
                rumqttc::Event::Incoming(incoming) => event_handler.handle_event(incoming, &repository).await,
                _ => (),
            }
        } else if let Err(e) = poll {
            println!("Error: {:?}", e);
        }
    }
    
}
