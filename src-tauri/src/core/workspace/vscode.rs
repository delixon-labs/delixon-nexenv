use serde::Serialize;
use serde_json::json;
use std::path::Path;

use crate::core::detection::DetectedStack;
use crate::core::error::DelixonError;
use crate::core::manifest::ProjectManifest;
use crate::core::models::project::Project;
use crate::core::utils::fs::ensure_dir;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VscodeGenerationResult {
    pub files_created: Vec<String>,
    pub files_skipped: Vec<String>,
    pub warnings: Vec<String>,
}

// ---------------------------------------------------------------------------
// Orquestador principal
// ---------------------------------------------------------------------------

pub fn generate_all(
    project: &Project,
    manifest: &ProjectManifest,
    stack: Option<&DetectedStack>,
) -> Result<VscodeGenerationResult, DelixonError> {
    let project_path = Path::new(&project.path);
    let vscode_dir = project_path.join(".vscode");
    let mut result = VscodeGenerationResult {
        files_created: vec![],
        files_skipped: vec![],
        warnings: vec![],
    };

    // 1. .code-workspace (siempre sobreescribir — es de Delixon)
    let ws_content = generate_workspace_content(project, manifest, stack);
    let ws_name = project
        .name
        .replace(' ', "-")
        .replace(['/', '\\', '.', '~'], "_")
        .to_lowercase();
    let ws_file = format!("{}.code-workspace", ws_name);
    let ws_path = project_path.join(&ws_file);
    std::fs::write(&ws_path, serde_json::to_string_pretty(&ws_content)?)?;
    result.files_created.push(ws_file);

    // 2. Crear .vscode/ si no existe
    ensure_dir(&vscode_dir)?;

    // 3. tasks.json — solo si no existe y hay commands
    let tasks_path = vscode_dir.join("tasks.json");
    if tasks_path.exists() {
        result.files_skipped.push(".vscode/tasks.json".into());
    } else if !manifest.commands.is_empty() {
        let tasks = generate_tasks(manifest);
        std::fs::write(&tasks_path, serde_json::to_string_pretty(&tasks)?)?;
        result.files_created.push(".vscode/tasks.json".into());
    }

    // 4. launch.json — solo si no existe
    let launch_path = vscode_dir.join("launch.json");
    if launch_path.exists() {
        result.files_skipped.push(".vscode/launch.json".into());
    } else {
        let launch = generate_launch(project, manifest, stack);
        if !launch["configurations"]
            .as_array()
            .map_or(true, |a| a.is_empty())
        {
            std::fs::write(&launch_path, serde_json::to_string_pretty(&launch)?)?;
            result.files_created.push(".vscode/launch.json".into());
        }
    }

    // 5. extensions.json — solo si no existe
    let ext_path = vscode_dir.join("extensions.json");
    if ext_path.exists() {
        result.files_skipped.push(".vscode/extensions.json".into());
    } else {
        let extensions = generate_extensions(project, manifest, stack);
        std::fs::write(&ext_path, serde_json::to_string_pretty(&extensions)?)?;
        result.files_created.push(".vscode/extensions.json".into());
    }

    Ok(result)
}

// ---------------------------------------------------------------------------
// Backward compat: funciones anteriores que delegan al nuevo sistema
// ---------------------------------------------------------------------------

pub fn generate_workspace(project: &Project) -> Result<String, DelixonError> {
    let manifest = crate::core::manifest::generate_manifest_from_project(project);
    let content = generate_workspace_content(project, &manifest, None);
    Ok(serde_json::to_string_pretty(&content)?)
}

pub fn write_workspace(project: &Project) -> Result<(), DelixonError> {
    let content = generate_workspace(project)?;
    let ws_name = project
        .name
        .replace(' ', "-")
        .replace(['/', '\\', '.', '~'], "_")
        .to_lowercase();
    let file_path = Path::new(&project.path).join(format!("{}.code-workspace", ws_name));
    std::fs::write(&file_path, content)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// 1. .code-workspace
// ---------------------------------------------------------------------------

fn generate_workspace_content(
    project: &Project,
    manifest: &ProjectManifest,
    stack: Option<&DetectedStack>,
) -> serde_json::Value {
    let extensions = collect_extensions(project, manifest, stack);
    let settings = generate_settings(manifest, stack);

    json!({
        "folders": [{ "path": "." }],
        "settings": settings,
        "extensions": {
            "recommendations": extensions
        }
    })
}

fn generate_settings(
    manifest: &ProjectManifest,
    stack: Option<&DetectedStack>,
) -> serde_json::Value {
    let mut s = serde_json::Map::new();
    let linter = stack.and_then(|st| st.linter.as_deref());
    let uses_biome = linter == Some("biome");
    let uses_ruff = linter == Some("ruff");
    let uses_black = linter == Some("black");

    s.insert("editor.formatOnSave".into(), json!(true));

    // --- Formatters: solo extensiones base del runtime (siempre se recomiendan) ---
    // No podemos poner Prisma, Biome, Ruff, Black como defaultFormatter
    // porque VS Code rechaza formatters de extensiones no instaladas.
    // Esas extensiones, al instalarse, se configuran solas.

    match manifest.runtime.as_str() {
        "node" => {
            s.insert("editor.defaultFormatter".into(), json!("esbenp.prettier-vscode"));
            s.insert("[typescript]".into(), json!({ "editor.defaultFormatter": "esbenp.prettier-vscode" }));
            s.insert("[typescriptreact]".into(), json!({ "editor.defaultFormatter": "esbenp.prettier-vscode" }));
            s.insert("[javascript]".into(), json!({ "editor.defaultFormatter": "esbenp.prettier-vscode" }));
            s.insert("[javascriptreact]".into(), json!({ "editor.defaultFormatter": "esbenp.prettier-vscode" }));

            if uses_biome {
                // Este proyecto usa Biome — deshabilitar ESLint y Prettier
                // que podrian estar instalados globalmente por otros proyectos
                s.insert("eslint.enable".into(), json!(false));
                s.insert("prettier.enable".into(), json!(false));
            } else {
                s.insert("eslint.validate".into(), json!(["javascript", "javascriptreact", "typescript", "typescriptreact"]));
                // Deshabilitar Biome si esta instalado globalmente por otro proyecto
                s.insert("biome.enabled".into(), json!(false));
            }
        }
        "python" => {
            s.insert("[python]".into(), json!({ "editor.defaultFormatter": "ms-python.python" }));

            if uses_ruff {
                // Este proyecto usa Ruff — deshabilitar linters que podrian
                // estar instalados globalmente por otros proyectos
                s.insert("python.linting.pylintEnabled".into(), json!(false));
                s.insert("python.linting.flake8Enabled".into(), json!(false));
                s.insert("black-formatter.enable".into(), json!(false));
            } else if uses_black {
                // Este proyecto usa Black — deshabilitar autopep8 si esta global
                s.insert("autopep8.enable".into(), json!(false));
                s.insert("ruff.enable".into(), json!(false));
            } else {
                // Ni Ruff ni Black — deshabilitar ambos si estan globales
                s.insert("ruff.enable".into(), json!(false));
                s.insert("black-formatter.enable".into(), json!(false));
            }

            if stack.and_then(|st| st.testing.as_deref()) == Some("pytest") {
                s.insert("python.testing.pytestEnabled".into(), json!(true));
                s.insert("python.testing.unittestEnabled".into(), json!(false));
            }
        }
        "rust" => {
            s.insert("[rust]".into(), json!({ "editor.defaultFormatter": "rust-lang.rust-analyzer" }));
            s.insert("rust-analyzer.check.command".into(), json!("clippy"));
        }
        "go" => {
            s.insert("[go]".into(), json!({ "editor.defaultFormatter": "golang.go" }));
            s.insert("go.lintTool".into(), json!("golangci-lint"));
        }
        _ => {}
    }

    // Extras por tecnologias detectadas
    let techs = &manifest.technologies;
    let tags = &project_tags_from_manifest(manifest);

    if techs.iter().any(|t| t == "tailwind") || tags.iter().any(|t| t == "tailwind") {
        s.insert("[css]".into(), json!({ "editor.defaultFormatter": "esbenp.prettier-vscode" }));
    }

    serde_json::Value::Object(s)
}

fn project_tags_from_manifest(manifest: &ProjectManifest) -> Vec<String> {
    manifest.technologies.clone()
}

// ---------------------------------------------------------------------------
// 2. tasks.json
// ---------------------------------------------------------------------------

fn generate_tasks(manifest: &ProjectManifest) -> serde_json::Value {
    let mut tasks = Vec::new();

    // Ordenar keys para salida determinista
    let mut keys: Vec<&String> = manifest.commands.keys().collect();
    keys.sort();

    for key in keys {
        let cmd = &manifest.commands[key];
        let task = build_task(key, cmd, &manifest.runtime);
        tasks.push(task);
    }

    json!({
        "version": "2.0.0",
        "tasks": tasks
    })
}

fn build_task(key: &str, command: &str, runtime: &str) -> serde_json::Value {
    let is_dev = key == "dev" || key == "start" || key == "serve";
    let is_build = key == "build";
    let is_test = key == "test";
    let is_lint = key == "lint" || key == "check";

    let group = if is_build {
        json!({ "kind": "build", "isDefault": true })
    } else if is_test {
        json!({ "kind": "test", "isDefault": true })
    } else {
        json!("none")
    };

    let problem_matcher = if is_build {
        match runtime {
            "node" => json!(["$tsc"]),
            "rust" => json!(["$rustc"]),
            "go" => json!(["$go"]),
            _ => json!([]),
        }
    } else if is_lint {
        match runtime {
            "node" => json!(["$eslint-stylish"]),
            "rust" => json!(["$rustc"]),
            _ => json!([]),
        }
    } else {
        json!([])
    };

    let mut task = json!({
        "label": key,
        "type": "shell",
        "command": command,
        "group": group,
        "problemMatcher": problem_matcher
    });

    if is_dev {
        task["isBackground"] = json!(true);
        task["presentation"] = json!({ "reveal": "always", "panel": "dedicated" });
    }

    task
}

// ---------------------------------------------------------------------------
// 3. launch.json
// ---------------------------------------------------------------------------

fn generate_launch(
    project: &Project,
    manifest: &ProjectManifest,
    stack: Option<&DetectedStack>,
) -> serde_json::Value {
    let mut configs: Vec<serde_json::Value> = Vec::new();
    let has_env = !manifest.env_vars.required.is_empty()
        || stack.is_some_and(|s| s.has_env_example);

    let env_file = if has_env {
        Some("${workspaceFolder}/.env")
    } else {
        None
    };

    // Generar configs por cada runtime del proyecto
    for rt in &project.runtimes {
        match rt.runtime.as_str() {
            "node" => {
                configs.extend(node_launch_configs(manifest, env_file));
            }
            "python" => {
                configs.extend(python_launch_configs(manifest, stack, env_file));
            }
            "rust" => {
                configs.extend(rust_launch_configs());
            }
            "go" => {
                configs.extend(go_launch_configs(env_file));
            }
            _ => {}
        }
    }

    // Fallback: si no hay runtimes en project pero manifest tiene runtime
    if configs.is_empty() && !manifest.runtime.is_empty() {
        match manifest.runtime.as_str() {
            "node" => configs.extend(node_launch_configs(manifest, env_file)),
            "python" => configs.extend(python_launch_configs(manifest, stack, env_file)),
            "rust" => configs.extend(rust_launch_configs()),
            "go" => configs.extend(go_launch_configs(env_file)),
            _ => {}
        }
    }

    json!({
        "version": "0.2.0",
        "configurations": configs
    })
}

fn node_launch_configs(
    manifest: &ProjectManifest,
    env_file: Option<&str>,
) -> Vec<serde_json::Value> {
    let mut configs = Vec::new();

    // Config principal: launch dev
    if let Some(dev_cmd) = manifest.commands.get("dev") {
        let parts: Vec<&str> = dev_cmd.splitn(2, ' ').collect();
        let (exe, args) = if parts.len() > 1 {
            (parts[0], parts[1])
        } else {
            (parts[0], "")
        };

        let mut config = json!({
            "name": "Launch Dev Server",
            "type": "node",
            "request": "launch",
            "runtimeExecutable": exe,
            "runtimeArgs": args.split_whitespace().collect::<Vec<&str>>(),
            "console": "integratedTerminal",
            "skipFiles": ["<node_internals>/**"]
        });
        if let Some(ef) = env_file {
            config["envFile"] = json!(ef);
        }
        configs.push(config);
    }

    // Attach
    let port = manifest.ports.first().copied().unwrap_or(3000);
    configs.push(json!({
        "name": format!("Attach (port {})", port),
        "type": "node",
        "request": "attach",
        "port": 9229,
        "restart": true,
        "skipFiles": ["<node_internals>/**"]
    }));

    configs
}

fn python_launch_configs(
    manifest: &ProjectManifest,
    stack: Option<&DetectedStack>,
    env_file: Option<&str>,
) -> Vec<serde_json::Value> {
    let mut configs = Vec::new();
    let techs = &manifest.technologies;

    // Detectar framework
    if techs.iter().any(|t| t == "fastapi") {
        let mut config = json!({
            "name": "Launch FastAPI",
            "type": "debugpy",
            "request": "launch",
            "module": "uvicorn",
            "args": ["app.main:app", "--reload"],
            "console": "integratedTerminal"
        });
        if let Some(ef) = env_file {
            config["envFile"] = json!(ef);
        }
        configs.push(config);
    } else if techs.iter().any(|t| t == "django") {
        let mut config = json!({
            "name": "Launch Django",
            "type": "debugpy",
            "request": "launch",
            "program": "${workspaceFolder}/manage.py",
            "args": ["runserver"],
            "console": "integratedTerminal"
        });
        if let Some(ef) = env_file {
            config["envFile"] = json!(ef);
        }
        configs.push(config);
    } else if techs.iter().any(|t| t == "flask") {
        let mut config = json!({
            "name": "Launch Flask",
            "type": "debugpy",
            "request": "launch",
            "module": "flask",
            "args": ["run", "--debug"],
            "console": "integratedTerminal"
        });
        if let Some(ef) = env_file {
            config["envFile"] = json!(ef);
        }
        configs.push(config);
    } else {
        // Python generico
        let mut config = json!({
            "name": "Launch Python",
            "type": "debugpy",
            "request": "launch",
            "program": "${file}",
            "console": "integratedTerminal"
        });
        if let Some(ef) = env_file {
            config["envFile"] = json!(ef);
        }
        configs.push(config);
    }

    // Debug tests
    if stack.and_then(|s| s.testing.as_deref()) == Some("pytest") {
        configs.push(json!({
            "name": "Debug Tests (pytest)",
            "type": "debugpy",
            "request": "launch",
            "module": "pytest",
            "args": ["-v"],
            "console": "integratedTerminal"
        }));
    }

    configs
}

fn rust_launch_configs() -> Vec<serde_json::Value> {
    vec![
        json!({
            "name": "Debug (cargo)",
            "type": "lldb",
            "request": "launch",
            "cargo": { "args": ["build"] },
            "args": [],
            "cwd": "${workspaceFolder}"
        }),
        json!({
            "name": "Debug Tests",
            "type": "lldb",
            "request": "launch",
            "cargo": { "args": ["test", "--no-run"] },
            "args": [],
            "cwd": "${workspaceFolder}"
        }),
    ]
}

fn go_launch_configs(env_file: Option<&str>) -> Vec<serde_json::Value> {
    let mut launch = json!({
        "name": "Launch",
        "type": "go",
        "request": "launch",
        "mode": "auto",
        "program": "${workspaceFolder}"
    });
    if let Some(ef) = env_file {
        launch["envFile"] = json!(ef);
    }

    vec![
        launch,
        json!({
            "name": "Debug Tests",
            "type": "go",
            "request": "launch",
            "mode": "test",
            "program": "${workspaceFolder}"
        }),
    ]
}

// ---------------------------------------------------------------------------
// 4. extensions.json
// ---------------------------------------------------------------------------

fn generate_extensions(
    project: &Project,
    manifest: &ProjectManifest,
    stack: Option<&DetectedStack>,
) -> serde_json::Value {
    let recommendations = collect_extensions(project, manifest, stack);
    let unwanted = collect_unwanted(manifest, stack);
    json!({
        "recommendations": recommendations,
        "unwantedRecommendations": unwanted
    })
}

/// Extensiones que chocan con las que recomendamos — desaconsejarlas para este workspace
fn collect_unwanted(
    manifest: &ProjectManifest,
    stack: Option<&DetectedStack>,
) -> Vec<String> {
    let mut unwanted: Vec<String> = Vec::new();
    let linter = stack.and_then(|st| st.linter.as_deref());

    // Node: Biome reemplaza ESLint + Prettier
    if manifest.runtime == "node" && linter == Some("biome") {
        unwanted.push("dbaeumer.vscode-eslint".into());
        unwanted.push("esbenp.prettier-vscode".into());
    }

    // Python: Ruff reemplaza pylint, flake8, autopep8, black
    if manifest.runtime == "python" && linter == Some("ruff") {
        unwanted.push("ms-python.pylint".into());
        unwanted.push("ms-python.flake8".into());
        unwanted.push("ms-python.autopep8".into());
        unwanted.push("ms-python.black-formatter".into());
    }

    // Python: Black reemplaza autopep8
    if manifest.runtime == "python" && linter == Some("black") {
        unwanted.push("ms-python.autopep8".into());
    }

    unwanted.sort();
    unwanted.dedup();
    unwanted
}

fn collect_extensions(
    project: &Project,
    manifest: &ProjectManifest,
    stack: Option<&DetectedStack>,
) -> Vec<String> {
    let mut exts: Vec<String> = Vec::new();
    let linter = stack.and_then(|st| st.linter.as_deref());
    let uses_biome = linter == Some("biome");

    // Por runtime
    for rt in &project.runtimes {
        match rt.runtime.as_str() {
            "node" => {
                if uses_biome {
                    exts.push("biomejs.biome".into());
                } else {
                    exts.push("dbaeumer.vscode-eslint".into());
                    exts.push("esbenp.prettier-vscode".into());
                }
            }
            "python" => {
                exts.push("ms-python.python".into());
                exts.push("ms-python.vscode-pylance".into());
            }
            "rust" => {
                exts.push("rust-lang.rust-analyzer".into());
            }
            "go" => {
                exts.push("golang.go".into());
            }
            _ => {}
        }
    }

    // Fallback por manifest.runtime si no hay runtimes
    if project.runtimes.is_empty() {
        match manifest.runtime.as_str() {
            "node" => {
                if uses_biome {
                    exts.push("biomejs.biome".into());
                } else {
                    exts.push("dbaeumer.vscode-eslint".into());
                    exts.push("esbenp.prettier-vscode".into());
                }
            }
            "python" => {
                exts.push("ms-python.python".into());
                exts.push("ms-python.vscode-pylance".into());
            }
            "rust" => exts.push("rust-lang.rust-analyzer".into()),
            "go" => exts.push("golang.go".into()),
            _ => {}
        }
    }

    // Por tags y technologies
    let all_tags: Vec<&str> = project
        .tags
        .iter()
        .chain(manifest.technologies.iter())
        .map(|s| s.as_str())
        .collect();

    for tag in &all_tags {
        match *tag {
            "docker" => exts.push("ms-azuretools.vscode-docker".into()),
            "tailwind" => exts.push("bradlc.vscode-tailwindcss".into()),
            "typescript" => exts.push("ms-vscode.vscode-typescript-next".into()),
            "vue" => exts.push("vue.volar".into()),
            "svelte" => exts.push("svelte.svelte-vscode".into()),
            "graphql" => exts.push("graphql.vscode-graphql".into()),
            _ => {}
        }
    }

    // Por stack detectado
    if let Some(st) = stack {
        if let Some("prisma") = st.orm.as_deref() {
            exts.push("Prisma.prisma".into());
        }
        match st.testing.as_deref() {
            Some("jest") => exts.push("orta.vscode-jest".into()),
            Some("vitest") => exts.push("vitest.explorer".into()),
            _ => {}
        }
        match st.linter.as_deref() {
            Some("biome") => exts.push("biomejs.biome".into()),
            Some("ruff") => exts.push("charliermarsh.ruff".into()),
            _ => {}
        }
        if let Some("github-actions") = st.ci.as_deref() {
            exts.push("github.vscode-github-actions".into());
        }
        if st.docker.is_some() {
            exts.push("ms-azuretools.vscode-docker".into());
        }
    }

    // env vars -> dotenv
    if !manifest.env_vars.required.is_empty()
        || stack.is_some_and(|s| s.has_env_example)
    {
        exts.push("mikestead.dotenv".into());
    }

    // Deduplicar y ordenar
    exts.sort();
    exts.dedup();
    exts
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::project::{Project, ProjectStatus, RuntimeConfig};
    use std::collections::HashMap;

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

    fn make_manifest(runtime: &str, commands: Vec<(&str, &str)>, techs: Vec<&str>) -> ProjectManifest {
        let mut cmds = HashMap::new();
        for (k, v) in commands {
            cmds.insert(k.to_string(), v.to_string());
        }
        ProjectManifest {
            name: "test".to_string(),
            runtime: runtime.to_string(),
            commands: cmds,
            technologies: techs.into_iter().map(|t| t.to_string()).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn test_workspace_has_settings_and_extensions() {
        let proj = make_project(vec![("node", "20")], vec!["tailwind"]);
        let manifest = make_manifest("node", vec![], vec!["tailwind"]);
        let ws = generate_workspace_content(&proj, &manifest, None);

        assert!(ws["settings"]["editor.formatOnSave"].as_bool().unwrap());
        assert!(ws["settings"]["eslint.validate"].is_array());
        let exts = ws["extensions"]["recommendations"].as_array().unwrap();
        assert!(exts.iter().any(|e| e == "dbaeumer.vscode-eslint"));
        assert!(exts.iter().any(|e| e == "bradlc.vscode-tailwindcss"));
    }

    #[test]
    fn test_tasks_from_manifest_commands() {
        let manifest = make_manifest(
            "node",
            vec![("dev", "npm run dev"), ("build", "npm run build"), ("test", "npm run test")],
            vec![],
        );
        let tasks = generate_tasks(&manifest);
        let task_list = tasks["tasks"].as_array().unwrap();
        assert_eq!(task_list.len(), 3);

        let build_task = task_list.iter().find(|t| t["label"] == "build").unwrap();
        assert_eq!(build_task["group"]["kind"], "build");

        let dev_task = task_list.iter().find(|t| t["label"] == "dev").unwrap();
        assert!(dev_task["isBackground"].as_bool().unwrap());
    }

    #[test]
    fn test_launch_node_with_dev_command() {
        let proj = make_project(vec![("node", "20")], vec![]);
        let manifest = make_manifest("node", vec![("dev", "npm run dev")], vec![]);
        let launch = generate_launch(&proj, &manifest, None);
        let configs = launch["configurations"].as_array().unwrap();

        assert!(configs.len() >= 2); // launch + attach
        assert!(configs.iter().any(|c| c["name"] == "Launch Dev Server"));
    }

    #[test]
    fn test_launch_python_fastapi() {
        let proj = make_project(vec![("python", "3.12")], vec![]);
        let manifest = make_manifest("python", vec![], vec!["fastapi"]);
        let launch = generate_launch(&proj, &manifest, None);
        let configs = launch["configurations"].as_array().unwrap();

        assert!(configs.iter().any(|c| c["name"] == "Launch FastAPI"));
        assert!(configs.iter().any(|c| c["module"] == "uvicorn"));
    }

    #[test]
    fn test_launch_python_django() {
        let proj = make_project(vec![("python", "3.12")], vec![]);
        let manifest = make_manifest("python", vec![], vec!["django"]);
        let launch = generate_launch(&proj, &manifest, None);
        let configs = launch["configurations"].as_array().unwrap();

        assert!(configs.iter().any(|c| c["name"] == "Launch Django"));
    }

    #[test]
    fn test_launch_rust() {
        let proj = make_project(vec![("rust", "1.78")], vec![]);
        let manifest = make_manifest("rust", vec![], vec![]);
        let launch = generate_launch(&proj, &manifest, None);
        let configs = launch["configurations"].as_array().unwrap();

        assert_eq!(configs.len(), 2);
        assert!(configs.iter().any(|c| c["name"] == "Debug (cargo)"));
        assert!(configs.iter().any(|c| c["name"] == "Debug Tests"));
    }

    #[test]
    fn test_launch_go() {
        let proj = make_project(vec![("go", "1.22")], vec![]);
        let manifest = make_manifest("go", vec![], vec![]);
        let launch = generate_launch(&proj, &manifest, None);
        let configs = launch["configurations"].as_array().unwrap();

        assert_eq!(configs.len(), 2);
        assert!(configs.iter().any(|c| c["name"] == "Launch"));
        assert!(configs.iter().any(|c| c["name"] == "Debug Tests"));
    }

    #[test]
    fn test_extensions_comprehensive() {
        let proj = make_project(vec![("node", "20")], vec!["docker", "tailwind"]);
        let manifest = make_manifest("node", vec![], vec!["prisma"]);

        let stack = DetectedStack {
            runtimes: vec![],
            tags: vec![],
            package_manager: None,
            orm: Some("prisma".to_string()),
            auth: None,
            ci: Some("github-actions".to_string()),
            testing: Some("vitest".to_string()),
            docker: None,
            linter: None,
            is_fullstack: false,
            has_env_example: true,
            has_readme: false,
            has_types: false,
            readiness_score: crate::core::detection::ReadinessScore {
                total: 0, max: 10, breakdown: vec![], suggestions: vec![],
            },
        };

        let exts = collect_extensions(&proj, &manifest, Some(&stack));
        assert!(exts.contains(&"dbaeumer.vscode-eslint".to_string()));
        assert!(exts.contains(&"bradlc.vscode-tailwindcss".to_string()));
        assert!(exts.contains(&"ms-azuretools.vscode-docker".to_string()));
        assert!(exts.contains(&"Prisma.prisma".to_string()));
        assert!(exts.contains(&"vitest.explorer".to_string()));
        assert!(exts.contains(&"github.vscode-github-actions".to_string()));
        assert!(exts.contains(&"mikestead.dotenv".to_string()));
    }

    #[test]
    fn test_multi_runtime_launch() {
        let proj = make_project(vec![("node", "20"), ("rust", "1.78")], vec![]);
        let manifest = make_manifest("node", vec![("dev", "npm run dev")], vec![]);
        let launch = generate_launch(&proj, &manifest, None);
        let configs = launch["configurations"].as_array().unwrap();

        // Node (launch + attach) + Rust (debug + test) = 4
        assert_eq!(configs.len(), 4);
    }

    #[test]
    fn test_workspace_json_valid() {
        let proj = make_project(vec![("node", "20"), ("rust", "1.78")], vec!["docker"]);
        let manifest = make_manifest("node", vec![("dev", "npm run dev")], vec![]);
        let ws = generate_workspace_content(&proj, &manifest, None);

        // Debe ser JSON valido
        let serialized = serde_json::to_string(&ws).unwrap();
        let _: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    }

    #[test]
    fn test_settings_python_ruff() {
        let manifest = make_manifest("python", vec![], vec![]);
        let stack = DetectedStack {
            runtimes: vec![],
            tags: vec![],
            package_manager: None,
            orm: None,
            auth: None,
            ci: None,
            testing: Some("pytest".to_string()),
            docker: None,
            linter: Some("ruff".to_string()),
            is_fullstack: false,
            has_env_example: false,
            has_readme: false,
            has_types: false,
            readiness_score: crate::core::detection::ReadinessScore {
                total: 0, max: 10, breakdown: vec![], suggestions: vec![],
            },
        };

        let settings = generate_settings(&manifest, Some(&stack));
        assert_eq!(settings["[python]"]["editor.defaultFormatter"], "ms-python.python");
        assert!(settings["python.testing.pytestEnabled"].as_bool().unwrap());
        // Ruff deshabilita conflictos globales
        assert_eq!(settings["python.linting.pylintEnabled"], false);
        assert_eq!(settings["python.linting.flake8Enabled"], false);
        assert_eq!(settings["black-formatter.enable"], false);
    }

    #[test]
    fn test_settings_node_biome_disables_conflicts() {
        let manifest = make_manifest("node", vec![], vec![]);
        let stack = DetectedStack {
            runtimes: vec![],
            tags: vec![],
            package_manager: None,
            orm: None,
            auth: None,
            ci: None,
            testing: None,
            docker: None,
            linter: Some("biome".to_string()),
            is_fullstack: false,
            has_env_example: false,
            has_readme: false,
            has_types: false,
            readiness_score: crate::core::detection::ReadinessScore {
                total: 0, max: 10, breakdown: vec![], suggestions: vec![],
            },
        };

        let settings = generate_settings(&manifest, Some(&stack));
        // Biome deshabilita ESLint y Prettier en este workspace
        assert_eq!(settings["eslint.enable"], false);
        assert_eq!(settings["prettier.enable"], false);
    }

    #[test]
    fn test_settings_node_eslint_disables_biome() {
        let manifest = make_manifest("node", vec![], vec![]);
        let stack = DetectedStack {
            runtimes: vec![],
            tags: vec![],
            package_manager: None,
            orm: None,
            auth: None,
            ci: None,
            testing: None,
            docker: None,
            linter: Some("eslint".to_string()),
            is_fullstack: false,
            has_env_example: false,
            has_readme: false,
            has_types: false,
            readiness_score: crate::core::detection::ReadinessScore {
                total: 0, max: 10, breakdown: vec![], suggestions: vec![],
            },
        };

        let settings = generate_settings(&manifest, Some(&stack));
        // ESLint+Prettier deshabilita Biome en este workspace
        assert_eq!(settings["biome.enabled"], false);
        assert!(settings["eslint.validate"].is_array());
    }
}
