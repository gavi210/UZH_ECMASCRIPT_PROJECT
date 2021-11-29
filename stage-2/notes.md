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


## Measure Code Execution With Alive Workers
Since previous comparison shows no difference in performances within Main and WebWorkers, MainWorkers are used to measure 
execution time of function in alive workers.