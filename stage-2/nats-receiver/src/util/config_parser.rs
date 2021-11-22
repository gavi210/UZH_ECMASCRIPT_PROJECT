use clap::{App, Arg};
use std::fs;

use serde::{Deserialize, Serialize};
use crate::functions::FunctionDefinition;

#[derive(Serialize, Deserialize, Debug)]
pub struct Arguments {
    config_file: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Configuration {
    pub nats_server: String,
    pub functions: Vec<FunctionDefinition>
}

const CONFIG_FILE : &str = "./config.json";
pub fn get_configuration_object() -> Configuration {
    let matches = App::new("Execute functions with WebWorkers")
        .version("1.0")
        .author("Maximilian & Riccardo")
        .about("Execute functions in WebWorkers")
        .arg(
            Arg::new("config-file")
                .short('c')
                .long("config-file")
                .about("Configuration file to be used")
                .required(false)
                .default_value(CONFIG_FILE),
        )
        .get_matches();

    let config_file = match matches.value_of_t("config-file") {
        Ok(configuration) => configuration,
        Err(err) => panic!("Unable to process input args: {:?}", err),
    };

     match read_config_file(config_file) {
        Ok(config_obj) => return config_obj,
        Err(err) => panic!("Unable to load and parse configuration: {:?}", err),
     };
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
