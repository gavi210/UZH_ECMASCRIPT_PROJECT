globalThis.greet = function() {
    console.log("greetings from the worker");
    return "greets from worker";
}

globalThis.loop = function(iterations) {
    for (var i = 0; i < iterations; i++) {
    }
    return 1;
}

globalThis.log_this = function(message) {
    console.log(message);
    return 1;
}

globalThis.double = function(value) {
    return value * 2;
}