const helloWorld = require('../hello-world');
const threadCount = require('../thread-count');
const primitives = require('../primitives');
const arrays = require('../arrays');
const objects = require('../objects');
const args = require('../arguments');
const functions = require('../functions');
const fibonacciTask = require('../fibonacci-async-task');
const sharingBinaryData = require('../sharing-binary-data');
const json = require('../json');
const classes = require('../classes');
const modules = require('../modules');
const errors = require('../errors');
const async = require('../async');
const Emitter = require('../event-emitter/lib/emitter');
require('../word-counting');

describe('tests', () => {
  it('should run hello world', () => {
    expect(helloWorld.helloWorld()).toEqual('hello world!');
  });

  it('should run thread count', () => {
    expect(Number.isInteger(threadCount.threadCount())).toEqual(true);
  });

  it('should run primitives', () => {
    primitives.primitives();
  });

  it('should run arrays', () => {
    expect(arrays.convertVecToArray()).toEqual([]);
    expect(arrays.returnJsArray()).toEqual([]);
    expect(arrays.returnJsArrayWithNumber()).toEqual([9000]);
    expect(arrays.returnJsArrayWithString()).toEqual(['hello node']);
  });

  it('should run objects', () => {
    expect(objects.convertStructToJsObject()).toEqual({
      myNumberProperty: 1234,
      myStringProperty: 'baz'
    });
  });

  it('should run arguments', () => {
    args.printFunction(() => {});
    expect(args.add1ToArgument(1)).toEqual(2);
    expect(args.getArgsLen(1, 2, 3)).toEqual(3);
    args.argsOpt(1);
    args.defaultArgs();
  });

  it('should run functions', () => {
    expect(functions.requireObjectThis()).toEqual(undefined);
    expect(functions.callJsFunction(() => 12)).toEqual(12);
    expect(functions.constructJsFunction(Date)).toEqual(1970);
    expect(functions.returnJsFunction()(1)).toEqual(2);
  });

  it('should run json', () => {
    expect(json.sayHello('should run john')).toEqual('Hello, should run john!');
  });

  it('should run share binary data', () => {
    expect(new Float32Array(sharingBinaryData.hello())).toEqual(
      new Float32Array([0, 0])
    );
  });

  it('should run classes', () => {
    const { User } = classes;
    new User(0, 'John', 'Doe', 'johndoe@gmail.com');
  });

  it('should run fibonacci-task', async () => {
    const fib10 = await new Promise((resolve, reject) => {
      fibonacciTask.fibonacci(10, (err, result) => {
        if (err) reject(err);
        resolve(result);
      });
    });
    expect(fib10).toEqual('55');
    expect(fibonacciTask.fibonacciSync(10)).toEqual('55');
  });

  it('should run modules', () => {
    expect(modules.hello()).toEqual('hello node');
    new modules.User(0, 'Jane', 'Doe', 'janedoe@gmail.com');
    expect(modules.baz).toEqual('baz');
  });

  it('should run errors', () => {
    expect(() => errors.throwError()).toThrow('');
    expect(() => errors.throwCustomError('type_error')).toThrow(
      'throwing a TypeError'
    );
  });

  it('should run async', async () => {
    const result = await new Promise(resolve => {
      async.performAsyncTask((err, value) => resolve(value));
    });
    expect(result).toEqual(17);
  });

  it('should run event emitter', () => {
    const e = new Emitter();
    const spy = jest.fn();
    e.on('tick', spy);
    expect.assertions(2);
    setTimeout(() => {
      e.shutdown();
    }, 1250);
    return new Promise(resolve => {
      setTimeout(() => {
        expect(spy).toHaveBeenCalledTimes(2);
        expect(spy).toHaveBeenLastCalledWith({ count: 1 });
        resolve();
      }, 1750);
    });
  });
});
