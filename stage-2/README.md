# Stage-2
This folder contains code to meet the new specified [project objectives](https://github.com/gavi210/UZH_ECMASCRIPT_PROJECT/blob/main/project-objectives/README.md).

## Nats Docker Execution
To run NATS server in Docker container: 
```
docker run -p 4222:4222 -ti nats:latest
```

## Performance Comparison (MainWorker vs WebWorker vs ReusedMainWorker)
To execute multiple times the same function within a MainWorker, the function to be tested is declared at worker's instantiation time.
Using the ``globalThis`` object, the function is made accessible by scripts executed in ``execute_script()``.

### Function Execution being measured
To run the performance comparison, a simple function is executed.
```
for (var i = 0; i < iterations; i++) {
        ;
    }
```
``iterations`` parameter could be dynamically modified.

### Performance Comparison
Following performance data have been obtained by setting ``iterations = 10000000``.

MainWorker: [
459.543518ms,
456.006672ms,
448.712379ms,
439.332617ms,
447.088314ms,
445.702962ms,
458.252155ms,
447.97186ms,
443.961152ms,
444.470175ms
]

WebWorkers: [
459.740987,
462.12933,
454.1517849999999,
445.3357199999998,
453.29438099999993,
450.8542970000003,
470.313803,
448.2627430000002,
451.08487200000036,
452.7584130000005
]

Reuse of MainWorker: [
468.768474ms, 
465.836208ms, 
5.259826ms, 
4.900947ms, 
4.783687ms, 
4.802829ms, 
5.697033ms, 
4.846011ms, 
4.693492ms, 
4.692548ms
]

As could be noticed, the execution time when reusing a MainWorker drops significantly, by two orders of magnitude.
By looking at the execution times when reusing the worker, it could be notice that the first two times were much bigger than the 
subsequent one. This could be explained by the JIT compilation used by ``v8`` engine. When the test function is compiled and optimized, 
performances increases significantly.

### Conclusion
- No significant difference could be noticed if using MainWorkers and WebWorkers to execute a function once,
- Reusing a MainWorker to execute multiple time the same function will result in drastic increase of performances.

