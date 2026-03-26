use clap::{Parser, Subcommand};
use colored::Colorize;
use delixon_lib::core::{catalog, config, detection, doctor, health, manifest, portable, ports, rules, storage, templates};

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

    /// Muestra el manifest de un proyecto
    Manifest {
        /// Nombre del proyecto
        project: String,
    },

    /// Navega el catalogo de tecnologias
    Catalog {
        /// ID de tecnologia (opcional, sin ID lista todas)
        id: Option<String>,
    },

    /// Valida una combinacion de tecnologias
    Validate {
        /// IDs de tecnologias a validar
        techs: Vec<String>,
    },

    /// Ejecuta health checks para un proyecto
    Health {
        /// Nombre del proyecto
        project: String,
    },

    /// Muestra puertos en uso por proyectos
    Ports,
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
        Commands::Manifest { project } => cmd_manifest(&project),
        Commands::Catalog { id } => cmd_catalog(id.as_deref()),
        Commands::Validate { techs } => cmd_validate(&techs),
        Commands::Health { project } => cmd_health(&project),
        Commands::Ports => cmd_ports(),
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

    let report = doctor::run_doctor().map_err(|e| e.to_string())?;

    for check in &report.checks {
        let icon = if check.ok { "ok".green() } else { "!!".yellow() };
        println!("  {} {}: {}", icon, check.name.bold(), check.message);
    }

    if report.overall_ok {
        println!("\n{}", "Todo en orden.".green().bold());
    } else {
        println!("\n{}", "Hay items que requieren atencion.".yellow());
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

fn cmd_manifest(project_name: &str) -> Result<(), String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    let lower = project_name.to_lowercase();
    let project = projects
        .iter()
        .find(|p| p.name.to_lowercase().contains(&lower))
        .ok_or_else(|| format!("No se encontro proyecto '{}'", project_name))?;

    match manifest::load_manifest(&project.path).map_err(|e| e.to_string())? {
        Some(m) => {
            println!("{} {}", "Manifest:".bold(), project.name);
            println!("{}", "=".repeat(40));
            println!("Tipo:      {}", m.project_type.cyan());
            println!("Perfil:    {}", m.profile);
            println!("Runtime:   {}", m.runtime.green());
            if !m.technologies.is_empty() {
                println!("Techs:     {}", m.technologies.join(", "));
            }
            if !m.ports.is_empty() {
                let ports: Vec<String> = m.ports.iter().map(|p| p.to_string()).collect();
                println!("Puertos:   {}", ports.join(", "));
            }
            if !m.commands.is_empty() {
                println!("\n{}", "Comandos:".bold());
                for (key, val) in &m.commands {
                    println!("  {:<10} {}", key.bold(), val);
                }
            }
            if !m.services.is_empty() {
                println!("\n{}", "Servicios:".bold());
                for svc in &m.services {
                    println!("  {} (puerto {})", svc.name, svc.port);
                }
            }
        }
        None => {
            println!("{}", "No se encontro manifest. Generando...".yellow());
            let m = manifest::generate_manifest_from_project(project);
            manifest::save_manifest(&project.path, &m).map_err(|e| e.to_string())?;
            println!("{} Manifest generado en {}/.delixon/manifest.yaml", "ok".green().bold(), project.path);
        }
    }
    Ok(())
}

fn cmd_health(project_name: &str) -> Result<(), String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    let lower = project_name.to_lowercase();
    let project = projects
        .iter()
        .find(|p| p.name.to_lowercase().contains(&lower))
        .ok_or_else(|| format!("No se encontro proyecto '{}'", project_name))?;

    let report = health::check_project_health(project).map_err(|e| e.to_string())?;

    let overall_icon = match report.overall {
        health::HealthStatus::Ok => "OK".green().bold(),
        health::HealthStatus::Warning => "!!".yellow().bold(),
        health::HealthStatus::Error => "ERR".red().bold(),
    };

    println!("{} Health: {} {}", overall_icon, report.project_name.bold(), format!("({})", report.project_id).dimmed());
    println!("{}", "=".repeat(50));

    for check in &report.checks {
        let icon = match check.status {
            health::HealthStatus::Ok => "ok".green(),
            health::HealthStatus::Warning => "!!".yellow(),
            health::HealthStatus::Error => "ERR".red(),
        };
        println!("  {} {}: {}", icon, check.name.bold(), check.message);
        if !check.fix_suggestion.is_empty() && check.status != health::HealthStatus::Ok {
            println!("     {} {}", "fix:".dimmed(), check.fix_suggestion.dimmed());
        }
    }
    Ok(())
}

fn cmd_ports() -> Result<(), String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;

    let port_list = ports::list_project_ports(&projects).map_err(|e| e.to_string())?;
    let conflicts = ports::detect_port_conflicts(&projects).map_err(|e| e.to_string())?;

    if port_list.is_empty() {
        println!("{}", "No hay puertos registrados en los manifests de proyectos.".dimmed());
        return Ok(());
    }

    println!("{}", "Puertos por proyecto:".bold());
    println!("{:<8} {:<25} {}", "PUERTO".bold(), "PROYECTO".bold(), "EN USO".bold());
    println!("{}", "-".repeat(50));
    for p in &port_list {
        let in_use = if p.in_use { "si".red() } else { "no".green() };
        println!(":{:<7} {:<25} {}", p.port, p.project, in_use);
    }

    if !conflicts.is_empty() {
        println!("\n{}", "Conflictos detectados:".red().bold());
        for c in &conflicts {
            println!(
                "  {} Puerto {} compartido por: {}",
                "!!".yellow(),
                c.port,
                c.projects.join(", ")
            );
        }
    }
    Ok(())
}

fn cmd_validate(techs: &[String]) -> Result<(), String> {
    if techs.is_empty() {
        return Err("Especifica al menos una tecnologia para validar".to_string());
    }

    println!(
        "{} {}",
        "Validando:".cyan().bold(),
        techs.join(", ")
    );

    let result = rules::validate_stack(techs);

    if result.valid {
        println!("\n{} Stack valido", "OK".green().bold());
    } else {
        println!("\n{} Stack tiene errores", "ERROR".red().bold());
    }

    for issue in &result.issues {
        let prefix = match issue.level {
            rules::IssueLevel::Error => "ERR".red().bold(),
            rules::IssueLevel::Warning => "!!".yellow().bold(),
            rules::IssueLevel::Info => ">>".cyan(),
        };
        println!("  {} {}", prefix, issue.message);
    }

    if !result.resolved_dependencies.is_empty() {
        println!(
            "\n{} {}",
            "Dependencias resueltas:".bold(),
            result.resolved_dependencies.join(", ").green()
        );
    }

    if !result.port_assignments.is_empty() {
        println!("\n{}", "Puertos asignados:".bold());
        let mut ports: Vec<_> = result.port_assignments.iter().collect();
        ports.sort_by_key(|(_, p)| *p);
        for (tech, port) in ports {
            println!("  {:<20} :{}", tech, port);
        }
    }

    if !result.suggestions.is_empty() {
        println!(
            "\n{} {}",
            "Sugerencias:".dimmed(),
            result.suggestions.join(", ").dimmed()
        );
    }

    Ok(())
}

fn cmd_catalog(id: Option<&str>) -> Result<(), String> {
    let techs = catalog::load_all_technologies();

    if let Some(tech_id) = id {
        let tech = techs
            .iter()
            .find(|t| t.id == tech_id)
            .ok_or_else(|| format!("Tecnologia no encontrada: {}", tech_id))?;

        println!("{} {}", tech.name.bold(), format!("({})", tech.id).dimmed());
        println!("{}", "=".repeat(50));
        println!("Categoria:    {}", tech.category.cyan());
        println!("Descripcion:  {}", tech.description);
        if !tech.default_version.is_empty() {
            println!("Version:      {}", tech.default_version.green());
        }
        if tech.default_port > 0 {
            println!("Puerto:       {}", tech.default_port);
        }
        if !tech.docker_image.is_empty() {
            println!("Docker:       {}", tech.docker_image);
        }
        if !tech.requires.is_empty() {
            println!("Requiere:     {}", tech.requires.join(", "));
        }
        if !tech.incompatible_with.is_empty() {
            println!("Incompatible: {}", tech.incompatible_with.join(", ").red());
        }
        if !tech.suggested_with.is_empty() {
            println!("Sugerido con: {}", tech.suggested_with.join(", ").green());
        }
        if !tech.env_vars.is_empty() {
            println!("\n{}", "Variables de entorno:".bold());
            for (k, v) in &tech.env_vars {
                println!("  {}={}", k, v.dimmed());
            }
        }
        if !tech.tags.is_empty() {
            println!("\nTags: {}", tech.tags.join(", ").dimmed());
        }
    } else {
        println!("{} ({} tecnologias)", "Catalogo Delixon".bold(), techs.len());
        println!("{}", "=".repeat(60));

        let categories = catalog::all_categories();
        for cat in &categories {
            let cat_techs: Vec<&catalog::Technology> =
                techs.iter().filter(|t| t.category == *cat).collect();
            println!(
                "\n{} ({})",
                cat.to_uppercase().cyan().bold(),
                cat_techs.len()
            );
            for tech in cat_techs {
                let port = if tech.default_port > 0 {
                    format!(":{}", tech.default_port)
                } else {
                    String::new()
                };
                println!("  {:<20} {}{}", tech.id, tech.name, port.dimmed());
            }
        }
    }
    Ok(())
}
