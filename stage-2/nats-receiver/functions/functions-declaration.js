globalThis.greet = function() {
    return "greets from worker";
}

globalThis.loop = function(iterations) {
    for (var i = 0; i < iterations; i++) {
    }
    return "loop completed";
}

globalThis.log_this = function(message) {
    console.log(message);
    return 1;
}

globalThis.double = function(value) {
    return value * 2;
}