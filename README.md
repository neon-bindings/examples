# neon-examples

![Github Actions](https://github.com/neon-bindings/examples/workflows/Test/badge.svg?branch=main)

Examples and common patterns for [Neon][neon].

All examples are for [`napi-backend`][napi-migration]. For examples using `legacy-backend` see the [`legacy`][legacy] branch. 

[neon]: https://github.com/neon-bindings/neon
[napi-migration]: https://github.com/neon-bindings/neon/blob/main/MIGRATION_GUIDE.md#n-api-migration-guide
[legacy]: https://github.com/neon-bindings/examples/tree/legacy

## Table of Contents

| Example                        | Description                                |
| ------------------------------ | ------------------------------------------ |
| [`async-sqlite`][async-sqlite] | Async interface to a SQLite database       |
| [`cpu-count`][cpu-count]       | Return the number of CPUs                  |
| [`gzip-stream`][gzip-stream]   | Asynchronously compress a stream of data   |
| [`hello-world`][hello-world]   | Return a JS String with a greeting message |

[async-sqlite]: examples/async-sqlite
[cpu-count]: examples/cpu-count
[gzip-stream]: examples/gzip-stream
[hello-world]: examples/hello-world

## Contributing

### Setup

The `examples` repository uses the npm 7 [workspaces] feature.

[workspaces]: https://docs.npmjs.com/cli/v7/using-npm/workspaces

```
# npm 7 is required
npm --version

git clone https://github.com/neon-bindings/examples.git
cd examples
npm install
```

### Tests

```
npm test
```
