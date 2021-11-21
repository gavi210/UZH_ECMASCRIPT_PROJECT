# Stage-2
This folder contains code to meet the new specified [project objectives](https://github.com/gavi210/UZH_ECMASCRIPT_PROJECT/blob/main/project-objectives/README.md).

## Nats Docker Execution
To run NATS server in Docker container: 
```
docker run -p 4222:4222 -ti nats:latest
```

## Security Implication using WebWorkers
As mentioned [here](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API), workers executes under a different global scope.
Therefore, data in the parent workers shouldn't be accessible from the WebWorker. The only way to share data is through message
passing technique. No global objects could be created to impact other WebWorkers.

```deno_runtime::permissions::Permissions;``` allows to specify which permissions each WebWorker has. 
The list of all permissions are: ```read, write, net, env, run, ffi, hrtime```.

## Performance Implications
To assets performance implications, WebWorkers has to be instantiated from a MainWorker. 
Currently, creation of web workers within MainWorkers has to be implemented. 

The following code snippet implements the function invoked from the MainWorker to instantiate a new WebWorker
```
let create_web_worker_cb = Arc::new(|_| { // function is invoked when instantiating a new worker
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

        let js_path = Path::new(<path_to_file>); 

        let module_specifier = match deno_core::resolve_path(&js_path.to_string_lossy()) {
          Ok(module_specifier) => module_specifier,
          Err(e) => panic!("Cannot load function definition, {:?}", e),
        };

        let workerId = WorkerId::default(); // maybe cause problems, duplicate worker ids
        return WebWorker::bootstrap_from_options(WORKER_NAME, permissions,
              module_specifier.clone(), workerId, web_worker_options)
    });
```

Currently, the MainWorker fails to instantiate the WebWorker, cause **invalid URL** error is thrown.


