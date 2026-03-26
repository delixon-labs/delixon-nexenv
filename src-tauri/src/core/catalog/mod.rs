use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Technology {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub versions: Vec<TechVersion>,
    #[serde(default)]
    pub default_version: String,
    #[serde(default)]
    pub default_port: u16,
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub incompatible_with: Vec<String>,
    #[serde(default)]
    pub suggested_with: Vec<String>,
    #[serde(default)]
    pub official_scaffold: Option<String>,
    #[serde(default)]
    pub docker_image: String,
    #[serde(default)]
    pub health_check: Option<TechHealthCheck>,
    #[serde(default)]
    pub env_vars: HashMap<String, String>,
    #[serde(default)]
    pub config_files: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TechVersion {
    pub version: String,
    #[serde(default)]
    pub lts: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TechHealthCheck {
    #[serde(default)]
    pub endpoint: String,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub interval: String,
    #[serde(default)]
    pub timeout: String,
    #[serde(default)]
    pub retries: u32,
}

use std::sync::OnceLock;

static CATALOG: OnceLock<Vec<Technology>> = OnceLock::new();

fn init_catalog() -> Vec<Technology> {
    let yaml_files: &[&str] = &[
        include_str!("technologies/nodejs.yaml"),
        include_str!("technologies/python.yaml"),
        include_str!("technologies/go.yaml"),
        include_str!("technologies/rust.yaml"),
        include_str!("technologies/react.yaml"),
        include_str!("technologies/nextjs.yaml"),
        include_str!("technologies/vue.yaml"),
        include_str!("technologies/nuxt.yaml"),
        include_str!("technologies/svelte.yaml"),
        include_str!("technologies/express.yaml"),
        include_str!("technologies/fastify.yaml"),
        include_str!("technologies/fastapi.yaml"),
        include_str!("technologies/django.yaml"),
        include_str!("technologies/nestjs.yaml"),
        include_str!("technologies/postgresql.yaml"),
        include_str!("technologies/mysql.yaml"),
        include_str!("technologies/mongodb.yaml"),
        include_str!("technologies/redis.yaml"),
        include_str!("technologies/sqlite.yaml"),
        include_str!("technologies/prisma.yaml"),
        include_str!("technologies/drizzle.yaml"),
        include_str!("technologies/sqlalchemy.yaml"),
        include_str!("technologies/nextauth.yaml"),
        include_str!("technologies/clerk.yaml"),
        include_str!("technologies/tailwindcss.yaml"),
        include_str!("technologies/shadcn-ui.yaml"),
        include_str!("technologies/docker.yaml"),
        include_str!("technologies/typescript.yaml"),
        include_str!("technologies/vitest.yaml"),
        include_str!("technologies/github-actions.yaml"),
    ];

    yaml_files
        .iter()
        .filter_map(|s| serde_yml::from_str::<Technology>(s).ok())
        .collect()
}

/// Returns a reference to the cached technology catalog (parsed once, shared across calls)
pub fn load_all_technologies() -> &'static [Technology] {
    CATALOG.get_or_init(init_catalog)
}

pub fn get_technology(id: &str) -> Option<&'static Technology> {
    load_all_technologies().iter().find(|t| t.id == id)
}

pub fn get_by_category(category: &str) -> Vec<&'static Technology> {
    load_all_technologies()
        .iter()
        .filter(|t| t.category == category)
        .collect()
}

pub fn all_categories() -> Vec<String> {
    let mut cats: Vec<String> = load_all_technologies()
        .iter()
        .map(|t| t.category.clone())
        .collect();
    cats.sort();
    cats.dedup();
    cats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_all_technologies() {
        let techs = load_all_technologies();
        assert!(techs.len() >= 30, "Expected at least 30 technologies, got {}", techs.len());
    }

    #[test]
    fn test_get_technology() {
        let nodejs = get_technology("nodejs");
        assert!(nodejs.is_some());
        let nodejs = nodejs.unwrap();
        assert_eq!(nodejs.name, "Node.js");
        assert_eq!(nodejs.category, "runtime");
    }

    #[test]
    fn test_get_by_category() {
        let runtimes = get_by_category("runtime");
        assert!(runtimes.len() >= 4);
    }

    #[test]
    fn test_all_categories() {
        let cats = all_categories();
        assert!(cats.contains(&"runtime".to_string()));
        assert!(cats.contains(&"frontend".to_string()));
        assert!(cats.contains(&"backend".to_string()));
        assert!(cats.contains(&"database".to_string()));
    }
}
