use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::core::catalog;
use crate::core::error::DelixonError;
use crate::core::manifest::{ManifestEnvVars, ManifestMetadata, ManifestService, ProjectManifest, CURRENT_SCHEMA_VERSION};
use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};
use crate::core::rules;
use crate::core::storage;
use crate::core::utils::fs::ensure_dir;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScaffoldConfig {
    pub name: String,
    pub project_type: String,
    pub profile: String,
    pub technologies: Vec<String>,
    pub path: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScaffoldResult {
    pub files_created: Vec<String>,
    pub manifest: ProjectManifest,
    pub validation: rules::ValidationResult,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScaffoldPreview {
    pub files: Vec<PreviewFile>,
    pub validation: rules::ValidationResult,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PreviewFile {
    pub path: String,
    pub content_preview: String,
}

pub fn preview_scaffold(config: &ScaffoldConfig) -> ScaffoldPreview {
    let validation = rules::validate_stack(&config.technologies);
    let files = generate_file_list(config);

    ScaffoldPreview {
        files: files
            .iter()
            .map(|(path, content)| PreviewFile {
                path: path.clone(),
                content_preview: content.chars().take(200).collect(),
            })
            .collect(),
        validation,
    }
}

pub fn generate_project(config: &ScaffoldConfig) -> Result<ScaffoldResult, DelixonError> {
    let validation = rules::validate_stack(&config.technologies);
    let project_path = Path::new(&config.path);

    ensure_dir(project_path)?;

    let files = generate_file_list(config);
    let mut files_created = Vec::new();

    for (rel_path, content) in &files {
        let full_path = project_path.join(rel_path);
        if let Some(parent) = full_path.parent() {
            ensure_dir(parent)?;
        }
        std::fs::write(&full_path, content)?;
        files_created.push(rel_path.clone());
    }

    let manifest = build_manifest(config, &validation);
    let delixon_dir = project_path.join(".delixon");
    ensure_dir(&delixon_dir)?;
    let manifest_yaml = serde_yml::to_string(&manifest)
        .map_err(|e| DelixonError::InvalidConfig(format!("Error serializando manifest: {}", e)))?;
    std::fs::write(delixon_dir.join("manifest.yaml"), manifest_yaml)?;
    files_created.push(".delixon/manifest.yaml".to_string());

    Ok(ScaffoldResult {
        files_created,
        manifest,
        validation,
    })
}

fn generate_file_list(config: &ScaffoldConfig) -> Vec<(String, String)> {
    let mut files = Vec::new();
    let all_techs = catalog::load_all_technologies();

    // .gitignore
    files.push((".gitignore".to_string(), generate_gitignore(config)));

    // .env.example
    files.push((".env.example".to_string(), generate_env_example(config, all_techs)));

    // README.md
    files.push(("README.md".to_string(), generate_readme(config)));

    // docker-compose.yml (if DB or Docker selected)
    let compose = generate_docker_compose(config, all_techs);
    if !compose.is_empty() {
        files.push(("docker-compose.yml".to_string(), compose));
    }

    // Makefile
    files.push(("Makefile".to_string(), generate_makefile(config)));

    // .vscode/settings.json
    files.push((".vscode/settings.json".to_string(), generate_vscode_settings(config)));

    // .vscode/extensions.json
    files.push((".vscode/extensions.json".to_string(), generate_vscode_extensions(config)));

    // GitHub Actions CI
    if config.technologies.contains(&"github-actions".to_string()) || config.profile == "production" {
        files.push((".github/workflows/ci.yml".to_string(), generate_ci_workflow(config)));
    }

    files
}

fn generate_gitignore(config: &ScaffoldConfig) -> String {
    let mut lines = vec![
        "# Dependencies",
        "node_modules/",
        ".pnp.*",
        "",
        "# Build output",
        "dist/",
        "build/",
        ".next/",
        ".nuxt/",
        "target/",
        "",
        "# Environment",
        ".env",
        ".env.local",
        ".env.*.local",
        "",
        "# IDE",
        ".idea/",
        "*.swp",
        "*.swo",
        "",
        "# OS",
        ".DS_Store",
        "Thumbs.db",
        "",
        "# Logs",
        "*.log",
        "npm-debug.log*",
    ];

    if config.technologies.iter().any(|t| t == "python" || t == "fastapi" || t == "django") {
        lines.extend_from_slice(&[
            "",
            "# Python",
            "__pycache__/",
            "*.py[cod]",
            ".venv/",
            "venv/",
            "*.egg-info/",
        ]);
    }

    if config.technologies.contains(&"docker".to_string()) {
        lines.extend_from_slice(&["", "# Docker", ".docker/"]);
    }

    lines.join("\n")
}

fn generate_env_example(config: &ScaffoldConfig, all_techs: &[catalog::Technology]) -> String {
    let mut vars = Vec::new();
    vars.push(format!("# {} environment variables", config.name));
    vars.push(String::new());

    for tech_id in &config.technologies {
        if let Some(tech) = all_techs.iter().find(|t| t.id == *tech_id) {
            if !tech.env_vars.is_empty() {
                vars.push(format!("# {}", tech.name));
                let mut sorted: Vec<_> = tech.env_vars.iter().collect();
                sorted.sort_by_key(|(k, _)| (*k).clone());
                for (key, val) in sorted {
                    vars.push(format!("{}={}", key, val));
                }
                vars.push(String::new());
            }
        }
    }

    if vars.len() <= 2 {
        vars.push("# PORT=3000".to_string());
    }

    vars.join("\n")
}

fn generate_readme(config: &ScaffoldConfig) -> String {
    let mut sections = Vec::new();
    sections.push(format!("# {}\n", config.name));
    sections.push(format!("Tipo: {} | Perfil: {}\n", config.project_type, config.profile));

    sections.push("## Stack\n".to_string());
    sections.push("| Tecnologia | Categoria |".to_string());
    sections.push("|------------|-----------|".to_string());

    let all_techs = catalog::load_all_technologies();
    for tech_id in &config.technologies {
        if let Some(tech) = all_techs.iter().find(|t| t.id == *tech_id) {
            sections.push(format!("| {} | {} |", tech.name, tech.category));
        }
    }

    sections.push(String::new());
    sections.push("## Setup\n".to_string());
    sections.push("```bash".to_string());
    sections.push("cp .env.example .env".to_string());

    let has_node = config.technologies.iter().any(|t| t == "nodejs" || t == "react" || t == "nextjs" || t == "vue" || t == "nuxt" || t == "svelte" || t == "express" || t == "fastify" || t == "nestjs");
    let has_python = config.technologies.iter().any(|t| t == "python" || t == "fastapi" || t == "django");

    if has_node {
        sections.push("npm install".to_string());
        sections.push("npm run dev".to_string());
    }
    if has_python {
        sections.push("python -m venv .venv && source .venv/bin/activate".to_string());
        sections.push("pip install -r requirements.txt".to_string());
    }
    sections.push("```".to_string());

    sections.join("\n")
}

fn generate_docker_compose(config: &ScaffoldConfig, all_techs: &[catalog::Technology]) -> String {
    let db_techs: Vec<&catalog::Technology> = config
        .technologies
        .iter()
        .filter_map(|id| all_techs.iter().find(|t| t.id == *id))
        .filter(|t| t.category == "database" && !t.docker_image.is_empty())
        .collect();

    if db_techs.is_empty() {
        return String::new();
    }

    let mut lines = vec!["services:".to_string()];

    for tech in &db_techs {
        let svc_name = tech.id.replace('-', "_");
        lines.push(format!("  {}:", svc_name));
        lines.push(format!("    image: {}", tech.docker_image));
        if tech.default_port > 0 {
            lines.push(format!("    ports:\n      - \"{}:{}\"", tech.default_port, tech.default_port));
        }
        if !tech.env_vars.is_empty() {
            lines.push("    environment:".to_string());
            let mut sorted: Vec<_> = tech.env_vars.iter().collect();
            sorted.sort_by_key(|(k, _)| (*k).clone());
            for (key, val) in sorted {
                lines.push(format!("      {}: {}", key, val));
            }
        }
        lines.push(format!("    volumes:\n      - {}_data:/var/lib/{}", svc_name, tech.id));
        lines.push(String::new());
    }

    lines.push("volumes:".to_string());
    for tech in &db_techs {
        let svc_name = tech.id.replace('-', "_");
        lines.push(format!("  {}_data:", svc_name));
    }

    lines.join("\n")
}

fn generate_makefile(config: &ScaffoldConfig) -> String {
    let has_node = config.technologies.iter().any(|t| t == "nodejs" || t == "react" || t == "nextjs" || t == "express" || t == "fastify" || t == "nestjs" || t == "vue" || t == "nuxt" || t == "svelte");
    let has_python = config.technologies.iter().any(|t| t == "python" || t == "fastapi" || t == "django");
    let has_docker = config.technologies.contains(&"docker".to_string()) || config.technologies.iter().any(|t| {
        let all = catalog::load_all_technologies();
        all.iter().any(|tech| tech.id == *t && tech.category == "database")
    });

    let mut lines = vec![
        format!(".PHONY: dev build test lint setup clean{}", if has_docker { " up down logs" } else { "" }),
        String::new(),
    ];

    if has_node {
        lines.push("dev:\n\tnpm run dev\n".to_string());
        lines.push("build:\n\tnpm run build\n".to_string());
        lines.push("test:\n\tnpm run test\n".to_string());
        lines.push("lint:\n\tnpm run lint\n".to_string());
        lines.push("setup:\n\tnpm install\n\tcp -n .env.example .env 2>/dev/null || true\n".to_string());
    } else if has_python {
        lines.push("dev:\n\tuvicorn app.main:app --reload\n".to_string());
        lines.push("test:\n\tpytest\n".to_string());
        lines.push("lint:\n\truff check .\n".to_string());
        lines.push("setup:\n\tpython -m venv .venv\n\t. .venv/bin/activate && pip install -r requirements.txt\n\tcp -n .env.example .env 2>/dev/null || true\n".to_string());
    }

    if has_docker {
        lines.push("up:\n\tdocker compose up -d\n".to_string());
        lines.push("down:\n\tdocker compose down\n".to_string());
        lines.push("logs:\n\tdocker compose logs -f\n".to_string());
    }

    lines.push("clean:\n\trm -rf node_modules dist build .next .nuxt target __pycache__\n".to_string());

    lines.join("\n")
}

fn generate_vscode_settings(config: &ScaffoldConfig) -> String {
    let mut settings: HashMap<&str, serde_json::Value> = HashMap::new();
    settings.insert("editor.formatOnSave", serde_json::Value::Bool(true));

    if config.technologies.contains(&"typescript".to_string())
        || config.technologies.iter().any(|t| t == "nextjs" || t == "react" || t == "vue" || t == "nuxt" || t == "svelte")
    {
        settings.insert(
            "editor.defaultFormatter",
            serde_json::Value::String("esbenp.prettier-vscode".to_string()),
        );
    }

    serde_json::to_string_pretty(&settings).unwrap_or_else(|_| "{}".to_string())
}

fn generate_vscode_extensions(config: &ScaffoldConfig) -> String {
    let mut recs = Vec::new();

    if config.technologies.iter().any(|t| t == "typescript" || t == "nextjs" || t == "react" || t == "express" || t == "nestjs") {
        recs.push("esbenp.prettier-vscode");
        recs.push("dbaeumer.vscode-eslint");
    }
    if config.technologies.iter().any(|t| t == "tailwindcss") {
        recs.push("bradlc.vscode-tailwindcss");
    }
    if config.technologies.iter().any(|t| t == "prisma") {
        recs.push("Prisma.prisma");
    }
    if config.technologies.iter().any(|t| t == "docker") {
        recs.push("ms-azuretools.vscode-docker");
    }
    if config.technologies.iter().any(|t| t == "python" || t == "fastapi" || t == "django") {
        recs.push("ms-python.python");
        recs.push("charliermarsh.ruff");
    }
    if config.technologies.iter().any(|t| t == "rust") {
        recs.push("rust-lang.rust-analyzer");
    }

    let val: HashMap<&str, Vec<&str>> = [("recommendations", recs)].into_iter().collect();
    serde_json::to_string_pretty(&val).unwrap_or_else(|_| "{}".to_string())
}

fn generate_ci_workflow(config: &ScaffoldConfig) -> String {
    let has_node = config.technologies.iter().any(|t| t == "nodejs" || t == "react" || t == "nextjs" || t == "express" || t == "nestjs" || t == "vue" || t == "nuxt");
    let has_python = config.technologies.iter().any(|t| t == "python" || t == "fastapi" || t == "django");

    let mut lines = vec![
        "name: CI".to_string(),
        "on:".to_string(),
        "  push:".to_string(),
        "    branches: [main, develop]".to_string(),
        "  pull_request:".to_string(),
        "    branches: [develop]".to_string(),
        "jobs:".to_string(),
    ];

    if has_node {
        lines.extend_from_slice(&[
            "  build:".to_string(),
            "    runs-on: ubuntu-latest".to_string(),
            "    steps:".to_string(),
            "      - uses: actions/checkout@v4".to_string(),
            "      - uses: actions/setup-node@v4".to_string(),
            "        with:".to_string(),
            "          node-version: 20".to_string(),
            "          cache: npm".to_string(),
            "      - run: npm ci".to_string(),
            "      - run: npm run lint".to_string(),
            "      - run: npm run test -- --run".to_string(),
            "      - run: npm run build".to_string(),
        ]);
    }

    if has_python {
        if has_node {
            lines.push(String::new());
        }
        lines.extend_from_slice(&[
            "  test:".to_string(),
            "    runs-on: ubuntu-latest".to_string(),
            "    steps:".to_string(),
            "      - uses: actions/checkout@v4".to_string(),
            "      - uses: actions/setup-python@v5".to_string(),
            "        with:".to_string(),
            "          python-version: '3.12'".to_string(),
            "      - run: pip install -r requirements.txt".to_string(),
            "      - run: pytest".to_string(),
        ]);
    }

    lines.join("\n")
}

fn build_manifest(config: &ScaffoldConfig, validation: &rules::ValidationResult) -> ProjectManifest {
    let all_techs = catalog::load_all_technologies();
    let primary_runtime = config
        .technologies
        .iter()
        .find_map(|id| {
            all_techs
                .iter()
                .find(|t| t.id == *id && t.category == "runtime")
                .map(|t| t.id.clone())
        })
        .unwrap_or_default();

    let mut commands = HashMap::new();
    let has_node = config.technologies.iter().any(|t| t == "nodejs" || t == "react" || t == "nextjs" || t == "express");
    let has_python = config.technologies.iter().any(|t| t == "python" || t == "fastapi" || t == "django");
    if has_node {
        commands.insert("dev".to_string(), "npm run dev".to_string());
        commands.insert("build".to_string(), "npm run build".to_string());
        commands.insert("test".to_string(), "npm run test".to_string());
        commands.insert("lint".to_string(), "npm run lint".to_string());
    } else if has_python {
        commands.insert("dev".to_string(), "uvicorn app.main:app --reload".to_string());
        commands.insert("test".to_string(), "pytest".to_string());
    }

    let ports: Vec<u16> = validation
        .port_assignments
        .values()
        .copied()
        .collect();

    let services: Vec<ManifestService> = config
        .technologies
        .iter()
        .filter_map(|id| {
            all_techs
                .iter()
                .find(|t| t.id == *id && t.category == "database")
                .map(|t| ManifestService {
                    name: t.id.clone(),
                    docker: true,
                    port: t.default_port,
                    health_check: String::new(),
                })
        })
        .collect();

    let mut required_env: Vec<String> = Vec::new();
    for tech_id in &config.technologies {
        if let Some(tech) = all_techs.iter().find(|t| t.id == *tech_id) {
            for key in tech.env_vars.keys() {
                if !required_env.contains(key) {
                    required_env.push(key.clone());
                }
            }
        }
    }
    required_env.sort();

    let now = chrono::Utc::now().to_rfc3339();

    ProjectManifest {
        schema_version: CURRENT_SCHEMA_VERSION,
        name: config.name.clone(),
        project_type: config.project_type.clone(),
        profile: config.profile.clone(),
        runtime: primary_runtime,
        technologies: config.technologies.clone(),
        services,
        env_vars: ManifestEnvVars {
            required: required_env,
            optional: Vec::new(),
        },
        commands,
        ports,
        recipes_applied: Vec::new(),
        health_checks: Vec::new(),
        metadata: ManifestMetadata {
            description: String::new(),
            created_at: now,
            author: String::new(),
        },
        editor: None,
    }
}

/// Registra un proyecto scaffolded en el storage — fuente unica de verdad para CLI y Tauri
pub fn register_scaffolded_project(
    config: &ScaffoldConfig,
    result: &ScaffoldResult,
) -> Result<Project, DelixonError> {
    let all_techs = catalog::load_all_technologies();
    let now = chrono::Utc::now().to_rfc3339();

    let runtimes: Vec<RuntimeConfig> = result
        .manifest
        .technologies
        .iter()
        .filter_map(|tech_id| {
            all_techs
                .iter()
                .find(|t| t.id == *tech_id && t.category == "runtime")
                .map(|t| RuntimeConfig {
                    runtime: t.id.clone(),
                    version: t.default_version.clone(),
                })
        })
        .collect();

    let project = Project {
        id: uuid::Uuid::new_v4().to_string(),
        name: config.name.clone(),
        path: config.path.clone(),
        description: None,
        runtimes,
        status: ProjectStatus::Active,
        created_at: now.clone(),
        last_opened_at: Some(now),
        template_id: None,
        tags: config.technologies.clone(),
    };

    let mut projects = storage::load_projects()?;
    projects.push(project.clone());
    storage::save_projects(&projects)?;

    Ok(project)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    fn basic_config() -> ScaffoldConfig {
        ScaffoldConfig {
            name: "test-project".to_string(),
            project_type: "api".to_string(),
            profile: "standard".to_string(),
            technologies: vec!["nodejs".to_string(), "express".to_string(), "typescript".to_string()],
            path: String::new(),
        }
    }

    #[test]
    fn test_preview_scaffold() {
        let config = basic_config();
        let preview = preview_scaffold(&config);
        assert!(!preview.files.is_empty());
        assert!(preview.files.iter().any(|f| f.path == ".gitignore"));
        assert!(preview.files.iter().any(|f| f.path == "README.md"));
        assert!(preview.files.iter().any(|f| f.path == "Makefile"));
    }

    #[test]
    #[serial(disk)]
    fn test_generate_project() {
        let dir = tempfile::tempdir().unwrap();
        let path_str = dir.path().to_str().unwrap().to_string();
        let config = ScaffoldConfig {
            path: path_str.clone(),
            ..basic_config()
        };
        let result = generate_project(&config).unwrap();
        assert!(!result.files_created.is_empty());
        assert!(dir.path().join(".gitignore").exists());
        assert!(dir.path().join("README.md").exists());
        assert!(dir.path().join(".delixon/manifest.yaml").exists());

        // Cleanup: eliminar proyecto de projects.json
        if let Ok(projects) = crate::core::storage::load_projects() {
            let filtered: Vec<_> = projects.into_iter().filter(|p| p.path != path_str).collect();
            let _ = crate::core::storage::save_projects(&filtered);
        }
    }

    #[test]
    fn test_generate_docker_compose_with_db() {
        let config = ScaffoldConfig {
            technologies: vec!["nodejs".to_string(), "postgresql".to_string()],
            ..basic_config()
        };
        let all_techs = catalog::load_all_technologies();
        let compose = generate_docker_compose(&config, &all_techs);
        assert!(compose.contains("postgresql"));
        assert!(compose.contains("services:"));
    }

    #[test]
    fn test_generate_docker_compose_no_db() {
        let config = basic_config();
        let all_techs = catalog::load_all_technologies();
        let compose = generate_docker_compose(&config, &all_techs);
        assert!(compose.is_empty());
    }

    #[test]
    fn test_generate_gitignore_python() {
        let config = ScaffoldConfig {
            technologies: vec!["python".to_string(), "fastapi".to_string()],
            ..basic_config()
        };
        let gitignore = generate_gitignore(&config);
        assert!(gitignore.contains("__pycache__"));
        assert!(gitignore.contains(".venv/"));
    }

    #[test]
    fn test_build_manifest() {
        let config = basic_config();
        let validation = rules::validate_stack(&config.technologies);
        let manifest = build_manifest(&config, &validation);
        assert_eq!(manifest.name, "test-project");
        assert_eq!(manifest.project_type, "api");
        assert_eq!(manifest.runtime, "nodejs");
    }
}
