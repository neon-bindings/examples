const {
  convertVecToArray,
  returnJsArray,
  returnJsArrayWithNumber,
  returnJsArrayWithString
} = require('../native/index.node');

console.log(
  convertVecToArray(),
  returnJsArray(),
  returnJsArrayWithNumber(),
  returnJsArrayWithString()
);

module.exports = require('../native/index.node');
