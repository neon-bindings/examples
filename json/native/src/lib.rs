#[macro_use]
extern crate neon;
#[macro_use]
extern crate neon_serde;
#[macro_use]
extern crate serde_derive;

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
}
