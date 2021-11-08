# Tests Description

## Folder organization
- **src/** contains the source code to run the light-weight management system
- **support_modules/** stores all the **.js** and **.mjs** files to run the tests

### support_modules/
The folder is divided into **deno_core** and **quickJs** modules.

## Build phase
Firstly the project needs to be build:

``
cargo build
``

### build.rs file
This piece of code is executed when compiling the project. It will copy the support_modules to the execution directory 
of the management system, so that relative import paths within test files could be used.

### Run
To run the tests, run the following command:
```
./target/debug/jsruntime_test <number_of_files> <list of .js test files>
```
```
./target/debug/jsruntime_test 2 support_modules/test_scripts/test_01.js support_modules/test_scripts/test_02.js
```

## How js snippets are executed
### deno_core
To execute a js piece of code in deno, two alternatives are possible. If the code is not a module (does not contain any 
import), it could be directly executed through the **execute_script()** function.
If the code is a module, the **load_side_module()/load_main_module()** functions has to be used, so to import and compile
all its dependencies.

### quickJs
To execute js in quickJs, the **.compile()** function is used.

## To be done
- Define more complex functions, describe the tests executed and define possible outcome expectations
- Define a better folder organization for the test files


### To be done - Code Structure
- analysis of performances -> should be printed out into an output file showing comparison of results: refactor 
deno_wrapper **run_tests** function to print elements on to file