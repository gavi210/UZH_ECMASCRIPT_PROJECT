//!  This example shows you how to load a side module and execute a script which dynamically
//!  loads the side module.

mod util;
use util::deno_wrapper;
use util::quick_js_wrapper;
use util::par_parser;

use std::env;
use std::process;

fn main() {

  let working_dir_path_buf = env::current_dir().unwrap_or_else(|err| {
    println!("Error reading current working directory: {}", err);
    process::exit(1);
  });

  let working_dir = working_dir_path_buf.to_str().unwrap();

  // assume we have two arguments - the module to load and the script to run
  let args: Vec<String> = env::args().collect();

  let mut module_names = Vec::new();
  par_parser::parse_args(&args, &working_dir, &mut module_names).unwrap_or_else(|err| {
    println!("Error parsing arguments: {}", err);
    process::exit(1);
  });

  let code = r#"
                Deno.core.print("Running test 01\n");

                function run(n) {
                    Deno.core.print("Hello from run function");
                }

                for (let i = 0; i < 10000; i++) {
                    run(i);
                }"#;
  let code_hello = r#"Deno.core.print("Hello from str")"#;

  //deno_wrapper::execute_code(code);

  let mut deno_exec_times = Vec::new();
  deno_wrapper::run_tests(&mut module_names, &mut deno_exec_times).unwrap_or_else(|err| {
    println!("Error running deno_tests: {}", err);
    process::exit(1);
  });

  println!("Execution times for deno are: {:?}", deno_exec_times);


  let mut quick_js_exec_times = Vec::new();
  quick_js_wrapper::run_tests(&mut module_names, &mut quick_js_exec_times);

}

