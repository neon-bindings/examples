mod functions;

use functions::{try_as_async_js_callback, try_as_js_callback};
use neon::prelude::*;
use serde::{Serialize, Deserialize};

use crate::functions::try_as_js_promise;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    pub test: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    pub test: String,
}




pub fn example_try_as_js_promise(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let arg = cx.argument::<JsValue>(0)?;
    let request: Request = neon_serde2::from_value(&mut cx, arg).or_else(|err| {
        cx.throw_error(err.to_string())
    })?;

    try_as_js_promise(cx, move || async move {
        // This may be async code
        Ok(Response { test: request.test})
    })
}

pub fn example_try_as_js_callback(mut cx: FunctionContext) -> JsResult<JsValue> {
    let arg = cx.argument::<JsString>(0)?;
    let test = arg.value(&mut cx) ;
    let callback_function = cx.argument::<JsFunction>(1)?;

    try_as_js_callback(
        &mut cx,
        || async { 
            // This may be async code
            Ok(Request { test })
        },
        |cx, request| {
            let this = cx.this_value();
            callback_function.call_with(cx).this(this).arg(request).apply(cx)
        },
    )
}

pub fn example_try_as_async_js_callback(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let arg = cx.argument::<JsString>(0)?;
    let test = arg.value(&mut cx) ;
    let callback_function = cx.argument::<JsFunction>(1)?;


    try_as_async_js_callback(
        &mut cx,
        || async {
             // This may be async code
             Ok(Request { test })
        },
        |cx, request| {
            let this = cx.this_value();
            let request = neon_serde2::to_value(cx, &request)
                .or_else(|err| cx.throw_error(err.to_string()))?;
            callback_function.call_with(cx).this(this).arg(request).apply(cx)
        },
        move |response: Response| async move {
            Ok(response)
        },
    )
}




#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("tryAsJsPromise", example_try_as_js_promise)?;
    cx.export_function("tryAsJsCallback", example_try_as_js_callback)?;
    cx.export_function("tryAsAsyncJsCallback", example_try_as_async_js_callback)?;

    Ok(())
}
