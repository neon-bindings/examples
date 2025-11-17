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

#[neon::export(async)]
fn node_release_date(
    cx: &mut Cx,
) -> NeonResult<impl Future<Output = Result<String, Error>> + use<>> {
    let version = node_version(cx)?;

    Ok(async move {
        let release = fetch_node_release(&version)
            .await?
            .ok_or_else(|| format!("Could not find version: {version}"))?;

        Ok(release.date)
    })
}
