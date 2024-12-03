use std::{fmt::Debug, future::Future};

use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

use neon::{
    handle::Handle,
    prelude::Context,
    result::{JsResult, ResultExt, NeonResult},
    types::{JsPromise, JsValue, Value},
};
use anyhow;
use serde::{de::DeserializeOwned, Serialize};

// Return a global tokio runtime or create one if it doesn't exist.
// Throws a JavaScript exception if the `Runtime` fails to create.
fn get_runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();

    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}

pub fn try_as_js_promise<'a, T, F, O>(mut cx: impl Context<'a>, lambda: F) -> JsResult<'a, JsPromise>
where
    F: FnOnce() -> O + Send + 'static,
    O: Future<Output = anyhow::Result<T>> + Send,
    T: Serialize + Send + Debug + 'static,
{
    let rt = get_runtime(&mut cx)?;
    let channel = cx.channel();

    let (deferred, promise) = cx.promise();

    rt.spawn(async move {
        let response = lambda().await;

        deferred.settle_with(&channel, move |mut cx| {
            response
                .and_then(|response| {
                    neon_serde2::to_value(&mut cx, &response).map_err(|err| anyhow::anyhow!(err.to_string()))
                })
                .or_else(|err| cx.throw_error(format!("ET000005 Converting to value: {err}")))
        });
    });

    Ok(promise)
}

pub fn try_as_js_callback<'a, C, F, G, S, T>(
    cx: &mut C,
    prepare_params: F,
    prepare_callback: G,
) -> JsResult<'a, JsValue>
where
    C: Context<'a>,
    F: FnOnce() -> S,
    G: FnOnce(&mut C, Handle<'a, JsValue>) -> JsResult<'a, JsValue>,
    S: Future<Output = anyhow::Result<T>>,
    T: Serialize + DeserializeOwned + Send + Clone + 'static,
{
    let rt = get_runtime(cx)?;
    let params = rt
        .block_on(async { prepare_params().await })
        .or_else(|err| cx.throw_error(err.to_string()))?;
    let value = neon_serde2::to_value(cx, &params)
        .or_else(|err| cx.throw_error(err.to_string()))?;

    let result =
        prepare_callback(cx, value).or_else(|err| cx.throw_error(err.to_string()))?;

    Ok(result)
}

pub fn try_as_async_js_callback<'a, C, F, G, H, T, S, U, V, Z>(
    cx: &mut C,
    prepare_params: F,
    prepare_callback: G,
    after_callback: H,
) -> JsResult<'a, JsPromise>
where
    C: Context<'a>,
    F: FnOnce() -> S,
    G: FnOnce(&mut C, U) -> JsResult<'a, JsPromise>,
    H: (FnOnce(T) -> V) + Send + 'static,
    V: Future<Output = anyhow::Result<Z>> + Send + 'static,
    T: Serialize + DeserializeOwned + Send + Clone + 'static,
    S: Future<Output = anyhow::Result<U>>,
    Z: Serialize + Send + Clone + 'static,
{
    let rt = get_runtime(cx)?;

    let channel = cx.channel();
    let (deferred, promise) = cx.promise();

    match rt.block_on(async { prepare_params().await }) {
        Ok(params) => {
            let callback = prepare_callback(cx, params)?;

            let callback_future = callback.to_future(cx, move |mut cx, result| {
                let value = result.or_throw(&mut cx)?.as_value(&mut cx);

                let response: T = neon_serde2::from_value(&mut cx, value).or_else(|err| cx.throw_error(err.to_string()))?;
                Ok(response)
            })?;

            rt.spawn(async move {
                match callback_future.await {
                    Ok(result) => match after_callback(result.clone()).await {
                        Ok(after_result) => {
                            deferred.settle_with(&channel, move |mut cx| {
                                let value = neon_serde2::to_value(&mut cx, &after_result.clone()).or_else(|err| cx.throw_error(err.to_string()))?;
                                Ok(value)
                            });
                        }
                        Err(error) => {
                            deferred.settle_with(&channel, move |mut cx| {
                                let error = cx
                                    .error(error.to_string())
                                    .or_else(|err| cx.throw_error(err.to_string()))?;
                                Ok(error.upcast::<JsValue>())
                            });
                        }
                    },
                    Err(error) => {
                        deferred.settle_with(&channel, move |mut cx| {
                            let error = cx.error(error.to_string()).or_else(|err| cx.throw_error(err.to_string()))?;
                            Ok(error.upcast::<JsValue>())
                        });
                    }
                }
            });
        }
        Err(error) => {
            deferred.settle_with(&channel, move |mut cx| {
                let error = cx
                    .error(error.to_string())
                    .or_else(|err| cx.throw_error(err.to_string()))?;
                Ok(error.upcast::<JsValue>())
            });
        }
    }
    Ok(promise)
}
