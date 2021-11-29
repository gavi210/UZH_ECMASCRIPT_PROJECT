# Stage-2
This folder contains code to meet the new specified [project objectives](https://github.com/gavi210/UZH_ECMASCRIPT_PROJECT/blob/main/project-objectives/README.md).

## Nats Docker Execution
To run NATS server in Docker container: 
```
docker run -p 4222:4222 -ti nats:latest
```

## Performance Comparison (MainWorker vs WebWorker vs ReusedMainWorker)
### Testing Environment
To compare performances, testing environment has to be set.
The environment is composed by: 
- test function: [web-worker-module.js](nats-receiver/functions/web-worker-module.js),
- runtime comparison value: as observed with a previous performance analysis, MainWorker executes [web-worker-module.js](nats-receiver/functions/web-worker-module.js) in around 1.7 seconds,
- architecture to execute [web-worker-module.js](nats-receiver/functions/web-worker-module.js) in WebWorkers.

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

### Function Execution being measured
To run the performance comparison, a simple function is executed.
```
for (var i = 0; i < iterations; i++) {
        ;
    }
```
``iterations`` parameter could be dynamically modified.

### Performance Comparison
Following performance data have been obtained by setting ``iterations = 10000000``.

MainWorker: [
459.543518ms,
456.006672ms,
448.712379ms,
439.332617ms,
447.088314ms,
445.702962ms,
458.252155ms,
447.97186ms,
443.961152ms,
444.470175ms
]

WebWorkers: [
459.740987,
462.12933,
454.1517849999999,
445.3357199999998,
453.29438099999993,
450.8542970000003,
470.313803,
448.2627430000002,
451.08487200000036,
452.7584130000005
]

Reuse of MainWorker: [
468.768474ms, 
465.836208ms, 
5.259826ms, 
4.900947ms, 
4.783687ms, 
4.802829ms, 
5.697033ms, 
4.846011ms, 
4.693492ms, 
4.692548ms
]

As could be noticed, no significant performance difference could be noticed.

### Outcome Conclusion
As opposite to what the developers had expected, no significant performance difference could be noticed.
After an inspection of the code, emerged that the procedure to instantiate each MainWorker and WebWorker is in ``deno`` implemented the same: 
at every instantiation, a new ``Isolate`` and ``Context`` is loaded. Therefore, since the set-up work for the workers is the same, 
no difference in performance emerged. 
To speed-up function execution in WebWorkers, workers must be reused, so to avoid set-up delays. Further investigation will 
be done in this direction. 

## MainWorker <-> WebWorker Communication
[ops::worker_host](https://docs.rs/deno_runtime/latest/src/deno_runtime/ops/worker_host.rs.html#3-357) implementation specifies
that the only way to communicate with the worker in via ``worker.internal_channel``.

Each WebWorker opens a new ``std::sync::mpsc::Channel``, which is used to communicate with the parent.
An instance of ``std::sync::mpsc::Receiver`` is named ***external handle***, and it is sent back to the parent, so that it is able
to receive messages sent by the child.
An instance of ``std:.sync::mpsc::Sender`` is named ***internal handle***, and it is used by the web worker to send messages over the ``mpsc::Channel``.

