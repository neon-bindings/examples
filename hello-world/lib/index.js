const addon = require('../native/index.node');

console.log(addon.threadingHint());

module.exports = addon;
