pub mod ops;
pub mod err;
pub mod model;
pub(crate) mod util;

use std::collections::HashMap;
use reqwest::Url;
use ops::{get_package_versions::RegistrationLeaf, service_index::ServiceIndex};

pub use err::{Result, Error};

pub struct NugetRegistry {
    client: reqwest::Client,
    registry_base_url: Url,
    pub index: HashMap<String, Url>,
}

impl NugetRegistry {
    pub async fn connect(index_url: Url) -> reqwest::Result<Self> {
        let client = reqwest::Client::builder()
            .gzip(true)
            .build()?;
        let index = client.get(index_url.clone()).send().await?.bytes().await?;
        let index : ServiceIndex = serde_json::from_slice(&index).unwrap();

        let mut index2 = HashMap::with_capacity(index.resources.len());
        for resource in index.resources {
            let url = Url::parse(&resource.id).unwrap();
            index2.insert(resource.type_, url);
        }

        Ok(Self { client, registry_base_url: index_url, index: index2 })
    }

    pub async fn get_package_version(&self, package_name: &str) -> Result<Option<ops::package_base_address::PackageBaseAddressResponse>> {
        ops::package_base_address::execute(self, package_name).await
    }

    pub async fn get_package_registrations<'a>(&'a self, package_name: &str) -> Result<impl futures::Stream<Item=Result<RegistrationLeaf>> + Send + Unpin + 'a> {
        ops::get_package_versions::execute(self, package_name).await
    }

    pub(crate) fn try_resource(&self, name: &str) -> Option<&Url> {
        self.index.get(name)
    }

    pub(crate) fn resource(&self, name: &str) -> Result<&Url> {
        self.try_resource(name).ok_or_else(|| Error::UnsupportedOperation(name.to_string()))
    }

    pub(crate) fn resource_arr(&self, names: &[&str]) -> Result<&Url> {
        for name in names {
            if let Some(url) = self.try_resource(name) {
                return Ok(url);
            }
        }
        Err(Error::UnsupportedOperation(names.join(", ")))
    }
}

