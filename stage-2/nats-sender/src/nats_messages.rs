use serde::{Deserialize, Serialize};

// allows to specify the characteristics to be provided to trigger and run each function
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NatsMessage {
    pub test_iterations: usize,
    pub loop_iterations: usize
}