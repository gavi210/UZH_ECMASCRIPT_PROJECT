# Tests Description

## Folder organization
- **src/** contains the source code to run the light-weight management system
- **support_modules/** stores all the **.js** and **.mjs** files to run the tests. The folder divides itself into **deno** and 
**quickJs** scripts.

## build.rs file
This piece of code is executed when compiling the project. It will copy the support_modules to the execution directory 
of the management system, so that relative import paths within test files could be used.

## How execute js code
### deno_core
To execute a js piece of code in deno, two alternatives are possible. 
If the code is not a module (does not contain any import), it could be directly executed through the **execute_script()** function.
If the code is a module, the **load_side_module()/load_main_module()** functions has to be used, so to import and compile
all its dependencies.

### quickJs
To execute js in quickJs, the **.compile()** function is used.

## JS module imports
In deno, module file paths have to be used. In quickJs, module names are directly used to refer to files.

## Embed with NATS
Future steps of the project is to save all side modules needed to run the different tests. Once this is done, js code could 
be received from NATS, parsed and performances return back.

Two different messages has to be sent, one containing the deno code and the other the quickJs code.


# out of date (for the moment)
### Run
To run the tests, run the following command:
```
./target/debug/jsruntime_test <number_of_files> <list of .js test files>
```
```
./target/debug/jsruntime_test 2 support_modules/test_scripts/test_01.js support_modules/test_scripts/test_02.js
```