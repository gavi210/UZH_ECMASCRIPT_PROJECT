/*
console.log("Creating WebWorker 1");
var myWorker = new Worker('/Users/riccardo_rigoni/University/erasmus_svizzera/courses_material/SoftwareMaintenanceAndEvolution/project/UZH_ECMASCRIPT_PROJECT/stage-2/nats-receiver/functions/function-1.js',{type: "module"});
//var myWorker2 = new Worker('/Users/riccardo_rigoni/University/erasmus_svizzera/courses_material/SoftwareMaintenanceAndEvolution/project/UZH_ECMASCRIPT_PROJECT/stage-2/nats-receiver/functions/function-2.js',{type: "module"});

myWorker.postMessage("Sample message");

// Listen for messages from the worker
myWorker.addEventListener("message", function(event) {
    console.log("Message from worker:", event.data); // ["foo", "bar", "baz"]
});
 */

function createWorker(path) {
    return new Promise(function(resolve, reject) {
        var v = new Worker(path,{type: "module"});
        v.postMessage(i);
        v.onmessage = function(event){
            // If you report errors via messages, you'd have a branch here for checking
            // for an error and either calling `reject` or `resolve` as appropriate.
            resolve(event.data)
            console.log("Received Message: ", event.data);
            return event.data;
        };
        // OR:
        v.onerror = function(event) {
            // Rejects the promise using the error associated with the ErrorEvent
            reject(event.error);
        };
    });
}

const PATH = '/Users/riccardo_rigoni/University/erasmus_svizzera/courses_material/SoftwareMaintenanceAndEvolution/project/UZH_ECMASCRIPT_PROJECT/stage-2/nats-receiver/functions/function-1.js';
var execution_times = [];
for(var i = 0; i < 10; i++) {
    const t0 = performance.now();
    await createWorker(PATH);
    const t1 = performance.now();
    execution_times.push(t1-t0);
}

console.log(execution_times);

