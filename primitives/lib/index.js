const primitives = require('../native/index.node');

const {
  generateBoolean,
  generateNull,
  generateUndefined,
  generateNumber,
  generateBigInt,
  generateString,
  generateSymbol
} = primitives;

const mySimpleTester = (generator, expectedType) => {
  // Test: Is the function type?
  if (typeof generator !== typeof mySimpleTester) {
    console.log(
      `[ -- ] Expected type = "${expectedType}", But unfortunately the gave generator is not functional; This type is not supported in NEON yet, help wanted!`
    );
    return;
  }

  // Test: Is the generated value the expected type?
  const generatedValue = generator();
  const generatedType = typeof generatedValue;
  const result = generatedType === expectedType ? 'OK' : 'NG';
  console.log(
    `[ ${result} ] Expected type = "${expectedType}", Actual generated value = ${generatedValue}`
  );
};

mySimpleTester(generateBoolean, typeof true); // Boolean
mySimpleTester(generateNull, typeof null); // Null
mySimpleTester(generateUndefined, typeof _); // Undefined; The variable `_` is not defined in this context then it is the same to `undefined` typed value.
mySimpleTester(generateNumber, typeof 42); // Number
mySimpleTester(generateBigInt, typeof 42n); // BigInt
mySimpleTester(generateString, typeof 'Hello, NEON!'); // String
mySimpleTester(generateSymbol, typeof Symbol('My unique key')); // Symbol

exports = primitives;
