const helloWorld = require('../hello-world');
const threadCount = require('../thread-count');
const primitives = require('../primitives');
const arrays = require('../arrays');
const objects = require('../objects');
const args = require('../arguments');
const functions = require('../functions');
require('../word-counting');
// const fibonacciTask = require('../fibonacci-task');
const sharingBinaryData = require('../sharing-binary-data');
const json = require('../json');
const classes = require('../classes');

describe('tests', () => {
  it('should run hello world', () => {
    expect(helloWorld.helloWorld()).toMatchSnapshot();
  });

  it('should run thread count', () => {
    expect(threadCount.threadCount()).toMatchSnapshot();
  });

  it('should run primitives', () => {
    expect(primitives.primitives()).toMatchSnapshot();
  });

  it('should run arrays', () => {
    expect(arrays.convertVecToArray()).toMatchSnapshot();
    expect(arrays.returnJsArray()).toMatchSnapshot();
    expect(arrays.returnJsArrayWithNumber()).toMatchSnapshot();
    expect(arrays.returnJsArrayWithString()).toMatchSnapshot();
  });

  it('should run objects', () => {
    expect(objects.convertStructToJsObject()).toMatchSnapshot();
  });

  it('should run arguments', () => {
    expect(args.printFunction(() => {})).toMatchSnapshot();
    expect(args.add1ToArgument(1)).toMatchSnapshot();
    expect(args.getArgsLen(1, 2, 3)).toMatchSnapshot();
    expect(args.argsOpt(1)).toMatchSnapshot();
    expect(args.defaultArgs()).toMatchSnapshot();
  });

  it('should run functions', () => {
    expect(functions.requireObjectThis()).toMatchSnapshot();
    expect(functions.callJsFunction(() => 12)).toMatchSnapshot();
    expect(functions.constructJsFunction(Date)).toMatchSnapshot();
    expect(functions.returnJsFunction()).toMatchSnapshot();
  });

  it('should run json', () => {
    expect(json.say_hello('should run john')).toMatchSnapshot();
  });

  it('should run share binary data', () => {
    expect(new Float32Array(sharingBinaryData.hello())).toMatchSnapshot();
  });

  it('should run classes', () => {
    const { User } = classes;
    const user = new User(0, 'John', 'Doe', 'johndoe@gmail.com');
    expect(user).toMatchSnapshot();
  });
});
