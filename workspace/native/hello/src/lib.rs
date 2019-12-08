pub fn say_hello() -> String {
    "Hello, World!".to_string()
}

#[cfg(test)]
mod tests {
    use crate::say_hello;

    #[test]
    fn hello() {
        assert_eq!(say_hello(), "Hello, World!");
    }
}
