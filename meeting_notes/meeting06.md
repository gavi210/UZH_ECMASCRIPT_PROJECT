# 2021/11/16

## Meta

- Time, date: 2021-11-16, 0900
- Location: Teams
- Present: MH, SM, RR

## Meeting Overview

### Discussion about change of objectives
- Is it a lot? Yes! -> Work list from top to bottom and see how far we get.
- About security: Not indepth analysis but look at non obvious things like the possibility to write to console.log from two webworkers or the ability to share variables.
- Suggestion to join the public Deno discord server

### Discussion about work done
- NATS and MainWorker vs. WebWorker. Separately both take the same time as expected.
 
### Actions for next week
- Have a MainWorker spawn different WebWorkers and communicate with them (MH, RR)
- Have a look at how the Webworker behave using console.log and the ability to share variables over different WebWorkers  (MH, RR)
