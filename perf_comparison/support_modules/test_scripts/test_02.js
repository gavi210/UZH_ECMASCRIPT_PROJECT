const loop_module_file = './../deno/loop.mjs';
const loop_module = await import(loop_module_file);

Deno.core.print("Running test 02\n");

function run(n) {
    loop_module.loop(n);
}

for (let i = 0; i < 10000; i++) {
    run(i);
}



