const { EventEmitter } = require('events');
const { EventEmitter: RustChannel } = require('../native/index.node');

// The `MyEventEmitter` class provides glue code to abstract the `poll`
// interface provided by the Neon class. It may be constructed and used
// as a normal `EventEmitter`, including use by multiple subscribers.
class MyEventEmitter extends EventEmitter {
  constructor() {
    super();

    // Create an instance of the Neon class
    const channel = new RustChannel();

    // Marks the emitter as shutdown to stop iteration of the `poll` loop
    this.isShutdown = false;

    // The `loop` method is called continuously to receive data from the Rust
    // work thread.
    const loop = () => {
      // Stop the receiving loop and shutdown the work thead. However, since
      // the `poll` method uses a blocking `recv`, this code will not execute
      // until either the next event is sent on the channel or a receive
      // timeout has occurred.
      if (this.isShutdown) {
        channel.shutdown();
        return;
      }

      // Poll for data
      channel.poll((err, e) => {
        if (err) this.emit('error', err);
        else if (e) {
          const { event, ...data } = e;

          // Emit the event
          this.emit(event, data);
        }
        // Otherwise, timeout on poll, no data to emit

        // Schedule the next iteration of the loop. This is performed with
        // a `setImmediate` to yield to the event loop, to let JS code run
        // and avoid a stack overflow.
        setImmediate(loop);
      });
    };

    // Start the polling loop on next iteration of the JS event loop to prevent zalgo.
    setImmediate(loop);
  }

  // Mark the channel for shutdown
  shutdown() {
    this.isShutdown = true;
    return this;
  }
}

module.exports = MyEventEmitter;
