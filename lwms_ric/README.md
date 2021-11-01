# Tests Description

## Folder organization
- **src/** contains the source code to run the light-weight management system
- **support_modules/** stores all the **.js** and **.mjs** files to run the tests
### support_modules/
The folder is divided into:
- **test_scripts**: contains the JavaScript file to run the test
- **side_modules**: contains the .mjs files defining the ES support Modules 

## Run the test
To build and make the code running on the machine, run:
``
cargo build
``

This will copy the support_modules to the execution directory of the management system, so to not deal 
with absolute paths.
Furthermore, since the module imports in the JavaScript test files are absolute paths, they will be
adjusted, so to run in the machine. (Still to be implemented).


To run the tests, run the following command:
```
./target/debug/jsruntime_test <number_of_files> <list of .js test files>
```
```
./target/debug/jsruntime_test 2 support_modules/test_scripts/test_01.js support_modules/test_scripts/test_02.js
```

## Tasks Achieved
- Project is organized into folders, with relative module imports
- Extended functionalities of the args parser
- Run multiple test and prompt the execution time

## To be done
- Relative import manager at build time (avoid dealing with hard coded absolute paths)
- Define more complex functions
- Define a better folder organization for the test files
- Embed QuickJs runtime to have performance comparisons