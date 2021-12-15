# Stage-2
This folder contains code to meet the new specified [project objectives](https://github.com/gavi210/UZH_ECMASCRIPT_PROJECT/blob/main/project-objectives/README.md).

## Nats Docker Execution
To run NATS server in Docker container: 
```
docker run -p 4222:4222 -ti nats:latest
```

## Dispatch of Function Execution
To dispatch function execution to the first available worker, a simple shared ``Queue`` object is created. 
A ``Nats Client`` receives messages and place them in the queue, and the ``MainWorker`` instances will process the message
as soon as they are available.


