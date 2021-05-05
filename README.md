# neon-examples

![Github Actions](https://github.com/neon-bindings/examples/workflows/Test/badge.svg?branch=main)

## Table of Contents

|  | Example | Descrption |
| --- | --- | --- |
| 1.|  [hello world](./hello-world) | Return a hello world string to Node |
| 2.|  [primitives](./primitives) | Creating JS primitives in Rust |
| 3.|  [arrays](./arrays) | Creating and using JS arrays in Rust |
| 4.|  [objects](./objects) | Creating and using JS objects in Rust |
| 5.|  [arguments](./arguments) | Getting and checking function arguments |
| 6.|  [functions](./functions) | Creating and calling JS functions from Rust |
| 7.|  [classes](./classes) | Creating classes |
| 8.|  [class factory](./class-factory) | Creating classes from a class. Useful for resource pools. |
| 9.|  [modules](./modules) | Exporting functions, classes, and values |
| 10.|  [json](./json) | Handling JSON passed between JS and Rust |
| 11.|  [errors](./errors) | Creating and throwing errors |
| 12.|  [async](./async) | Creating and scheduling async background tasks in Node's thread pool |
| 13.|  [thread count](./thread-count) | Expose the `num_cpus` Rust library to JS | 
| 14.|  [fibonacci async task](./fibonacci-async-task) | Computing the nth fibonacci number in Rust and passing the result to JS |
| 15.|  [word counting](./word-counting) | A word counting demo in Rust and JS with benchmarks |
| 16.|  [sharing binary data](./sharing-binary-data) | Handling binary data passed from Node to Rust |
| 17.|  [electron app](./electron-app) | A simple electron app using Neon modules |
| 18.|  [publishing modules](https://github.com/amilajack/disk-utility) | Using [`node-pre-gyp`](https://github.com/mapbox/node-pre-gyp) to build and publish binaries for multiple platforms |
| 19.|  [event emitter](./event-emitter) | An example of creating an `EventEmitter` with Neon |
| 20.|  [workspace](./workspace) | An example of using Neon in a Cargo workspace |
| 21.|  Bindgen | Planned |

## Setup

```bash
git clone https://github.com/neon-bindings/examples
cd neon-examples
yarn

# Compile and run an example:
cd primitives
node ./lib/index.js
```
