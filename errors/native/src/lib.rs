#[macro_use]
extern crate neon;

use neon::prelude::*;

/// Rust panics will throw an Error in Node
fn throw_error(mut cx: FunctionContext) -> JsResult<JsString> {
    // Get the value of the first argument and assert that it is a string
    let arg0 = cx.argument::<JsString>(0)?.value();

    // If the argument does not contain hello, throw an Error
    if !arg0.contains("hello") {
        // Equaivalent to `throw new Error('Expected you to say hello')` in JS
        panic!("Expected you to say hello");
    }

    Ok(cx.string("hello to you too!"))
}

/// Throw type error
fn throw_type_error(mut cx: FunctionContext) -> JsResult<JsValue> {
    let foo: JsResult<JsError> = cx.throw_type_error("not enough arguments");
    Ok(cx.string("throw_error node").upcast())
}

/// Throw custom errors
fn throw_custom_error(mut cx: FunctionContext) -> JsResult<JsError> {
    let arg0 = cx.argument::<JsString>(0)?.value();

    let error = match arg0.as_str() {
        "type_error" => cx.throw_type_error("throwing a TypeError"),
        "range_error" => cx.throw_range_error("throwing a RangeError"),
        "error" => cx.throw_error("throwing an Error"),
        _ => panic!("please pass an expected error type")
    };

    error
}

/// Creating error objects. These will not throw errors. They will
/// only create them
fn create_error_obj(mut cx: FunctionContext) -> JsResult<JsError> {
    let arg0 = cx.argument::<JsString>(0)?.value();

    let error = match arg0.as_str() {
        "type_error" => cx.type_error("creating a TypeError"),
        "range_error" => cx.range_error("creating a RangeError"),
        "error" => cx.error("creating an Error"),
        _ => panic!("please pass an expected error type")
    };

    error
}

register_module!(mut m, {
    m.export_function("throwError", throw_error)?;
    m.export_function("throwCustomError", throw_custom_error)?;
    Ok(())
});
