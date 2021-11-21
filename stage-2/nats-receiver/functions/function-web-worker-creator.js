console.log("Creating WebWorker 1");
var myWorker = new Worker('function-1.js');

console.log("Creating WebWorker 2");
// here the code will fail
// var myWorker2 = new Worker('function-2.js');



