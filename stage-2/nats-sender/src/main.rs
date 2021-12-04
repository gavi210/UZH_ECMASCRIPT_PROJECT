use clap::{App, Arg};
use log::{info};
use rants::{Client, Subject, Address};
use serde::{Deserialize, Serialize};
use tokio::task;
use std::str;

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

    let address : Address = configuration.nats_server.parse().unwrap();
    // clone address for the two threads
    let address_sender_thread = address.clone();
    let address_receiver_thread = address.clone();

    // get names of sender and receiver subjects
    let subject_function_executor : Subject = configuration.subject_function_executor.parse().unwrap();
    let subject_result_receiver : Subject = configuration.subject_result_receiver.parse().unwrap();

    // thread for the sender
    let nats_sender_handle = task::spawn(async move {
      let client = Client::new(vec![address_sender_thread]);
      client.connect().await;




      /*
      let message_loop = b"loop(10);";
      let message_log = b"log_this(\"Helloooooo\");";

      let message_double = b"double(2);";
      */
      let mut counter : usize = 0;
      for _ in 0..5 {
        let message_greet = nats_messages::NatsMessage {
          id : counter,
          message : String::from("greet()"),
        };

        counter = counter +1;
        let str_message_greet = serde_json::to_string(&message_greet).unwrap();
        client.publish(&subject_function_executor, str_message_greet.as_bytes())
           .await
           .unwrap();

        let message_double = nats_messages::NatsMessage {
          id : counter,
          message : String::from("double(2);")
        };
        let str_message_double = serde_json::to_string(&message_double).unwrap();
        counter = counter +1;
        // publish message -> request to engine to double the number
        client.publish(&subject_function_executor, str_message_double.as_bytes())
          .await
          .unwrap();
      }

      // stop function executor
      client
        .publish(&subject_function_executor, b"STOP")
        .await
        .unwrap();

      client.disconnect().await;
    });

    // thread for the sender
    let nats_receiver_handle = task::spawn(async move {

      let client = Client::new(vec![address_receiver_thread]);
      client.connect().await;
      const BUFFER_SIZE: usize = 1024;
      let (_, mut sub) = client.subscribe(&subject_result_receiver, BUFFER_SIZE).await.unwrap();

      loop {
          let message = sub.recv().await.unwrap();
          let payload = message.payload();

          match payload {
            b"STOP" => {
              info!("Results receiver stops");
              break;
            },
            _ => {
              let msg = match str::from_utf8(payload) {
                  Ok(v) => v,
                  Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
              };
              info!("Result received: {:?}", msg);
            }
          }
      }

      // when outside the loop -> client stopped, disconnect and close thread
      client.disconnect().await;
    });

    // wait for two threads to stop
    nats_sender_handle.await.unwrap();
    nats_receiver_handle.await.unwrap();
    Ok(())
}
