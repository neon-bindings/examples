use neon::prelude::*;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

#[macro_export]
macro_rules! handle_error {
    ($cx:expr, $result:expr, $error_message:expr) => {
        match $result {
            Err(err) => {
                println!("Throwing error, error was: {:?}", err);
                return $cx.throw_error($error_message);
            }
            Ok(value) => value,
        }
    };
}

pub fn get_html_async(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    // These types (`f64`, `Root<JsFunction>`, `EventQueue`) may all be sent
    // across threads.
    let url = cx.argument::<JsString>(0)?.value(&mut cx);
    // The callback must be into_inner'd or dropped eventually, so don't
    // put fallible code that returns early after it has been bound.
    let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
    let queue = cx.queue();
    // Spawn a thread to complete the execution. This will _not_ block the
    // JavaScript event loop.
    RUNTIME.spawn(async move {
        let response = reqwest::get(url).await;
        let body = match response {
            Err(err) => Err(err),
            Ok(successful_response) => successful_response.text().await,
        };
        // .and_then(async |response| Ok(response.text().await));
        // Send a closure as a task to be executed by the JavaScript event
        // queue. This _will_ block the event queue while executing.
        queue.send(move |mut cx| {
            let callback = callback.into_inner(&mut cx);
            let this = cx.undefined();
            let args = match body {
                Err(reqwest_error) => {
                    let js_error_object = JsObject::new(&mut cx);
                    let js_reqwest_error = cx.string(format!("{:?}", reqwest_error));
                    js_error_object.set(&mut cx, "reqwestError", js_reqwest_error)?;
                    vec![
                        js_error_object.upcast::<JsValue>(),
                        cx.null().upcast::<JsValue>(),
                    ]
                }
                Ok(successful_body) => {
                    vec![
                        cx.null().upcast::<JsValue>(),
                        cx.string(successful_body).upcast(),
                    ]
                }
            };
            callback.call(&mut cx, this, args)?;
            Ok(())
        });
    });

    Ok(cx.undefined())
}

register_module!(mut m, {
    m.export_function("getHtmlAsync", get_html_async)?;
    Ok(())
});
