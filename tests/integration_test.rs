#[tokio::test]
async fn test_basic() {
    let index_url = reqwest::Url::parse("https://api.nuget.org/v3/index.json").unwrap();
    let registry = nuget_client::NugetRegistry::connect(index_url).await.unwrap();
    
    let content = registry.get_package_version("System.Text.Json").await.unwrap();
    dbg!(content);
}