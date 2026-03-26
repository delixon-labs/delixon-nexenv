use crate::core::error::DelixonError;
use crate::core::models::project::Project;
use std::path::Path;

/// Genera un archivo .code-workspace para el proyecto
pub fn generate_workspace(project: &Project) -> Result<String, DelixonError> {
    let mut extensions: Vec<&str> = Vec::new();

    for rt in &project.runtimes {
        match rt.runtime.as_str() {
            "node" => {
                extensions.push("dbaeumer.vscode-eslint");
                extensions.push("esbenp.prettier-vscode");
            }
            "python" => {
                extensions.push("ms-python.python");
                extensions.push("ms-python.vscode-pylance");
            }
            "rust" => {
                extensions.push("rust-lang.rust-analyzer");
            }
            "go" => {
                extensions.push("golang.go");
            }
            _ => {}
        }
    }

    // Detectar extras por tags
    for tag in &project.tags {
        match tag.as_str() {
            "docker" => extensions.push("ms-azuretools.vscode-docker"),
            "tailwind" => extensions.push("bradlc.vscode-tailwindcss"),
            "typescript" => extensions.push("ms-vscode.vscode-typescript-next"),
            _ => {}
        }
    }

    extensions.sort();
    extensions.dedup();

    let extensions_json: Vec<String> = extensions
        .iter()
        .map(|e| format!("\"{}\"", e))
        .collect();

    let workspace = format!(
        r#"{{
  "folders": [
    {{
      "path": "."
    }}
  ],
  "settings": {{}},
  "extensions": {{
    "recommendations": [
      {}
    ]
  }}
}}"#,
        extensions_json.join(",\n      ")
    );

    Ok(workspace)
}

/// Genera y escribe el archivo .code-workspace en el directorio del proyecto
pub fn write_workspace(project: &Project) -> Result<(), DelixonError> {
    let content = generate_workspace(project)?;
    let workspace_name = project.name.replace(' ', "-").to_lowercase();
    let file_path = Path::new(&project.path).join(format!("{}.code-workspace", workspace_name));
    std::fs::write(&file_path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};

    fn make_project(runtimes: Vec<(&str, &str)>, tags: Vec<&str>) -> Project {
        Project {
            id: "vscode-test".to_string(),
            name: "VSCode Test".to_string(),
            path: "/tmp/vscode-test".to_string(),
            description: None,
            runtimes: runtimes
                .into_iter()
                .map(|(r, v)| RuntimeConfig {
                    runtime: r.to_string(),
                    version: v.to_string(),
                })
                .collect(),
            status: ProjectStatus::Active,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            last_opened_at: None,
            template_id: None,
            tags: tags.into_iter().map(|t| t.to_string()).collect(),
        }
    }

    #[test]
    fn test_generate_workspace_node() {
        let proj = make_project(vec![("node", "20")], vec![]);
        let ws = generate_workspace(&proj).unwrap();
        assert!(ws.contains("dbaeumer.vscode-eslint"));
        assert!(ws.contains("esbenp.prettier-vscode"));
    }

    #[test]
    fn test_generate_workspace_python() {
        let proj = make_project(vec![("python", "3.12")], vec![]);
        let ws = generate_workspace(&proj).unwrap();
        assert!(ws.contains("ms-python.python"));
        assert!(ws.contains("ms-python.vscode-pylance"));
    }

    #[test]
    fn test_generate_workspace_rust() {
        let proj = make_project(vec![("rust", "1.78")], vec![]);
        let ws = generate_workspace(&proj).unwrap();
        assert!(ws.contains("rust-lang.rust-analyzer"));
    }

    #[test]
    fn test_generate_workspace_multi_runtime() {
        let proj = make_project(vec![("node", "20"), ("python", "3.12")], vec![]);
        let ws = generate_workspace(&proj).unwrap();
        assert!(ws.contains("dbaeumer.vscode-eslint"));
        assert!(ws.contains("ms-python.python"));
        // No duplicates: count occurrences of eslint
        assert_eq!(ws.matches("dbaeumer.vscode-eslint").count(), 1);
    }

    #[test]
    fn test_generate_workspace_with_tags() {
        let proj = make_project(vec![], vec!["docker", "tailwind"]);
        let ws = generate_workspace(&proj).unwrap();
        assert!(ws.contains("ms-azuretools.vscode-docker"));
        assert!(ws.contains("bradlc.vscode-tailwindcss"));
    }

    #[test]
    fn test_generate_workspace_empty() {
        let proj = make_project(vec![], vec![]);
        let ws = generate_workspace(&proj).unwrap();
        // recommendations should be empty (no extensions between the brackets)
        assert!(ws.contains("\"recommendations\": [\n      \n    ]"));
    }

    #[test]
    fn test_workspace_json_valid() {
        let proj = make_project(vec![("node", "20"), ("rust", "1.78")], vec!["docker"]);
        let ws = generate_workspace(&proj).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&ws)
            .expect("workspace output should be valid JSON");
        assert!(parsed.get("folders").is_some());
        assert!(parsed.get("extensions").is_some());
    }
}
