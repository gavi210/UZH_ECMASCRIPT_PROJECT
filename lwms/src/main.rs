// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.
//!  This example shows you how to define ops in Rust and then call them from
//!  JavaScript.
use deno_core::JsRuntime;
use deno_core::FsModuleLoader;
use deno_core::RuntimeOptions;
use std::path::Path;
use std::fs;
use url::Url;
use std::rc::Rc;
use tokio::runtime::Runtime;
// load a side module
fn load_side_module(rt: &Runtime, runtime: &mut JsRuntime) {
    //let module_url = Url::parse("file:///home/ubuntu/deno/runtime_test/target/debug/double.mjs").unwrap();
    let module_url = Url::parse("file:///home/ubuntu/deno/runtime_test/target/debug/tfjs.mjs").unwrap();
    let mut module_id = 0;
    let async_block = async {
        module_id = runtime.load_side_module(&module_url, None).await.unwrap();
        println!("Module id = {}", module_id);
    };
    rt.block_on(async_block);
    //let mut module_evaluation = runtime.mod_evaluate(module_id) ;
    let mut receiver = runtime.mod_evaluate(module_id);
    let mod_eval_async = async {
        tokio::select! {
      maybe_result = &mut receiver => {
        maybe_result.expect("Module evaluation result not provided.")
      }
      event_loop_result = runtime.run_event_loop(false) => {
        event_loop_result?;
        let maybe_result = receiver.await;
        maybe_result.expect("Module evaluation result not provided.")
      }
    }
    };
    //rt.block_on(mod_eval_async);
    let mod_eval_result = rt.block_on(mod_eval_async);
    match mod_eval_result {
        Ok(result) => println!("Module evaluated successfully..."),
        Err(error) => println!("Error evaluating module {}", error),
    }
    //if let Ok(mod_eval_result) = rt.block_on(mod_eval_async) {
    //} else {
    //    println!("Error evaluating module {}", mod_eval_result);
    //}
}
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    // Initialize a runtime instance
    let loader = Rc::new(FsModuleLoader);
    let mut runtime = JsRuntime::new(RuntimeOptions {
        module_loader: Some(loader),
        ..Default::default()
    });
    load_side_module(&rt, &mut runtime) ;
    let filename = Path::new("./input.tfjs.js");
    let script_file = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    // Now we see how to invoke the op we just defined. The runtime automatically
    // contains a Deno.core object with several functions for interacting with it.
    // You can find its definition in core.js.
    if let Ok(_execute_result) = runtime.execute_script(
        "test-script",
        &script_file,
    ) {
        println!("Execute script successful...");
    } else {
        println!("Error");
    }
    println!("Passing control to deno runtime via event_loop...");
    let event_loop_async  = async {
        //let resull = runtime.run_event_loop(false).await ;
        if let Ok(_event_loop_result) = runtime.run_event_loop(false).await {
            println!("Event loop terminated successfully...");
        } else {
            println!("Event loop terminated unsuccessfully");
        }
    };
    rt.block_on(event_loop_async);
}