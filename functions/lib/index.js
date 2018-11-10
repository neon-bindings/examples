const {
  requireObjectThis,
  callJsFunction,
  constructJsFunction,
  returnJsFunction
} = require('../native/index.node');

console.log(
  requireObjectThis(),
  callJsFunction(() => 12),
  constructJsFunction(Date),
  returnJsFunction()
);

module.exports = require('../native/index.node');
