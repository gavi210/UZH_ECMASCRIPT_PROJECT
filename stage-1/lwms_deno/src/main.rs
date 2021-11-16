use deno_core::JsRuntime;

fn main() {

  // Initialize a runtime instance
  let mut runtime = JsRuntime::new(Default::default());

    let js_code = r#"
    let x = 2

    let loop = (x) => {
        return x + x
        }

    loop(2)
    "#;

    let ret = runtime.execute_script("<anon>", js_code);
    println!("ret => {:?}", ret);
}