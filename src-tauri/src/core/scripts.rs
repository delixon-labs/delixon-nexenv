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
}
