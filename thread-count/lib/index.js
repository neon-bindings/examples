const addon = require('../native/index.node');

console.log(addon.threadCount());

module.exports = addon;
