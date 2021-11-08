use rquickjs::{
    BuiltinLoader, BuiltinResolver, Context, FileResolver, Func, ModuleLoader, NativeLoader,
    Runtime, ScriptLoader,
};

use std::time::{Duration};

fn print(msg: String) {
    println!("{}", msg);
}

pub fn run_tests<'a>(test_files: &'a mut Vec<String>, quick_js_exec_times: &'a mut Vec<Duration>) -> Result<(), &'a str> {
    let resolver = (
        BuiltinResolver::default(),
        FileResolver::default()
            .with_path("./support_modules/side_modules")
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

        println!("import script module");
        ctx.compile(
            "test",
            r#"
import { n, s, f } from "script_module";
print(`n = ${n}`);
print(`s = "${s}"`);
print(`f(2, 4) = ${f(2, 4)}`);
"#,
        )
        .unwrap();


    });
    Ok(())
}
