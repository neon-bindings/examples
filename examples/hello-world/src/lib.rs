#[neon::export]
fn hello() -> &'static str {
    "hello node"
}
