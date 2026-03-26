use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeConfig {
    pub runtime: String,
    pub version: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    #[default]
    Active,
    Idle,
    Archived,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub runtimes: Vec<RuntimeConfig>,
    pub status: ProjectStatus,
    pub created_at: String,
    pub last_opened_at: Option<String>,
    pub template_id: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectInput {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub template_id: Option<String>,
    pub runtimes: Vec<RuntimeConfig>,
    pub tags: Option<Vec<String>>,
}
