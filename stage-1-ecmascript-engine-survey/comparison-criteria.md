# Comparison Criteria

- ECMAScript compatibility
  - What versions are supported?
- size of runtime in MB
  - This is something we will explore somehow throughout the project, but it's good to see if there are any hints upfront on this point
- ease of integration with a manager and supported management functionalities
  - languages bindings available
  - support for bidirectional interaction, eg callbacks from javascript world to manager's world and vice versa
- Open Source (or not)
  - project health in terms of github momentum
- WASM support
  - This is a nice to have; we don't specifically want it/need it now, but it will be a future need
- ability to precompile ECMAScript
- intelligence within the runtime
  - how much optimization is going on under the hood? (lots for v8)
- support for isolation
  - Can we run different functions within the same runtime which do not have access to each other's data?
- multithreading support
  - Is there support for running in multithreaded mode
