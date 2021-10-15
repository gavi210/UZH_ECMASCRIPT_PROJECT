# Comparison Table

| Runtime/Platform (Engine) | ECMAScript compatibility                    | Size of runtime in MB      | Ease of integration                         | Open Source  | WASM support                       | Ability to precompile Scripts | Intelligence within runtime                 | Support for isolation             | Multithreading support                      | Planned support          |
|---------------------------|---------------------------------------------|----------------------------|---------------------------------------------|--------------|------------------------------------|-------------------------------|---------------------------------------------|-----------------------------------|---------------------------------------------|--------------------------|
| Chakra                    | 5.1 (full) 6.x (partial)                    |                            | C++, C#, Python, Linux, OS X,CMake          | Yes          | Yes, but not provided for embedder | Should not be able ?          | Simple JIT: low opt.Full JIT: high opt      | Should be offered by the Closures | No                                          | Community project        |
| JavaScriptCore            | ECMA-262                                    |                            | No integration found                        | Should be    | Yes                                | Yes                           | DFG and FTL compilers                       | Should be offered by the Closures | No                                          |                          |
| V8                        | ECMA-262                                    |                            | C++, C# .NET,Python                         | Yes          | Yes                                | Yes                           | A lot                                       | Should be offered by the Closures | No                                          |                          |
| Hermes                    | React Native framework & ECMA-402           |                            | C, C++, Python                              | Yes          | No                                 | yes                           | No JIT precompilation, but ahed compilation | Should be offered by the Closures | No                                          |                          |
| Js-interpreter            | Limited set of recognized language features |                            | Not found integration                       | Yes          | No                                 |                               | No intelligence                             | Sandbox each running instance     | Yes, multiple instances together            |                          |
| (Rhino)                   | ECMA-357 (R-1)                              |                            | Java (R-2)                                  | Yes (R-3)    |                                    |                               | Yes (R-4)                                   | Yes (R-5)                         | Yes, because of the JVM?                    | Active (R-3)             |
| (Duktape)   		        | E5, E6&E7 (partial) (Du-1)                  | 148KB ROM, 78KB RAM (Du-2) | C&C++ (Du-1), Python (Du-3), Go&Java (Du-4) | Yes (Du-1)   |                                    |                               | Almost no optimization (Du-5)               | Yes (Du-6)                        | Only one thread per heap (Du-7)             | Active (Du-1)            |
| Iot.Js (JerryScript)      | ECMAScript 5.1 (J-1)                        | 256KB ROM, 256KB RAM (J-2) | C (J-1)                         	         | Yes (J-1)    |                                    | Snapshot (J-1)                |                                             |                                   |                                             | IoT.js not active (J-3)  |
| Deno (V8)                 | ES6 (D-1)                                   |                            | Rust (D-2)                      	         | Yes (D-3)    | Yes (D-4)                          | Using TypeScript (D-5)        | JIT (D-6)                                   | Sandboxing (D-7)                  | Using workers (D-8)                         | Active Development (D-9) |

## Sources
R-1 http://web.archive.org/web/20190108031118/https://developer.mozilla.org/en-US/docs/Mozilla/Projects/Rhino/Overview<br />
R-2 http://web.archive.org/web/20160514071428/https://developer.mozilla.org/en-US/docs/Mozilla/Projects/Rhino/Embedding_tutorial<br />
R-3 https://github.com/mozilla/rhino<br />
R-4 http://web.archive.org/web/20160809015503/https://developer.mozilla.org/en-US/docs/Mozilla/Projects/Rhino/Optimization<br />
R-5 https://github.com/javadelight/delight-rhino-sandbox<br />
R-6 

Du-1 https://github.com/svaarala/duktape/<br />
Du-2 https://ieeexplore.ieee.org/document/9243749<br />
Du-3 https://github.com/stefano/pyduktape<br />
Du-4 https://github.com/rosbit/duktape-bridge<br />
Du-5 https://wiki.duktape.org/compiler.html<br />
Du-6 https://github.com/svaarala/duktape/blob/master/doc/sandboxing.rst<br />
Du-7 https://github.com/svaarala/duktape/blob/master/doc/threading.rst<br />

J-1 https://github.com/jerryscript-project/jerryscript<br />
J-2 https://ieeexplore.ieee.org/abstract/document/7724687<br />
J-3 https://github.com/jerryscript-project/iotjs<br />

D-1 https://de.wikipedia.org/wiki/Deno_(Software)<br />
D-2 https://github.com/denoland/rusty_v8<br />
D-3 https://github.com/denoland/deno<br />
D-4 https://deno.land/manual@v1.15.1/webassembly/using_wasm<br />
D-5 https://deno.land/manual/tools/compiler<br />
D-6 https://stackoverflow.com/questions/59807938/the-confusion-with-jit-compilation-in-v8<br />
D-7 https://github.com/denoland/deno<br />
D-8 https://deno.land/manual/runtime/workers<br />
D-9 https://github.com/denoland/deno/pulse<br />
