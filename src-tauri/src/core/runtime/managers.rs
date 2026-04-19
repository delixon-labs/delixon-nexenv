use crate::core::models::project::RuntimeConfig;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Manager {
    Nvm,
    Fnm,
    Pyenv,
    Rustup,
}

pub fn detect_managers(home: &Path) -> Vec<Manager> {
    let mut found = Vec::new();
    if home.join(".nvm").is_dir() {
        found.push(Manager::Nvm);
    }
    if home.join(".fnm").is_dir() || home.join(".local/share/fnm").is_dir() {
        found.push(Manager::Fnm);
    }
    if home.join(".pyenv").is_dir() {
        found.push(Manager::Pyenv);
    }
    if home.join(".rustup").is_dir() {
        found.push(Manager::Rustup);
    }
    found
}

pub fn nvm_bin_path(home: &Path, version: &str) -> PathBuf {
    let v = normalize_node_version(version);
    if cfg!(target_os = "windows") {
        home.join(".nvm").join("versions").join("node").join(&v)
    } else {
        home.join(".nvm").join("versions").join("node").join(&v).join("bin")
    }
}

pub fn fnm_bin_path(home: &Path, version: &str) -> PathBuf {
    let v = normalize_node_version(version);
    let base = if home.join(".fnm").is_dir() {
        home.join(".fnm")
    } else {
        home.join(".local/share/fnm")
    };
    if cfg!(target_os = "windows") {
        base.join("node-versions").join(&v).join("installation")
    } else {
        base.join("node-versions").join(&v).join("installation").join("bin")
    }
}

pub fn pyenv_bin_path(home: &Path, version: &str) -> PathBuf {
    if cfg!(target_os = "windows") {
        home.join(".pyenv").join("pyenv-win").join("versions").join(version)
    } else {
        home.join(".pyenv").join("versions").join(version).join("bin")
    }
}

pub fn rustup_bin_path(home: &Path, toolchain: &str) -> PathBuf {
    let host = rustup_host_triple();
    let full = if toolchain.contains('-') {
        toolchain.to_string()
    } else {
        format!("{}-{}", toolchain, host)
    };
    home.join(".rustup").join("toolchains").join(full).join("bin")
}

pub fn resolve_bin_paths(
    runtimes: &[RuntimeConfig],
    managers: &[Manager],
    home: &Path,
) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for rc in runtimes {
        let lower = rc.runtime.to_ascii_lowercase();
        let candidates: Vec<PathBuf> = match lower.as_str() {
            "node" | "nodejs" => {
                let mut c = Vec::new();
                if managers.contains(&Manager::Fnm) {
                    c.push(fnm_bin_path(home, &rc.version));
                }
                if managers.contains(&Manager::Nvm) {
                    c.push(nvm_bin_path(home, &rc.version));
                }
                c
            }
            "python" | "python3" => {
                if managers.contains(&Manager::Pyenv) {
                    vec![pyenv_bin_path(home, &rc.version)]
                } else {
                    Vec::new()
                }
            }
            "rust" | "rustc" => {
                if managers.contains(&Manager::Rustup) {
                    vec![rustup_bin_path(home, &rc.version)]
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        };
        for c in candidates {
            if c.is_dir() {
                out.push(c);
                break;
            }
        }
    }
    out
}

fn normalize_node_version(v: &str) -> String {
    let trimmed = v.trim();
    if trimmed.starts_with('v') {
        trimmed.to_string()
    } else {
        format!("v{}", trimmed)
    }
}

fn rustup_host_triple() -> &'static str {
    if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        "x86_64-pc-windows-msvc"
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        "aarch64-apple-darwin"
    } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
        "x86_64-apple-darwin"
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        "x86_64-unknown-linux-gnu"
    } else if cfg!(all(target_os = "linux", target_arch = "aarch64")) {
        "aarch64-unknown-linux-gnu"
    } else {
        "unknown-unknown-unknown"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_node_version_with_or_without_v() {
        assert_eq!(normalize_node_version("22.16.0"), "v22.16.0");
        assert_eq!(normalize_node_version("v22.16.0"), "v22.16.0");
        assert_eq!(normalize_node_version("  22.0.0  "), "v22.0.0");
    }

    #[test]
    fn nvm_path_uses_v_prefix() {
        let home = Path::new("/home/u");
        let p = nvm_bin_path(home, "22.16.0");
        let s = p.to_string_lossy();
        assert!(s.contains("v22.16.0"), "got: {s}");
        assert!(s.contains(".nvm"));
    }

    #[test]
    fn rustup_appends_host_triple_when_missing() {
        let home = Path::new("/home/u");
        let p = rustup_bin_path(home, "stable");
        let s = p.to_string_lossy();
        assert!(s.contains("stable-"), "got: {s}");
        assert!(s.contains(".rustup"));
    }

    #[test]
    fn rustup_keeps_full_toolchain_name() {
        let home = Path::new("/home/u");
        let p = rustup_bin_path(home, "stable-x86_64-unknown-linux-gnu");
        let s = p.to_string_lossy();
        assert!(s.contains("stable-x86_64-unknown-linux-gnu"), "got: {s}");
    }

    #[test]
    fn detect_managers_empty_when_nothing_present() {
        let tmp = tempfile::tempdir().unwrap();
        let found = detect_managers(tmp.path());
        assert!(found.is_empty());
    }

    #[test]
    fn detect_managers_finds_nvm_and_pyenv() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join(".nvm")).unwrap();
        std::fs::create_dir_all(tmp.path().join(".pyenv")).unwrap();
        let found = detect_managers(tmp.path());
        assert!(found.contains(&Manager::Nvm));
        assert!(found.contains(&Manager::Pyenv));
        assert!(!found.contains(&Manager::Fnm));
        assert!(!found.contains(&Manager::Rustup));
    }

    #[test]
    fn resolve_bin_paths_returns_existing_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path();
        let bin_dir = if cfg!(target_os = "windows") {
            home.join(".nvm").join("versions").join("node").join("v22.16.0")
        } else {
            home.join(".nvm").join("versions").join("node").join("v22.16.0").join("bin")
        };
        std::fs::create_dir_all(&bin_dir).unwrap();
        std::fs::create_dir_all(home.join(".nvm")).unwrap();

        let runtimes = vec![RuntimeConfig {
            runtime: "node".into(),
            version: "22.16.0".into(),
        }];
        let managers = vec![Manager::Nvm];
        let paths = resolve_bin_paths(&runtimes, &managers, home);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], bin_dir);
    }

    #[test]
    fn resolve_bin_paths_skips_unknown_runtime() {
        let tmp = tempfile::tempdir().unwrap();
        let runtimes = vec![RuntimeConfig {
            runtime: "go".into(),
            version: "1.22".into(),
        }];
        let paths = resolve_bin_paths(&runtimes, &[Manager::Nvm], tmp.path());
        assert!(paths.is_empty());
    }
}
