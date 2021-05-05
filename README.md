# neon-examples

![Github Actions](https://github.com/neon-bindings/examples/workflows/Test/badge.svg?branch=main)

Examples and common patterns for [Neon][neon].

All examples are for [`napi-backend`][napi-migration]. For examples using `legacy-backend` see the [`legacy`][legacy] branch. 

[neon]: https://github.com/neon-bindings/neon
[napi-migration]: https://github.com/neon-bindings/neon/blob/main/MIGRATION_GUIDE.md#n-api-migration-guide
[legacy]: https://github.com/neon-bindings/examples/tree/legacy

## Table of Contents

| Example                      | Description                                |
| ---------------------------- | ------------------------------------------ |
| [`hello-world`][hello-world] | Return a JS String with a greeting message |

[hello-world]: examples/hello-world

## Contributing

### Setup

[`pnpm`](https://www.npmjs.com/package/pnpm) provides monorepo workspace tooling. It is _not_ a requirement of Neon projects and is only used for building and testing the collection of examples as a single unit.

```
# Install `pnpm` globally
npm install -g pnpm

git clone https://github.com/neon-bindings/examples.git
cd examples
pnpm install
```

### Tests

```
pnpm test -r
```
