use clap::{App, Arg};
use futures::stream::StreamExt;
use deno_core::error::AnyError;
use log::{info, trace, warn};
use rants::{Client, Subject};
use serde::{Deserialize, Serialize};
use tokio::task;

use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

use std::time::{Duration, Instant};

mod config;
mod functions;
mod worker;

#[derive(Serialize, Deserialize, Debug)]
struct Arguments {
    config_file: String,
}

fn parse_args() -> Result<Arguments, clap::Error> {
    let CONFIG_FILE = "./config.json";
    let matches = App::new("Triggers Function Execution using MainWorkers and WebWorkers")
        .version("1.0")
        .author("Maximilian & Riccardo")
        .about("Trigger js function execution in deno through NATS using different types of workers")
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
        Err(err) => return Err(err),
    };

    let c = Arguments {
        config_file: config_file,
    };

    Ok(c)
}

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


#[tokio::main] // async function -> executed when runtime instance is created
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

    //info!("Configuration = {:?}", configuration);

    // connect to NATS server
    let address = configuration.nats_server.parse().unwrap();
    let client = Client::new(vec![address]); // nats client listening to the given port

    // Connect to the server
    client.connect().await;

    // define a handle for each function to be triggered
    let mut handles = vec![];
    const BUFFER_SIZE: usize = 1024;

    for f in configuration.functions {

        // one subject per NATS receiver
        let subject = f.nats_subject_trigger.parse::<Subject>().unwrap();
        let (_, mut sub) = client.subscribe(&subject, BUFFER_SIZE).await.unwrap();


        handles.push(task::spawn(async move {
            // info!("Executing subject: {}...", f.nats_subject_trigger);

            loop {
                let message = sub.recv().await.unwrap();
                let payload = message.payload();

                match payload {
                  b"STOP" => { // terminate the subject
                    info!("{:?} has been stopped!", f.nats_subject_trigger);
                    break;
                  },
                  _ => {
                    info!("Executing function for {:?}", f.nats_subject_trigger);
                    let function = f.clone();
                    tokio::task::spawn_blocking(|| { // spawn on a thread that could be blocked during execution

                      let start_time = Instant::now();
                      let worker_output = match worker::execute_function(function, message)  {
                        Ok(worker_output) => (),
                        Err(err) => panic!("Function execution terminated in error: {:?}", err),
                      };
                      let duration = start_time.elapsed();
                      println!("Execution with MainWorkers: {:?}", duration);
                      /*
                      let start_time = Instant::now();
                      let worker_output = match worker::execute_function_web_worker(function, message)  {
                        Ok(worker_output) => (),
                        Err(err) => panic!("Function execution terminated in error: {:?}", err),
                      };
                      let duration = start_time.elapsed();
                      println!("Execution with WebWorkers: {:?}", duration);
                      */
                    })
                    .await
                    .expect("Task panicked")
                  }
                }
            }
        }));
    }

    futures::future::join_all(handles).await;

    // stop receiving messages
    client.disconnect().await;

    Ok(())

}
