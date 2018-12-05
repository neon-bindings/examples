const addon = require('../native/index.node');

const {
  requireObjectThis,
  callJsFunction,
  constructJsFunction,
  returnJsFunction
} = addon;

console.log(
  requireObjectThis(),
  callJsFunction(() => 12),
  constructJsFunction(Date),
  returnJsFunction()
);

module.exports = addon;
