mod version_range;

use serde::{Deserialize, Serialize};
use crate::util::string_or_list;

pub use version_range::{RangeSpecifier, VersionRange};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PackageMetadata {
    #[serde(rename = "@id")]
    pub uri_id: String,
    #[serde(deserialize_with = "string_or_list")]
    pub authors: Vec<String>,
    #[serde(default="default")]
    pub dependency_groups: Vec<DependencyGroup>,
    pub deprecation: Option<Deprecation>,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    #[serde(rename="id")]
    pub package_id: String,
    pub language: Option<String>,
    pub license_url: Option<String>,
    pub license_expression: Option<String>,
    #[serde(default="true_default")]
    pub listed: bool,
    pub min_client_version: Option<String>,
    pub package_content: Option<String>,
    pub project_url: Option<String>,
    pub published: Option<String>,
    pub readme_url: Option<String>,
    #[serde(default="false_default")]
    pub require_license_acceptance: bool,
    pub summary: Option<String>,
    #[serde(deserialize_with = "string_or_list", default="default")]
    pub tags: Vec<String>,
    pub title: Option<String>,
    pub version: String,
    pub vulnerabilities: Option<Vec<Vulnerability>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct DependencyGroup {
    pub target_framework: Option<String>,
    #[serde(default="default")]
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub id: String,
    pub range: Option<serde_json::Value>,
    pub registration: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Deprecation {
    pub reasons: Vec<String>,
    pub message: Option<String>,
    pub alternate_package: Option<AlternatePackage>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct AlternatePackage {
    pub id: String,
    pub range: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Vulnerability {
    pub advisory_url: String,
    pub severity: String, // string containing a number... TODO: parse
}

fn true_default() -> bool { true }
fn false_default() -> bool { false }
fn default<T>() -> T where T: Default { Default::default() }
