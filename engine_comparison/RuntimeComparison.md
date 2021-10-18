# List of ECMAScript Engines (https://en.wikipedia.org/wiki/List_of_ECMAScript_engines)

Here we list the ECMAScript engines, which we did not cosider as candidates and why did not consider them.

- Carakan: Not used anymore. Generally hard to find information
- JScript: Support dropped (https://en.wikipedia.org/wiki/List_of_ECMAScript_engines)
- Tamarin: Discontinued (https://en.wikipedia.org/wiki/Tamarin_(software))
- Nashorn: Deprecated (https://en.wikipedia.org/wiki/Nashorn_(JavaScript_engine))
- iv: Last commit in 2015 (https://github.com/Constellation/iv)
- CL-JavaScript: Last version is of 2012 (https://marijnhaverbeke.nl/cl-javascript/)
- BESEN: Looks like an older project. Written in Pascal.(https://github.com/BeRo1985/besen)

- Continuum: Different use and older project (https://github.com/joskid/continuum)
- Futhark (Presto): Old project (https://en.wikipedia.org/wiki/Presto_(browser_engine)#ECMAScript_engines)
- InScript: Barely any information found.
- JScript: Last stable release was 2011 (https://en.wikipedia.org/wiki/JScript)
- KJS: Seen as predecessor to JavaScriptCore(https://en.wikipedia.org/wiki/KJS_(software)), which we already consider.
- Linear B: Barely any information found.
- Narcissus: Different use (https://en.wikipedia.org/wiki/List_of_ECMAScript_engines).
- QtScript: Different use (https://en.wikipedia.org/wiki/List_of_ECMAScript_engines).
- V4 (QJSEngine): See QtScript.
- YAJI: Barely any information found.
- XS JavaScript Engine: Very promising but hard to get information (https://github.com/Moddable-OpenSource/moddable-xst).
- Jsish: Not old but momentum of the project seems gone (https://github.com/pcmacdon/jsish#readme).
- Websocket.js: Barely any information found.
- Espruino: Seems like it is intended to work with espruino boards (https://github.com/espruino).
- MuJS: Very promising candidate. Active and easily embeddable in C. Hard to get information (https://github.com/ccxvii/mujs).
- GNU Guile features an ECMAScript interpreter as of version 1.9: Uncertainty about status of project (https://www.gnu.org/software/guile/manual/html_node/ECMAScript.html).
- njs: Different use (https://en.wikipedia.org/wiki/List_of_ECMAScript_engines).
- engine262: Different use (https://en.wikipedia.org/wiki/List_of_ECMAScript_engines).

# Comparison Table

| Runtime/Platform (Engine) | ECMAScript compatibility                    | Size of runtime in MB      | Ease of integration                         | Open Source  | WASM support                       | Ability to precompile Scripts | Intelligence within runtime                 | Support for isolation             | Multithreading support                      | Planned support          |
|---------------------------|---------------------------------------------|----------------------------|---------------------------------------------|--------------|------------------------------------|-------------------------------|---------------------------------------------|-----------------------------------|---------------------------------------------|--------------------------|
| Chakra                    | 5.1, 6.x (partial)  (C-1)                   |                            | C++, C#, Linux, OS X, CMake, Python(C-2,3)  | Yes          | Yes (C-4) 			    	     |                               | Simple JIT: low opt.Full JIT: high opt (C-5)|                                   | Not directly, though Web Worker API. (C-4)  | Community project        | 
| JavaScriptCore            | ECMA-262 (JSC-1)                            |                            | No integration found                        | Yes (JSC-2)  | Yes                                | Yes                           | DFG and FTL compilers (JSC-3)               | Yes (JSC-3)                       | Yes(JSC-3)                                  | Active Project           |
| V8                        | ES6 (V-1)                                   |                            | C++, C# .NET, Python, Go (V-2,3,4,5)        | Yes          | Yes                                | Yes   (V-6)                   | A lot                                       | Yes (V-7)                         | Yes (V-8)                                   | Active Development       |
| Hermes                    | ES6 (H-1)                                   |                            | C, C++, Python, React Native App (H-2)      | Yes          | No                                 |                               | No JIT, but ahed compilation (H-3)          | No                                |                                             |                          |
| Js-interpreter            | Limited set of language features (JS-1)     |                            |                                             | Yes          | No                                 |                               | No intelligence (JS-1)                      | Different instances (JS-1)        | Yes, multiple instances together (JS-1)     |                          |
| (Rhino)                   | ECMA-262 & -357 (R-1)                       |                            | Java (R-2)                                  | Yes (R-3)    |                                    |                               | Yes (R-4)                                   | Yes (R-5)                         | Yes, because of the JVM?                    | Active (R-3)             |
| (Duktape)   		        | E5, E6&E7 (partial) (Du-1)                  | 148KB ROM, 78KB RAM (Du-2) | C&C++ (Du-1), Python (Du-3), Go&Java (Du-4) | Yes (Du-1)   | No (Du-8)                          |                               | Almost no optimization (Du-5)               | Yes (Du-6)                        | On separate heaps (Du-7)                    | Active (Du-1)            |
| Iot.Js (JerryScript)      | ECMAScript 5.1 (J-1)                        | 256KB ROM, 256KB RAM (J-2) | C (J-1)                         	   	     | Yes (J-1)    |                                    | Snapshot (J-1)                |                                             |                                   |                                             | IoT.js not active (J-3)  |
| Deno (V8)                 | ES6 (D-1)                                   |                            | Rust (D-2)                      	         | Yes (D-3)    | Yes (D-4)                          | Using TypeScript (D-5)        | JIT (D-6)                                   | Sandboxing (D-7)                  | Using workers (D-8)                         | Active Development (D-9) |
| (Spidermonkey)            | ECMA-262 (S-1)                              |                            | C++&Rust (S-2)			      			     | Yes (S-1)    | Yes (S-2)			     		     |                               | JIT (S-3)                                   |                                   |                                             |                          |
| mJS    		            | ECMA-262 (S-1)                              |                            | C++&Rust (S-2)			      			     | Yes (S-1)    | Yes (S-2)			     		     |                               | JIT (S-3)                                   |                                   |                                             |                          |
| Tiny-JS                   | ECMA-262 (S-1)                              |                            | C++&Rust (S-2)			      			     | Yes (S-1)    | Yes (S-2)			     		     |                               | JIT (S-3)                                   |                                   |                                             |                          |
| QuickJS                   | ECMA-262 (S-1)                              |                            | C++&Rust (S-2)			      			     | Yes (S-1)    | Yes (S-2)			     		     |                               | JIT (S-3)                                   |                                   |                                             |                          |
| graaljs		            | ECMA-262 (S-1)                              |                            | C++&Rust (S-2)			      			     | Yes (S-1)    | Yes (S-2)			     		     |                               | JIT (S-3)                                   |                                   |                                             |                          |


## References

### Chakra 
C-1 https://en.wikipedia.org/wiki/Chakra_(JavaScript_engine)<br />
C-2 https://github.com/chakra-core/ChakraCore/wiki/Embedding-ChakraCore<br />
C-3 https://github.com/microsoft/Chakra-Samples/tree/master/ChakraCore%20Samples/Hello%20World/Python<br />
C-4 https://blogs.windows.com/msedgedev/2017/04/20/improved-javascript-performance-webassembly-shared-memory/<br />
C-5 https://github.com/chakra-core/ChakraCore/wiki/Architecture-Overview<br />

### JavaScriptCore
JSC-1 https://trac.webkit.org/wiki/JavaScriptCore<br />
JSC-2 https://github.com/WebKit/webkit/tree/main/Source/JavaScriptCore<br />
JSC-3 https://developer.apple.com/documentation/javascriptcore/jsvirtualmachine<br />

### v8
V-1 https://nodejs.org/en/docs/es6/<br />
V-2 https://v8.dev/docs/embed<br />
V-3 https://pypi.org/project/PyV8/<br />
V-4 https://github.com/rogchap/v8go<br />
V-5 https://github.com/denoland/rusty_v8<br />
V-6 https://v8.dev/blog/custom-startup-snapshots<br />
V-7 https://github.com/fulcrumapp/v8-sandbox<br />
V-8 https://stackoverflow.com/questions/39657985/how-can-i-run-a-bunch-of-js-codes-simultaneously-using-v8-from-c.<br />


### Hermes
H-1 https://hermesengine.dev/docs/language-features<br />
H-2 https://hermesengine.dev/docs/react-native-integration<br />
H-3 https://engineering.fb.com/2019/07/12/android/hermes/<br />

### Js-interpreter
JS-1 https://neil.fraser.name/software/JS-Interpreter/docs.html<br />

### Rhino
R-1 http://web.archive.org/web/20190108031118/https://developer.mozilla.org/en-US/docs/Mozilla/Projects/Rhino/Overview<br />
R-2 http://web.archive.org/web/20160514071428/https://developer.mozilla.org/en-US/docs/Mozilla/Projects/Rhino/Embedding_tutorial<br />
R-3 https://github.com/mozilla/rhino<br />
R-4 http://web.archive.org/web/20160809015503/https://developer.mozilla.org/en-US/docs/Mozilla/Projects/Rhino/Optimization<br />
R-5 https://github.com/javadelight/delight-rhino-sandbox<br />

### Duktape
Du-1 https://github.com/svaarala/duktape/<br />
Du-2 https://ieeexplore.ieee.org/document/9243749<br />
Du-3 https://github.com/stefano/pyduktape<br />
Du-4 https://github.com/rosbit/duktape-bridge<br />
Du-5 https://wiki.duktape.org/compiler.html<br />
Du-6 https://github.com/svaarala/duktape/blob/master/doc/sandboxing.rst<br />
Du-7 https://github.com/svaarala/duktape/blob/master/doc/threading.rst<br />
Du-8 https://github.com/svaarala/duktape/blob/master/doc/emscripten-status.rst<br />

### JerryScript
J-1 https://github.com/jerryscript-project/jerryscript<br />
J-2 https://ieeexplore.ieee.org/abstract/document/7724687<br />
J-3 https://github.com/jerryscript-project/iotjs<br />

### Deno
D-1 https://de.wikipedia.org/wiki/Deno_(Software)<br />
D-2 https://github.com/denoland/rusty_v8<br />
D-3 https://github.com/denoland/deno<br />
D-4 https://deno.land/manual@v1.15.1/webassembly/using_wasm<br />
D-5 https://deno.land/manual/tools/compiler<br />
D-6 https://stackoverflow.com/questions/59807938/the-confusion-with-jit-compilation-in-v8<br />
D-7 https://github.com/denoland/deno<br />
D-8 https://deno.land/manual/runtime/workers<br />
D-9 https://github.com/denoland/deno/pulse<br />

### Spidermonkey
S-1 https://en.wikipedia.org/wiki/SpiderMonkey<br />
S-2 https://spidermonkey.dev/<br />
S-3 https://en.wikipedia.org/wiki/SpiderMonkey<br />

