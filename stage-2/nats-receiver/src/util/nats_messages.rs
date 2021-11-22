use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct NatsMessage {
    pub test_iterations: usize,
    pub loop_iterations: usize,
}