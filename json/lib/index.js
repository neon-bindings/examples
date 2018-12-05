const addon = require('../native/index.node');

// console.log(addon.sayHello());
// fails: TypeError: not enough arguments

console.log(addon.sayHello('john'));
// Hello, john!

// console.log(addon.greet({ name: "afsd" }));
// Error(Msg("missing field `age`"), State { next_error: None, backtrace: None })

module.exports = addon;
