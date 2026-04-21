use crate::core::models::project::Project;
use crate::core::store;
use crate::core::utils::fs::pretty_path;

#[derive(Debug, Clone)]
pub enum CwdContext {
    /// Cwd coincide con un proyecto ya registrado en la BD.
    KnownProject(Project),
    /// Cwd parece un proyecto (tiene .git, package.json, etc.) pero no esta registrado.
    UnregisteredCandidate { path: String, signals: Vec<String> },
    /// Cwd no parece un proyecto.
    Unknown { path: String },
}

/// Inspecciona el cwd y devuelve el contexto detectado.
pub fn detect_cwd() -> CwdContext {
    let cwd = std::env::current_dir()
        .ok()
        .and_then(|p| std::fs::canonicalize(&p).ok())
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| ".".to_string());

    if let Some(project) = find_registered_project(&cwd) {
        return CwdContext::KnownProject(project);
    }

    let signals = detect_project_signals(&cwd);
    if !signals.is_empty() {
        return CwdContext::UnregisteredCandidate {
            path: pretty_path(&cwd),
            signals,
        };
    }

    CwdContext::Unknown {
        path: pretty_path(&cwd),
    }
}

fn find_registered_project(cwd: &str) -> Option<Project> {
    let projects = store::get().list_projects().ok()?;
    let cwd_norm = normalize(cwd);
    projects
        .into_iter()
        .find(|p| normalize(&p.path) == cwd_norm)
}

fn normalize(path: &str) -> String {
    pretty_path(path)
        .trim_end_matches(['/', '\\'])
        .to_lowercase()
}

fn detect_project_signals(cwd: &str) -> Vec<String> {
    let p = std::path::Path::new(cwd);
    let markers: &[(&str, &str)] = &[
        (".git", "git"),
        ("package.json", "node"),
        ("Cargo.toml", "rust"),
        ("pyproject.toml", "python"),
        ("requirements.txt", "python"),
        ("go.mod", "go"),
        ("pom.xml", "java"),
        ("Gemfile", "ruby"),
        ("composer.json", "php"),
        (".nexenv", "nexenv"),
        ("docker-compose.yml", "docker"),
    ];
    markers
        .iter()
        .filter(|(file, _)| p.join(file).exists())
        .map(|(_, label)| label.to_string())
        .collect()
}
