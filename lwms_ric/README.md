# Tests Description

## Folder organization
- **src/** contains the source code to run the light-weight management system
- **support_modules/** stores all the **.js** and **.mjs** files to run the tests
### Support_modules/
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

## Tasks Achieved
- Project is organized into folders, with relative module imports
- The main.rs function successfully finds the file in the project folder
- 