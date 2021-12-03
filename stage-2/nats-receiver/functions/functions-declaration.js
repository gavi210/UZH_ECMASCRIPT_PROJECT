globalThis.greet = function() {
    console.log("greetings from the worker");
}

globalThis.loop = function(iterations) {
    console.log("Before loop, iterations: " + iterations);
    for (var i = 0; i < iterations; i++) {
        console.log("In loop");
    }
    console.log("After loop, iterations: " + iterations);
}

globalThis.log_this = function(message) {
    console.log(message);
}