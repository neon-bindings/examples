use neon::register_module;
use neon_serde::export;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u16,
}

export! {
    fn sayHello(name: String) -> String {
        format!("Hello, {}!", name)
    }

    fn greet(user: User) -> String {
        format!("{} is {} years old", user.name, user.age)
    }
}
