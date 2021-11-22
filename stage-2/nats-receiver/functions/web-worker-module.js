// Listen for messages from the main thread
self.addEventListener("message", function(event) {

    for (var i = 0; i < event.data; i++) {
        ;
    }

    self.postMessage(null);
});