use neon::{prelude::*, types::extract::Error};
use serde::Deserialize;

#[derive(Deserialize)]
struct NodeRelease {
    version: String,
    date: String,
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

// Get the version of the currently running node process from [`process.version`](https://nodejs.org/api/process.html#processversion)
fn node_version(cx: &mut Cx) -> NeonResult<String> {
    cx.global::<JsObject>("process")?.prop(cx, "version").get()
}

// Export an async JavaScript function where the body is executed on the tokio thread pool
#[neon::export]
async fn node_release_date(version: String) -> Result<String, Error> {
    let release = fetch_node_release(&version)
        .await?
        .ok_or_else(|| format!("Could not find version: {version}"))?;

    Ok(release.date)
}

// Similar to `node_release_date`, but includes some setup code synchronously executed
// on the JavaScript main thread before return a task for tokio. Since this is not
// an `async fn`, we need to explicitly tell the export macro that it returns a future.
#[neon::export(async)]
fn current_node_release_date(
    cx: &mut Cx,
) -> NeonResult<impl Future<Output = Result<String, Error>> + use<>> {
    // Executes synchronously on the JavaScript main thread
    let version = node_version(cx)?;

    // This task is executed asynchronously on the tokio thread pool
    Ok(node_release_date(version))
}
