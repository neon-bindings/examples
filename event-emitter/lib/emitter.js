const { EventEmitter } = require('events');
const { promisify } = require('util');
const { EventEmitter: RustChannel } = require('../native/index.node');

class MyEventEmitter extends EventEmitter {
  constructor() {
    super();

    const channel = new RustChannel();
    const poll = promisify(channel.poll.bind(channel));

    this.isShutdown = false;

    const loop = () => {
      if (this.isShutdown) {
        return channel.shutdown();
      }

      return poll()
        .then(({ event, ...data }) => this.emit(event, data))
        .catch(err => this.emit('error', err))
        .then(() => setImmediate(loop));
    };

    loop();
  }

  shutdown() {
    this.isShutdown = true;
    return this;
  }
}

function run() {
  const emitter = new MyEventEmitter();
  emitter.on('tick', ({ count }) => console.log(count));
  return new Promise(resolve => {
    setTimeout(() => resolve(emitter.shutdown()), 5000);
  });
}

if (process.env.NODE_ENV !== 'test') {
  run();
}

module.exports = MyEventEmitter;
