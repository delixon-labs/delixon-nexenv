use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub id: String,
    pub name: String,
    pub description: String,
    pub runtimes: Vec<String>,
    pub tags: Vec<String>,
    pub is_official: bool,
    pub author: Option<String>,
    pub version: String,
}
