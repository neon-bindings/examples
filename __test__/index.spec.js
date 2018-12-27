const helloWorld = require('../hello-world');
const threadCount = require('../thread-count');
const primitives = require('../primitives');
const arrays = require('../arrays');
const objects = require('../objects');
const args = require('../arguments');
const functions = require('../functions');
const fibonacciTask = require('../fibonacci-task');
const sharingBinaryData = require('../sharing-binary-data');
const json = require('../json');
const classes = require('../classes');
const modules = require('../modules');
const errors = require('../errors');
const async = require('../async');
const {
  parse,
  stringify,
  performAsyncTask: performAsyncTaskCB
} = require('../event-emitter');
require('../word-counting');

describe('tests', () => {
  it('should run hello world', () => {
    expect(helloWorld.helloWorld()).toMatchSnapshot();
  });

  it('should run thread count', () => {
    expect(Number.isInteger(threadCount.threadCount())).toEqual(true);
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
    expect(json.sayHello('should run john')).toMatchSnapshot();
  });

  it('should run share binary data', () => {
    expect(new Float32Array(sharingBinaryData.hello())).toMatchSnapshot();
  });

  it('should run classes', () => {
    const { User } = classes;
    const user = new User(0, 'John', 'Doe', 'johndoe@gmail.com');
    expect(user).toMatchSnapshot();
  });

  it('should run fibonacci-task', async () => {
    const fib10 = await new Promise(resolve => {
      fibonacciTask.fibonacci(10, (err, result) => resolve(result));
    });
    expect(fib10).toMatchSnapshot();
    expect(fibonacciTask.fibonacciSync(10)).toMatchSnapshot();
  });

  it('should run modules', () => {
    expect(modules.hello).toMatchSnapshot();
    expect(modules.User).toMatchSnapshot();
    expect(modules.baz).toMatchSnapshot();
  });

  it('should run errors', () => {
    expect(errors.throwError).toMatchSnapshot();
    expect(errors.throwCustomError).toMatchSnapshot();
  });

  it('should run async', async () => {
    const result = await new Promise(resolve => {
      async.performAsyncTask((err, value) => resolve(value));
    });
    expect(result).toMatchSnapshot();
  });

  it('should run event emitter', async () => {
    const performAsyncTask = () =>
      new Promise(resolve => {
        performAsyncTaskCB((err, result) => {
          resolve(result);
        });
      });

    const fixture = {
      a: 1,
      b: {
        c: [
          2,
          3,
          {
            d: '4'
          }
        ]
      }
    };

    expect(parse(JSON.stringify(fixture))).toMatchSnapshot();
    expect(JSON.parse(stringify(fixture))).toMatchSnapshot();
    expect(parse(stringify(fixture))).toMatchSnapshot();
    const result = await performAsyncTask();
    expect(result).toEqual(17);
  });
});
