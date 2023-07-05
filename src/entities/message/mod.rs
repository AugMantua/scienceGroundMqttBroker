use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    payload: String,
}

impl Message {
    pub fn new(payload: String) -> Self {
        Message { payload }
    }
}
