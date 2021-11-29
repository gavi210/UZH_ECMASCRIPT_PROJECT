use clap::{App, Arg};
use futures::stream::StreamExt;
use log::{info, trace, warn};
use rants::{Client, Subject};
use serde::{Deserialize, Serialize};
use tokio::task;

use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

mod config;
mod functions;
mod nats_messages;

#[derive(Serialize, Deserialize, Debug)]
struct Arguments { // stores arguments passed as input parameters - allows custom execution
    config_file: String,
}

// emulate cmd arguments
fn parse_args() -> Result<Arguments, clap::Error> {
    let CONFIG_FILE = "./config.json";
    let matches = App::new("Message Sender")
        .version("1.0")
        .author("Maximilian & Riccardo")
        .about("Send messages to NATS subjects")
        .arg(
            Arg::new("config-file")
                .short('c')
                .long("config-file")
                .about("Name of the configuration file to use")
                .required(false)
                .default_value(CONFIG_FILE),
        )
        .get_matches();

    let config_file = match matches.value_of_t("config-file") {
        Ok(configuration) => configuration, // return path to configuration file
        Err(err) => return Err(err),
    };

    let c = Arguments {
        config_file: config_file,
    };

    Ok(c)
}

// logging on the activities being done
fn configure_logger() {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info) // discard everything below level INFO
        .init();
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    configure_logger();

    let a = match parse_args() {
        Ok(configuration) => configuration,
        Err(err) => panic!("Unable to parse arguments: {:?}", err),
    };

    let configuration = match config::read_config_file(a.config_file) {
        Ok(configuration) => configuration, // loads config object from file
        Err(err) => panic!("Unable to load and parse configuration: {:?}", err),
    };

    // info!("Configuration = {:?}", configuration);

    let address = configuration.nats_server.parse().unwrap();
    let client = Client::new(vec![address]);

    // Connect to the server
    client.connect().await;

    let mut subjects: Vec<Subject> = Vec::new();

    // get reference for each subject
    for subject in configuration.subjects.iter() {
      subjects.push(subject.parse().unwrap());
    }

    const TEST_ITERATIONS : usize = 10;
    const LOOP_ITERATIONS : usize = 10000;
    const NBR_WORKERS : usize = 3;

    let nats_message = nats_messages::NatsMessage {
      test_iterations : TEST_ITERATIONS,
      loop_iterations : LOOP_ITERATIONS,
      nbr_workers : NBR_WORKERS
    };

    let json_str = serde_json::to_string(&nats_message)?;
    let bytes = json_str.as_bytes();

    // send message -> repeat test 10 times
    for subject in subjects.clone() {
      client.publish(&subject, bytes)
        .await
        .unwrap();
    }

    // stop subjects
    for subject in subjects.iter() {
      client
        .publish(&subject, b"STOP")
        .await
        .unwrap();

    }

    // stop receiving messages
    client.disconnect().await;
    Ok(())

}
