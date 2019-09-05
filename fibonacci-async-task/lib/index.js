const addon = require('../native/index.node');

if (process.env.NODE_ENV !== 'test') {
  addon.fibonacci(100000, (err, result) => {
    console.log('async result:');
    console.log(result);
  });

  console.log('computing fibonacci(1000000) in background thread...');
  console.log('main thread is still responsive!');
}

module.exports = addon;
