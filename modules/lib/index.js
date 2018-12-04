const addon = require('../native/index.node');

console.log(
  'the following are exported from this module:',
  addon.hello,
  addon.User,
  addon.baz
);

module.exports = addon;
