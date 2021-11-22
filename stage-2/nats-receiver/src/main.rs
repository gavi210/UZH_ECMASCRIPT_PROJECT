use deno_core::error::AnyError;
use rants::{Client, Subject};
use tokio::task;
use log::info;

use std::time::{Instant};
use std::str;

mod functions;
mod worker;
mod web_worker_manager;
mod util;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    util::logger::configure_logger();

    let configuration = util::config_parser::get_configuration_object();

    let client = get_nats_client(&configuration.nats_server).await.unwrap();

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
                      //info!("{:?} stopped!", f.nats_subject_trigger);
                      break;
                    },
                    _ => {
                        let function = f.clone();
                        // execute web worker code
                        if (function.nats_subject_trigger.eq("subject-web-worker-creator")) {
                           tokio::task::spawn_blocking(|| { // spawn on a thread that could be blocked during execution

                              let _worker_output = match web_worker_manager::run_test(function, message)  {
                                Ok(_worker_output) => (),
                                Err(err) => panic!("Function execution terminated in error: {:?}", err),
                              };
                              })
                           .await
                           .expect("Task panicked")
                        }
                        else {
                            tokio::task::spawn_blocking(|| { // spawn on a thread that could be blocked during execution
                                let execution_times = match worker::run_test(function, message)  {
                                  Ok(worker_output) => worker_output,
                                  Err(err) => panic!("Function execution terminated in error: {:?}", err),
                                };
                                println!("Main Worker Execution Times: {:?}", execution_times);
                                })
                            .await
                            .expect("Task panicked")
                        }

                    }
                }
            }
        }));
    }


    /*
    let web_worker_fn = configuration.functions[0];
    let main_worker_fn = configuration.functions[1];

    let web_worker_subject = web_worker_fn.nats_subject_trigger.parse::<Subject>().unwrap();
    let (_, mut web_worker_sub) = client.subscribe(&web_worker_subject, BUFFER_SIZE).await.unwrap();

    let main_worker_subject = web_worker_fn.nats_subject_trigger.parse::<Subject>().unwrap();
    let (_, mut main_worker_sub) = client.subscribe(&main_worker_subject, BUFFER_SIZE).await.unwrap();

    // create handle for main worker
    handles.push(task::spawn(async move {
        loop {
            let message = main_worker_sub.recv().await.unwrap();
            let payload = message.payload();

            match payload {
                b"STOP" => {
                  //info!("{:?} stopped!", f.nats_subject_trigger);
                  break;
                },
                _ => {
                    let payload_str = str::from_utf8(&payload).unwrap();
                    let nats_message : util::nats_messages::NatsMessage = serde_json::from_str(&payload_str).unwrap();
                    let function = main_worker_fn.clone();

                    tokio::task::spawn_blocking(|| { // spawn on a thread that could be blocked during execution
                        let mut execution_times = vec![];

                        for i in 0..nats_message.test_iterations {
                            let start_time = Instant::now();
                            let _worker_output = match worker::execute_function(function.clone(), nats_message.loop_iterations)  {
                                Ok(_worker_output) => (),
                                Err(err) => panic!("Function execution terminated in error: {:?}", err),
                            };
                            let duration = start_time.elapsed();
                            execution_times.push(duration);
                        }
                        info!("MainWorker Execution Times: \n{:?}", execution_times);
                    })
                    .await
                    .expect("Main Worker Execution Failed")
                }
            }
        }
    }));

    // create handle for web worker
    handles.push(task::spawn(async move {
        loop {
            let message = web_worker_sub.recv().await.unwrap();
            let payload = message.payload();

            match payload {
                b"STOP" => {
                  //info!("{:?} stopped!", f.nats_subject_trigger);
                  break;
                },
                _ => {
                    let payload_str = str::from_utf8(&payload).unwrap();
                    let nats_message : util::nats_messages::NatsMessage = serde_json::from_str(&payload_str).unwrap();
                    //let function = web_worker_fn.clone();

                    tokio::task::spawn_blocking(|| { // spawn on a thread that could be blocked during execution
                        // since communication within worker and rust not implemented, js code will log the execution times
                        let _worker_output = match web_worker_manager::execute_function(web_worker_fn.clone(), nats_message)  {
                            Ok(_worker_output) => (),
                            Err(err) => panic!("Function execution terminated in error: {:?}", err),
                        };
                    })
                    .await
                    .expect("Web Worker Execution Failed")
                }
            }
        }
    }));

    */
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