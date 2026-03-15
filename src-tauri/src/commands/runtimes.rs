use tauri::command;

#[derive(serde::Serialize)]
pub struct DetectedRuntime {
    pub name: String,
    pub version: String,
    pub path: String,
}

/// Detecta los runtimes instalados en el sistema
#[command]
pub async fn detect_runtimes() -> Result<Vec<DetectedRuntime>, String> {
    let mut runtimes = vec![];

    // Detectar Node.js
    if let Ok(path) = which::which("node") {
        runtimes.push(DetectedRuntime {
            name: "node".to_string(),
            version: get_version("node", &["--version"]).unwrap_or_default(),
            path: path.to_string_lossy().to_string(),
        });
    }

    // Detectar Python
    if let Ok(path) = which::which("python") {
        runtimes.push(DetectedRuntime {
            name: "python".to_string(),
            version: get_version("python", &["--version"]).unwrap_or_default(),
            path: path.to_string_lossy().to_string(),
        });
    }

    // Detectar Rust
    if let Ok(path) = which::which("rustc") {
        runtimes.push(DetectedRuntime {
            name: "rust".to_string(),
            version: get_version("rustc", &["--version"]).unwrap_or_default(),
            path: path.to_string_lossy().to_string(),
        });
    }

    // Detectar Go
    if let Ok(path) = which::which("go") {
        runtimes.push(DetectedRuntime {
            name: "go".to_string(),
            version: get_version("go", &["version"]).unwrap_or_default(),
            path: path.to_string_lossy().to_string(),
        });
    }

    Ok(runtimes)
}

fn get_version(cmd: &str, args: &[&str]) -> Option<String> {
    std::process::Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
}
