# primitives

Examples of creating primitives.

## Setup

```sh
yarn
```

## Run the app

```
node ./lib
```

### The expected output

```sh
[ OK ] Expected type = "boolean", Actual generated value = true
[ OK ] Expected type = "object", Actual generated value = null
[ OK ] Expected type = "undefined", Actual generated value = undefined
[ OK ] Expected type = "number", Actual generated value = 23
[ -- ] Expected type = "bigint", But unfortunately the gave generator is not functional; This type is not supported in NEON yet, help wanted!
[ OK ] Expected type = "string", Actual generated value = foobar
[ -- ] Expected type = "symbol", But unfortunately the gave generator is not functional; This type is not supported in NEON yet, help wanted!
```

Note: Not yet primitives support in NEON

| Language Feature | NEON    | Node.js              |
|------------------|---------|----------------------|
| `BigInt`         | Not yet [#376][] | 10.4.0; Sep 11, 2018 |
| `Symbol`         | Not yet [#502][] | (From the beginning) |

[#376]: https://github.com/neon-bindings/neon/issues/376
[#502]: https://github.com/neon-bindings/neon/issues/502
