const MyEventEmitter = require('./emitter');

function run() {
  const emitter = new MyEventEmitter();

  // Log `tick` events. There can be multiple subscribers.
  emitter.on('tick', ({ count }) => console.log(count));

  // Shutdown the emitter after 5s.
  return new Promise(resolve => setTimeout(resolve, 5000)).then(() =>
    emitter.shutdown()
  );
}

if (process.env.NODE_ENV !== 'test') {
  run();
}

module.exports = run;
