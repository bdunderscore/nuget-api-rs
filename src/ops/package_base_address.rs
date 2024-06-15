use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PackageBaseAddressResponse {
    pub versions: Vec<String>,
}

pub async fn execute(nuget: &crate::NugetRegistry, package_name: &str) -> crate::Result<Option<PackageBaseAddressResponse>> {
    let url = nuget.resource("PackageBaseAddress/3.0.0")?;
    let mut extension = format!("{}/index.json", package_name);
    extension.make_ascii_lowercase();
    
    let url = url.join(&extension)?;

    let request = nuget.client.get(url).send().await?;
    if request.status() == 404 {
        return Ok(None);
    }
    request.error_for_status_ref()?;

    let response = request.bytes().await?;
    let response : PackageBaseAddressResponse = serde_json::from_slice(&response)?;
    
    Ok(Some(response))
}