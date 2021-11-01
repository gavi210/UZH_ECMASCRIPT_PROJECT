//!  This example shows you how to load a side module and execute a script which dynamically
//!  loads the side module.

mod util;
use util::module_loader;
use util::par_parser;

use deno_core::JsRuntime;
use deno_core::FsModuleLoader;
use deno_core::RuntimeOptions;

use url::Url;
use std::rc::Rc;
use tokio::runtime::Runtime;
use std::env;
use std::process;

use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
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
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  //println!("Module names: {:?}", module_names);
  let rt = tokio::runtime::Runtime::new().unwrap();

  // Initialize a runtime instance
  let loader = Rc::new(FsModuleLoader);
  let mut runtime = JsRuntime::new(RuntimeOptions {
    module_loader: Some(loader),
    ..Default::default()
  });


  for file_name in module_names.iter() {
    let start = Instant::now();
    module_loader::load_side_module(&rt, &mut runtime, file_name.to_string());
    let duration = start.elapsed();
    println!("Time elapsed in loading & executing the module is: {:?}", duration);

  }

  /*
  module_loader::load_main_module(&rt, &mut runtime, main_module_filename.to_string());

  println!("Main module test_02 loaded and executed");
  let start = Instant::now();
  module_loader::load_main_module(&rt, &mut runtime, main_module_filename.to_string());
  let duration = start.elapsed();
  println!("Time elapsed in loading & executing the module is: {:?}", duration);
  */
}

