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
  // assume we have two arguments - the module to load and the script to run
  let args: Vec<String> = env::args().collect();
  let (module_filename, main_module_filename) = par_parser::parse_args(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  let rt = tokio::runtime::Runtime::new().unwrap();

  // Initialize a runtime instance
  let loader = Rc::new(FsModuleLoader);
  let mut runtime = JsRuntime::new(RuntimeOptions {
    module_loader: Some(loader),
    ..Default::default()
  });

  module_loader::load_side_module(&rt, &mut runtime, module_filename.to_string());
  module_loader::load_main_module(&rt, &mut runtime, main_module_filename.to_string());

  println!("Main module test_02 loaded and executed");
  let start = Instant::now();
  module_loader::load_main_module(&rt, &mut runtime, main_module_filename.to_string());
  let duration = start.elapsed();
  println!("Time elapsed in loading & executing the module is: {:?}", duration);

}

