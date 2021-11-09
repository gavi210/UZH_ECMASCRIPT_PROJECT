# 2021/11/09

## Meta

- Time, date: 2021-11-09, 0900
- Location: Teams
- Present: MH, SM, RR

## Meeting Overview

### Discussion about performance comparison 
- Description of the comparison set-up
- Discussion about the outcomes: deno_core results 3 orders of magnitude faster than quickJs (expectations have been met)

### Discussion about next project steps 
deno_core has been proved to be faster than quickJs, despite a bigger runtime size. Therefore, it could be deduced that 
quickJs suits better than deno_core only when few resources are available (i.e. in a micro-controller). In the other contexts
deno_core will outperform quickJs.
Now that an answer to the initial research questions have been obtained, a discussion about the project development has been done. 
It has been agreed to focus on the deno_core runtime and to implement a set of interesting features for the lightweight management
system embedding it.
This features are:
- function execution through NATS
- add multi-threaded function execution. 

### Actions for next week
- trigger function execution through NATS (MH, RR)
