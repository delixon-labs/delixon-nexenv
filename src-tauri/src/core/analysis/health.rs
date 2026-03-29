use serde::Serialize;
use std::path::Path;

use crate::core::error::DelixonError;
use crate::core::manifest;
use crate::core::models::project::Project;
use crate::core::store;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HealthReport {
    pub project_id: String,
    pub project_name: String,
    pub overall: HealthStatus,
    pub checks: Vec<HealthCheck>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Ok,
    Warning,
    Error,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheck {
    pub name: String,
    pub status: HealthStatus,
    pub message: String,
    #[serde(default)]
    pub fix_suggestion: String,
}

pub fn check_project_health(project: &Project) -> Result<HealthReport, DelixonError> {
    let mut checks = Vec::new();
    let project_path = Path::new(&project.path);

    // 1. Directory exists
    if project_path.exists() && project_path.is_dir() {
        checks.push(HealthCheck {
            name: "Directorio".to_string(),
            status: HealthStatus::Ok,
            message: "El directorio del proyecto existe".to_string(),
            fix_suggestion: String::new(),
        });
    } else {
        checks.push(HealthCheck {
            name: "Directorio".to_string(),
            status: HealthStatus::Error,
            message: format!("El directorio no existe: {}", project.path),
            fix_suggestion: format!("Crear el directorio: mkdir -p {}", project.path),
        });
    }

    // 2. README exists
    if project_path.join("README.md").exists() {
        checks.push(HealthCheck {
            name: "README".to_string(),
            status: HealthStatus::Ok,
            message: "README.md presente".to_string(),
            fix_suggestion: String::new(),
        });
    } else {
        checks.push(HealthCheck {
            name: "README".to_string(),
            status: HealthStatus::Warning,
            message: "No se encontro README.md".to_string(),
            fix_suggestion: "Crear un README.md con documentacion basica del proyecto".to_string(),
        });
    }

    // 3. Git initialized
    if project_path.join(".git").exists() {
        checks.push(HealthCheck {
            name: "Git".to_string(),
            status: HealthStatus::Ok,
            message: "Repositorio Git inicializado".to_string(),
            fix_suggestion: String::new(),
        });
    } else {
        checks.push(HealthCheck {
            name: "Git".to_string(),
            status: HealthStatus::Warning,
            message: "No hay repositorio Git".to_string(),
            fix_suggestion: "Inicializar: git init".to_string(),
        });
    }

    // 4. .gitignore exists
    if project_path.join(".gitignore").exists() {
        checks.push(HealthCheck {
            name: "Gitignore".to_string(),
            status: HealthStatus::Ok,
            message: ".gitignore presente".to_string(),
            fix_suggestion: String::new(),
        });
    } else {
        checks.push(HealthCheck {
            name: "Gitignore".to_string(),
            status: HealthStatus::Warning,
            message: "No se encontro .gitignore".to_string(),
            fix_suggestion: "Crear .gitignore apropiado para tu stack".to_string(),
        });
    }

    // 5. Env vars configured
    let env_vars = store::get().load_env_vars(&project.id).unwrap_or_default();
    if let Ok(Some(m)) = manifest::load_manifest(&project.path) {
        let missing_required: Vec<_> = m
            .env_vars
            .required
            .iter()
            .filter(|k| !env_vars.contains_key(*k))
            .collect();

        if missing_required.is_empty() && !m.env_vars.required.is_empty() {
            checks.push(HealthCheck {
                name: "Env vars".to_string(),
                status: HealthStatus::Ok,
                message: "Todas las variables requeridas configuradas".to_string(),
                fix_suggestion: String::new(),
            });
        } else if !missing_required.is_empty() {
            checks.push(HealthCheck {
                name: "Env vars".to_string(),
                status: HealthStatus::Warning,
                message: format!(
                    "Variables requeridas sin configurar: {}",
                    missing_required
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                fix_suggestion: "Configurar las variables desde la UI o CLI: delixon env <project> set <key> <value>".to_string(),
            });
        }
    }

    // 6. Dependencies installed (node_modules / venv / target)
    let has_deps = project_path.join("node_modules").exists()
        || project_path.join("venv").exists()
        || project_path.join(".venv").exists()
        || project_path.join("target").exists()
        || project_path.join("vendor").exists();

    if has_deps {
        checks.push(HealthCheck {
            name: "Dependencias".to_string(),
            status: HealthStatus::Ok,
            message: "Directorio de dependencias encontrado".to_string(),
            fix_suggestion: String::new(),
        });
    } else if project_path.join("package.json").exists()
        || project_path.join("Cargo.toml").exists()
        || project_path.join("requirements.txt").exists()
    {
        checks.push(HealthCheck {
            name: "Dependencias".to_string(),
            status: HealthStatus::Warning,
            message: "Proyecto tiene dependencias pero no estan instaladas".to_string(),
            fix_suggestion: "Instalar dependencias: npm install / pip install -r requirements.txt / cargo build".to_string(),
        });
    }

    // 7. .env.example exists
    if project_path.join(".env.example").exists() {
        checks.push(HealthCheck {
            name: "Env example".to_string(),
            status: HealthStatus::Ok,
            message: ".env.example presente".to_string(),
            fix_suggestion: String::new(),
        });
    } else if !env_vars.is_empty() {
        checks.push(HealthCheck {
            name: "Env example".to_string(),
            status: HealthStatus::Warning,
            message: "Tienes env vars pero no hay .env.example para documentarlas".to_string(),
            fix_suggestion: "Crear .env.example con las keys (sin valores secretos)".to_string(),
        });
    }

    // 8. Testing configured
    let has_testing = project_path.join("vitest.config.ts").exists()
        || project_path.join("vitest.config.js").exists()
        || project_path.join("jest.config.js").exists()
        || project_path.join("jest.config.ts").exists()
        || project_path.join("pytest.ini").exists()
        || project_path.join("pyproject.toml").exists()
        || (project_path.join("Cargo.toml").exists() && project_path.join("tests").exists());

    if has_testing {
        checks.push(HealthCheck {
            name: "Testing".to_string(),
            status: HealthStatus::Ok,
            message: "Configuracion de tests detectada".to_string(),
            fix_suggestion: String::new(),
        });
    } else {
        checks.push(HealthCheck {
            name: "Testing".to_string(),
            status: HealthStatus::Warning,
            message: "No se detecto configuracion de tests".to_string(),
            fix_suggestion: "Agregar testing: delixon add testing (futuro)".to_string(),
        });
    }

    // 9. Manifest present
    if let Ok(Some(_)) = manifest::load_manifest(&project.path) {
        checks.push(HealthCheck {
            name: "Manifest".to_string(),
            status: HealthStatus::Ok,
            message: ".delixon/manifest.yaml presente".to_string(),
            fix_suggestion: String::new(),
        });
    } else {
        checks.push(HealthCheck {
            name: "Manifest".to_string(),
            status: HealthStatus::Warning,
            message: "No hay manifest — informacion limitada del proyecto".to_string(),
            fix_suggestion: "Generar: delixon manifest <project>".to_string(),
        });
    }

    // Calculate overall status
    let overall = if checks.iter().any(|c| c.status == HealthStatus::Error) {
        HealthStatus::Error
    } else if checks.iter().any(|c| c.status == HealthStatus::Warning) {
        HealthStatus::Warning
    } else {
        HealthStatus::Ok
    };

    Ok(HealthReport {
        project_id: project.id.clone(),
        project_name: project.name.clone(),
        overall,
        checks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};

    fn ensure_store() {
        crate::core::store::init_test_store();
    }

    fn make_project(path: &str) -> Project {
        Project {
            id: "test-health".to_string(),
            name: "health-test".to_string(),
            path: path.to_string(),
            description: None,
            runtimes: vec![RuntimeConfig {
                runtime: "node".to_string(),
                version: "20".to_string(),
            }],
            status: ProjectStatus::Active,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            last_opened_at: None,
            template_id: None,
            tags: vec![],
        }
    }

    #[test]
    fn test_health_existing_project() {
        ensure_store();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_str().unwrap();
        // Create README and .gitignore
        std::fs::write(dir.path().join("README.md"), "# Test").unwrap();
        std::fs::write(dir.path().join(".gitignore"), "node_modules").unwrap();

        let project = make_project(path);
        let report = check_project_health(&project).unwrap();

        assert!(report.checks.iter().any(|c| c.name == "Directorio" && c.status == HealthStatus::Ok));
        assert!(report.checks.iter().any(|c| c.name == "README" && c.status == HealthStatus::Ok));
    }

    #[test]
    fn test_health_missing_directory() {
        ensure_store();
        let project = make_project("/nonexistent/path/xyz");
        let report = check_project_health(&project).unwrap();

        assert_eq!(report.overall, HealthStatus::Error);
        assert!(report.checks.iter().any(|c| c.name == "Directorio" && c.status == HealthStatus::Error));
    }

    #[test]
    fn test_health_empty_project() {
        ensure_store();
        let dir = tempfile::tempdir().unwrap();
        let project = make_project(dir.path().to_str().unwrap());
        let report = check_project_health(&project).unwrap();

        // Should have warnings for missing README, git, gitignore, etc.
        assert!(report.checks.iter().any(|c| c.status == HealthStatus::Warning));
    }
}
