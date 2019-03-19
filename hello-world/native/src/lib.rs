#[macro_use]
extern crate neon;

use neon::prelude::*;

fn hello_world(mut cx: FunctionContext<'_>) -> JsResult<'_, JsString> {
    Ok(cx.string("hello world!"))
}

register_module!(mut m, {
    m.export_function("helloWorld", hello_world)
});
