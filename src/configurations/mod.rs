use std::env;

pub struct Config {
    pub db_url: String,
    pub mqtt_host: String,
    pub mqtt_port: u16,
}

impl Config {
    pub fn from_env() -> Config {
        let db_url = env::var("DB_URL").unwrap_or("mongodb://localhost:27017".to_string());
        let mqtt_host = env::var("MQTT_HOST").unwrap_or("localhost".to_string());
        let mqtt_port_str = env::var("MQTT_PORT").unwrap_or("1883".to_string());
        let mqtt_port = mqtt_port_str.parse::<u16>().unwrap_or(1883);

        Config { db_url, mqtt_host, mqtt_port }
    }
}
