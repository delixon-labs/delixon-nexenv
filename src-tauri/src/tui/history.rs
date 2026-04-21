use std::fs;
use std::path::PathBuf;

const MAX_HISTORY: usize = 500;

fn history_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".nexenv").join("history"))
}

/// Path publico del historial (usado por rustyline).
pub fn path() -> Option<PathBuf> {
    let p = history_path()?;
    if let Some(dir) = p.parent() {
        let _ = fs::create_dir_all(dir);
    }
    Some(p)
}

pub fn load() -> Vec<String> {
    let Some(p) = history_path() else {
        return Vec::new();
    };
    fs::read_to_string(&p)
        .unwrap_or_default()
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.trim().is_empty())
        .collect()
}

pub fn save(entries: &[String]) {
    let Some(p) = history_path() else { return };
    if let Some(dir) = p.parent() {
        let _ = fs::create_dir_all(dir);
    }
    let tail = if entries.len() > MAX_HISTORY {
        &entries[entries.len() - MAX_HISTORY..]
    } else {
        entries
    };
    let _ = fs::write(&p, tail.join("\n"));
}

/// Agrega una entrada al final, deduplicando con la anterior consecutiva.
pub fn push(entries: &mut Vec<String>, line: &str) {
    let line = line.trim();
    if line.is_empty() {
        return;
    }
    if entries.last().map(|s| s.as_str()) == Some(line) {
        return;
    }
    entries.push(line.to_string());
    if entries.len() > MAX_HISTORY {
        let drop = entries.len() - MAX_HISTORY;
        entries.drain(0..drop);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_deduplicates_consecutive() {
        let mut h = Vec::new();
        push(&mut h, "list");
        push(&mut h, "list");
        push(&mut h, "doctor");
        push(&mut h, "list");
        assert_eq!(h, vec!["list", "doctor", "list"]);
    }

    #[test]
    fn push_ignores_empty() {
        let mut h = Vec::new();
        push(&mut h, "");
        push(&mut h, "   ");
        assert!(h.is_empty());
    }

    #[test]
    fn push_caps_at_max() {
        let mut h: Vec<String> = (0..MAX_HISTORY + 10).map(|i| format!("{}", i)).collect();
        push(&mut h, "last");
        assert_eq!(h.len(), MAX_HISTORY);
        assert_eq!(h.last().unwrap(), "last");
    }
}
