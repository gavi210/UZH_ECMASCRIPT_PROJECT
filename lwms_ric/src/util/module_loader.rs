use deno_core::JsRuntime;
use tokio::runtime::Runtime;
use url::Url;
use std::process;

/// url to match the module file
/// in format file:///<path-to-file>
fn get_module_url(module_filename: String) -> Url {
    let module_filename_as_url = "file://".to_string() + &module_filename;
    return Url::parse(&module_filename_as_url).unwrap();
}

/// load a side module
pub fn execute_side_module(
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
pub fn execute_main_module(
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