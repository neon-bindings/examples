use neon::prelude::*;
use neon::register_module;

use hello::say_hello;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(say_hello()))
}

register_module!(mut cx, { cx.export_function("hello", hello) });

#[cfg(test)]
mod tests {
    use crate::say_hello;

    #[test]
    fn hello() {
        assert_eq!(say_hello(), "Hello, World!");
    }
}
