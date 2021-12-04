use deno_core::error::AnyError;
use rants::{Client, Subject};
use tokio::task;
use log::info;
use tokio::runtime::Runtime;
use deno_core::serde_v8;
use deno_core::v8;

use std::str;

use std::sync::{Arc, Mutex};
use std::time;
use std::thread;

use queues::*;

mod functions;
mod worker;
mod util;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    util::logger::configure_logger();

    // queue for sharing the messages
    let messages_queue: Queue<String> = queue![];
    // thread-safe pointer for the queue
    let messages_queue_arc = Arc::new(Mutex::new(messages_queue));

    let THREADS = 2; // support threads running a main worker -> could be customized: maybe as cmd parameter

    // THREAD FOR FUNCTION INVOCATION'S REQUESTS
    let messages_queue_cln_nats_receiver = Arc::clone(&messages_queue_arc);

    let nats_receiver_handle = task::spawn(async move {

      let configuration = util::config_parser::get_configuration_object();

      let client = get_nats_client(&configuration.nats_server).await.unwrap();
      let subject_message_receiver = configuration.subject_function_invoker.clone().parse::<Subject>().unwrap();
      let (_, mut sub) = client.subscribe(&subject_message_receiver, 1024).await.unwrap();

      loop {

        let message = sub.recv().await.unwrap();
        let payload = message.payload();

        match payload {
          b"STOP" => {
            //info!("Received stopping sequence from nats subject");
            // for each thread add stopping message
            let mut message_queue = messages_queue_cln_nats_receiver.lock().unwrap();
            for _ in 0..THREADS {
              message_queue.add(String::from("STOP")).unwrap();
            }
            drop(message_queue);
            info!("Stops the support threads");
            break;
          },
          _ => {
            // add message to the queue
            messages_queue_cln_nats_receiver.lock().unwrap().
              add(str::from_utf8(payload).unwrap().to_string()).unwrap();
          }
        }
      }

      client.disconnect().await;
    });

   // THREADS FOR FUNCTION EXECUTIONS
    let mut thread_handles = vec![];

    for _ in 0..THREADS {
         // each thread has its own queue reference
         let messages_queue_cln_worker = Arc::clone(&messages_queue_arc);

         thread_handles.push(
           thread::spawn(move || {

              let configuration = util::config_parser::get_configuration_object();
              let function_definition = &configuration.functions[0].function_definition;
              let STOPPING_SEQUENCE : String = String::from("STOP");

             // within the thread create a new tokio event loop
             let runtime = Runtime::new().expect("Unable to create the runtime");

             runtime.block_on(async move {
                 let mut main_worker = worker::get_new_worker(&function_definition.to_string()).await.unwrap();

                 let client = get_nats_client(&configuration.nats_server).await.unwrap();
                 let subject_results_receiver = configuration.subject_results_receiver.parse::<Subject>().unwrap();
                 loop {
                   // at every iteration, get access to the queue to process new incoming messages
                   let mut queue_messages = messages_queue_cln_worker.lock().unwrap();

                   if queue_messages.size() == 0 { // no messages
                     drop(queue_messages);

                     // wait for random time -> give priority to NATS subject to inject messages
                     let ten_millis = time::Duration::from_millis(1);
                     thread::sleep(ten_millis);
                   }
                   else {
                     // execute requested function
                     let message = queue_messages.remove().unwrap();
                     let queue_size = queue_messages.size();
                     drop(queue_messages);

                     // message in the queue to stop the workers
                     if message.eq(&STOPPING_SEQUENCE) {
                        if queue_size == 1 { // means the last stopping sequence has been processed -> stop result receiver
                          client.publish(&subject_results_receiver, b"STOP").await.unwrap();
                        }
                       info!("Main Worker stops!");
                       break;
                     }
                     else {
                        let context = &mut main_worker.js_runtime;

                        // deserialize message into object
                        let c: util::nats_messages::NatsMessage = serde_json::from_str(&message).unwrap();

                        let res = context.execute_script("<test>", &c.message);
                        let result = match res {
                           Ok(global) => {
                             let scope = &mut context.handle_scope();
                             let local = v8::Local::new(scope, global);
                             // Deserialize a `v8` object into a Rust type using `serde_v8`,
                             // in this case deserialize to a JSON `Value`.
                             let deserialized_value =
                               serde_v8::from_v8::<serde_json::Value>(scope, local);

                             match deserialized_value {
                               Ok(value) => Ok(value),
                               Err(err) => Err(format!("Cannot deserialize value: {:?}", err)),
                             }
                           }
                           Err(err) => Err(format!("Evaling error: {:?}", err)),
                         }.unwrap();

                         let mut str_result : String;
                         // deserialize the result depending on the type
                          if result.is_string() {
                             str_result = result.as_str().unwrap().to_string();
                          }
                          else {
                            str_result = result.as_i64().unwrap().to_string();
                          }

                          let nats_message = util::nats_messages::NatsMessage {
                            id : c.id,
                            message : str_result
                          };
                          let str_nats_message = serde_json::to_string(&nats_message).unwrap();
                          client.publish(&subject_results_receiver,str_nats_message.as_bytes()).await.unwrap();
                     }
                   }
                 }

                 client.disconnect().await;
             });
         })
       );
    }

    // nats receiver is added to the event_loop associated with the main thread
    nats_receiver_handle.await.unwrap();

    // handle for another thread
    for handle in thread_handles {
      handle.join().unwrap();
    }

    Ok(())
}

// instantiate and connect NATS client
async fn get_nats_client(nats_server : &String) -> Result<Client, AnyError> {
  let address = nats_server.parse().unwrap();
  let client = Client::new(vec![address]);
  client.connect().await;

  Ok(client)
}