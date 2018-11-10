const {
  hello,
  requireObjectThis,
  callJsFunction,
  constructJsFunction,
  returnJsFunction
} = require('../native/index.node');

console.log(
  hello(),
  requireObjectThis(),
  callJsFunction(),
  constructJsFunction(),
  returnJsFunction()
);
