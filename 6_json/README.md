# json

This is an example of using the `neon-serde` crate

## Setup

```bash
git clone https://github.com/amilajack/neon-serde-example
cd neon-serde-example
neon build
```

## Native

```rust
// ./native/src/lib.rs
#[macro_use]
extern crate neon;
#[macro_use]
extern crate neon_serde;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u16,
}

export! {
    fn say_hello(name: String) -> String {
        format!("Hello, {}!", name)
    }

    fn greet(user: User) -> String {
        format!("{} is {} years old", user.name, user.age)
    }

    fn fibonacci(n: i32) -> i32 {
        match n {
            1 | 2 => 1,
            n => fibonacci(n - 1) + fibonacci(n - 2)
        }
    }
}
```

## Node

```js
// ./lib/index.js
const addon = require('../native');

// console.log(addon.say_hello());
// fails: TypeError: not enough arguments

console.log(addon.say_hello("john"));
// Hello, john!

// console.log(addon.greet({ name: "afsd" }));
// Error(Msg("missing field `age`"), State { next_error: None, backtrace: None })

console.log(addon.fibonacci(32));
```
