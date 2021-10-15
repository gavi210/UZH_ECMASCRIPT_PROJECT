# Comparison Table

|                | ECMAScript compatibility                    | Size of runtime in MB | Ease of integration                | Open Source  | WASM support                       | Ability to precompile Scripts | Intelligence within runtime                 | Support for isolation             | Multithreading support                      | OS support/architecture support | Planned support   |
|----------------|---------------------------------------------|-----------------------|------------------------------------|--------------|------------------------------------|-------------------------------|---------------------------------------------|-----------------------------------|---------------------------------------------|---------------------------------|-------------------|
| Chakra         | 5.1 (full) 6.x (partial)                    |                       | C++, C#, Python, Linux, OS X,CMake | Yes          | Yes, but not provided for embedder | Should not be able ?          | Simple JIT: low opt.Full JIT: high opt      | Should be offered by the Closures | No                                          |                                 | Community project |
| JavaScriptCore | ECMA-262                                    |                       | No integration found               | Should be    | Yes                                | Yes                           | DFG and FTL compilers                       | Should be offered by the Closures | No                                          |                                 |                   |
| V8             | ECMA-262                                    |                       | C++, C# .NET,Python                | Yes          | Yes                                | Yes                           | A lot                                       | Should be offered by the Closures | No                                          |                                 |                   |
| Hermes         | React Native framework & ECMA-402           |                       | C, C++, Python                     | Yes          | No                                 | yes                           | No JIT precompilation, but ahed compilation | Should be offered by the Closures | No                                          |                                 |                   |
| Js-interpreter | Limited set of recognized language features |                       | Not found integration              | Yes          | No                                 |                               | No intelligence                             | Sandbox each running instance     | Yes, multiple instances together            |                                 |                   |
| Rhino          | ES6, ES2016+                                |                       | Java                               | Yes          | Should not be                      | Yes                           | Yes                                         | Yes                               | Yes                                         |                                 |                   |
| Duktape        | E5, Partinal E6, E7                         |                       | C, C++, Python , Go, Java          | Yes          | should not be                      | No JIT compilation            | Not much since small footprint              |                                   | Only one active thread per Duktape instance |                                 |                   |
| JerryScript    | E5.1                                        |                       | C                                  | Yes          | Yes                                | Yes (Snapshot Support)        | ?                                           | Not sure nothing found            | Iot.js does (uses JerryScript)              |                                 |                   |
| Deno (V8)      | ES6 (D-1)                                   |                       | Rust (D-2)                         | Yes (D-3)    | Yes (D-4)                          | Using TypeScript (D-5)        | JIT (D-6)                                   | Sandboxing (D-7)                  | Using workers (D-8)                         |                                 | Active Development (D-9) |


## Sources

D-1 https://de.wikipedia.org/wiki/Deno_(Software)
D-2 https://github.com/denoland/rusty_v8
D-3 https://github.com/denoland/deno
D-4 https://deno.land/manual@v1.15.1/webassembly/using_wasm
D-5 https://deno.land/manual/tools/compiler
D-6 https://stackoverflow.com/questions/59807938/the-confusion-with-jit-compilation-in-v8
D-7 https://github.com/denoland/deno
D-8 https://deno.land/manual/runtime/workers
D-9 https://github.com/denoland/deno/pulse
