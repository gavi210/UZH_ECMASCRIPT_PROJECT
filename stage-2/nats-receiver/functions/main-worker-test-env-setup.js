console.log("Setting up testing environment...");
globalThis.loop_iterations = Deno.args[0];

globalThis.test_function = function test_function() {
    for (var i = 0; i < loop_iterations; i++) {
        ;
    }
}

globalThis.greets = function greets() {
    console.log("Invocation worked properly!");
}

console.log("Could proceed with testing...");