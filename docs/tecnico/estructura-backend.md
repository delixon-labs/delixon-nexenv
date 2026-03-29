# Estructura Backend — `src-tauri/src/`

Arquitectura modular del backend Rust de Delixon. Cada subdirectorio agrupa archivos por **dominio funcional**.

---

## Vista general

```
src-tauri/src/
  main.rs               Punto de entrada del binario Tauri
  lib.rs                Registro de plugins y comandos IPC
  bin/
    cli.rs              CLI standalone (delixon-cli)
  commands/             Handlers IPC Tauri (capa fina, delega a core/)
  core/                 Logica de negocio
```

---

## `core/` — Logica de negocio

```
core/
  mod.rs                Re-exports para backward compat
  error.rs              DelixonError — tipos de error centrales

  models/               Estructuras de datos compartidas
    project.rs          Project, RuntimeConfig, ProjectStatus, CreateProjectInput

  utils/                Utilidades transversales
    fs.rs               ensure_dir, write_private
    platform.rs         get_data_dir, ALLOWED_EDITORS, find_editor_in_path,
                        detect_installed_editors, EDITOR_LABELS
    editor.rs           open_in_editor, find_workspace_file (consolidado)

  project/              Ciclo de vida del proyecto
    storage.rs          Persistencia JSON (projects.json, env vars, history)
    config.rs           DelixonConfig (editor, tema, idioma)
    manifest.rs         ProjectManifest — schema, carga, generacion, validacion
    portable.rs         Export/import de proyectos (.delixon)
    notes.rs            Notas por proyecto (CRUD)

  analysis/             Deteccion, diagnostico y validacion (solo lectura)
    detection.rs        DetectedStack — escaneo de runtimes, frameworks,
                        ORM, testing, linter, CI, docker, package manager
    health.rs           HealthReport — salud del proyecto (directorio, deps, .env)
    doctor.rs           DoctorReport — diagnostico del sistema (runtimes, tools)
    rules.rs            ValidationResult — reglas de compatibilidad del catalogo

  runtime/              Interaccion con procesos externos y herramientas
    docker.rs           DockerComposeStatus — up/down/logs/status
    ports.rs            PortConflict, PortInfo — deteccion de puertos en uso
    processes.rs        ProjectProcess — listado y kill de procesos
    scripts.rs          ScriptResult — ejecucion de scripts (npm, cargo, etc.)
    git.rs              GitStatus, GitCommit — estado y log de git

  workspace/            Configuracion de IDE/editor
    vscode.rs           Generacion de .code-workspace, tasks.json, launch.json,
                        extensions.json con settings inteligentes por stack
    scaffold.rs         ScaffoldConfig — generacion de estructura de proyecto
                        desde catalogo de tecnologias

  history/              Snapshots y versionado
    env.rs              EnvSnapshot — captura de versiones de runtimes/deps
    versioning.rs       Snapshot, SnapshotDiff — versionado de manifests

  catalog/              Catalogo de tecnologias
    mod.rs              Technology — definiciones, versiones, compatibilidad
    technologies/       Archivos de datos por categoria

  templates/            Plantillas de proyecto
    mod.rs              TemplateInfo — metadatos de plantillas
    registry.rs         Registro central de plantillas disponibles
    *.rs                Generadores por plantilla (node_express, react_vite, etc.)
    files/              Contenido embebido de archivos de plantilla

  recipes/              Recetas automatizadas
    mod.rs              Recipe — archivos, dependencias, env vars, scripts
```

---

## `commands/` — Handlers IPC Tauri

Capa fina que expone la logica de `core/` como comandos Tauri. Cada archivo mapea 1:1 a un dominio.

| Archivo          | Comandos                                              |
|------------------|-------------------------------------------------------|
| projects.rs      | list, get, create, open, update, delete               |
| shell.rs         | open_terminal, open_in_editor, list_installed_editors |
| vscode.rs        | generate_vscode_workspace                             |
| detection.rs     | detect_project_stack, scan_and_register               |
| health.rs        | check_project_health, run_doctor, detect_port_conflicts, list_project_ports |
| manifest.rs      | get_manifest, regenerate_manifest                     |
| docker.rs        | docker_status, docker_up, docker_down, docker_logs    |
| git.rs           | git_status, git_log                                   |
| scripts.rs       | list_project_scripts, run_project_script              |
| processes.rs     | list_project_processes, kill_process                   |
| recipes.rs       | list_recipes, preview_recipe, apply_recipe            |
| catalog.rs       | list_catalog, get_catalog_tech, list_catalog_categories |
| rules.rs         | validate_stack                                        |
| scaffold.rs      | preview_scaffold, generate_scaffold                   |
| templates.rs     | create_from_template                                  |
| versioning.rs    | save_snapshot, list_snapshots, diff_snapshots, rollback_snapshot |
| snapshots.rs     | take_env_snapshot, diff_env_snapshot                   |
| config.rs        | get_config, set_config                                |
| environments.rs  | get_env_vars, set_env_vars                            |
| portable.rs      | export_project, import_project                        |
| notes.rs         | get_notes, add_note, delete_note                      |
| runtimes.rs      | detect_runtimes                                       |

---

## `bin/cli.rs` — CLI standalone

Binario independiente (`delixon-cli`) que usa `core/` directamente sin Tauri. Subcomandos: `list`, `open`, `create`, `doctor`, `config`, `export`, `import`.

---

## Flujo de dependencias

```
bin/cli.rs ──┐
commands/  ──┤──> core/
             │      ├── models/     (structs compartidos)
             │      ├── utils/      (helpers transversales)
             │      ├── error.rs    (tipos de error)
             │      ├── project/    (persistencia, config, manifest)
             │      ├── analysis/   (deteccion, health, doctor)
             │      ├── runtime/    (docker, git, scripts, ports)
             │      ├── workspace/  (vscode, scaffold)
             │      ├── history/    (snapshots, versioning)
             │      ├── catalog/    (tecnologias)
             │      ├── templates/  (plantillas)
             │      └── recipes/    (recetas)
```

---

## Re-exports

`core/mod.rs` re-exporta todos los submodulos para mantener compatibilidad:

```rust
// crate::core::storage  →  core/project/storage.rs
// crate::core::detection →  core/analysis/detection.rs
// crate::core::vscode   →  core/workspace/vscode.rs
// crate::core::snapshots →  core/history/env.rs
```

Ningun archivo en `commands/` o `bin/` necesita cambiar sus imports.
