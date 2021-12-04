use serde::{Deserialize, Serialize};

// allows to specify the characteristics to be provided to trigger and run each function
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NatsMessage {
    pub id: usize,
    pub message: String
}