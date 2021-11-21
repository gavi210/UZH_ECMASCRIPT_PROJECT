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

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

#[tokio::main]
pub async fn execute_function(
    f: functions::FunctionDefinition,
    message: rants::Msg,
) -> Result<(), AnyError> {

    let module_loader = Rc::new(FsModuleLoader); // maybe define the path

    let create_web_worker_cb = Arc::new(|_| { // function is invoked when instantiating a new worker
        println!("Instantiating WebWorker");

        let web_workers_no_child = Arc::new(|_| {
            todo!("Web workers are not supported in the example");
        });

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
            create_web_worker_cb: web_workers_no_child, // web worker doesn't have the possibility to instantiate sub workers
            js_error_create_fn: None,
            worker_type: WebWorkerType::Module, // so far only type::Module is supported
            maybe_inspector_server: None,
            get_error_class_fn: Some(&get_error_class_name),
            blob_store: BlobStore::default(),
            broadcast_channel: InMemoryBroadcastChannel::default(),
            shared_array_buffer_store: None,
            compiled_wasm_module_store: None,
        };

        let permissions = Permissions::allow_all();
        let WORKER_NAME: String = "Web_Worker".to_string();

        let js_path = Path::new("./functions/function-1.js"); // path to file -> same as one provided as input parameter

        let module_specifier = match deno_core::resolve_path(&js_path.to_string_lossy()) {
          Ok(module_specifier) => module_specifier,
          Err(e) => panic!("Cannot load function definition, {:?}", e),
        };

        println!("module_specifier: {:?}", module_specifier);

        let workerId = WorkerId::default();
        return WebWorker::bootstrap_from_options(WORKER_NAME, permissions,
              module_specifier.clone(), workerId, web_worker_options)
    });

    let location_as_url_string = "https://example.com".to_string();
    let parsed_location = Url::parse(&location_as_url_string).unwrap();

    let p = message.payload();
    let message_contents = match std::str::from_utf8(p) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // instantiate MainWorker with callback for WebWorkers
    let options = WorkerOptions {
        bootstrap: BootstrapOptions {
            apply_source_maps: false,
            args: vec![message_contents.to_string()], // accessible Deno.args
            cpu_count: 1,
            debug_flag: true,
            enable_testing_features: true,
            location: Some(parsed_location),
            no_color: false,
            runtime_version: "x".to_string(),
            ts_version: "x".to_string(),
            unstable: true,
        },
        extensions: vec![], // extensions for the moment not implemented
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

    let main_module_filename = f.function_definition;
    let js_path = Path::new(&main_module_filename);
    let module_specifier = match deno_core::resolve_path(&js_path.to_string_lossy()) {
      Ok(module_specifier) => module_specifier,
      Err(e) => panic!("Cannot load function definition, {:?}", e),
    };

    let permissions = Permissions::allow_all();

    let mut worker = MainWorker::bootstrap_from_options(module_specifier.clone(), permissions, options);
    worker.execute_main_module(&module_specifier).await?;

    info!("Finish executing function: {:?}", f.name);
    Ok(())
}


