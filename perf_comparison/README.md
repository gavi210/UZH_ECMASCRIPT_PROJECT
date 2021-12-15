# Comparison Description
To compare deno_core and quickJs, a following code snippet is run.

```
let loop_module = 'imported module'
function run(n) {
    loop_module.loop(n);
}

for (let i = 0; i < 10000; i++) {
    run(i);
}
```
**loop_module** defines the invoked **loop()** function.

Compilation and execution of the above code is done during the test, and time execution is measured.

## Module Import
Module import has been proved to not significantly affect engine performances. 

## Run the test
```
./target/debug/jsruntime_test 1 support_modules/test_scripts/test_01.js 
```

# Comparison Outcomes
Outcomes of the experiment shows a 3 orders of magnitude difference between the two execution times. 

```
Execution time for deno is: 43.372215 ms
Execution time for quickJs is: 9667.967495 ms
```
