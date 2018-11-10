const {
  printFunction,
  checkingArguments,
  add1,
  getArgsLen,
  argsOpt,
  defaultArgs
} = require('../native/index.node');

console.log(
  printFunction(),
  checkingArguments(),
  add1(),
  getArgsLen(),
  argsOpt(),
  defaultArgs()
);
