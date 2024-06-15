
use std::pin::pin;

use futures::{StreamExt, TryStreamExt};
use nuget_client::NugetRegistry;

#[tokio::main]
async fn main() {
    let reg = NugetRegistry::connect("https://api.nuget.org/v3/index.json".parse().unwrap()).await.unwrap();

    let mut versions = reg.get_package_registrations("System.Text.Json").await.unwrap();

    while let Some(v) = versions.try_next().await.unwrap() {
        dbg!(v);
    }
}