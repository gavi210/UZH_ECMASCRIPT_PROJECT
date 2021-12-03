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
    let f = &configuration.functions[0]; // main module to execute the main worker

    let client = get_nats_client(&configuration.nats_server).await.unwrap();
    const BUFFER_SIZE: usize = 1024;

    // define NATS subject
    let subject = f.nats_subject_trigger.parse::<Subject>().unwrap();
    let (_, mut sub) = client.subscribe(&subject, BUFFER_SIZE).await.unwrap();

    // queue for sharing the messages
    let mut messages_queue: Queue<String> = queue![];
    // thread-safe pointer
    let messages_queue_arc = Arc::new(Mutex::new(messages_queue));

    let THREADS = 2; // support threads running a main worker -> could be customized: maybe as cmd parameter

    // define thread for nats receiver
    let messages_queue_cln_nats_receiver = Arc::clone(&messages_queue_arc);
    let nats_receiver_handle = task::spawn(async move {
      // as input the number of threads currently running -> stop all of them
      // maybe passing integer still borrows the value
      loop {

        let message = sub.recv().await.unwrap();
        let payload = message.payload();

        match payload {
          b"STOP" => {
            info!("Received stopping sequence from nats subject");
            let mut message_queue = messages_queue_cln_nats_receiver.lock().unwrap();
            for i in 0..THREADS {
              message_queue.add(String::from("STOP"));
            }
            drop(message_queue);
            info!("Stops the support threads");
            break;
          },
          _ => {
            // add message to the queue
            messages_queue_cln_nats_receiver.lock().unwrap().add(str::from_utf8(payload).unwrap().to_string());
          }
        }
      }
    });

    // define threads for main workers
    let mut thread_handles = vec![];

    for i in 0..THREADS {
          // clone parameters being moved inside the thread
         let function_definition = f.function_definition.clone();
         let messages_queue_cln_worker = Arc::clone(&messages_queue_arc);

         thread_handles.push(
           thread::spawn(move || {
             // within the thread create a new tokio event loop
             let mut runtime = Runtime::new().expect("Unable to create the runtime");

             // block waiting the asynchronous function
             runtime.block_on(async move {
                 let mut main_worker = worker::get_new_worker(&function_definition.to_string()).await.unwrap();
                 loop {
                   // at every iteration, get access to the queue to process new incoming messages
                   let mut queue_messages = messages_queue_cln_worker.lock().unwrap();


                   if (queue_messages.size() == 0) { // no messages
                     drop(queue_messages);

                     // wait for random time -> give priority to NATS subject to inject messages
                     let ten_millis = time::Duration::from_millis(1);
                     thread::sleep(ten_millis);
                   }
                   else {
                     // process the execution
                     let message = queue_messages.remove().unwrap();

                     drop(queue_messages); // from here queue not needed anymore
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
         })
       );
    }

    // nats receiver is added to the event_loop associated with the main thread
    nats_receiver_handle.await;

    // handle for another thread
    for handle in thread_handles {
      handle.join();
    }

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