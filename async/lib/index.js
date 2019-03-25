const { promisify } = require('util');
const addon = require('../native/index.node');

console.log(
  addon.performAsyncTask((err, value) => {
    if (err) throw err;
    console.log(value);
  })
);

// Neon does not provide `Promise` return values from asynchronous
// tasks, but it does use node style callbacks that may be trivially
// promisified.
// Note: Neon uses a reference to `this` to unwrap the Rust struct when working with "classes".
// `util.promisify` preserves access to `this`. Alternatively, `bind` can be used.
addon.performAsyncTaskP = promisify(addon.performAsyncTask);

console.log(
  addon
    .performAsyncTaskP()
    .then(console.log)
    .catch(console.error)
);

module.exports = addon;
