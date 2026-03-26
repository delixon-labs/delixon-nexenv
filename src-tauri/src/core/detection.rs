use serde::Serialize;
use std::path::Path;

use crate::core::error::DelixonError;
use crate::core::models::project::RuntimeConfig;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DetectedStack {
    pub runtimes: Vec<RuntimeConfig>,
    pub tags: Vec<String>,
}

/// Escanea un directorio de proyecto y detecta su stack tecnologico
pub fn detect_stack(project_path: &str) -> Result<DetectedStack, DelixonError> {
    let path = Path::new(project_path);
    if !path.exists() || !path.is_dir() {
        return Err(DelixonError::InvalidPath(format!(
            "La ruta no existe o no es un directorio: {}",
            project_path
        )));
    }

    let mut runtimes = Vec::new();
    let mut tags = Vec::new();

    // Node.js — package.json
    let pkg_json = path.join("package.json");
    if pkg_json.exists() {
        let mut version = String::new();
        let mut detected_tags: Vec<String> = Vec::new();

        if let Ok(data) = std::fs::read_to_string(&pkg_json) {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&data) {
                // Detectar version de Node desde engines
                if let Some(engines) = parsed.get("engines") {
                    if let Some(node_ver) = engines.get("node").and_then(|v| v.as_str()) {
                        version = node_ver.to_string();
                    }
                }

                // Detectar frameworks desde dependencies + devDependencies
                let deps = [
                    parsed.get("dependencies"),
                    parsed.get("devDependencies"),
                ];
                for dep_obj in deps.iter().flatten() {
                    if let Some(obj) = dep_obj.as_object() {
                        for key in obj.keys() {
                            match key.as_str() {
                                "react" | "react-dom" => {
                                    if !detected_tags.contains(&"react".to_string()) {
                                        detected_tags.push("react".to_string());
                                    }
                                }
                                "next" => detected_tags.push("nextjs".to_string()),
                                "vue" => detected_tags.push("vue".to_string()),
                                "nuxt" => detected_tags.push("nuxt".to_string()),
                                "svelte" => detected_tags.push("svelte".to_string()),
                                "express" => detected_tags.push("express".to_string()),
                                "fastify" => detected_tags.push("fastify".to_string()),
                                "@nestjs/core" => detected_tags.push("nestjs".to_string()),
                                "@tauri-apps/api" => detected_tags.push("tauri".to_string()),
                                "electron" => detected_tags.push("electron".to_string()),
                                "vite" => detected_tags.push("vite".to_string()),
                                "tailwindcss" => detected_tags.push("tailwind".to_string()),
                                "typescript" => detected_tags.push("typescript".to_string()),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        runtimes.push(RuntimeConfig {
            runtime: "node".to_string(),
            version,
        });
        tags.extend(detected_tags);
    }

    // Rust — Cargo.toml
    let cargo_toml = path.join("Cargo.toml");
    if cargo_toml.exists() {
        let version = String::new();
        runtimes.push(RuntimeConfig {
            runtime: "rust".to_string(),
            version,
        });
        tags.push("rust".to_string());

        // Detectar si es CLI o lib
        if let Ok(data) = std::fs::read_to_string(&cargo_toml) {
            if data.contains("[[bin]]") || data.contains("[bin]") {
                tags.push("cli".to_string());
            }
        }
    }

    // Python — pyproject.toml o requirements.txt
    let pyproject = path.join("pyproject.toml");
    let requirements = path.join("requirements.txt");
    let setup_py = path.join("setup.py");
    if pyproject.exists() || requirements.exists() || setup_py.exists() {
        runtimes.push(RuntimeConfig {
            runtime: "python".to_string(),
            version: String::new(),
        });

        // Detectar frameworks desde archivos
        let files_to_scan = [&pyproject, &requirements];
        for file in &files_to_scan {
            if file.exists() {
                if let Ok(data) = std::fs::read_to_string(file) {
                    let lower = data.to_lowercase();
                    if lower.contains("fastapi") {
                        tags.push("fastapi".to_string());
                    }
                    if lower.contains("django") {
                        tags.push("django".to_string());
                    }
                    if lower.contains("flask") {
                        tags.push("flask".to_string());
                    }
                }
            }
        }
    }

    // Go — go.mod
    if path.join("go.mod").exists() {
        runtimes.push(RuntimeConfig {
            runtime: "go".to_string(),
            version: String::new(),
        });
        tags.push("go".to_string());
    }

    // .NET — *.csproj o *.sln
    if has_file_with_extension(path, "csproj") || has_file_with_extension(path, "sln") {
        runtimes.push(RuntimeConfig {
            runtime: "dotnet".to_string(),
            version: String::new(),
        });
        tags.push("dotnet".to_string());
    }

    // PHP — composer.json
    if path.join("composer.json").exists() {
        runtimes.push(RuntimeConfig {
            runtime: "php".to_string(),
            version: String::new(),
        });
        if let Ok(data) = std::fs::read_to_string(path.join("composer.json")) {
            let lower = data.to_lowercase();
            if lower.contains("laravel") {
                tags.push("laravel".to_string());
            }
            if lower.contains("symfony") {
                tags.push("symfony".to_string());
            }
        }
    }

    // Ruby — Gemfile
    if path.join("Gemfile").exists() {
        runtimes.push(RuntimeConfig {
            runtime: "ruby".to_string(),
            version: String::new(),
        });
        if let Ok(data) = std::fs::read_to_string(path.join("Gemfile")) {
            if data.contains("rails") {
                tags.push("rails".to_string());
            }
        }
    }

    // Docker
    if path.join("Dockerfile").exists() || path.join("docker-compose.yml").exists() || path.join("docker-compose.yaml").exists() {
        tags.push("docker".to_string());
    }

    // CI
    if path.join(".github").exists() {
        tags.push("ci".to_string());
    }

    // Git
    if path.join(".git").exists() {
        tags.push("git".to_string());
    }

    Ok(DetectedStack { runtimes, tags })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_stack_node() {
        let dir = tempfile::tempdir().unwrap();
        let pkg = dir.path().join("package.json");
        std::fs::write(
            &pkg,
            r#"{"name":"test","dependencies":{"express":"^4.0"},"devDependencies":{"typescript":"^5"}}"#,
        )
        .unwrap();

        let result = detect_stack(dir.path().to_str().unwrap()).unwrap();
        assert!(result.runtimes.iter().any(|r| r.runtime == "node"));
        assert!(result.tags.contains(&"express".to_string()));
        assert!(result.tags.contains(&"typescript".to_string()));
    }

    #[test]
    fn test_detect_stack_rust() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("Cargo.toml"), "[package]\nname = \"test\"\n").unwrap();

        let result = detect_stack(dir.path().to_str().unwrap()).unwrap();
        assert!(result.runtimes.iter().any(|r| r.runtime == "rust"));
    }

    #[test]
    fn test_detect_stack_python() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("requirements.txt"), "fastapi\nuvicorn\n").unwrap();

        let result = detect_stack(dir.path().to_str().unwrap()).unwrap();
        assert!(result.runtimes.iter().any(|r| r.runtime == "python"));
        assert!(result.tags.contains(&"fastapi".to_string()));
    }

    #[test]
    fn test_detect_stack_docker() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("Dockerfile"), "FROM node:20\n").unwrap();

        let result = detect_stack(dir.path().to_str().unwrap()).unwrap();
        assert!(result.tags.contains(&"docker".to_string()));
    }

    #[test]
    fn test_detect_stack_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let result = detect_stack(dir.path().to_str().unwrap()).unwrap();
        assert!(result.runtimes.is_empty());
        assert!(result.tags.is_empty());
    }

    #[test]
    fn test_detect_stack_invalid_path() {
        let result = detect_stack("/nonexistent/path/12345");
        assert!(result.is_err());
    }
}

fn has_file_with_extension(dir: &Path, ext: &str) -> bool {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(file_ext) = entry.path().extension() {
                if file_ext == ext {
                    return true;
                }
            }
        }
    }
    false
}
