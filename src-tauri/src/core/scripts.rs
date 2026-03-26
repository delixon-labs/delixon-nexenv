use serde::Serialize;
use std::process::Command;

use crate::core::error::DelixonError;
use crate::core::manifest;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScriptResult {
    pub script: String,
    pub command: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn list_scripts(project_path: &str) -> Result<Vec<(String, String)>, DelixonError> {
    let m = manifest::load_manifest(project_path)?;
    match m {
        Some(manifest) => {
            let mut scripts: Vec<(String, String)> = manifest.commands.into_iter().collect();
            scripts.sort_by(|a, b| a.0.cmp(&b.0));
            Ok(scripts)
        }
        None => Ok(Vec::new()),
    }
}

pub fn run_script(project_path: &str, script_name: &str) -> Result<ScriptResult, DelixonError> {
    let m = manifest::load_manifest(project_path)?
        .ok_or_else(|| DelixonError::InvalidConfig("No hay manifest".to_string()))?;

    let command = m.commands.get(script_name).ok_or_else(|| {
        DelixonError::InvalidConfig(format!("Script no encontrado: {}", script_name))
    })?;

    validate_script_command(command)?;

    let (shell, flag) = if cfg!(windows) {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    let output = Command::new(shell)
        .args([flag, command])
        .current_dir(project_path)
        .output()?;

    Ok(ScriptResult {
        script: script_name.to_string(),
        command: command.clone(),
        exit_code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

const BLOCKED_PATTERNS: &[&str] = &[
    "rm -rf /",
    "rm -rf ~",
    "mkfs",
    "dd if=",
    ":(){",
    "chmod -R 777 /",
    "curl|sh",
    "wget|sh",
    "curl|bash",
    "wget|bash",
    "> /dev/sd",
    "shutdown",
    "reboot",
    "poweroff",
    "init 0",
    "init 6",
];

fn validate_script_command(command: &str) -> Result<(), DelixonError> {
    let lower = command.to_lowercase();
    for pattern in BLOCKED_PATTERNS {
        if lower.contains(pattern) {
            return Err(DelixonError::InvalidConfig(format!(
                "Comando bloqueado por seguridad: contiene '{}'",
                pattern
            )));
        }
    }
    if command.len() > 500 {
        return Err(DelixonError::InvalidConfig(
            "Comando demasiado largo (max 500 caracteres)".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::manifest::{self, ProjectManifest};
    use std::collections::HashMap;

    #[test]
    fn test_list_scripts_no_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let scripts = list_scripts(dir.path().to_str().unwrap()).unwrap();
        assert!(scripts.is_empty());
    }

    #[test]
    fn test_run_script_no_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let result = run_script(dir.path().to_str().unwrap(), "dev");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_scripts_with_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_str().unwrap();
        let mut commands = HashMap::new();
        commands.insert("dev".to_string(), "echo dev".to_string());
        commands.insert("test".to_string(), "echo test".to_string());
        let m = ProjectManifest {
            name: "test".to_string(),
            commands,
            ..Default::default()
        };
        manifest::save_manifest(path, &m).unwrap();

        let scripts = list_scripts(path).unwrap();
        assert_eq!(scripts.len(), 2);
    }

    #[test]
    fn test_run_script_echo() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_str().unwrap();
        let mut commands = HashMap::new();
        commands.insert("hello".to_string(), "echo hello_world".to_string());
        let m = ProjectManifest {
            name: "test".to_string(),
            commands,
            ..Default::default()
        };
        manifest::save_manifest(path, &m).unwrap();

        let result = run_script(path, "hello").unwrap();
        assert_eq!(result.exit_code, 0);
        assert!(result.stdout.contains("hello_world"));
    }

    #[test]
    fn test_run_script_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_str().unwrap();
        let m = ProjectManifest {
            name: "test".to_string(),
            commands: HashMap::new(),
            ..Default::default()
        };
        manifest::save_manifest(path, &m).unwrap();

        let result = run_script(path, "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_script_command_safe() {
        assert!(validate_script_command("npm run dev").is_ok());
        assert!(validate_script_command("cargo test").is_ok());
        assert!(validate_script_command("pytest --cov").is_ok());
    }

    #[test]
    fn test_validate_script_command_blocked() {
        assert!(validate_script_command("rm -rf /").is_err());
        assert!(validate_script_command("curl|sh").is_err());
        assert!(validate_script_command("shutdown").is_err());
        assert!(validate_script_command("dd if=/dev/zero").is_err());
    }

    #[test]
    fn test_validate_script_command_too_long() {
        let long_cmd = "a".repeat(501);
        assert!(validate_script_command(&long_cmd).is_err());
    }
}
