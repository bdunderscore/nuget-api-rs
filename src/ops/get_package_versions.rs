use std::pin::Pin;

use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use crate::Result;

use crate::model::PackageMetadata;

#[derive(Debug, Deserialize, Clone, Serialize)]
struct RootResponse {
    count: usize,
    items: Vec<RegistrationPage>
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct RegistrationPage {
    #[serde(rename = "@id")]
    id: String,
    count: usize,
    items: Option<Vec<RegistrationLeaf>>,
    parent: Option<String>,
    lower: Option<String>,
    upper: Option<String>,
}
impl RegistrationPage {
    async fn content<'a>(self, nuget: &'a crate::NugetRegistry) -> Result<OpaqueRegLeafStream<'a>> {
        if let Some(items) = self.items {
            return Ok(Box::pin(futures::stream::iter(items.into_iter().map(Ok))));
        }

        let url = nuget.client.get(self.id.clone()).send().await?.error_for_status()?.bytes().await?;
        let url: RegistrationPage = serde_json::from_slice(&url)?;

        Ok(Box::pin(futures::stream::iter(url.items.unwrap().into_iter().map(Ok))))
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct RegistrationLeaf {
    #[serde(rename = "@id")]
    pub id: String,
    pub catalog_entry: PackageMetadata,
    pub package_content: String,
}

type OpaqueRegLeafStream<'a> = Pin<Box<dyn Stream<Item=Result<RegistrationLeaf>> + Send + 'a>>;

pub async fn execute<'a>(nuget: &'a crate::NugetRegistry, package_name: &str) -> Result<impl Stream<Item=Result<RegistrationLeaf>> + Send + Unpin + 'a> {
    let base_url = nuget.resource_arr(&[
        "RegistrationsBaseUrl/3.6.0",
        "RegistrationsBaseUrl/3.4.0",
        "RegistrationsBaseUrl/3.0.0-rc",
        "RegistrationsBaseUrl/3.0.0-beta",
        "RegistrationsBaseUrl/3.0.0",
    ])?;

    let mut suffix = format!("{}/index.json", package_name);
    suffix.make_ascii_lowercase();
    let url = base_url.join(&suffix)?;
    dbg!((&base_url, &suffix, &url));

    let root = nuget.client.get(url).send().await?.error_for_status()?.bytes().await?;
    let root: RootResponse = serde_json::from_slice(&root)?;

    dbg!(&root);

    let mut stream = futures::stream::iter(root.items);
    let mut stream = stream.flat_map(move |page| futures::stream::once(async move {
        match page.content(nuget).await {
            Ok(stream) => stream,
            Err(e) => Box::pin(futures::stream::once(async move { Err(e) })) as OpaqueRegLeafStream<'a>
        }
    }).flatten());

    Ok(Box::pin(stream))
}