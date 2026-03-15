use std::process::{Command, Output};

/// Ejecuta un comando y devuelve su output
pub fn run_command(cmd: &str, args: &[&str], cwd: Option<&std::path::Path>) -> std::io::Result<Output> {
    let mut command = Command::new(cmd);
    command.args(args);
    if let Some(dir) = cwd {
        command.current_dir(dir);
    }
    command.output()
}

/// Ejecuta un comando y devuelve stdout como String
pub fn run_command_output(cmd: &str, args: &[&str]) -> Option<String> {
    run_command(cmd, args, None)
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
