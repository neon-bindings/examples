const { EventEmitter } = require('events');
const { promisify } = require('util');
const { EventEmitter: RustChannel } = require('../native/index.node');

// The `MyEventEmitter` class provides glue code to abstract the `poll`
// interface provided by the Neon class. It may be constructed and used
// as a normal `EventEmitter`, including use by multiple subscribers.
class MyEventEmitter extends EventEmitter {
  constructor() {
    super();

    // Create an instance of the Neon class
    const channel = new RustChannel();

    // Neon does not provide `Promise` return values from asynchronous
    // tasks, but it does use node style callbacks that may be trivially
    // promisified.
    // Neon uses a reference to `this` to unwrap the Rust struct. The `poll`
    // method is bound to `channel` to ensure access.
    const poll = promisify(channel.poll.bind(channel));

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
        return channel.shutdown();
      }

      // Poll for data
      return (
        poll()
          .then(e => {
            // Timeout on poll, no data to emit
            if (!e) {
              return undefined;
            }

            const { event, ...data } = e;

            // Emit the event
            this.emit(event, data);

            return undefined;
          })

          // Emit errors
          .catch(err => this.emit('error', err))

          // Schedule the next iteration of the loop. This is performed with
          // a `setImmediate` to extending the promise chain indefinitely
          // and causing a memory leak.
          .then(() => setImmediate(loop))
      );
    };

    // Start the polling loop
    loop();
  }

  // Mark the channel for shutdown
  shutdown() {
    this.isShutdown = true;
    return this;
  }
}

module.exports = MyEventEmitter;
