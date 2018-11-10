#[macro_use]
extern crate neon;

use neon::prelude::*;

// Accessing `this` of the created function. The JS equivalent would be
// creating a function called require_object_this and setting `this.modified = true`
fn require_object_this(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let this = cx.this();
    // Downcast `this` so .set can be called on it
    let this = this.downcast::<JsObject>().or_throw(&mut cx)?;
    let t = cx.boolean(true);
    // Equivalent to  `this.modified = true` in JS
    this.set(&mut cx, "modified", t)?;
    Ok(cx.undefined())
}

// Calling JS Functions
fn call_js_function(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let f = cx.argument::<JsFunction>(0)?;
    let args: Vec<Handle<JsNumber>> = vec![cx.number(16.0)];
    let null = cx.null();
    f.call(&mut cx, null, args)?.downcast::<JsNumber>().or_throw(&mut cx)
}

// Constructing JS Functions
fn construct_js_function(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let f = cx.argument::<JsFunction>(0)?;
    let zero = cx.number(0.0);
    let o = f.construct(&mut cx, vec![zero])?;
    let get_utc_full_year_method = o.get(&mut cx, "getUTCFullYear")?.downcast::<JsFunction>().or_throw(&mut cx)?;
    let args: Vec<Handle<JsValue>> = vec![];
    get_utc_full_year_method.call(&mut cx, o.upcast::<JsValue>(), args)?.downcast::<JsNumber>().or_throw(&mut cx)
}

// Returning Functions
// Create a function to be returned
fn add1(mut cx: FunctionContext) -> JsResult<JsNumber> {
    // Attempt to cast the first argument to a JsNumber. Then
    // get the value if cast is successul
    let x = cx.argument::<JsNumber>(0)?.value();
    Ok(cx.number(x + 1.0))
}

// Then reutrn the function
fn return_js_function(mut cx: FunctionContext) -> JsResult<JsFunction> {
    JsFunction::new(&mut cx, add1)
}

register_module!(mut cx, {
    cx.export_function("requireObjectThis", require_object_this)?;
    cx.export_function("callJsFunction", call_js_function)?;
    cx.export_function("constructJsFunction", construct_js_function)?;
    cx.export_function("returnJsFunction", return_js_function)?;
    Ok(())
});
