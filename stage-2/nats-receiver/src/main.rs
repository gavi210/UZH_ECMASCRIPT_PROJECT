use deno_core::error::AnyError;
use rants::{Client, Subject};
use tokio::task;
use log::info;
use tokio::runtime::Runtime;

use std::time::{Instant};
use std::str;

use std::sync::{Arc, Mutex};
use std::time;
use std::thread;

use deno_runtime::worker::MainWorker;
use queues::*;

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
    // define NATS subject
    let subject = f.nats_subject_trigger.parse::<Subject>().unwrap();
    let (_, mut sub) = client.subscribe(&subject, BUFFER_SIZE).await.unwrap();

    // set of workers to be used by the nats receiver
    /*
    let mut workers : Vec<MainWorker> = vec![];
    let WORKERS = 2;
    // instantiate workers
    for i in 0..WORKERS {
      let new_worker = worker::get_new_worker(&f.function_definition).await.unwrap();
      workers.push(new_worker);
    }
    */

    // Create a simple Queue
    let mut messages_queue: Queue<String> = queue![];

    // thread-safe pointer
    let messages_queue_arc = Arc::new(Mutex::new(messages_queue));

    // encapsulates threads -> could be used for NATS subject and workers together?
    //let mut handles = vec![];

    /*
    // define try use task::spawn
    for _ in 0..10 {
        let messages_cln = Arc::clone(&messages);
        let handle = thread::spawn(move || {
            let mut messages_mut = messages_cln.lock().unwrap();

            messages_mut.push("Hello from thread!".to_string());
        });
        handles.push(handle);
    }


    for handle in handles {
            handle.join().unwrap();
        }

    println!("Vector content: {:?}", *messages.lock().unwrap());

    */

    // define thread for nats receiver
    let messages_queue_cln_nats_receiver = Arc::clone(&messages_queue_arc);
    let nats_receiver_handle = task::spawn(async move {
      loop {
        // receive message
        let message = sub.recv().await.unwrap();
        let payload = message.payload();

        // payload still reference from the message instantiated before

        match payload {
          b"STOP" => {
            info!("Received stopping sequence from nats subject");
            messages_queue_cln_nats_receiver.lock().unwrap().add(String::from("STOP"));
            drop(messages_queue_cln_nats_receiver);
            info!("Adding stop message to the queue");
            break;
          },
          _ => {
            // messages_cln_nats_receiver.lock().unwrap().push(str::from_utf8(payload).unwrap().to_string());
            messages_queue_cln_nats_receiver.lock().unwrap().add(str::from_utf8(payload).unwrap().to_string());
          }
        }
      }
    });


    //let mut main_worker = worker::get_new_worker(&"./functions/functions-declaration.js".to_string()).await.unwrap();

    // another arc pointer for the main worker
    let messages_queue_cln_worker = Arc::clone(&messages_queue_arc);

    // use tokio thread here to run worker
    let main_worker_handle = thread::spawn(move {
        // within the thread create a new tokio event loop

        // for the moment easier to implement -> run only one worker on a specific thread

        let mut main_worker = worker::get_new_worker(&"./functions/functions-declaration.js".to_string()).await.unwrap();
        loop {
          // get access to the messages
          let mut queue_messages = messages_queue_cln_worker.lock().unwrap();

          // if length == 0 -> no incoming functions to be executed
          if (queue_messages.size() == 0) {
            drop(queue_messages);
            // wait for random time
            let ten_millis = time::Duration::from_millis(1);
            thread::sleep(ten_millis); // execute the loop again -> check whether new messages incoming
          }
          else {
            // process the execution
            let message = queue_messages.remove().unwrap();
            let stopping_sequence : String = String::from("STOP");

            // message in the queue to stop the workers
            if(message.eq(&stopping_sequence)) {
              info!("Main Worker stops!");
              break;
            }
            else {
                main_worker.execute_script("<test>", &message).unwrap();
                main_worker.run_event_loop(false).await;
            }
          }
        }
    });


    nats_receiver_handle.await;
    main_worker_handle.await;

    /*
    // define thread for one worker
    for worker in workers {
      let handle = task::spawn(async move {

        let mut thread_worker = worker;
        // assign one worker per thread
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
    }
    */

    // future::future::join_all().unwrap();
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