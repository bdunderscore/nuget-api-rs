pub mod ops;
pub mod err;
pub mod model;
pub(crate) mod util;

use std::collections::HashMap;
use reqwest::Url;
use ops::{get_package_versions::RegistrationLeaf, service_index::ServiceIndex};

pub use err::{Result, Error};

/// A client for a nuget registry
pub struct NugetRegistry {
    client: reqwest::Client,
    index: HashMap<String, Url>,
}

impl NugetRegistry {
    /// Connects to a nuget registry.
    /// 
    /// # Arguments
    /// 
    /// * `index_url` - The URL of the registry's index.json file.
    pub async fn connect(index_url: Url) -> Result<Self> {
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

        Ok(Self { client, index: index2 })
    }

    /// Returns all known versions for a package.
    /// 
    /// # Arguments
    /// 
    /// * `package_name` - The name of the package.
    /// 
    /// # Returns
    /// 
    /// A list of all known versions for the package, or None if the package is not found.
    pub async fn get_package_versions(&self, package_name: &str) -> Result<Option<Vec<String>>> {
        ops::package_base_address::execute(self, package_name).await.map(|resp| resp.map(|resp| resp.versions))
    }

    /// Returns metadata for all versions of a package.
    /// 
    /// # Arguments
    /// 
    /// * `package_name` - The name of the package.
    /// 
    /// # Returns
    /// 
    /// A stream of metadata for each version of the package.
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

