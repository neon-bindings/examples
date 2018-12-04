const addon = require('../native/index.node');

console.log(addon.throwError, addon.throwCustomError);

module.exports = addon;
