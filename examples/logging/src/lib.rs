use neon::prelude::*;

use crate::logger::Logger;

mod logger;

// `init(debug)` must be called before using any other functionality.
//
// An exported initialization function is a common pattern in Neon. Since
// Node-API does not expose `require`, a JavaScript wrapper requires the
// `debug` module and passes it to `init` where logging is initialized.
fn init(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let debug = cx.argument::<JsFunction>(0)?;

    Logger::init(&mut cx, debug)?;

    log::info!("Module initialized");

    Ok(cx.undefined())
}

// Example function with logging
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    log::trace!("Called `hello` function");

    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("init", init)?;
    cx.export_function("hello", hello)?;

    Ok(())
}
