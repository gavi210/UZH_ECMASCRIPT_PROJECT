/*
console.log("Creating WebWorker 1");
var myWorker = new Worker('/Users/riccardo_rigoni/University/erasmus_svizzera/courses_material/SoftwareMaintenanceAndEvolution/project/UZH_ECMASCRIPT_PROJECT/stage-2/nats-receiver/functions/web-worker-module.js',{type: "module"});
//var myWorker2 = new Worker('/Users/riccardo_rigoni/University/erasmus_svizzera/courses_material/SoftwareMaintenanceAndEvolution/project/UZH_ECMASCRIPT_PROJECT/stage-2/nats-receiver/functions/main-worker-test-function.js',{type: "module"});

myWorker.postMessage("Sample message");

// Listen for messages from the worker
myWorker.addEventListener("message", function(event) {
    console.log("Message from worker:", event.data); // ["foo", "bar", "baz"]
});
 */

function createWorker(path, iterations) {
    return new Promise(function(resolve, reject) {
        var v = new Worker(path,{type: "module"});
        v.postMessage(iterations);

        v.onmessage = function(event){
            resolve(event.data)
            //console.log("Received Message: ", event.data);
            return event.data;
        };

        v.onerror = function(event) {
            reject(event.error);
        };
    });
}

const test_iterations = Deno.args[0];
const loop_iterations = Deno.args[1];

const PATH = '/Users/riccardo_rigoni/University/erasmus_svizzera/courses_material/SoftwareMaintenanceAndEvolution/project/UZH_ECMASCRIPT_PROJECT/stage-2/nats-receiver/functions/web-worker-module.js';
var execution_times = [];

for(var i = 0; i < test_iterations; i++) {
    const t0 = performance.now();
    await createWorker(PATH, loop_iterations);
    const t1 = performance.now();
    execution_times.push(t1-t0);
}

console.log("WebWorkers Execution Times");
console.log(execution_times);

