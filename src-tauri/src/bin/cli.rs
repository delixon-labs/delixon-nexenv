use clap::{Parser, Subcommand};
use colored::Colorize;
use delixon_lib::core::{
    catalog, config, detection, docker, doctor, git, health, manifest, notes,
    portable, ports, recipes, rules, scaffold, scripts, snapshots, storage, templates, versioning,
};

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

    /// Genera un proyecto desde scaffold
    New {
        /// Nombre del proyecto
        name: String,
        /// Ruta donde crear
        #[arg(long)]
        path: String,
        /// Tipo de proyecto (api/frontend/fullstack/cli/desktop)
        #[arg(long, default_value = "api")]
        r#type: String,
        /// Perfil (rapid/standard/production)
        #[arg(long, default_value = "standard")]
        profile: String,
        /// Tecnologias (separadas por coma)
        #[arg(long, value_delimiter = ',')]
        techs: Vec<String>,
    },

    /// Aplica una recipe a un proyecto
    Add {
        /// ID de recipe (testing-vitest, docker, ci-github, etc.)
        recipe: String,
        /// Nombre del proyecto
        #[arg(long)]
        project: Option<String>,
        /// Solo preview, no aplicar
        #[arg(long)]
        preview: bool,
    },

    /// Lista recipes disponibles
    Recipes,

    /// Muestra estado Git del proyecto
    Status {
        /// Nombre del proyecto
        project: String,
    },

    /// Docker Compose management
    #[command(subcommand)]
    Docker(DockerAction),

    /// Ejecuta un script del manifest
    Run {
        /// Nombre del script
        script: String,
        /// Nombre del proyecto
        #[arg(long)]
        project: Option<String>,
    },

    /// Guarda/lista/restaura snapshots del manifest
    #[command(subcommand)]
    Snapshot(SnapshotAction),

    /// Muestra cambios de entorno desde ultimo snapshot
    Diff {
        /// Nombre del proyecto
        project: String,
    },

    /// Gestiona notas de proyecto
    Note {
        /// Nombre del proyecto
        project: String,
        /// Texto de la nota (si se omite, lista notas existentes)
        text: Option<String>,
    },

    /// Lista procesos en puertos del proyecto
    Ps {
        /// Nombre del proyecto (opcional)
        project: Option<String>,
    },
}

#[derive(Subcommand)]
enum DockerAction {
    /// Inicia servicios
    Up {
        /// Nombre del proyecto
        project: String,
    },
    /// Detiene servicios
    Down {
        /// Nombre del proyecto
        project: String,
    },
    /// Muestra estado de servicios
    Status {
        /// Nombre del proyecto
        project: String,
    },
    /// Muestra logs
    Logs {
        /// Nombre del proyecto
        project: String,
        /// Lineas a mostrar
        #[arg(long, default_value = "50")]
        lines: u32,
    },
}

#[derive(Subcommand)]
enum SnapshotAction {
    /// Guarda snapshot actual
    Save {
        /// Nombre del proyecto
        project: String,
    },
    /// Lista snapshots
    List {
        /// Nombre del proyecto
        project: String,
    },
    /// Compara dos versiones
    Diff {
        /// Nombre del proyecto
        project: String,
        /// Version origen
        v1: u32,
        /// Version destino
        v2: u32,
    },
    /// Restaura manifest a version anterior
    Rollback {
        /// Nombre del proyecto
        project: String,
        /// Version a restaurar
        version: u32,
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
        Commands::Manifest { project } => cmd_manifest(&project),
        Commands::Catalog { id } => cmd_catalog(id.as_deref()),
        Commands::Validate { techs } => cmd_validate(&techs),
        Commands::Health { project } => cmd_health(&project),
        Commands::Ports => cmd_ports(),
        Commands::New { name, path, r#type, profile, techs } => cmd_new(&name, &path, &r#type, &profile, &techs),
        Commands::Add { recipe, project, preview } => cmd_add(&recipe, project.as_deref(), preview),
        Commands::Recipes => cmd_recipes(),
        Commands::Status { project } => cmd_status(&project),
        Commands::Docker(action) => cmd_docker(action),
        Commands::Run { script, project } => cmd_run(&script, project.as_deref()),
        Commands::Snapshot(action) => cmd_snapshot(action),
        Commands::Diff { project } => cmd_diff(&project),
        Commands::Note { project, text } => cmd_note(&project, text.as_deref()),
        Commands::Ps { project } => cmd_ps(project.as_deref()),
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
    let project = find_project(name)?;
    let cfg = config::load_config().map_err(|e| e.to_string())?;
    let editor = &cfg.default_editor;

    use delixon_lib::core::utils::platform::ALLOWED_EDITORS;
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

    // Extended detection info
    if let Some(ref pm) = stack.package_manager {
        println!("\n{} {}", "Package manager:".bold(), pm.green());
    }
    if let Some(ref orm) = stack.orm {
        println!("{} {}", "ORM:".bold(), orm.green());
    }
    if let Some(ref auth) = stack.auth {
        println!("{} {}", "Auth:".bold(), auth.green());
    }
    if let Some(ref ci) = stack.ci {
        println!("{} {}", "CI/CD:".bold(), ci.green());
    }
    if let Some(ref testing) = stack.testing {
        println!("{} {}", "Testing:".bold(), testing.green());
    }
    if let Some(ref linter) = stack.linter {
        println!("{} {}", "Linter:".bold(), linter.green());
    }
    if let Some(ref docker) = stack.docker {
        let parts: Vec<&str> = [
            if docker.has_dockerfile { Some("Dockerfile") } else { None },
            if docker.has_compose { Some("Compose") } else { None },
        ]
        .iter()
        .filter_map(|x| *x)
        .collect();
        println!("{} {}", "Docker:".bold(), parts.join(" + ").green());
    }
    if stack.is_fullstack {
        println!("{} {}", "Estructura:".bold(), "fullstack".cyan());
    }

    // Readiness Score
    let score = &stack.readiness_score;
    let score_color = if score.total >= 8 {
        format!("{}/{}",score.total, score.max).green().bold()
    } else if score.total >= 5 {
        format!("{}/{}", score.total, score.max).yellow().bold()
    } else {
        format!("{}/{}", score.total, score.max).red().bold()
    };
    println!("\n{} {}", "Readiness Score:".bold(), score_color);

    for item in &score.breakdown {
        let icon = if item.present { "ok".green() } else { "--".dimmed() };
        println!(
            "  {} {:<15} {}/{}",
            icon, item.name, item.points, item.max_points
        );
    }

    if !score.suggestions.is_empty() {
        println!(
            "\n{} {}",
            "Recipes sugeridas:".dimmed(),
            score.suggestions.join(", ").yellow()
        );
    }

    Ok(())
}

fn cmd_doctor() -> Result<(), String> {
    println!("{}", "Delixon Doctor".bold());
    println!("{}", "=".repeat(40));

    let report = doctor::run_doctor().map_err(|e| e.to_string())?;

    let mut current_group = String::new();
    for check in &report.checks {
        if check.group != current_group {
            current_group = check.group.clone();
            println!("\n  {}", current_group.bold().underline());
        }
        let icon = if check.ok { "ok".green() } else { "!!".yellow() };
        println!("    {} {}: {}", icon, check.name, check.message);
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
            println!("{} {} configurada", "ok".green().bold(), key);
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
            println!("Schema:    v{}", m.schema_version);
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
            if let Some(ref editor) = m.editor {
                println!("Editor:    {}", editor);
            }
            if !m.metadata.description.is_empty() {
                println!("Desc:      {}", m.metadata.description);
            }
            if !m.metadata.author.is_empty() {
                println!("Autor:     {}", m.metadata.author);
            }
            if !m.metadata.created_at.is_empty() {
                println!("Creado:    {}", m.metadata.created_at);
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
            if !m.env_vars.required.is_empty() {
                println!("\n{}", "Env vars requeridas:".bold());
                for key in &m.env_vars.required {
                    println!("  {}", key);
                }
            }
            if !m.recipes_applied.is_empty() {
                println!("\n{}", "Recipes aplicadas:".bold());
                for recipe in &m.recipes_applied {
                    println!("  {}", recipe.green());
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

fn find_project(name: &str) -> Result<delixon_lib::core::models::project::Project, String> {
    let projects = storage::load_projects().map_err(|e| e.to_string())?;
    let lower = name.to_lowercase();
    projects
        .into_iter()
        .find(|p| p.name.to_lowercase().contains(&lower))
        .ok_or_else(|| format!("No se encontro proyecto '{}'", name))
}

fn cmd_new(name: &str, path: &str, project_type: &str, profile: &str, techs: &[String]) -> Result<(), String> {
    println!(
        "{} proyecto '{}' tipo={} perfil={}",
        "Generando".green().bold(),
        name.bold(),
        project_type.cyan(),
        profile
    );

    let config = scaffold::ScaffoldConfig {
        name: name.to_string(),
        project_type: project_type.to_string(),
        profile: profile.to_string(),
        technologies: techs.to_vec(),
        path: path.to_string(),
    };

    let result = scaffold::generate_project(&config).map_err(|e| e.to_string())?;

    println!("\n{}", "Archivos creados:".bold());
    for f in &result.files_created {
        println!("  {} {}", "+".green(), f);
    }

    if !result.validation.issues.is_empty() {
        println!("\n{}", "Validacion:".bold());
        for issue in &result.validation.issues {
            let prefix = match issue.level {
                rules::IssueLevel::Error => "ERR".red().bold(),
                rules::IssueLevel::Warning => "!!".yellow().bold(),
                rules::IssueLevel::Info => ">>".cyan(),
            };
            println!("  {} {}", prefix, issue.message);
        }
    }

    // Register the project (shared with Tauri command)
    scaffold::register_scaffolded_project(&config, &result).map_err(|e| e.to_string())?;

    println!("\n{} Proyecto generado y registrado en {}", "ok".green().bold(), path);
    Ok(())
}

fn cmd_add(recipe_id: &str, project_name: Option<&str>, preview_only: bool) -> Result<(), String> {
    if preview_only {
        let project_path = if let Some(name) = project_name {
            find_project(name)?.path
        } else {
            ".".to_string()
        };

        let preview = recipes::preview_recipe(&project_path, recipe_id).map_err(|e| e.to_string())?;
        println!("{} {}", "Recipe:".bold(), preview.recipe.name);
        println!("{}", preview.recipe.description.dimmed());
        println!("\n{}", "Archivos:".bold());
        for f in &preview.recipe.files_to_create {
            let exists = preview.files_that_exist.contains(&f.path);
            let icon = if exists { "EXISTS".yellow() } else { "+".green() };
            println!("  {} {}", icon, f.path);
        }
        return Ok(());
    }

    let project_path = if let Some(name) = project_name {
        find_project(name)?.path
    } else {
        ".".to_string()
    };

    println!(
        "{} recipe '{}'...",
        "Aplicando".green().bold(),
        recipe_id
    );

    let result = recipes::apply_recipe(&project_path, recipe_id).map_err(|e| e.to_string())?;

    for f in &result.files_created {
        println!("  {} {}", "+".green(), f);
    }
    for f in &result.files_skipped {
        println!("  {} {} (ya existe)", "~".yellow(), f);
    }
    if !result.env_vars_added.is_empty() {
        println!("  {} .env.example: {}", "+".green(), result.env_vars_added.join(", "));
    }

    println!("{} Recipe aplicada", "ok".green().bold());
    Ok(())
}

fn cmd_recipes() -> Result<(), String> {
    let recipes = recipes::list_recipes();
    println!("{} ({} disponibles)", "Recipes Delixon".bold(), recipes.len());
    println!("{}", "=".repeat(50));
    for r in &recipes {
        println!("  {:<20} {} [{}]", r.id.green(), r.description, r.category.dimmed());
    }
    Ok(())
}

fn cmd_status(project_name: &str) -> Result<(), String> {
    let project = find_project(project_name)?;

    println!("{} {}", "Proyecto:".bold(), project.name);
    println!("{}", "-".repeat(40));

    // Git status
    match git::git_status(&project.path) {
        Ok(gs) => {
            let branch_info = if gs.has_remote {
                if gs.ahead > 0 || gs.behind > 0 {
                    format!("{} (+{} -{}) ", gs.branch, gs.ahead, gs.behind)
                } else {
                    format!("{} (up to date) ", gs.branch)
                }
            } else {
                format!("{} (sin remote) ", gs.branch)
            };
            println!("  {} {}", "Git:".bold(), branch_info);

            if gs.is_clean {
                println!("  {} Working tree limpio", "ok".green());
            } else {
                println!("  {} {} modificados, {} sin trackear", "!!".yellow(), gs.modified_files, gs.untracked_files);
            }

            if let Some(commit) = &gs.last_commit {
                println!("  {} {} {}", "Ultimo:".dimmed(), commit.hash[..7].to_string().cyan(), commit.message);
            }
        }
        Err(_) => {
            println!("  {} No es un repositorio Git", "!!".yellow());
        }
    }

    // Health summary
    let report = health::check_project_health(&project).map_err(|e| e.to_string())?;
    let icon = match report.overall {
        health::HealthStatus::Ok => "ok".green().bold(),
        health::HealthStatus::Warning => "!!".yellow().bold(),
        health::HealthStatus::Error => "ERR".red().bold(),
    };
    let ok_count = report.checks.iter().filter(|c| c.status == health::HealthStatus::Ok).count();
    println!("\n  {} Health: {}/{} checks ok", icon, ok_count, report.checks.len());

    Ok(())
}

fn cmd_docker(action: DockerAction) -> Result<(), String> {
    match action {
        DockerAction::Up { project } => {
            let p = find_project(&project)?;
            println!("{} servicios de {}...", "Iniciando".green().bold(), p.name);
            let output = docker::compose_up(&p.path).map_err(|e| e.to_string())?;
            println!("{}", output);
            println!("{} Servicios iniciados", "ok".green().bold());
            Ok(())
        }
        DockerAction::Down { project } => {
            let p = find_project(&project)?;
            println!("{} servicios de {}...", "Deteniendo".yellow().bold(), p.name);
            let output = docker::compose_down(&p.path).map_err(|e| e.to_string())?;
            println!("{}", output);
            println!("{} Servicios detenidos", "ok".green().bold());
            Ok(())
        }
        DockerAction::Status { project } => {
            let p = find_project(&project)?;
            let status = docker::compose_status(&p.path).map_err(|e| e.to_string())?;
            if !status.has_compose {
                println!("{}", "No hay docker-compose en este proyecto".dimmed());
                return Ok(());
            }
            println!("{} {}", "Docker Compose:".bold(), status.compose_file);
            if status.services.is_empty() {
                println!("  {}", "No hay servicios corriendo".dimmed());
            } else {
                for svc in &status.services {
                    println!("  {} {} {}", svc.name.green(), svc.status, svc.ports.dimmed());
                }
            }
            Ok(())
        }
        DockerAction::Logs { project, lines } => {
            let p = find_project(&project)?;
            let logs = docker::compose_logs(&p.path, lines).map_err(|e| e.to_string())?;
            println!("{}", logs);
            Ok(())
        }
    }
}

fn cmd_run(script: &str, project_name: Option<&str>) -> Result<(), String> {
    let project_path = if let Some(name) = project_name {
        find_project(name)?.path
    } else {
        ".".to_string()
    };

    let available = scripts::list_scripts(&project_path).map_err(|e| e.to_string())?;
    if available.is_empty() {
        println!("{}", "No hay scripts definidos en el manifest".dimmed());
        return Ok(());
    }

    if script == "list" || script == "ls" {
        println!("{}", "Scripts disponibles:".bold());
        for (name, cmd) in &available {
            println!("  {:<15} {}", name.green(), cmd);
        }
        return Ok(());
    }

    println!("{} {}...", "Ejecutando".cyan().bold(), script);
    let result = scripts::run_script(&project_path, script).map_err(|e| e.to_string())?;
    println!("{}", result.stdout);
    if !result.stderr.is_empty() {
        eprintln!("{}", result.stderr);
    }
    if result.exit_code != 0 {
        println!("{} Exit code: {}", "!!".yellow(), result.exit_code);
    }
    Ok(())
}

fn cmd_snapshot(action: SnapshotAction) -> Result<(), String> {
    match action {
        SnapshotAction::Save { project } => {
            let p = find_project(&project)?;
            let snap = versioning::save_snapshot(&p.id, &p.path).map_err(|e| e.to_string())?;
            println!("{} Snapshot v{} guardado", "ok".green().bold(), snap.version);
            Ok(())
        }
        SnapshotAction::List { project } => {
            let p = find_project(&project)?;
            let list = versioning::list_snapshots(&p.id).map_err(|e| e.to_string())?;
            if list.is_empty() {
                println!("{}", "No hay snapshots".dimmed());
                return Ok(());
            }
            println!("{}", "Snapshots:".bold());
            for s in &list {
                println!("  v{:<5} {}", s.version, s.timestamp.dimmed());
            }
            Ok(())
        }
        SnapshotAction::Diff { project, v1, v2 } => {
            let p = find_project(&project)?;
            let diff = versioning::diff_snapshots(&p.id, v1, v2).map_err(|e| e.to_string())?;
            println!("{} v{} vs v{}", "Diff:".bold(), v1, v2);
            for t in &diff.added_techs {
                println!("  {} {}", "+".green(), t);
            }
            for t in &diff.removed_techs {
                println!("  {} {}", "-".red(), t);
            }
            for r in &diff.added_recipes {
                println!("  {} recipe: {}", "+".green(), r);
            }
            if diff.added_techs.is_empty() && diff.removed_techs.is_empty() && diff.added_recipes.is_empty() {
                println!("  {}", "Sin cambios".dimmed());
            }
            Ok(())
        }
        SnapshotAction::Rollback { project, version } => {
            let p = find_project(&project)?;
            versioning::rollback_snapshot(&p.id, &p.path, version).map_err(|e| e.to_string())?;
            println!("{} Manifest restaurado a v{}", "ok".green().bold(), version);
            Ok(())
        }
    }
}

fn cmd_diff(project_name: &str) -> Result<(), String> {
    let project = find_project(project_name)?;
    match snapshots::diff_snapshot(&project.id, &project.path) {
        Ok(Some(diff)) => {
            if diff.changed_runtimes.is_empty() && !diff.deps_changed {
                println!("{} Sin cambios desde ultimo snapshot", "ok".green());
            } else {
                if !diff.changed_runtimes.is_empty() {
                    println!("{}", "Runtimes cambiados:".bold());
                    for c in &diff.changed_runtimes {
                        println!("  {} {} -> {}", c.name, c.old_version.red(), c.new_version.green());
                    }
                }
                if diff.deps_changed {
                    println!("{} Dependencias cambiaron desde ultimo snapshot", "!!".yellow());
                }
            }
            Ok(())
        }
        Ok(None) => {
            println!("{}", "No hay snapshot previo. Tomando uno ahora...".dimmed());
            snapshots::take_snapshot(&project.id, &project.path).map_err(|e| e.to_string())?;
            println!("{} Snapshot tomado", "ok".green().bold());
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

fn cmd_note(project_name: &str, text: Option<&str>) -> Result<(), String> {
    let project = find_project(project_name)?;

    match text {
        Some(t) => {
            let note = notes::add_note(&project.id, t).map_err(|e| e.to_string())?;
            println!("{} Nota agregada ({})", "ok".green().bold(), note.id[..8].to_string().dimmed());
            Ok(())
        }
        None => {
            let project_notes = notes::get_notes(&project.id).map_err(|e| e.to_string())?;
            if project_notes.is_empty() {
                println!("{}", "No hay notas".dimmed());
            } else {
                println!("{} ({} notas)", project.name.bold(), project_notes.len());
                for n in &project_notes {
                    println!("  {} {} {}", n.id[..8].to_string().dimmed(), n.created_at[..10].to_string().dimmed(), n.text);
                }
            }
            Ok(())
        }
    }
}

fn cmd_ps(project_name: Option<&str>) -> Result<(), String> {
    if let Some(name) = project_name {
        let project = find_project(name)?;
        let procs = delixon_lib::core::processes::list_processes_on_ports(&project.path).map_err(|e| e.to_string())?;
        if procs.is_empty() {
            println!("{}", "No hay procesos en los puertos del proyecto".dimmed());
        } else {
            println!("{:<10} {:<20} {}", "PID".bold(), "NOMBRE".bold(), "PUERTO".bold());
            for p in &procs {
                println!("{:<10} {:<20} :{}", p.pid, p.name, p.port.unwrap_or(0));
            }
        }
    } else {
        println!("{}", "Especifica un proyecto: delixon ps <nombre>".dimmed());
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
