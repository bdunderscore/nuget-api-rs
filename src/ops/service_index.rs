use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceIndex {
    pub version: String,
    pub resources: Vec<ServiceResource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceResource {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_: String,
    pub comment: Option<String>,
}