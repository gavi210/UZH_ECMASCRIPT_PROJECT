use rquickjs::{
    BuiltinLoader, BuiltinResolver, Context, FileResolver, Func, ModuleLoader, NativeLoader,
    Runtime, ScriptLoader, Evaluated
};

use std::time::{Duration, Instant};

fn print(msg: String) {
    println!("{}", msg);
}

pub fn run_tests<'a>(test_files: &'a mut Vec<String>, quick_js_exec_times: &'a mut Vec<Duration>) -> Result<(), &'a str> {
    let resolver = (
        BuiltinResolver::default(),
        FileResolver::default()
            .with_path("./support_modules/quick_js")
            .with_native()
    );
    let loader = (
        BuiltinLoader::default(),
        ModuleLoader::default(),
        ScriptLoader::default(),
        NativeLoader::default(),
    );

    let rt = Runtime::new().unwrap();
    let ctx = Context::full(&rt).unwrap();

    rt.set_loader(resolver, loader);
    ctx.with(|ctx| {
        let global = ctx.globals();
        global.set("print", Func::new("print", print)).unwrap();
        let start_time = Instant::now();
        let module = ctx.compile(
            "",
            r#"
              import { loop } from "loop"

              print("Running test 01 from quickJs");

              function run(n) {
                  loop(n);
              }

              for (let i = 0; i < 10000; i++) {
                  run(i);
              };
            "#,).unwrap();
        let duration = start_time.elapsed();
        quick_js_exec_times.push(duration);
    });
    Ok(())
}
