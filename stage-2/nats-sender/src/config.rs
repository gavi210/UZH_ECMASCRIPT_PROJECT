use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub nats_server: String,
    pub subject_function_executor: String,
    pub subject_result_receiver: String,
}

// reads in the specified configuration file...
pub fn read_config_file(filename: String) -> Result<Configuration, std::io::Error> {
    let file_contents = match fs::read_to_string(filename) {
        Ok(read_file) => read_file,
        Err(err) => return Err(err),
    };

    // instantiate configuration instance from the input string
    let c: Configuration = serde_json::from_str(&file_contents).unwrap();

    Ok(c)
}
