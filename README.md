# neon-examples

[![Build Status](https://travis-ci.com/amilajack/neon-examples.svg?branch=master)](https://travis-ci.com/amilajack/neon-examples)

## Table of Contents

1. [hello world](https://github.com/amilajack/neon-examples/tree/master/hello-world)
2. [primitives](https://github.com/amilajack/neon-examples/tree/master/primitives)
3. [arrays](https://github.com/amilajack/neon-examples/tree/master/arrays)
4. [objects](https://github.com/amilajack/neon-examples/tree/master/objects)
5. [arguments](https://github.com/amilajack/neon-examples/tree/master/arguments)
6. [functions](https://github.com/amilajack/neon-examples/tree/master/functions)
7. [word counting](https://github.com/amilajack/neon-examples/tree/master/word-counting)
8. [fibonacci task](https://github.com/amilajack/neon-examples/tree/master/fibonacci-task)
9. [sharing binary data](https://github.com/amilajack/neon-examples/tree/master/sharing-binary-data)
10. [json](https://github.com/amilajack/neon-examples/tree/master/json)
12. [classes](https://github.com/amilajack/neon-examples/tree/master/classes)
13. [electron app](https://github.com/amilajack/neon-examples/tree/master/electron-app)

## Setup

```bash
git clone https://github.com/amilajack/neon-examples
cd neon-examples

# Compiling and running a single example:
cd primitives
npm install # OR `yarn`
node ./lib/index.js

# Compiling and running all the examples:
npm install -g lerna
lerna bootstrap
lerna run install
cd primitives
node ./lib/index.js
```
