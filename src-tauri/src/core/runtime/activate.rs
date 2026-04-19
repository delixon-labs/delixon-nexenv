use crate::core::models::project::RuntimeConfig;
use crate::core::runtime::managers::{detect_managers, resolve_bin_paths, Manager};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Activation {
    pub bin_paths: Vec<PathBuf>,
    pub managers_detected: Vec<Manager>,
    pub elapsed_ms: u128,
}

impl Activation {
    pub fn prefix_path(&self, current_path: &str) -> String {
        prefix_path(&self.bin_paths, current_path)
    }
}

pub fn activate(runtimes: &[RuntimeConfig]) -> Activation {
    let start = Instant::now();
    let home = home_dir();
    let managers = detect_managers(&home);
    let bin_paths = resolve_bin_paths(runtimes, &managers, &home);
    Activation {
        bin_paths,
        managers_detected: managers,
        elapsed_ms: start.elapsed().as_millis(),
    }
}

fn home_dir() -> PathBuf {
    if let Some(h) = std::env::var_os("HOME") {
        return PathBuf::from(h);
    }
    if let Some(h) = std::env::var_os("USERPROFILE") {
        return PathBuf::from(h);
    }
    PathBuf::from(".")
}

pub fn prefix_path(extras: &[PathBuf], current_path: &str) -> String {
    let sep = if cfg!(target_os = "windows") { ";" } else { ":" };
    if extras.is_empty() {
        return current_path.to_string();
    }
    let mut parts: Vec<String> = extras
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();
    if !current_path.is_empty() {
        parts.push(current_path.to_string());
    }
    parts.join(sep)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_path_empty_extras_returns_current() {
        assert_eq!(prefix_path(&[], "/usr/bin"), "/usr/bin");
    }

    #[test]
    fn prefix_path_uses_correct_separator() {
        let extras = vec![PathBuf::from("/opt/a"), PathBuf::from("/opt/b")];
        let got = prefix_path(&extras, "/usr/bin");
        let sep = if cfg!(target_os = "windows") { ";" } else { ":" };
        let expected = format!("/opt/a{sep}/opt/b{sep}/usr/bin");
        assert_eq!(got, expected);
    }

    #[test]
    fn prefix_path_skips_empty_current() {
        let extras = vec![PathBuf::from("/opt/a")];
        assert_eq!(prefix_path(&extras, ""), "/opt/a");
    }

    #[test]
    fn activate_with_empty_runtimes_returns_no_paths() {
        let act = activate(&[]);
        assert!(act.bin_paths.is_empty());
    }
}
