## Current Project State
- deno_core is able to execute predefined test files
- quickJs receives as input code snippets and reference cached modules to execute them

## To be checked
- deno_core: send via NATS the modules to be used, saved in folder and triggered with subsequent functions.
by doing so, 1-to-1 correspondence to user modules from nats messages (or predefined set of functions)
- quickJs -> use imports as ES modules 

one relative paths and the other direct module name: how deal with the problem?

## deno_core
- function to load all modules into the runtime instance -> not possible since modules are loaded through file path and
  not through module name as in quickJs
- reference modules loaded within code execution

### Solution to deno_core modules
Generate two different type of messages triggered by NATS -> one for Deno and one for QuickJS

## quickJs
- function to load all modules into the runtime instance

