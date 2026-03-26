use clap::{Parser, Subcommand};
use colored::Colorize;
use delixon_lib::core::{config, detection, portable, storage, templates};

#[derive(Parser)]
#[command(name = "delixon", version = "1.0.0", about = "Workspace manager for developers")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lista todos los proyectos registrados
    List,

    /// Abre un proyecto en el editor configurado
    Open {
        /// Nombre del proyecto (busqueda parcial)
        name: String,
    },

    /// Crea un nuevo proyecto
    Create {
        /// Nombre del proyecto
        name: String,
        /// Ruta donde crear el proyecto
        #[arg(long)]
        path: String,
        /// ID de plantilla a usar
        #[arg(long)]
        template: Option<String>,
    },

    /// Detecta el stack de un proyecto existente
    Scan {
        /// Ruta del proyecto a escanear
        path: String,
    },

    /// Verifica el estado del sistema
    Doctor,

    /// Gestiona variables de entorno de un proyecto
    Env {
        /// Nombre del proyecto
        project: String,
        #[command(subcommand)]
        action: EnvAction,
    },

    /// Exporta un proyecto como archivo .delixon
    Export {
        /// Nombre del proyecto
        project: String,
        /// Archivo de salida (default: {nombre}.delixon)
        #[arg(long, short)]
        output: Option<String>,
    },

    /// Importa un proyecto desde archivo .delixon
    Import {
        /// Archivo .delixon a importar
        file: String,
        /// Ruta donde registrar el proyecto
        #[arg(long)]
        path: String,
    },
}

#[derive(Subcommand)]
enum EnvAction {
    /// Muestra las variables de entorno
    Get,
    /// Establece una variable de entorno
    Set {
        /// Clave
        key: String,
        /// Valor
        value: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run_command(cli.command) {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run_command(cmd: Commands) -> Result<(), String> {
    match cmd {
        Commands::List => cmd_list(),
        Commands::Open { name } => cmd_open(&name),
        Commands::Create { name, path, template } => cmd_create(&name, &path, template.as_deref()),
        Commands::Scan { path } => cmd_scan(&path),
        Commands::Doctor => cmd_doctor(),
        Commands::Env { project, action } => cmd_env(&project, action),
        Commands::Export { project, output } => cmd_export(&project, output.as_deref()),
        Commands::Import { file, path } => cmd_import(&file, &path),
    }
}

fn cmd_list() -> Result<(), String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;

    if projects.is_empty() {
        println!("{}", "No hay proyectos registrados.".dimmed());
        return Ok(());
    }

    println!(
        "{:<30} {:<10} {:<20} {}",
        "NOMBRE".bold(),
        "ESTADO".bold(),
        "RUNTIMES".bold(),
        "RUTA".bold()
    );
    println!("{}", "-".repeat(90));

    for p in &projects {
        let runtimes: Vec<String> = p.runtimes.iter().map(|r| r.runtime.clone()).collect();
        let status = match &p.status {
            delixon_lib::core::models::project::ProjectStatus::Active => "activo".green(),
            delixon_lib::core::models::project::ProjectStatus::Idle => "inactivo".yellow(),
            delixon_lib::core::models::project::ProjectStatus::Archived => "archivado".dimmed(),
        };
        println!(
            "{:<30} {:<10} {:<20} {}",
            p.name,
            status,
            runtimes.join(", "),
            p.path.dimmed()
        );
    }

    println!("\n{} proyecto(s)", projects.len());
    Ok(())
}

fn cmd_open(name: &str) -> Result<(), String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    let lower = name.to_lowercase();
    let project = projects
        .iter()
        .find(|p| p.name.to_lowercase().contains(&lower))
        .ok_or_else(|| format!("No se encontro un proyecto que coincida con '{}'", name))?;

    let cfg = config::load_config().map_err(|e| e.to_string())?;
    let editor = &cfg.default_editor;

    // Whitelist de editores permitidos (misma que commands/shell.rs)
    const ALLOWED_EDITORS: &[&str] = &[
        "code", "code-insiders", "cursor", "zed", "subl", "atom", "nvim",
        "vim", "nano", "emacs", "gedit", "kate", "mousepad", "pluma",
        "webstorm", "phpstorm", "idea", "clion", "goland", "rustrover",
        "fleet", "lapce", "helix",
    ];

    if !ALLOWED_EDITORS.contains(&editor.as_str()) {
        return Err(format!(
            "Editor '{}' no permitido. Editores disponibles: {}",
            editor,
            ALLOWED_EDITORS.join(", ")
        ));
    }

    println!(
        "{} {} en {}...",
        "Abriendo".green().bold(),
        project.name.bold(),
        editor
    );

    std::process::Command::new(editor)
        .arg(&project.path)
        .spawn()
        .map_err(|e| format!("Error abriendo {}: {}", editor, e))?;

    Ok(())
}

fn cmd_create(name: &str, path: &str, template: Option<&str>) -> Result<(), String> {
    if let Some(tpl_id) = template {
        println!(
            "{} proyecto '{}' desde plantilla '{}'...",
            "Creando".green().bold(),
            name.bold(),
            tpl_id
        );
        let project = templates::create_from_template(tpl_id, path, name)
            .map_err(|e| e.to_string())?;
        println!("{} Proyecto creado: {}", "ok".green().bold(), project.path);
    } else {
        println!(
            "{} proyecto '{}'...",
            "Registrando".green().bold(),
            name.bold()
        );

        // Crear directorio si no existe
        let p = std::path::Path::new(path);
        if !p.exists() {
            std::fs::create_dir_all(p).map_err(|e| format!("Error creando directorio: {}", e))?;
        }

        let now = chrono::Utc::now().to_rfc3339();
        let project = delixon_lib::core::models::project::Project {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            path: path.to_string(),
            description: None,
            runtimes: vec![],
            status: delixon_lib::core::models::project::ProjectStatus::Active,
            created_at: now.clone(),
            last_opened_at: Some(now),
            template_id: None,
            tags: vec![],
        };

        let mut projects = storage::load_projects().map_err(|e| e.to_string())?;
        projects.push(project);
        storage::save_projects(&projects).map_err(|e| e.to_string())?;
        println!("{} Proyecto registrado en {}", "ok".green().bold(), path);
    }
    Ok(())
}

fn cmd_scan(path: &str) -> Result<(), String> {
    println!("{} {}...", "Escaneando".cyan().bold(), path);
    let stack = detection::detect_stack(path).map_err(|e| e.to_string())?;

    if stack.runtimes.is_empty() && stack.tags.is_empty() {
        println!("{}", "No se detecto ningun stack conocido.".dimmed());
        return Ok(());
    }

    if !stack.runtimes.is_empty() {
        println!("\n{}", "Runtimes detectados:".bold());
        for rt in &stack.runtimes {
            let ver = if rt.version.is_empty() {
                "(version no especificada)".dimmed().to_string()
            } else {
                rt.version.clone()
            };
            println!("  {} {}", rt.runtime.green(), ver);
        }
    }

    if !stack.tags.is_empty() {
        println!("\n{}", "Tags:".bold());
        println!("  {}", stack.tags.join(", ").cyan());
    }

    Ok(())
}

fn cmd_doctor() -> Result<(), String> {
    println!("{}", "Delixon Doctor".bold());
    println!("{}", "=".repeat(40));

    // Check data dir
    let data_dir = delixon_lib::core::utils::platform::get_data_dir();
    match &data_dir {
        Some(dir) => {
            if dir.exists() {
                println!("{} Directorio de datos: {}", "ok".green(), dir.display());
            } else {
                println!("{} Directorio de datos no existe (se creara al primer uso)", "!!".yellow());
            }
        }
        None => println!("{} No se pudo determinar el directorio de datos", "ERR".red()),
    }

    // Check config
    match config::load_config() {
        Ok(cfg) => {
            println!("{} Config: editor={}, tema={}, idioma={}", "ok".green(), cfg.default_editor, cfg.theme, cfg.language);
        }
        Err(e) => println!("{} Config: {}", "ERR".red(), e),
    }

    // Check projects
    match storage::load_projects() {
        Ok(projects) => println!("{} Proyectos registrados: {}", "ok".green(), projects.len()),
        Err(e) => println!("{} Proyectos: {}", "ERR".red(), e),
    }

    // Check runtimes
    println!("\n{}", "Runtimes:".bold());
    let runtimes = [("node", "--version"), ("python", "--version"), ("rustc", "--version"), ("go", "version")];
    for (cmd, arg) in &runtimes {
        match which::which(cmd) {
            Ok(path) => {
                let version = std::process::Command::new(cmd)
                    .arg(arg)
                    .output()
                    .ok()
                    .and_then(|o| String::from_utf8(o.stdout).ok())
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|| "?".to_string());
                println!("  {} {} {} ({})", "ok".green(), cmd, version, path.display().to_string().dimmed());
            }
            Err(_) => println!("  {} {} no encontrado", "--".dimmed(), cmd),
        }
    }

    // Check editor
    let cfg = config::load_config().unwrap_or_default();
    match which::which(&cfg.default_editor) {
        Ok(_) => println!("\n{} Editor '{}' disponible", "ok".green(), cfg.default_editor),
        Err(_) => println!("\n{} Editor '{}' no encontrado en PATH", "!!".yellow(), cfg.default_editor),
    }

    Ok(())
}

fn cmd_env(project_name: &str, action: EnvAction) -> Result<(), String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    let lower = project_name.to_lowercase();
    let project = projects
        .iter()
        .find(|p| p.name.to_lowercase().contains(&lower))
        .ok_or_else(|| format!("No se encontro proyecto '{}'", project_name))?;

    match action {
        EnvAction::Get => {
            let vars = storage::load_env_vars(&project.id).map_err(|e| e.to_string())?;
            if vars.is_empty() {
                println!("{}", "No hay variables de entorno configuradas.".dimmed());
            } else {
                for (k, v) in &vars {
                    println!("{}={}", k.bold(), v);
                }
            }
        }
        EnvAction::Set { key, value } => {
            let mut vars = storage::load_env_vars(&project.id).map_err(|e| e.to_string())?;
            vars.insert(key.clone(), value.clone());
            storage::save_env_vars(&project.id, &vars).map_err(|e| e.to_string())?;
            println!("{} {}={}", "ok".green().bold(), key, value);
        }
    }
    Ok(())
}

fn cmd_export(project_name: &str, output: Option<&str>) -> Result<(), String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    let lower = project_name.to_lowercase();
    let project = projects
        .iter()
        .find(|p| p.name.to_lowercase().contains(&lower))
        .ok_or_else(|| format!("No se encontro proyecto '{}'", project_name))?;

    let json = portable::export_project(&project.id).map_err(|e| e.to_string())?;
    let filename = output
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("{}.delixon", project.name));

    std::fs::write(&filename, &json).map_err(|e| format!("Error escribiendo archivo: {}", e))?;
    println!("{} Exportado a {}", "ok".green().bold(), filename);
    Ok(())
}

fn cmd_import(file: &str, path: &str) -> Result<(), String> {
    let json = std::fs::read_to_string(file)
        .map_err(|e| format!("Error leyendo {}: {}", file, e))?;
    let project = portable::import_project(&json, path).map_err(|e| e.to_string())?;
    println!(
        "{} Proyecto '{}' importado en {}",
        "ok".green().bold(),
        project.name,
        project.path
    );
    Ok(())
}
