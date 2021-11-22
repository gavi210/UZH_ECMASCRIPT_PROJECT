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

## Performance Comparison (MainWorker vs WebWorker)
### Testing Environment
To compare performances, testing environment has to be set.
The environment is composed by: 
- test function: [function-1.js](nats-receiver/functions/function-1.js),
- runtime comparison value: as observed with a previous performance analysis, MainWorker executes [function-1.js](nats-receiver/functions/function-1.js) in around 1.7 seconds,
- architecture to execute [function-1.js](nats-receiver/functions/function-1.js) in WebWorkers.

#### Testing Architecture
A simple architecture has been proposed to trigger function execution from NATS into WebWorkers.  
The main idea is to instantiate a MainWorker parsing the received NATS messages, and for each message, execute the corresponding function 
in a different WebWorker. 
To allow such triggering, a communication technique between the Rust Management System and the MainWorker has to be developed. 
Furthermore, the MainWorker capabilities has be extended so to instantiate WebWorkers, measure their execution times and send back the performance data.

#### Management System <-> MainWorker Communication
The communications between Management System and MainWorker could be summarized as follows.
![plot](report_images/ManagementSystem-MainWorkerCommunication.png)
[Local Ispector](https://docs.rs/deno_core/0.108.0/deno_core/struct.LocalInspectorSession.html) could be used to exchange messages.

#### WebWorker Execution
The [execute_function()](nats-receiver/src/web_worker_manager.rs) provides an example on how to instantiate WebWorkers.
The [create_web_worker_cb](https://docs.rs/deno_runtime/0.34.0/deno_runtime/ops/worker_host/type.CreateWebWorkerCb.html) dynamic function
has been implemented, and it is invoked every time the main worker instantiates a new worker. 

Nevertheless, after an in-depth analysis of the [deno_runtime](https://docs.rs/deno_runtime/0.34.0/deno_runtime/index.html) crate and a 
talk with the deno community, emerged that dynamic WebWorkers instantiation is not supported yet. Therefore, performance comparison 
cannot be conducted for the moment. 
Follows a description of the concerns related to WebWorkers instantiation.

#### WebWorkers instantiation concerns
WebWorkers are instantiated via the MainWorker with the following js code snippet.
```javascript
var worker = new Worker('<module>.js');
```
To evaluate the statement, the MainWorker has to instantiate a new WebWorker instance by referencing the ``<module>.js`` module.
To do this, a ``ModuleSpecifier`` in the format ``file:///<path_to_module/<module>.js`` has to be provided. But import using such reference
is currently not supported by the WebWorkers. Therefore, the MainWorker fails to create the WebWorker, since its main module cannot be loaded.

#### Outcomes
Even though we weren't able to instantiate WebWorkers, we learned more in detail how WebWorkers are created and how to deal with them. 
The [execute_function()](nats-receiver/src/web_worker_manager.rs) could be used in the future as a reference on how to create WebWorkers.


Execution with MainWorkers: 809.815604ms

WebWorkers:
[
725.966544,
710.617326,
712.24197,
715.0483869999998,
717.833928,
715.6802699999998,
708.4899869999999,
711.336241,
716.0352069999999,
714.6845409999996
]
