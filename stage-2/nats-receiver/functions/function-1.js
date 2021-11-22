// Listen for messages from the main thread
self.addEventListener("message", function(event) {
    console.log("Message from parent:", event.data); // "Sample message"
    for (var i = 0; i < 1000000000; i++) {
    }
    // Send information to the main thread (parent window)
    self.postMessage(["foo", "bar", "baz"]);
});