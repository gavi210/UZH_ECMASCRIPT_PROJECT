use deno_core::error::AnyError;
use rants::{Client, Subject};
use tokio::task;
use log::info;

use std::time::{Instant};
use std::str;

use std::sync::{Arc, Mutex};
use std::thread;

use deno_runtime::worker::MainWorker;

mod functions;
mod worker;
mod web_worker_manager;
mod util;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    util::logger::configure_logger();

    let configuration = util::config_parser::get_configuration_object();

    let client = get_nats_client(&configuration.nats_server).await.unwrap();

    const BUFFER_SIZE: usize = 1024;

    // get unique function being declared -> unique subject being triggered by sender
    let f = &configuration.functions[0];
    //define NATS subject
    let subject = f.nats_subject_trigger.parse::<Subject>().unwrap();
    let (_, mut sub) = client.subscribe(&subject, BUFFER_SIZE).await.unwrap();

    // set of workers to be used by the nats receiver
    let mut workers : Vec<MainWorker> = vec![];

    // try dispatching script execution

    let WORKERS = 2;
    for i in 0..WORKERS {
      let new_worker = worker::get_new_worker(&f.function_definition).await.unwrap();
      workers.push(new_worker);
    }

    let mut messages_vec : Vec<String> = vec![];
    let messages = Arc::new(Mutex::new(messages_vec));
    let mut handles = vec![];

    // define try use task::spawn
    for _ in 0..10 {
        let messages_cln = Arc::clone(&messages);
        let handle = thread::spawn(move || {
            let mut messages_mut = messages_cln.lock().unwrap();

            *messages_mut.push("Hello from thread!".to_string());
        });
        handles.push(handle);
    }


    for handle in handles {
            handle.join().unwrap();
        }

    println!("Vector content: {:?}", *messages.lock().unwrap());



    let handle = task::spawn(async move {
        loop {
            let message = sub.recv().await.unwrap();
            let payload = message.payload();

            match payload {
                b"STOP" => { // terminate the subject
                  break;
                },
                _ => {
                    // find an available worker among the workers
                    // trigger function execution -> should allow per NATS to send the function code to be executed
                }
            }
        }
    });

    handle.await;
    client.disconnect().await;

    Ok(())
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




// instantiate and connect NATS client
async fn get_nats_client(nats_server : &String) -> Result<Client, AnyError> {
  let address = nats_server.parse().unwrap();
  let client = Client::new(vec![address]);
  client.connect().await;

  Ok(client)
}