use crate::core::catalog;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use std::path::PathBuf;

/// Resultado del wizard: lista de args que se pasan al binario nexenv-cli.
/// El caller los ejecuta como subprocess igual que cualquier otro comando.
pub type WizardArgs = Vec<String>;

/// Wizard interactivo para `new`. Guia al usuario a completar todos los campos.
/// Retorna None si el usuario cancelo.
pub fn new_project() -> Result<Option<WizardArgs>, String> {
    let theme = ColorfulTheme::default();

    println!();
    println!("{}", "═══ Nuevo proyecto ═══".bright_magenta().bold());
    println!(
        "  {}",
        "Tip: deja cualquier campo vacio o pulsa Ctrl+C para cancelar.".dimmed()
    );
    println!();

    // --- Paso 1: nombre ---
    let name_raw: String = Input::with_theme(&theme)
        .with_prompt("Nombre del proyecto (vacio = cancelar)")
        .allow_empty(true)
        .validate_with(|s: &String| -> Result<(), &str> {
            let t = s.trim();
            if t.is_empty() {
                return Ok(()); // permitimos vacio para cancelar
            }
            if t.contains(|c: char| !c.is_alphanumeric() && c != '-' && c != '_') {
                Err("Solo letras, numeros, - y _ (o vacio para cancelar)")
            } else {
                Ok(())
            }
        })
        .interact_text()
        .map_err(|e| e.to_string())?;

    let name = name_raw.trim().to_string();
    if name.is_empty() {
        println!("  {}", "Cancelado.".dimmed());
        return Ok(None);
    }

    // --- Paso 2: carpeta destino ---
    let mut cwd = std::env::current_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| ".".to_string());

    println!();
    println!("  {} {}", "Carpeta actual:".dimmed(), cwd.bright_white());
    print_dir_preview(&cwd);
    println!();
    println!("  {}", "┌─ Carpeta destino ─────────────────────────────────────┐".dimmed());
    println!("  {}", "│ Escribe una RUTA donde crear el proyecto, o navega:   │".dimmed());
    println!("  {}", "│   Enter       → usa la carpeta actual                 │".dimmed());
    println!("  {}", "│   D:\\foo\\bar  → ruta absoluta                         │".dimmed());
    println!("  {}", "│   ./mi-app    → relativa al cwd                       │".dimmed());
    println!("  {}", "│   cd <ruta>   → cambia el cwd y vuelve a preguntar    │".dimmed());
    println!("  {}", "│   !ls / !dir  → lista la carpeta actual               │".dimmed());
    println!("  {}", "│   !<comando>  → ejecuta PowerShell/bash real          │".dimmed());
    println!("  {}", "└───────────────────────────────────────────────────────┘".dimmed());

    // Loop: permite navegar con `cd`, ejecutar `!cmd` y valida la ruta.
    let path_input: String = loop {
        let raw: String = Input::with_theme(&theme)
            .with_prompt("Carpeta destino")
            .default(cwd.clone())
            .interact_text()
            .map_err(|e| e.to_string())?;

        // `cd <ruta>` → cambiar cwd y volver a preguntar.
        if raw == "cd" || raw.starts_with("cd ") {
            let target = raw.strip_prefix("cd").unwrap_or("").trim();
            let new_path = if target.is_empty() {
                match dirs::home_dir() {
                    Some(h) => h,
                    None => {
                        println!("  {} No se pudo determinar HOME", "✗".red());
                        continue;
                    }
                }
            } else {
                resolve_path(target)
            };
            if !new_path.exists() || !new_path.is_dir() {
                println!(
                    "  {} '{}' no existe o no es directorio",
                    "✗".red(),
                    new_path.display()
                );
                continue;
            }
            if let Err(e) = std::env::set_current_dir(&new_path) {
                println!("  {} cd: {}", "✗".red(), e);
                continue;
            }
            cwd = new_path.to_string_lossy().into_owned();
            println!("  {} Ahora en: {}", "→".bright_magenta(), cwd.bright_white());
            print_dir_preview(&cwd);
            println!();
            continue;
        }

        // `!comando` → ejecutar shell y volver a preguntar.
        if let Some(cmd) = raw.strip_prefix('!') {
            let cmd = cmd.trim();
            // Heuristica: si parece una ruta con `!` por error, avisar.
            if looks_like_path(cmd) {
                println!(
                    "  {}  '{}' parece una ruta, no un comando.",
                    "⚠".yellow(),
                    cmd.bright_white()
                );
                println!(
                    "  {}  Si queres usarla como destino, quita el '!' inicial.",
                    "→".dimmed()
                );
                println!();
                continue;
            }
            run_native_shell(cmd);
            println!();
            continue;
        }

        // Validar la ruta: existe? parent existe? vamos a crear?
        let resolved = resolve_path(&raw);
        let resolved_str = resolved.to_string_lossy().into_owned();

        if resolved.exists() && resolved.is_file() {
            println!(
                "  {} '{}' es un archivo, no un directorio.",
                "✗".red(),
                resolved_str
            );
            continue;
        }

        if !resolved.exists() {
            // ¿El parent existe?
            let parent_ok = resolved
                .parent()
                .map(|p| p.exists())
                .unwrap_or(false);
            if !parent_ok {
                println!(
                    "  {} La carpeta padre no existe: {}",
                    "✗".red(),
                    resolved.parent().map(|p| p.display().to_string()).unwrap_or_default()
                );
                continue;
            }
            let create = Confirm::with_theme(&theme)
                .with_prompt(format!("'{}' no existe. ¿Crearla?", resolved_str))
                .default(true)
                .interact()
                .map_err(|e| e.to_string())?;
            if !create {
                continue;
            }
            if let Err(e) = std::fs::create_dir_all(&resolved) {
                println!("  {} No pude crearla: {}", "✗".red(), e);
                continue;
            }
        }

        break raw;
    };

    let path_resolved = resolve_path(&path_input);
    let path_str = path_resolved.to_string_lossy().into_owned();

    // Chequeo: ¿ya hay un manifest.yaml?
    let manifest_path = path_resolved.join(".nexenv").join("manifest.yaml");
    if manifest_path.exists() {
        println!(
            "  {} {}",
            "⚠".yellow(),
            format!("Ya existe .nexenv/manifest.yaml en {}", path_str).yellow()
        );
        let cont = Confirm::with_theme(&theme)
            .with_prompt("¿Continuar y sobrescribir?")
            .default(false)
            .interact()
            .map_err(|e| e.to_string())?;
        if !cont {
            println!("  {}", "Cancelado.".dimmed());
            return Ok(None);
        }
    }

    // --- Paso 3: tipo ---
    let types = ["api", "frontend", "fullstack", "cli", "desktop"];
    let type_idx = Select::with_theme(&theme)
        .with_prompt("Tipo de proyecto")
        .items(&types)
        .default(0)
        .max_length(10)
        .interact_opt()
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "__cancel__".to_string())?;
    let project_type = types[type_idx];

    // --- Paso 4: perfil ---
    let profiles = ["rapid", "standard", "production"];
    let profile_idx = Select::with_theme(&theme)
        .with_prompt("Perfil")
        .items(&profiles)
        .default(1) // standard
        .max_length(10)
        .interact_opt()
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "__cancel__".to_string())?;
    let profile = profiles[profile_idx];

    // --- Paso 5: tecnologias (seleccion inteligente segun tipo de proyecto) ---
    let mut techs: Vec<String> = Vec::new();

    match project_type {
        "frontend" => {
            pick_category(&mut techs, "Framework / librería frontend", &["frontend"], true)?;
            pick_category(&mut techs, "Bundler (opcional)", &["bundler"], false)?;
        }
        "api" => {
            pick_category(&mut techs, "Runtime", &["runtime"], true)?;
            pick_category(&mut techs, "Framework backend", &["backend"], false)?;
            pick_category(&mut techs, "Base de datos (opcional)", &["database"], false)?;
            pick_category(&mut techs, "Cache / queue (opcional)", &["cache", "queue"], false)?;
        }
        "fullstack" => {
            println!();
            println!("{}", "── Backend ──".bright_magenta());
            pick_category(&mut techs, "Runtime backend", &["runtime"], true)?;
            pick_category(&mut techs, "Framework backend", &["backend"], false)?;
            pick_category(&mut techs, "Base de datos", &["database"], false)?;
            pick_category(&mut techs, "Cache / queue (opcional)", &["cache", "queue"], false)?;
            println!();
            println!("{}", "── Frontend ──".bright_magenta());
            pick_category(&mut techs, "Framework frontend", &["frontend"], false)?;
        }
        "cli" => {
            pick_category(&mut techs, "Runtime", &["runtime", "cli"], true)?;
        }
        "desktop" => {
            pick_category(&mut techs, "Runtime", &["runtime"], true)?;
            pick_category(&mut techs, "Framework desktop", &["desktop", "frontend"], false)?;
        }
        _ => {
            pick_category(&mut techs, "Tecnologias", &["frontend", "backend", "runtime", "database"], false)?;
        }
    }

    // --- Validacion: incompatibilidades ---
    let conflicts = detect_incompatibilities(&techs);
    if !conflicts.is_empty() {
        println!();
        println!(
            "  {}",
            "⚠ Se detectaron incompatibilidades entre tecnologias:".yellow().bold()
        );
        for (a, b) in &conflicts {
            println!("    {} vs {}", a.bright_red(), b.bright_red());
        }
        println!();

        let opts = [
            "Cancelar wizard",
            "Continuar de todas formas (se reasignaran puertos)",
            "Volver a elegir tecnologias",
        ];
        let pick = Select::with_theme(&theme)
            .with_prompt("¿Que hago?")
            .items(&opts)
            .default(0)
            .max_length(5)
            .interact()
            .map_err(|e| e.to_string())?;

        match pick {
            0 => {
                println!("  {}", "Cancelado.".dimmed());
                return Ok(None);
            }
            2 => {
                // Quitar uno de los pares en conflicto y seguir.
                let drop_me: Vec<String> = conflicts.iter().map(|(_, b)| b.clone()).collect();
                techs.retain(|t| !drop_me.contains(t));
                println!(
                    "  {} {}",
                    "Removidos:".dimmed(),
                    drop_me.join(", ").bright_white()
                );
            }
            _ => {}
        }
    }

    // --- Paso 6: confirmar ---
    println!();
    println!("{}", "── Resumen ──".bright_magenta());
    println!("  {} {}", "Nombre:".dimmed(), name.bright_white());
    println!("  {} {}", "Ruta:".dimmed(), path_str.bright_white());
    println!("  {} {}", "Tipo:".dimmed(), project_type);
    println!("  {} {}", "Perfil:".dimmed(), profile);
    println!(
        "  {} {}",
        "Tecnologias:".dimmed(),
        if techs.is_empty() {
            "(ninguna)".to_string()
        } else {
            techs.join(", ")
        }
    );
    println!();

    let confirm = Confirm::with_theme(&theme)
        .with_prompt("¿Crear proyecto?")
        .default(true)
        .interact_opt()
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "__cancel__".to_string())?;

    if !confirm {
        println!("  {}", "Cancelado.".dimmed());
        return Ok(None);
    }

    // Construir argumentos para `nexenv-cli new`.
    let mut args = vec![
        "new".to_string(),
        name,
        "--path".to_string(),
        path_str,
        "--type".to_string(),
        project_type.to_string(),
        "--profile".to_string(),
        profile.to_string(),
    ];
    if !techs.is_empty() {
        args.push("--techs".to_string());
        args.push(techs.join(","));
    }
    Ok(Some(args))
}

fn resolve_path(input: &str) -> PathBuf {
    let p = PathBuf::from(input);
    if p.is_absolute() {
        return p;
    }
    match std::env::current_dir() {
        Ok(cwd) => cwd.join(p),
        Err(_) => p,
    }
}

/// Muestra un MultiSelect filtrado por categorias del catalogo y agrega los
/// ids elegidos a `techs`. Si `required` y no eligio nada, avisa.
fn pick_category(
    techs: &mut Vec<String>,
    prompt: &str,
    categories: &[&str],
    required: bool,
) -> Result<(), String> {
    let all: Vec<&catalog::Technology> = catalog::load_all_technologies()
        .iter()
        .filter(|t| categories.contains(&t.category.as_str()))
        .collect();
    if all.is_empty() {
        return Ok(());
    }
    let labels: Vec<String> = all
        .iter()
        .map(|t| format!("{:<18} [{}] {}", t.id, t.category, t.description))
        .collect();

    println!();
    println!(
        "  {} {}",
        "·".dimmed(),
        format!("{} (Space marcar · Enter confirmar · a marca todo)", prompt).dimmed()
    );

    let picks = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&labels)
        .max_length(12)
        .interact_opt()
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "__cancel__".to_string())?;

    if required && picks.is_empty() {
        println!(
            "  {} necesitas elegir al menos uno.",
            "⚠".yellow()
        );
        return pick_category(techs, prompt, categories, required);
    }

    for i in picks {
        let id = all[i].id.clone();
        if !techs.contains(&id) {
            techs.push(id);
        }
    }
    Ok(())
}

/// Heuristica: ¿el texto parece una ruta de archivo (no un comando)?
/// Usado para detectar errores tipo `!D:\path\...` y avisar al usuario.
fn looks_like_path(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }
    // Windows: `C:\`, `D:\`, `\\server\share`
    if s.len() >= 3 {
        let bytes = s.as_bytes();
        if bytes[1] == b':' && (bytes[2] == b'\\' || bytes[2] == b'/') {
            return true;
        }
    }
    // Unix: `/`, `~/`
    if s.starts_with('/') || s.starts_with("~/") {
        return true;
    }
    // Relativa: `./`, `../`
    if s.starts_with("./") || s.starts_with("../") || s.starts_with(".\\") || s.starts_with("..\\") {
        return true;
    }
    false
}

/// Ejecuta un comando en la shell nativa e imprime stdout/stderr inline.
/// Usado dentro del wizard para permitir `!ls` etc sin salir.
fn run_native_shell(cmd: &str) {
    if cmd.is_empty() {
        println!("  {} uso: !<comando>", "·".dimmed());
        return;
    }
    let (program, args): (&str, Vec<&str>) = if cfg!(target_os = "windows") {
        ("powershell", vec!["-NoProfile", "-Command", cmd])
    } else {
        ("sh", vec!["-c", cmd])
    };

    let status = std::process::Command::new(program)
        .args(&args)
        .status();
    if let Err(e) = status {
        eprintln!("  shell error: {}", e);
    }
}

/// Detecta pares incompatibles entre las tecnologias elegidas segun el catalogo.
fn detect_incompatibilities(techs: &[String]) -> Vec<(String, String)> {
    let mut conflicts = Vec::new();
    let catalog_all: Vec<&catalog::Technology> = catalog::load_all_technologies().iter().collect();
    for (i, a) in techs.iter().enumerate() {
        let Some(tech_a) = catalog_all.iter().find(|t| &t.id == a) else {
            continue;
        };
        for b in techs.iter().skip(i + 1) {
            if tech_a.incompatible_with.contains(b) {
                conflicts.push((a.clone(), b.clone()));
            }
        }
    }
    conflicts
}

/// Muestra hasta 10 entradas del directorio para ayudar a elegir la ruta.
fn print_dir_preview(path: &str) {
    let Ok(rd) = std::fs::read_dir(path) else {
        return;
    };
    let mut entries: Vec<String> = rd
        .flatten()
        .take(30)
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            let is_dir = e.file_type().map(|t| t.is_dir()).unwrap_or(false);
            if name.starts_with('.') {
                None
            } else if is_dir {
                Some(format!("{}/", name))
            } else {
                Some(name)
            }
        })
        .collect();
    entries.sort();
    entries.truncate(10);
    if entries.is_empty() {
        return;
    }
    let line = entries.join("  ");
    let clipped = if line.len() > 80 {
        format!("{}…", &line[..80])
    } else {
        line
    };
    println!("  {} {}", "Contenido:".dimmed(), clipped.dimmed());
}

