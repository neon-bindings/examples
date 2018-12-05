const addon = require('../native/index.node');

console.log(
  addon.performAsyncTask((err, value) => {
    if (err) throw err;
    console.log(value);
  })
);

module.exports = addon;
