const addon = require('../native/index.node');

// console.log(addon.say_hello());
// fails: TypeError: not enough arguments

console.log(addon.say_hello('john'));
// Hello, john!

// console.log(addon.greet({ name: "afsd" }));
// Error(Msg("missing field `age`"), State { next_error: None, backtrace: None })

console.log(addon.fibonacci(32));
