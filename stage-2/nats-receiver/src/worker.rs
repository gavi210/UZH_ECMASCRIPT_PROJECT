use crate::functions;
use deno_core::error::AnyError;
use deno_core::FsModuleLoader;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::worker::MainWorker;
use deno_runtime::web_worker::WebWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use deno_runtime::web_worker::WorkerId;
use deno_runtime::web_worker::WebWorkerOptions;
use deno_runtime::web_worker::WebWorkerType;
use log::{info, trace, warn};
use rants;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use url::Url;
use std::time::{Instant, Duration};

use crate::util::nats_messages::NatsMessage;

#[derive(Debug, Clone)]
pub struct Worker {
    pub function: functions::FunctionDefinition,
}

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

#[tokio::main]
pub async fn run_test(
    f: functions::FunctionDefinition, // obj describing function to be executed
    raw_message : rants::Msg
) -> Result<Vec<Duration>, AnyError> {


    let main_module_filename = f.function_definition; // where function is defined

    let payload = raw_message.payload();
    let str_message = match std::str::from_utf8(payload) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let nats_message : NatsMessage = serde_json::from_str(&str_message).unwrap();

    let location_as_url_string = "https://approuter.dev".to_string();
    let parsed_location = Url::parse(&location_as_url_string).unwrap();



    let js_path = Path::new(&main_module_filename); // path to file

    let module_specifier = match deno_core::resolve_path(&js_path.to_string_lossy()) {
      Ok(module_specifier) => module_specifier,
      Err(e) => panic!("Cannot load function definition, {:?}", e),
    };

    let permissions = Permissions::allow_all();

    let mut execution_times = vec![];

    for i in 0..nats_message.test_iterations {
        let start_time = Instant::now();

        let module_loader = Rc::new(FsModuleLoader);
        let create_web_worker_cb = Arc::new(|_| { // thread safe pointer
            todo!("Web workers are not supported in the example");
        });

        let location_as_url_string = "https://approuter.dev".to_string();
        let parsed_location = Url::parse(&location_as_url_string).unwrap();

        let options = WorkerOptions {
          bootstrap: BootstrapOptions {
              apply_source_maps: false,
              args: vec![nats_message.loop_iterations.to_string()],
              cpu_count: 1,
              debug_flag: false,
              enable_testing_features: true,
              location: Some(parsed_location),
              no_color: false,
              runtime_version: "x".to_string(),
              ts_version: "x".to_string(),
              unstable: true,
          },
          extensions: vec![],
          unsafely_ignore_certificate_errors: None,
          root_cert_store: None,
          user_agent: "hello_runtime".to_string(),
          seed: None,
          js_error_create_fn: None,
          create_web_worker_cb,
          maybe_inspector_server: None,
          should_break_on_first_statement: false,
          module_loader,
          get_error_class_fn: Some(&get_error_class_name),
          origin_storage_dir: None,
          blob_store: BlobStore::default(),
          broadcast_channel: InMemoryBroadcastChannel::default(),
          shared_array_buffer_store: None,
          compiled_wasm_module_store: None,
      };
      let mut worker = MainWorker::bootstrap_from_options(module_specifier.clone(), permissions.clone(), options);

      worker.execute_main_module(&module_specifier).await?;
      worker.run_event_loop(false).await?;

      let duration = start_time.elapsed();
      execution_times.push(duration);
    }
    Ok(execution_times)
}

pub async fn get_new_worker(
  main_module_filename: &String // file path to function
) -> Result<MainWorker, AnyError> {

    let js_path = Path::new(&main_module_filename); // path to file
    let module_specifier = match deno_core::resolve_path(&js_path.to_string_lossy()) {
      Ok(module_specifier) => module_specifier,
      Err(e) => panic!("Cannot load function definition, {:?}", e),
    };
    // since payload not accessible at beginning, sender should customize function invocation through parameters in the NATS message

    let location_as_url_string = "https://approuter.dev".to_string();
    let parsed_location = Url::parse(&location_as_url_string).unwrap();

    let permissions = Permissions::allow_all();
    let module_loader = Rc::new(FsModuleLoader);
    let create_web_worker_cb = Arc::new(|_| { // thread safe pointer
        todo!("Web workers are not supported in the example");
    });

    let options = WorkerOptions {
        bootstrap: BootstrapOptions {
            apply_source_maps: false,
            args: vec![],
            cpu_count: 1,
            debug_flag: false,
            enable_testing_features: true,
            location: Some(parsed_location),
            no_color: false,
            runtime_version: "x".to_string(),
            ts_version: "x".to_string(),
            unstable: true,
        },
        extensions: vec![],
        unsafely_ignore_certificate_errors: None,
        root_cert_store: None,
        user_agent: "hello_runtime".to_string(),
        seed: None,
        js_error_create_fn: None,
        create_web_worker_cb,
        maybe_inspector_server: None,
        should_break_on_first_statement: false,
        module_loader,
        get_error_class_fn: Some(&get_error_class_name),
        origin_storage_dir: None,
        blob_store: BlobStore::default(),
        broadcast_channel: InMemoryBroadcastChannel::default(),
        shared_array_buffer_store: None,
        compiled_wasm_module_store: None,
    };

    let mut worker = MainWorker::bootstrap_from_options(module_specifier.clone(), permissions, options);

    worker.execute_main_module(&module_specifier).await?;
    worker.run_event_loop(false).await?;
    Ok(worker)
}

#[tokio::main]
pub async fn execute_function_web_worker(
    f: functions::FunctionDefinition, // obj describing function to be executed
    message: rants::Msg, // input received from NATS
) -> Result<(), AnyError> {

    //info!("Executing function {} ... from web workers", f.name);
    let main_module_filename = f.function_definition; // where function is defined

    let module_loader = Rc::new(FsModuleLoader);
    let create_web_worker_cb = Arc::new(|_| { // thread safe pointer
        todo!("Web workers are not supported in the example");
    });

    let location_as_url_string = "https://approuter.dev".to_string();
    let parsed_location = Url::parse(&location_as_url_string).unwrap();

    let p = message.payload();
    let message_contents = match std::str::from_utf8(p) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let options = WebWorkerOptions {
        bootstrap: BootstrapOptions {
            apply_source_maps: false,
            args: vec![message_contents.to_string()],
            cpu_count: 1,
            debug_flag: true,
            enable_testing_features: true,
            location: Some(parsed_location),
            no_color: false,
            runtime_version: "x".to_string(),
            ts_version: "x".to_string(),
            unstable: true,
        },
        use_deno_namespace: true,
        worker_type: WebWorkerType::Module,
        extensions: vec![],
        unsafely_ignore_certificate_errors: None,
        root_cert_store: None,
        user_agent: "hello_runtime".to_string(),
        seed: None,
        js_error_create_fn: None,
        create_web_worker_cb,
        maybe_inspector_server: None,
        module_loader,
        get_error_class_fn: Some(&get_error_class_name),
        blob_store: BlobStore::default(),
        broadcast_channel: InMemoryBroadcastChannel::default(),
        shared_array_buffer_store: None,
        compiled_wasm_module_store: None,
    };

    let js_path = Path::new(&main_module_filename); // path to module

    let module_specifier = match deno_core::resolve_path(&js_path.to_string_lossy()) {
      Ok(module_specifier) => module_specifier,
      Err(e) => panic!("Cannot load function definition, {:?}", e),
    };

    let permissions = Permissions::allow_all();
    let WORKER_NAME: String = "Web_Worker".to_string();

    let workerId = WorkerId::default();

    /*
    let web_worker_options = WebWorkerOptions {
        bootstrap: BootstrapOptions {
            apply_source_maps: true,
            args: vec![],
            cpu_count: 1,
            debug_flag: true,
            enable_testing_features: true,
            location: None,
            no_color: false,
            runtime_version: "x".to_string(),
            ts_version: "x".to_string(),
            unstable: true,
        },
        extensions: vec![],
        unsafely_ignore_certificate_errors: None,
        root_cert_store: None,
        user_agent: "web_worker".to_string(),
        use_deno_namespace: true,
        seed: None,
        module_loader: Rc::new(FsModuleLoader), // new default module loader
        create_web_worker_cb, // web worker doesn't have the possibility to instantiate sub workers
        js_error_create_fn: None,
        worker_type: WebWorkerType::Module, // so far only type::Module is supported
        maybe_inspector_server: None,
        get_error_class_fn: Some(&get_error_class_name),
        blob_store: BlobStore::default(),
        broadcast_channel: InMemoryBroadcastChannel::default(),
        shared_array_buffer_store: None,
        compiled_wasm_module_store: None,
    };

    */

    let permissions = Permissions::allow_all();
    let WORKER_NAME: String = "Web_Worker".to_string();

    let module_specifier = match deno_core::resolve_path(&js_path.to_string_lossy()) {
      Ok(module_specifier) => module_specifier,
      Err(e) => panic!("Cannot load function definition, {:?}", e),
    };

    let workerId = WorkerId::default();
    let (mut worker, mut handler) =  WebWorker::bootstrap_from_options(WORKER_NAME, permissions,
          module_specifier.clone(), workerId, options);

    worker.execute_main_module(&module_specifier).await?;

    //let event_loop = worker.run_event_loop(false).await; // this line takes forever
    // WITHOUT event loop still waits for completion

    info!("Finish executing function: {:?}", f.name);
    Ok(())
}


