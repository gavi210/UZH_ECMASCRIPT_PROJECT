# Notes for next week work

## Should look at
- rusty_v8: bindings with Rust and V8 engine -> could extract runtime information during code execution


## Deno Default Permissions
By default deno has no permissions granted to access the surrounding environment or the web. When initializing the v8 environment, 
deno should be allowed to have such permissions. (Can different Isolates withing the same v8 environment have different permissions?).
Workers cannot access the deno namespace, therefore, cannot read the file system where they are executed. (Permissions has to be granted
when instantiating the Worker).

Deno namespace (with all functionalities provided) are not by default accessible by the Workers. Permissions have to be granted.
To provide examples on permissions and functionalities when embedding Deno in Rust, examples provided as [here](https://deno.land/manual/runtime/workers).

Deno_Runtime -> look at whether is possible to share objects or not. This module is a subset of the initial functionalities
    therefore, only js execution is available. MainWorker encapsulates a JsRuntime. 
    Should be possible to have v8 inspector after loading the MainWorker. 

#### Console.log() method invocation
Looking at deno_core doc [initialize_context()](https://github.com/denoland/deno/blob/main/core/bindings.rs) function, some
built-in functionalities are loaded. These functionalities have been defined by deno_core developers and allows to trigger 
Rust functions from js code.
Therefore, the ability of Js code executed in deno_core to invoke console.log() is due to these predefined and always loaded bindings.

#### Extensions for the WebWorker instantiated
The [blog](https://fettblog.eu/dissecting-deno/) provides the set of extensions that could be provided to the WebWorker.
In the example is the MainWorker instantiated, but they should be extendable to the WebWorker instance.

## Concept of Web Worker
Worker runs a background .js file. It is loaded and instantiated over that file, executes it and communicates the results
back to the parent invoker. 
Communication happens via message passing techniques. Messages are needed, since Worker executes under a different global context.
Therefore, data from the parent invoker window are not directly available.

#### WebWorker in deno_core
**deno_core** crate doesn't support WebWorkers natively. It only allows to instantiate JsRuntimes in which execute js code.
Therefore, **deno_runtime** crate is used to instantiate and manage WebWorkers.

#### deno_runtime
This crate allows to instantiate a **MainWorker**, and subsequent **WebWorker** are descendent of it.

## v8_platform 
Within the same platform, multiple isolates could be dynamically allocated.
As mentioned [here](https://fettblog.eu/dissecting-deno/), ***"if you think Serverless platforms, Cloudflare workers or 
Deno Deploy work very similarly. Their workers run in one V8 platform, but with each call, you can boot up a new isolate. 
With all the safety guarantees.***
This means, that code executed in different isolates cannot access data in other isolates.

## Each Isolate has its own JsRuntime instance
v8_platform is like a Browser. Within a browser, multiple isolates could be instantiated, one per different tab opened.
Each isolate has a context, where js files are executed. From the deno [docs](https://denolib.gitbook.io/guide/advanced/interaction-with-v8)
emerges that each isolate is associated with one context, since when invoking the js code execution, there is no mean to 
specify and trigger one context instead of another. The code execution is triggered from the Isolate itself, therefore, 
the JsRuntime associated is automatically picked up.


## Todo
- Relative import -> instantiate web worker from js main module


## Discord Uploaded Question
Good morning, 
I'm embedding deno in rust and I'm facing some difficulties in instantiating web workers within a main worker.
I have the following .js code being executed by a MainWorker. 
```javascript
// ./functions/function-web-worker-creator.js
var myWorker = new Worker('function-1.js');
```
The code should instantiates a web worker running the "function-1.js" file. The "function-1.js" is located in the same folder
as the "function-web-worker-creator.js" file.
Follows the rust code embedding Main and WebWorkers.
```rust
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
    f: functions::FunctionDefinition, // input parameters are not used for the moment
    message: rants::Msg,
) -> Result<(), AnyError> {

    let module_loader = Rc::new(FsModuleLoader);

    let create_web_worker_cb = Arc::new(|_| {
        let web_workers_no_child = Arc::new(|_| {
            todo!("Web workers are not supported");
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
            module_loader: Rc::new(FsModuleLoader),
            create_web_worker_cb: web_workers_no_child,
            js_error_create_fn: None,
            worker_type: WebWorkerType::Module,
            maybe_inspector_server: None,
            get_error_class_fn: Some(&get_error_class_name),
            blob_store: BlobStore::default(),
            broadcast_channel: InMemoryBroadcastChannel::default(),
            shared_array_buffer_store: None,
            compiled_wasm_module_store: None,
        };

        let permissions = Permissions::allow_all();
        let WORKER_NAME: String = "WebWorker".to_string();

        let js_path = Path::new("./functions/function-1.js"); // hard-coded path to module for the WebWorker

        let module_specifier = match deno_core::resolve_path(&js_path.to_string_lossy()) {
          Ok(module_specifier) => module_specifier,
          Err(e) => panic!("Cannot load function definition, {:?}", e),
        };

        let workerId = WorkerId::default();
        
        // HERE WILL FAIL TO BOOTSTRAP
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

    let main_module_filename = f.function_definition;
    let js_path = Path::new(&main_module_filename);
    let module_specifier = match deno_core::resolve_path(&js_path.to_string_lossy()) {
      Ok(module_specifier) => module_specifier,
      Err(e) => panic!("Cannot load function definition, {:?}", e),
    };

    let permissions = Permissions::allow_all();

    let mut worker = MainWorker::bootstrap_from_options(module_specifier.clone(), permissions, options);
    
    // main module's execution fails cause web worker is not instantiated
    worker.execute_main_module(&module_specifier).await?;

    info!("Finish executing function: {:?}", f.name);
    Ok(())
}
```

For the MainWorker I implemented the function **create_web_worker_cb** returning the WebWorker and its handle, but the WebWorker fails
to be instantiated due to the following error: 
```
thread 'worker-0' panicked at 'Failed to execute worker bootstrap script: Uncaught TypeError: Invalid URL
    at deno:core/01_core.js:138:11', /Users/<user_name>/.cargo/registry/src/github.com-1ecc6299db9ec823/deno_runtime-0.31.0/web_worker.rs:431:8

```

I thought the problem to be related to the js_path for the .js file to be run by the WebWorker, but even providing a full file path
let the same error.

Does someone know why the WebWorker bootstrap fails? 
More generally, is this the right way to manage Main and WebWorkers when embedding deno? 

Thank you in advance! 
