use neon::prelude::*;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use tokio::runtime::Runtime;

#[derive(Deserialize)]
struct NodeRelease {
    version: String,
    date: String,
}

// Return a global tokio runtime or create one if it doesn't exist.
// Throws a JavaScript exception if the `Runtime` fails to create.
fn runtime<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<&'static Runtime> {
    static RUNTIME: OnceCell<Runtime> = OnceCell::new();

    RUNTIME.get_or_try_init(|| Runtime::new().or_else(|err| cx.throw_error(err.to_string())))
}

// Get the verson of the currently running node process from [`process.version`](https://nodejs.org/api/process.html#processversion)
fn node_version<'a, C: Context<'a>>(cx: &mut C) -> NeonResult<String> {
    let global = cx.global();
    let process = global.get::<JsObject, _, _>(cx, "process")?;
    let version = process.get::<JsString, _, _>(cx, "version")?.value(cx);

    Ok(version)
}

// Asynchronously fetch the list of Node releases. This will execute on the `tokio`
// thread pool.
async fn fetch_node_releases() -> Result<Vec<NodeRelease>, reqwest::Error> {
    reqwest::get("https://nodejs.org/dist/index.json")
        .await?
        .json()
        .await
}

// Asynchronously find a Node release from a version string
async fn fetch_node_release(version: &str) -> Result<Option<NodeRelease>, reqwest::Error> {
    let version = fetch_node_releases()
        .await?
        .into_iter()
        .find(|release| release.version == version);

    Ok(version)
}

// Get the release date of the currently running Node process.
// Returns a `Promise<string>` and executes asynchronously on the `tokio`
// thread pool.
fn node_release_date(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let rt = runtime(&mut cx)?;
    let version = node_version(&mut cx)?;
    let channel = cx.channel();

    // Create a JavaScript promise and a `deferred` handle for resolving it.
    // It is important to be careful not to perform failable actions after
    // creating the promise to avoid an unhandled rejection.
    let (deferred, promise) = cx.promise();

    // Spawn an `async` task on the tokio runtime. Only Rust types that are
    // `Send` may be moved into this block. `Context` may not be passed and all
    // JavaScript values must first be converted to Rust types.
    //
    // This task will _not_ block the JavaScript main thread.
    rt.spawn(async move {
        // Inside this block, it is possible to `await` Rust `Future`
        let release = fetch_node_release(&version).await;

        // Settle the promise from the result of a closure. JavaScript exceptions
        // will be converted to a Promise rejection.
        //
        // This closure will execute on the JavaScript main thread. It should be
        // limited to converting Rust types to JavaScript values. Expensive operations
        // should be performed outside of it.
        deferred.settle_with(&channel, move |mut cx| {
            // Convert a `reqwest::Error` to a JavaScript exception
            let release = release.or_else(|err| cx.throw_error(err.to_string()))?;

            match release {
                // Resolve the promise with the release date
                Some(release) => Ok(cx.string(release.date)),

                // Reject the `Promise` if the version could not be found
                None => cx.throw_error(format!("Could not find version: {}", version)),
            }
        });
    });

    // Return the promise back to JavaScript
    Ok(promise)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("nodeReleaseDate", node_release_date)?;

    Ok(())
}
