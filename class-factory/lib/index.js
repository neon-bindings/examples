const { LogFactory } = require('../native/index.node');

const factory = new LogFactory();

function run() {
  const loggerA = factory.create('A');

  loggerA.log('This is A');

  const loggerB = factory.create('B');

  loggerB.log('This is B');
  loggerA.log('This is A');
}

run();

// If running with `--expose-gc` force drop calls
if (typeof global.gc === 'function') {
  global.gc();
}

factory.create('C').log('This is C');
factory.lazy('D').log('This is D');
