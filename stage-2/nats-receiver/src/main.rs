use deno_core::error::AnyError;
use rants::{Client, Subject};
use tokio::task;
use log::info;

use std::time::{Instant};

mod functions;
mod worker;
mod web_worker_manager;
mod util;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    util::logger::configure_logger();

    let configuration = util::config_parser::get_configuration_object();

    let client = get_nats_client(&configuration.nats_server).await.unwrap();

    // define a handle for each function to be triggered
    let mut handles = vec![];
    const BUFFER_SIZE: usize = 1024;

    for f in configuration.functions {

        // one subject per NATS receiver
        let subject = f.nats_subject_trigger.parse::<Subject>().unwrap();
        let (_, mut sub) = client.subscribe(&subject, BUFFER_SIZE).await.unwrap();


        handles.push(task::spawn(async move {
            loop {
                let message = sub.recv().await.unwrap();
                let payload = message.payload();

                match payload {
                  b"STOP" => { // terminate the subject
                    info!("{:?} stopped!", f.nats_subject_trigger);
                    break;
                  },
                  _ => {
                    let function = f.clone();
                    tokio::task::spawn_blocking(|| { // spawn on a thread that could be blocked during execution

                      let start_time = Instant::now();
                      let _worker_output = match web_worker_manager::execute_function(function, message)  {
                      //let worker_output = match worker::execute_function_web_worker(function, message)  {
                        Ok(_worker_output) => (),
                        Err(err) => panic!("Function execution terminated in error: {:?}", err),
                      };
                      let duration = start_time.elapsed();
                      println!("Execution with MainWorkers: {:?}", duration);
                    })
                    .await
                    .expect("Task panicked")
                  }
                }
            }
        }));
    }

    futures::future::join_all(handles).await;
    client.disconnect().await;

    Ok(())

}

// instantiate and connect NATS client
async fn get_nats_client(nats_server : &String) -> Result<Client, AnyError> {
  let address = nats_server.parse().unwrap();
  let client = Client::new(vec![address]);
  client.connect().await;

  Ok(client)
}