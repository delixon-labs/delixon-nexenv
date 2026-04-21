use crate::core::models::project::Project;
use crate::core::store;
use crate::core::utils::editor::open_in_editor;
use crate::core::utils::fs::pretty_path;
use crate::tui::detect::CwdContext;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

/// Resultado de una accion del menu.
enum Outcome {
    /// Se ejecuto inline (ya se imprimio lo relevante).
    Done,
    /// Hay que delegar al binario nexenv-cli con estos args.
    Subcommand(Vec<String>),
}

/// Imprime un encabezado contextual con la informacion del cwd.
pub fn print_context(ctx: &CwdContext) {
    match ctx {
        CwdContext::KnownProject(p) => {
            println!(
                "  {} {}  {}",
                "Proyecto:".dimmed(),
                p.name.bright_cyan().bold(),
                format!("({})", pretty_path(&p.path)).dimmed()
            );
            if !p.runtimes.is_empty() {
                let rts: Vec<String> = p.runtimes.iter().map(|r| r.runtime.clone()).collect();
                println!("  {} {}", "Runtimes:".dimmed(), rts.join(", ").bright_white());
            }
        }
        CwdContext::UnregisteredCandidate { path, signals } => {
            println!(
                "  {} {}",
                "Directorio:".dimmed(),
                path.bright_yellow().bold()
            );
            println!(
                "  {} {} {}",
                "Detectado:".dimmed(),
                signals.join(", ").bright_white(),
                "(no registrado)".dimmed()
            );
        }
        CwdContext::Unknown { path } => {
            println!("  {} {}", "Directorio:".dimmed(), path.dimmed());
            println!("  {}", "No es un proyecto registrado.".dimmed());
        }
    }
    println!();
}

/// Entry: menu de nivel 1 (categorias).
pub fn run_menu_once(ctx: &CwdContext) -> Result<(), String> {
    print_context(ctx);

    let theme = ColorfulTheme::default();
    let mut options: Vec<(&str, fn(&CwdContext) -> Result<Outcome, String>)> = Vec::new();

    if matches!(ctx, CwdContext::KnownProject(_)) {
        options.push(("Acciones del proyecto actual ▸", submenu_project));
    }
    if matches!(ctx, CwdContext::UnregisteredCandidate { .. }) {
        options.push(("Registrar este directorio como proyecto", action_register_cwd));
    }
    options.push(("Proyectos (listar, crear, escanear) ▸", submenu_projects));
    options.push(("Sistema (doctor, puertos, catalogo, recipes) ▸", submenu_system));
    options.push(("Docker & servicios ▸", submenu_docker));
    options.push(("Datos (env, snapshots, notes, import/export) ▸", submenu_data));
    options.push(("Volver al shell", |_| Ok(Outcome::Done)));

    let labels: Vec<&str> = options.iter().map(|(l, _)| *l).collect();
    let selection = Select::with_theme(&theme)
        .with_prompt("Menu")
        .items(&labels)
        .default(0)
        .max_length(12)
        .interact_opt()
        .map_err(|e| e.to_string())?;
    let Some(selection) = selection else {
        // Esc/Ctrl+C → vuelve al shell sin error.
        return Ok(());
    };

    let outcome = options[selection].1(ctx)?;
    handle_outcome(outcome)?;
    println!();
    Ok(())
}

fn handle_outcome(outcome: Outcome) -> Result<(), String> {
    match outcome {
        Outcome::Done => Ok(()),
        Outcome::Subcommand(args) => run_nexenv(&args),
    }
}

/// Ejecuta `nexenv-cli <args...>` inline mostrando su stdout/stderr en la terminal.
fn run_nexenv(args: &[String]) -> Result<(), String> {
    println!();
    println!("{} nexenv {}", "→".bright_magenta(), args.join(" ").dimmed());
    println!();
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let status = std::process::Command::new(&exe)
        .args(args)
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        eprintln!(
            "{} comando fallo con codigo {}",
            "error:".red().bold(),
            status.code().unwrap_or(-1)
        );
    }
    println!();
    println!("{}", "(Presiona Enter para volver al menu)".dimmed());
    let _ = std::io::stdin().read_line(&mut String::new());
    Ok(())
}

// ─── Submenus ───────────────────────────────────────────────────────────

fn submenu_project(ctx: &CwdContext) -> Result<Outcome, String> {
    let p = match ctx {
        CwdContext::KnownProject(p) => p.clone(),
        _ => return Ok(Outcome::Done),
    };

    let options: &[(&str, ProjectAction)] = &[
        ("Abrir en editor", ProjectAction::OpenEditor),
        ("Abrir terminal aqui", ProjectAction::OpenTerminal),
        ("Estado git", ProjectAction::Status),
        ("Health checks", ProjectAction::Health),
        ("Mostrar manifest", ProjectAction::Manifest),
        ("Procesos en puertos", ProjectAction::Ps),
        ("Desvincular proyecto (unlink)", ProjectAction::Unlink),
        ("Volver", ProjectAction::Back),
    ];

    let labels: Vec<&str> = options.iter().map(|(l, _)| *l).collect();
    let pick = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Proyecto: {}", p.name))
        .items(&labels)
        .default(0)
        .max_length(12)
        .interact_opt()
        .map_err(|e| e.to_string())?;
    let Some(pick) = pick else {
        // Esc/Ctrl+C → vuelve al shell sin error.
        return Ok(Outcome::Done);
    };

    let name = &p.name;
    Ok(match &options[pick].1 {
        ProjectAction::OpenEditor => {
            let cfg = store::get().load_config().map_err(|e| e.to_string())?;
            open_in_editor(&p.path, &cfg.default_editor).map_err(|e| e.to_string())?;
            println!("  {} abierto en {}", name.bright_cyan(), cfg.default_editor);
            Outcome::Done
        }
        ProjectAction::OpenTerminal => {
            open_terminal_here(&p.path)?;
            Outcome::Done
        }
        ProjectAction::Status => Outcome::Subcommand(vec!["status".into(), name.clone()]),
        ProjectAction::Health => Outcome::Subcommand(vec!["health".into(), name.clone()]),
        ProjectAction::Manifest => Outcome::Subcommand(vec!["manifest".into(), name.clone()]),
        ProjectAction::Ps => Outcome::Subcommand(vec!["ps".into(), "--project".into(), name.clone()]),
        ProjectAction::Unlink => {
            let confirm = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "¿Desvincular '{}'? (no borra archivos)",
                    name
                ))
                .default(false)
                .interact()
                .map_err(|e| e.to_string())?;
            if confirm {
                Outcome::Subcommand(vec!["unlink".into(), name.clone()])
            } else {
                Outcome::Done
            }
        }
        ProjectAction::Back => Outcome::Done,
    })
}

fn submenu_projects(_ctx: &CwdContext) -> Result<Outcome, String> {
    let options = ["Listar todos", "Nuevo proyecto (wizard)", "Escanear un directorio", "Abrir proyecto…", "Volver"];
    let pick = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Proyectos")
        .items(&options)
        .default(0)
        .max_length(12)
        .interact_opt()
        .map_err(|e| e.to_string())?;
    let Some(pick) = pick else {
        // Esc/Ctrl+C → vuelve al shell sin error.
        return Ok(Outcome::Done);
    };

    Ok(match pick {
        0 => Outcome::Subcommand(vec!["list".into()]),
        1 => {
            // Delegamos al wizard externo — mismo flow que escribir `new`.
            // Retornamos un marker: ejecutar `new` vacio para activar wizard en REPL.
            println!("  {}", "Sal al shell y escribe 'new' para abrir el wizard.".dimmed());
            Outcome::Done
        }
        2 => {
            let path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Ruta a escanear")
                .default(".".into())
                .interact_text()
                .map_err(|e| e.to_string())?;
            Outcome::Subcommand(vec!["scan".into(), path])
        }
        3 => {
            let name = pick_project_name("Abrir proyecto")?;
            match name {
                Some(n) => Outcome::Subcommand(vec!["open".into(), n]),
                None => Outcome::Done,
            }
        }
        _ => Outcome::Done,
    })
}

fn submenu_system(_ctx: &CwdContext) -> Result<Outcome, String> {
    let options = ["Doctor", "Puertos en uso", "Catalogo de tecnologias", "Recipes", "Validar tecnologias", "Volver"];
    let pick = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Sistema")
        .items(&options)
        .default(0)
        .max_length(12)
        .interact_opt()
        .map_err(|e| e.to_string())?;
    let Some(pick) = pick else {
        // Esc/Ctrl+C → vuelve al shell sin error.
        return Ok(Outcome::Done);
    };

    Ok(match pick {
        0 => Outcome::Subcommand(vec!["doctor".into()]),
        1 => Outcome::Subcommand(vec!["ports".into()]),
        2 => Outcome::Subcommand(vec!["catalog".into()]),
        3 => Outcome::Subcommand(vec!["recipes".into()]),
        4 => {
            let techs: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Tecnologias (separadas por espacio)")
                .interact_text()
                .map_err(|e| e.to_string())?;
            let mut args = vec!["validate".into()];
            args.extend(techs.split_whitespace().map(|s| s.to_string()));
            Outcome::Subcommand(args)
        }
        _ => Outcome::Done,
    })
}

fn submenu_docker(ctx: &CwdContext) -> Result<Outcome, String> {
    let name = match project_name_or_pick(ctx, "Docker — proyecto")? {
        Some(n) => n,
        None => return Ok(Outcome::Done),
    };

    let options = ["Status", "Up", "Down", "Logs", "Volver"];
    let pick = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Docker")
        .items(&options)
        .default(0)
        .max_length(12)
        .interact_opt()
        .map_err(|e| e.to_string())?;
    let Some(pick) = pick else {
        // Esc/Ctrl+C → vuelve al shell sin error.
        return Ok(Outcome::Done);
    };

    Ok(match pick {
        0 => Outcome::Subcommand(vec!["docker".into(), "status".into(), name]),
        1 => Outcome::Subcommand(vec!["docker".into(), "up".into(), name]),
        2 => Outcome::Subcommand(vec!["docker".into(), "down".into(), name]),
        3 => Outcome::Subcommand(vec!["docker".into(), "logs".into(), name]),
        _ => Outcome::Done,
    })
}

fn submenu_data(ctx: &CwdContext) -> Result<Outcome, String> {
    let options = [
        "Env vars — ver",
        "Env vars — set",
        "Snapshot — guardar",
        "Snapshot — listar",
        "Diff de env desde ultimo snapshot",
        "Notes del proyecto",
        "Exportar proyecto",
        "Importar proyecto",
        "Volver",
    ];
    let pick = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Datos del proyecto")
        .items(&options)
        .default(0)
        .max_length(12)
        .interact_opt()
        .map_err(|e| e.to_string())?;
    let Some(pick) = pick else {
        // Esc/Ctrl+C → vuelve al shell sin error.
        return Ok(Outcome::Done);
    };

    Ok(match pick {
        0 => {
            let name = match project_name_or_pick(ctx, "Proyecto")? {
                Some(n) => n,
                None => return Ok(Outcome::Done),
            };
            Outcome::Subcommand(vec!["env".into(), name, "get".into()])
        }
        1 => {
            let name = match project_name_or_pick(ctx, "Proyecto")? {
                Some(n) => n,
                None => return Ok(Outcome::Done),
            };
            let key: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Clave")
                .interact_text()
                .map_err(|e| e.to_string())?;
            let value: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Valor")
                .allow_empty(true)
                .interact_text()
                .map_err(|e| e.to_string())?;
            Outcome::Subcommand(vec![
                "env".into(),
                name,
                "set".into(),
                key,
                value,
            ])
        }
        2 => {
            let name = match project_name_or_pick(ctx, "Proyecto")? {
                Some(n) => n,
                None => return Ok(Outcome::Done),
            };
            Outcome::Subcommand(vec!["snapshot".into(), "save".into(), name])
        }
        3 => {
            let name = match project_name_or_pick(ctx, "Proyecto")? {
                Some(n) => n,
                None => return Ok(Outcome::Done),
            };
            Outcome::Subcommand(vec!["snapshot".into(), "list".into(), name])
        }
        4 => {
            let name = match project_name_or_pick(ctx, "Proyecto")? {
                Some(n) => n,
                None => return Ok(Outcome::Done),
            };
            Outcome::Subcommand(vec!["diff".into(), name])
        }
        5 => {
            let name = match project_name_or_pick(ctx, "Proyecto")? {
                Some(n) => n,
                None => return Ok(Outcome::Done),
            };
            Outcome::Subcommand(vec!["note".into(), name])
        }
        6 => {
            let name = match project_name_or_pick(ctx, "Proyecto a exportar")? {
                Some(n) => n,
                None => return Ok(Outcome::Done),
            };
            Outcome::Subcommand(vec!["export".into(), name])
        }
        7 => {
            let file: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Archivo .nexenv")
                .interact_text()
                .map_err(|e| e.to_string())?;
            let path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Ruta destino")
                .default(".".into())
                .interact_text()
                .map_err(|e| e.to_string())?;
            Outcome::Subcommand(vec!["import".into(), file, "--path".into(), path])
        }
        _ => Outcome::Done,
    })
}

enum ProjectAction {
    OpenEditor,
    OpenTerminal,
    Status,
    Health,
    Manifest,
    Ps,
    Unlink,
    Back,
}

// ─── Helpers ────────────────────────────────────────────────────────────

fn project_name_or_pick(ctx: &CwdContext, prompt: &str) -> Result<Option<String>, String> {
    if let CwdContext::KnownProject(p) = ctx {
        return Ok(Some(p.name.clone()));
    }
    pick_project_name(prompt)
}

fn pick_project_name(prompt: &str) -> Result<Option<String>, String> {
    let projects: Vec<Project> = store::get().list_projects().map_err(|e| e.to_string())?;
    if projects.is_empty() {
        println!("  {}", "No hay proyectos registrados.".dimmed());
        return Ok(None);
    }
    let names: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();
    let pick = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&names)
        .default(0)
        .max_length(12)
        .interact_opt()
        .map_err(|e| e.to_string())?;
    let Some(pick) = pick else {
        return Ok(None);
    };
    Ok(Some(names[pick].clone()))
}

fn open_terminal_here(path: &str) -> Result<(), String> {
    let path = pretty_path(path);
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "powershell", "-NoExit", "-Command", &format!("Set-Location -Path '{}'", path)])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .args(["-a", "Terminal", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let term = std::env::var("TERMINAL").unwrap_or_else(|_| "x-terminal-emulator".to_string());
        std::process::Command::new(&term)
            .args(["--working-directory", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    println!("  {} Terminal abierto en {}", "ok".green().bold(), path);
    Ok(())
}

fn action_register_cwd(ctx: &CwdContext) -> Result<Outcome, String> {
    let path = match ctx {
        CwdContext::UnregisteredCandidate { path, .. } => path.clone(),
        _ => return Ok(Outcome::Done),
    };

    let default_name = std::path::Path::new(&path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("proyecto")
        .to_string();

    println!("  {}", "Tip: deja vacio o pulsa Esc para cancelar.".dimmed());
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Nombre del proyecto (vacio = cancelar)")
        .default(default_name)
        .allow_empty(true)
        .interact_text()
        .map_err(|e| e.to_string())?;

    let name = name.trim().to_string();
    if name.is_empty() {
        println!("  {}", "Cancelado.".dimmed());
        return Ok(Outcome::Done);
    }

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Registrar '{}' en {}?", name, path))
        .default(true)
        .interact()
        .map_err(|e| e.to_string())?;

    if !confirm {
        println!("  {}", "Cancelado.".dimmed());
        return Ok(Outcome::Done);
    }

    let now = chrono::Utc::now().to_rfc3339();
    let project = crate::core::models::project::Project {
        id: uuid::Uuid::new_v4().to_string(),
        name: name.clone(),
        path: path.clone(),
        description: None,
        runtimes: vec![],
        status: crate::core::models::project::ProjectStatus::Active,
        created_at: now.clone(),
        last_opened_at: Some(now),
        template_id: None,
        tags: vec![],
    };

    let mut projects = store::get().list_projects().map_err(|e| e.to_string())?;
    projects.push(project);
    store::get()
        .save_projects(&projects)
        .map_err(|e| e.to_string())?;
    println!(
        "  {} Proyecto '{}' registrado en {}",
        "ok".green().bold(),
        name.bright_cyan(),
        path
    );
    Ok(Outcome::Done)
}
