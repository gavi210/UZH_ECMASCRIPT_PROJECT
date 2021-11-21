console.log("Before loading the worker");

const module_name = './double.mjs'
const module = await import(module_name);

console.log("Imported module");
var myWorker = new Worker('function-1.js');

console.log("After loading the worker");
