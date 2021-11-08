use deno_core::JsRuntime;
use deno_core::FsModuleLoader;
use deno_core::RuntimeOptions;

use tokio::runtime::Runtime;
use url::Url;

use std::process;
use std::time::{Duration, Instant};
use std::rc::Rc;

/// invoked from main.rs
pub fn run_tests<'a>(test_files: &'a mut Vec<String>, deno_exec_times: &'a mut Vec<Duration>) -> Result<(), &'a str> {
    println!("Running tests with deno_core");

    // INITIALIZATION
    let rt = tokio::runtime::Runtime::new().unwrap();

    let loader = Rc::new(FsModuleLoader);
    let mut runtime = JsRuntime::new(RuntimeOptions {
      module_loader: Some(loader),
      ..Default::default()
    });

    // RUNNING TESTS
    for test_file in test_files.iter() {
        let start_time = Instant::now();
        execute_side_module(&rt, &mut runtime, test_file.to_string());
        let duration = start_time.elapsed();
        deno_exec_times.push(duration);
        //println!("Time elapsed in loading & executing the module is: {:?}", duration);
    }

    Ok(())
}

/// url to match the module file
/// in format file:///<path-to-file>
fn get_module_url(module_filename: String) -> Url {
    let module_filename_as_url = "file://".to_string() + &module_filename;
    return Url::parse(&module_filename_as_url).unwrap();
}

/// load a side module
fn execute_side_module(
    rt: &Runtime, // tokio runtime instance manager
    runtime: &mut JsRuntime, // runtime instance managed by tokio
    module_filename: String
  ) {

  let module_url = get_module_url(module_filename);
  let mut module_id = 0;
  let module_loading = async {
    module_id = runtime.load_side_module(&module_url, None).await.unwrap_or_else(|_err| {
        println!("Unable to load the module: {}", module_url);
        process::exit(1);
    });
  };

  rt.block_on(module_loading); // wait until module load is done
  let mut receiver = runtime.mod_evaluate(module_id); // receiver store the result of module evaluation

  let mod_eval_async = async {
    tokio::select! { // execute multiple branches in parallel, the first to finish matching the pattern on the left, will be chosen
      maybe_result = &mut receiver => { //IMPORTANT HERE -> IN DOCUMENTATION SHOULD BE ADDED .NEXT() FUNCTION
        maybe_result.expect("Module evaluation result not provided.")
      }

      event_loop_result = runtime.run_event_loop(false) => {
        event_loop_result?;
        let maybe_result = receiver.await;
        maybe_result.expect("Module evaluation result not provided.")
      }
    }
  };

  let mod_eval_result = rt.block_on(mod_eval_async);
    match mod_eval_result {
        Ok(_result) => (),
        Err(error) => println!("Error evaluating module {}", error),
    }
}

// load main module
fn execute_main_module(
    rt: &Runtime, // tokio runtime instance manager
    runtime: &mut JsRuntime, // runtime instance managed by tokio
    main_module_filename: String
  ) {

  let module_url = get_module_url(main_module_filename);
  let mut module_id = 0;
  let module_loading = async {
    module_id = runtime.load_main_module(&module_url, None).await.unwrap_or_else(|_err| {
        println!("Unable to load the module: {}", module_url);
        process::exit(1);
    });
  };

  rt.block_on(module_loading); // wait until module load is done

  // testing performance on module_evaluate
  let mut receiver = runtime.mod_evaluate(module_id); // receiver store the result of module evaluation

  let mod_eval_async = async {
    tokio::select! { // execute multiple branches in parallel, the first to finish matching the pattern on the left, will be chosen
      maybe_result = &mut receiver => { //IMPORTANT HERE -> IN DOCUMENTATION SHOULD BE ADDED .NEXT() FUNCTION
        maybe_result.expect("Module evaluation result not provided.")
      }

      event_loop_result = runtime.run_event_loop(false) => {
        event_loop_result?;
        let maybe_result = receiver.await;
        maybe_result.expect("Module evaluation result not provided.")
      }
    }
  };

  let mod_eval_result = rt.block_on(mod_eval_async);
    match mod_eval_result {
        Ok(_result) => (),
        Err(error) => println!("Error evaluating module {}", error),
    }
}