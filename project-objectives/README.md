# Project objectives

The initial concept for this project was to make a comparison between lightweight
JS runtimes and more sophisticated, heavyweight runtimes; the basic idea being that
it might be possible to mix these in the context of running functions with a very
long tail distribution (ie some functions could benefit from being run in a very
lightweight runtime and some functions would benefit from using a more sophisticated
runtime).

In this vein, a qualitative comparison of different runtimes was performed. Two
contrasting runtimes - deno and quickjs - were selected for more quantitative
comparison (as there were two folks working on the project). Having set up a
means to trigger basic functions in both of these runtimes using a Rust wrapper
in both cases, it became very evident that there were multiple orders of
magnitude performance differences between these different runtimes, with the
more powerful v8-based solution delivering much higher performance. This,
combined with published results for quickjs performance [link](https://bellard.org/quickjs/bench.html) meant that it
was unlikely that a mix of these runtimes would make sense in a server side
context.

Hence, it was necessary to perform some modest rescoping of the project goals
as a key aspect of the original plan was now moot. Such rescoping must, of course,
be agreed with all project pariticipants and should not introduce significant
extra load/work; further, it should relate to the work that has been done before
both in terms of the larger context as well as the concrete software that has
been developed.

Proposal:

- Overall objective:
  - Provide recommendations for how to manage deno runtimes distributed over
    multiple nodes which are triggered via NATS messages
    - This includes analysis of
      - Use of MainWorkers vs WebWorkers in terms of both security and performance
        - ie are there any security implications of using webworkers? can they
          create global objects which impact other webworkers?
        - what is the performance impact of creating a MainWorker for each function
          execution compared with running functions in web workers?
      - Management of multiple v8 engines within Rust/tokio
        - Given that each runtime runs on a single thread, how can we control
          the number of runtimes active at one time?
          - (this could be done via a config file, for example)
          - How can we schedule work such that it is picked up by any available
            runtime when received from NATS?
      - Analysis of function scheduling when functions perform io operations
        - If a function performs a call out to external resources, it must wait
          for a response
          - is it possible to identify this situation and schedule another
            webworker while the first worker is waiting
      - Dynamic addition of webworkers to running engine
        - Is it possible to dynamically add a new webworker to a running instance?
      - Load testing on a single node
        - Can we estimate what load can be handled by a single node for a given
          workload pattern?
          - Imagine we have a workload which comprises 5 functions which perform
            basic calculations and 5 which perform callouts (retrieve a local web
            page)
            - How many of these can we trigger via NATS until we start getting
              message handling problems on NATS
      - Load testing on multiple nodes
        - For the same workload distribution, does the capacity of the system
          increase linearly with the number of nodes as we run it in
          configurations of 3,5 nodes?
