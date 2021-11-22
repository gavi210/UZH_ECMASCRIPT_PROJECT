use serde::{Deserialize, Serialize};

// allows to specify the characteristics to be provided to trigger and run each function
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FunctionDefinition {
    pub name: String,
    pub nats_subject_trigger: String,
    pub nats_subject_trigger_type: String,
    pub runtime_type: String,
    pub function_definition: String,
    pub output_destinations: String,
    pub permit_network_access: bool,
    pub permitted_hosts: Vec<String>,
}


