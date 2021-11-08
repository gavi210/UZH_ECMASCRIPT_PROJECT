# Tests Description

## Folder organization
- **src/** contains the source code to run the light-weight management system
- **support_modules/** stores all the **.js** and **.mjs** files to run the tests

### support_modules/
The folder is divided into:
- **test_scripts**: contains the JavaScript file to run the test
- **side_modules**: contains the .mjs files defining the ES support Modules executed by the tests

## Run the tests
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

## To be done
- Define more complex functions, describe the tests executed and define possible outcome expectations
- Define a better folder organization for the test files
- Embed QuickJs runtime to have performance comparisons


### To be done - Code Structure
- analysis of performances -> should be printed out into an output file showing comparison of results: refactor 
deno_wrapper **run_tests** function to print elements on to file