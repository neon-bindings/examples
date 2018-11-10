const addon = require('../native/index.node');

addon.fibonacci(100000, (err, result) => {
    console.log('async result:');
    console.log(result);
});

console.log('computing fibonacci(1000000) in background thread...');
console.log('main thread is still responsive!');

module.exports = require('../native/index.node');
