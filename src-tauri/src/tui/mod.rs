pub mod banner;
pub mod commands;
pub mod detect;
pub mod history;
pub mod menu;
pub mod wizard;

use crate::tui::detect::CwdContext;
use colored::Colorize;
use reedline::{
    default_emacs_keybindings, ColumnarMenu, Completer, EditCommand, Emacs,
    FileBackedHistory, KeyCode, KeyModifiers, MenuBuilder, Prompt, PromptEditMode,
    PromptHistorySearch, Reedline, ReedlineEvent, ReedlineMenu, Signal, Span,
    Suggestion as RLSuggestion,
};
use std::borrow::Cow;
use std::sync::{Arc, Mutex};

const SLASH_MENU: &str = "slash_menu";

/// Entry point del shell interactivo (reedline).
pub fn run() -> Result<(), String> {
    banner::print();
    println!();

    let ctx = Arc::new(Mutex::new(detect::detect_cwd()));
    print_intro(&ctx.lock().unwrap());

    // Historial persistente.
    let history: Box<dyn reedline::History> = match history::path() {
        Some(p) => match FileBackedHistory::with_file(500, p) {
            Ok(h) => Box::new(h),
            Err(_) => Box::new(FileBackedHistory::new(500).expect("history")),
        },
        None => Box::new(FileBackedHistory::new(500).expect("history")),
    };

    let completer = NexenvCompleter::new(ctx.clone());

    // Menu IDE-style filtrable que aparece al escribir `/`.
    let slash_menu = ColumnarMenu::default()
        .with_name(SLASH_MENU)
        .with_columns(1)
        .with_column_padding(2)
        .with_column_width(Some(60));

    // Bindings:
    //   `/`     inserta el caracter Y abre el menu live.
    //   Enter   si menu abierto: acepta item y submitte. Si no: submit normal.
    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Char('/'),
        ReedlineEvent::Multiple(vec![
            ReedlineEvent::Edit(vec![EditCommand::InsertChar('/')]),
            ReedlineEvent::Menu(SLASH_MENU.to_string()),
        ]),
    );
    // Enter dentro del menu: usar UntilFound para "primera accion que aplique".
    // Si el menu esta abierto, MenuNext/Submit no aplican; ReedlineEvent::Enter
    // acepta el item visible. Combinamos para que ejecute en una sola pulsacion.
    keybindings.add_binding(
        KeyModifiers::NONE,
        KeyCode::Enter,
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Multiple(vec![
                ReedlineEvent::Enter,    // dentro de menu = aceptar item
                ReedlineEvent::Submit,   // y submit la linea ya completada
            ]),
            ReedlineEvent::Submit,
        ]),
    );
    let edit_mode = Box::new(Emacs::new(keybindings));

    let mut line_editor = Reedline::create()
        .with_history(history)
        .with_completer(Box::new(completer))
        .with_menu(ReedlineMenu::EngineCompleter(Box::new(slash_menu)))
        .with_edit_mode(edit_mode);

    let prompt = NexenvPrompt::new(ctx.clone());

    // Tracking de Ctrl+C para salir con dos pulsaciones (como Claude Code).
    let mut ctrl_c_armed = false;
    // Si el ultimo input fue vacio o cancelado, NO reimprimimos el header.
    // Ademas borramos las 2 lineas previas (header anterior + linea del prompt
    // anterior) para que la pantalla no acumule prompts vacios.
    let mut redraw_header = true;

    loop {
        if redraw_header {
            print_prompt_header(&ctx.lock().unwrap());
        } else {
            // Borrar 2 lineas hacia arriba (header + linea del prompt anterior).
            // Asi el siguiente prompt sobreescribe el sitio que ocupaba el ultimo.
            print!("\x1b[2A\x1b[2K\x1b[B\x1b[2K\x1b[A");
            use std::io::Write;
            std::io::stdout().flush().ok();
            // Ahora reimprimimos el header en el mismo lugar (limpiado) para que
            // visualmente quede igual que antes pero sin acumular.
            print_prompt_header(&ctx.lock().unwrap());
        }

        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(line)) => {
                ctrl_c_armed = false;
                let raw = line.trim().to_string();
                if raw.is_empty() {
                    // Enter sin texto: NO reimprimir header como bloque nuevo.
                    redraw_header = false;
                    continue;
                }
                if is_exit(&raw) {
                    break;
                }

                let cmd = raw.strip_prefix('/').unwrap_or(&raw).trim();

                {
                    let mut c = ctx.lock().unwrap();
                    dispatch(cmd, &mut c)?;
                }
                refresh_completer(&mut line_editor, ctx.clone());
                redraw_header = true;
            }
            Ok(Signal::CtrlC) => {
                if ctrl_c_armed {
                    println!("  {}", "Saliendo del shell.".dimmed());
                    break;
                }
                ctrl_c_armed = true;
                println!(
                    "  {}",
                    "(Ctrl+C de nuevo para salir, o sigue escribiendo)".dimmed()
                );
                redraw_header = true;
                continue;
            }
            Ok(Signal::CtrlD) => break,
            Err(e) => return Err(format!("readline: {}", e)),
        }
    }

    println!();
    println!("{}", "Hasta luego.".dimmed());
    Ok(())
}

fn refresh_completer(line_editor: &mut Reedline, ctx: Arc<Mutex<CwdContext>>) {
    // Reemplazar el completer para que tome el ctx actualizado y la lista de proyectos.
    let new_completer = NexenvCompleter::new(ctx);
    *line_editor = std::mem::replace(line_editor, Reedline::create())
        .with_completer(Box::new(new_completer));
}

fn is_exit(line: &str) -> bool {
    matches!(line, "exit" | "quit" | "q" | ":q")
}

fn print_intro(ctx: &CwdContext) {
    match ctx {
        CwdContext::KnownProject(p) => {
            println!(
                "  {} {} {}",
                "Proyecto activo:".bright_green().bold(),
                p.name.bright_cyan().bold(),
                format!("({})", crate::core::utils::fs::pretty_path(&p.path)).dimmed()
            );
            if !p.runtimes.is_empty() {
                let rts: Vec<String> = p.runtimes.iter().map(|r| r.runtime.clone()).collect();
                println!("  {} {}", "Runtimes:".dimmed(), rts.join(", "));
            }
        }
        CwdContext::UnregisteredCandidate { path, signals } => {
            println!(
                "  {} {} {}",
                "Directorio:".dimmed(),
                path.bright_yellow(),
                format!("[{}] sin registrar", signals.join(", ")).dimmed()
            );
        }
        CwdContext::Unknown { path } => {
            println!("  {} {}", "Directorio:".dimmed(), path.dimmed());
            println!("  {}", "No es un proyecto registrado.".dimmed());
        }
    }
    println!();
    println!(
        "  {} escribe {} para ver todos los comandos · {} ayuda · {} wizard",
        "Tip:".dimmed(),
        "/".bright_white(),
        "help".bright_white(),
        "new".bright_white()
    );
    println!(
        "       {} cambiar carpeta · {} shell OS · {} cancelar · {} salir",
        "cd".bright_white(),
        "!cmd".bright_white(),
        "Ctrl+C".bright_white(),
        "Ctrl+D".bright_white()
    );
    println!();
}

fn load_project_names() -> Vec<String> {
    crate::core::store::get()
        .list_projects()
        .map(|ps| ps.into_iter().map(|p| p.name).collect())
        .unwrap_or_default()
}

// ─── Dispatch de comandos ─────────────────────────────────────────────

fn dispatch(line: &str, ctx: &mut CwdContext) -> Result<(), String> {
    match line.trim() {
        "help" | "?" => {
            print_help();
            return Ok(());
        }
        "menu" => {
            if let Err(e) = menu::run_menu_once(ctx) {
                eprintln!("{} {}", "menu error:".red(), e);
            }
            *ctx = detect::detect_cwd();
            return Ok(());
        }
        "clear" | "cls" => {
            print!("\x1B[2J\x1B[1;1H");
            use std::io::Write;
            std::io::stdout().flush().ok();
            return Ok(());
        }
        "context" | "where" | "pwd" => {
            print_intro(ctx);
            return Ok(());
        }
        "new" => {
            run_wizard_new(ctx);
            return Ok(());
        }
        _ => {}
    }

    if let Some(cmd) = line.strip_prefix('!') {
        run_native_shell(cmd.trim());
        return Ok(());
    }

    if line == "cd" || line.starts_with("cd ") {
        handle_cd(line, ctx);
        return Ok(());
    }

    run_nexenv_subcommand(line);
    Ok(())
}

fn run_wizard_new(ctx: &mut CwdContext) {
    match wizard::new_project() {
        Ok(Some(args)) => {
            println!();
            println!(
                "  {} nexenv {}",
                "→".bright_magenta(),
                args.join(" ").dimmed()
            );
            let exe = match std::env::current_exe() {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("current_exe: {}", e);
                    return;
                }
            };
            let _ = std::process::Command::new(&exe).args(&args).status();
        }
        Ok(None) => {
            println!("  {}", "Wizard cancelado.".dimmed());
        }
        Err(e) if e == "__cancel__" => {
            println!("  {}", "Wizard cancelado (Esc/Ctrl+C).".dimmed());
        }
        Err(e) => {
            eprintln!("{} {}", "wizard error:".red(), e);
        }
    }
    *ctx = detect::detect_cwd();
}

fn run_native_shell(cmd: &str) {
    if cmd.is_empty() {
        eprintln!("  {} uso: !<comando> (ej: !ls, !pwd)", "·".dimmed());
        return;
    }
    let (program, args): (&str, Vec<&str>) = if cfg!(target_os = "windows") {
        ("powershell", vec!["-NoProfile", "-Command", cmd])
    } else {
        ("sh", vec!["-c", cmd])
    };
    let _ = std::process::Command::new(program).args(&args).status();
}

fn run_nexenv_subcommand(line: &str) {
    let parts = match shell_words::split(line) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{} parse: {}", "error:".red(), e);
            return;
        }
    };
    if parts.is_empty() {
        return;
    }
    if parts[0] == "shell" {
        eprintln!(
            "{} no puedes invocar 'shell' dentro del shell",
            "error:".red()
        );
        return;
    }
    let exe = match std::env::current_exe() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("current_exe: {}", e);
            return;
        }
    };
    let _ = std::process::Command::new(&exe).args(&parts).status();
}

fn handle_cd(line: &str, ctx: &mut CwdContext) {
    let rest = line.strip_prefix("cd").unwrap_or("").trim();
    let target: std::path::PathBuf = if rest.is_empty() {
        match dirs::home_dir() {
            Some(h) => h,
            None => {
                eprintln!("{} no se pudo determinar HOME", "error:".red());
                return;
            }
        }
    } else {
        let p = std::path::PathBuf::from(rest);
        if p.is_absolute() {
            p
        } else {
            match std::env::current_dir() {
                Ok(cwd) => cwd.join(p),
                Err(e) => {
                    eprintln!("current_dir: {}", e);
                    return;
                }
            }
        }
    };

    if let Err(e) = std::env::set_current_dir(&target) {
        eprintln!("{} cd: {}", "error:".red(), e);
        return;
    }

    *ctx = detect::detect_cwd();
    println!(
        "  {} {}",
        "Ahora en:".dimmed(),
        crate::core::utils::fs::pretty_path(&target.to_string_lossy()).bright_white()
    );
}

fn print_help() {
    let _ = render_help_interactive();
}

/// Vista de ayuda INLINE (debajo del prompt, no full-screen).
/// Tab/Shift+Tab o ←→ cambian de categoria. Esc o q salen.
/// Al cambiar de tab subimos el cursor N lineas, limpiamos hacia abajo y
/// redibujamos — asi el help "vive" en su lugar sin tapar el output anterior.
fn render_help_interactive() -> Result<(), String> {
    use crossterm::event::{self, Event, KeyCode, KeyEventKind};
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
    use std::io::{stdout, Write};

    let tabs = help_tabs();
    let mut active: usize = 0;

    enable_raw_mode().map_err(|e| format!("raw_mode: {}", e))?;

    println!();
    // Dibujamos el primer bloque y guardamos cuantas lineas ocupo.
    let mut last_lines = render_help_block(&tabs, active);
    stdout().flush().ok();

    let result: Result<(), String> = loop {
        match event::read().map_err(|e| format!("read: {}", e))? {
            Event::Key(k) if k.kind == KeyEventKind::Press => match k.code {
                KeyCode::Tab | KeyCode::Right => {
                    active = (active + 1) % tabs.len();
                }
                KeyCode::BackTab | KeyCode::Left => {
                    active = if active == 0 { tabs.len() - 1 } else { active - 1 };
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    let n = c.to_digit(10).unwrap() as usize;
                    if n >= 1 && n <= tabs.len() {
                        active = n - 1;
                    } else {
                        continue;
                    }
                }
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter => break Ok(()),
                _ => continue,
            },
            _ => continue,
        }

        // Subir last_lines lineas y limpiar todo lo de abajo, luego redibujar.
        if last_lines > 0 {
            print!("\x1b[{}A", last_lines);
        }
        print!("\x1b[J");
        last_lines = render_help_block(&tabs, active);
        stdout().flush().ok();
    };

    let _ = disable_raw_mode();
    result
}

/// Imprime el bloque de help y devuelve cuantas lineas (visuales) ocupo.
fn render_help_block(tabs: &[HelpTab], active: usize) -> u16 {
    let width = terminal_width().min(120);
    let bar = "─".repeat(width as usize);
    let mut count: u16 = 0;

    let print_line = |s: &str, c: &mut u16| {
        println!("{}", s);
        *c += 1;
    };
    let blank = |c: &mut u16| {
        println!();
        *c += 1;
    };

    print_line(&bar.bright_blue().to_string(), &mut count);

    let mut header = String::from(" ");
    header.push_str(&format!(
        "{} ",
        format!("Nexenv v{}", env!("CARGO_PKG_VERSION"))
            .bright_white()
            .bold()
    ));
    for (i, tab) in tabs.iter().enumerate() {
        if i == active {
            header.push_str(&format!(
                " {} ",
                format!(" {} ", tab.title).bright_white().on_blue().bold()
            ));
        } else {
            header.push_str(&format!("  {}  ", tab.title.dimmed()));
        }
    }
    print_line(&header, &mut count);
    print_line(&bar.bright_blue().to_string(), &mut count);
    blank(&mut count);

    let tab = &tabs[active];
    for line in tab.intro.split('\n') {
        print_line(&format!(" {}", line), &mut count);
    }
    blank(&mut count);

    for section in &tab.sections {
        print_line(
            &format!(" {}", section.title.bright_white().bold().underline()),
            &mut count,
        );
        for (cmd, desc) in &section.items {
            if desc.is_empty() {
                print_line(&format!("   {}", cmd.bright_cyan()), &mut count);
            } else {
                print_line(
                    &format!("   {:<28} {}", cmd.bright_cyan(), desc.dimmed()),
                    &mut count,
                );
            }
        }
        blank(&mut count);
    }

    print_line(&bar.bright_blue().to_string(), &mut count);
    print_line(
        &format!(
            " {} {} {}  {} {}  {} {}",
            "←→/Tab".bright_yellow(),
            "cambiar tab".dimmed(),
            "·".dimmed(),
            "1-9".bright_yellow(),
            "ir directo".dimmed(),
            "Esc/q/Enter".bright_yellow(),
            "salir".dimmed()
        ),
        &mut count,
    );

    count
}

struct HelpTab {
    title: &'static str,
    intro: &'static str,
    sections: Vec<HelpSection>,
}

struct HelpSection {
    title: &'static str,
    items: Vec<(&'static str, &'static str)>,
}

fn help_tabs() -> Vec<HelpTab> {
    vec![
        HelpTab {
            title: "general",
            intro: "Nexenv gestiona tus proyectos de desarrollo: scaffolding, env vars,\ndocker, snapshots, health-checks. Todo desde el terminal.",
            sections: vec![
                HelpSection {
                    title: "Empieza aqui",
                    items: vec![
                        ("/", "abre el menu live de comandos"),
                        ("list", "ver tus proyectos registrados"),
                        ("new", "wizard para crear un proyecto"),
                        ("doctor", "verifica que tienes lo necesario"),
                        ("menu", "menu navegable jerarquico"),
                        ("help", "esta vista de ayuda"),
                    ],
                },
                HelpSection {
                    title: "Atajos rapidos",
                    items: vec![
                        ("/ for commands", "menu live filtrable"),
                        ("! for shell OS", "!ls, !pwd, !git, ..."),
                        ("cd <ruta>", "cambia carpeta y refresca contexto"),
                        ("Ctrl+C", "cancela / corta proceso"),
                        ("Ctrl+D", "sale del shell"),
                        ("Esc", "cierra menus o vistas"),
                    ],
                },
            ],
        },
        HelpTab {
            title: "comandos",
            intro: "Todos los comandos de Nexenv. Escribelos directamente o usa / para buscarlos.",
            sections: vec![
                HelpSection {
                    title: "Proyectos",
                    items: vec![
                        ("list", "lista todos los proyectos registrados"),
                        ("open <proyecto>", "abre proyecto en el editor"),
                        ("scan <ruta>", "detecta el stack de un proyecto"),
                        ("new", "wizard interactivo para crear proyecto"),
                        ("status <proyecto>", "estado git"),
                        ("health <proyecto>", "health checks"),
                        ("manifest <proyecto>", "ver el manifest del proyecto"),
                        ("unlink <proyecto>", "desvincular (no borra archivos)"),
                    ],
                },
                HelpSection {
                    title: "Sistema y catalogo",
                    items: vec![
                        ("doctor", "verifica el estado del sistema"),
                        ("ports", "puertos en uso por proyectos"),
                        ("catalog", "catalogo de tecnologias soportadas"),
                        ("recipes", "lista de recipes disponibles"),
                        ("validate <techs>", "valida combinacion de techs"),
                    ],
                },
                HelpSection {
                    title: "Datos del proyecto",
                    items: vec![
                        ("env <proyecto>", "gestiona variables de entorno"),
                        ("snapshot save <proyecto>", "guarda snapshot del manifest"),
                        ("snapshot list <proyecto>", "lista snapshots"),
                        ("diff <proyecto>", "diff de env desde ultimo snapshot"),
                        ("note <proyecto>", "notas del proyecto"),
                        ("ps --project <p>", "procesos en puertos del proyecto"),
                        ("export <proyecto>", "exporta como .nexenv"),
                        ("import <archivo>", "importa proyecto de .nexenv"),
                        ("add <recipe> <proyecto>", "aplica recipe al proyecto"),
                    ],
                },
                HelpSection {
                    title: "Docker",
                    items: vec![
                        ("docker status <proyecto>", "estado compose"),
                        ("docker up <proyecto>", "levanta servicios"),
                        ("docker down <proyecto>", "detiene servicios"),
                        ("docker logs <proyecto>", "ver logs"),
                    ],
                },
            ],
        },
        HelpTab {
            title: "teclas",
            intro: "Atajos de teclado del shell.",
            sections: vec![
                HelpSection {
                    title: "Edicion",
                    items: vec![
                        ("Tab", "autocomplete / abre menu sugerencias"),
                        ("↑↓", "navega menu o historial"),
                        ("←→", "mueve cursor en linea"),
                        ("Home / End", "inicio / fin de linea"),
                        ("Backspace", "borra char anterior"),
                        ("Ctrl+W", "borra palabra anterior"),
                        ("Ctrl+U", "borra hasta inicio de linea"),
                    ],
                },
                HelpSection {
                    title: "Control",
                    items: vec![
                        ("Enter", "ejecuta el comando o item del menu"),
                        ("Esc", "cierra menu activo o vista"),
                        ("Ctrl+C", "cancela linea (2 veces sale)"),
                        ("Ctrl+D", "sale del shell"),
                        ("Ctrl+L", "limpia pantalla"),
                        ("Ctrl+R", "busqueda en historial"),
                    ],
                },
                HelpSection {
                    title: "Sintaxis especial",
                    items: vec![
                        ("/", "abre el menu live filtrable"),
                        ("!<comando>", "ejecuta en PowerShell/bash real"),
                        ("cd <ruta>", "cambia directorio actual"),
                    ],
                },
            ],
        },
    ]
}

fn terminal_width() -> u16 {
    use std::io::IsTerminal;
    if !std::io::stdout().is_terminal() {
        return 80;
    }
    crossterm::terminal::size().map(|(w, _)| w).unwrap_or(80)
}

// ─── Prompt custom ─────────────────────────────────────────────────────

struct NexenvPrompt;

impl NexenvPrompt {
    fn new(_ctx: Arc<Mutex<CwdContext>>) -> Self {
        Self
    }
}

impl Prompt for NexenvPrompt {
    fn render_prompt_left(&self) -> Cow<'_, str> {
        // Solo `╰─` — el header `╭─ contexto` se imprime manualmente antes del
        // read_line para poder controlarlo (Enter vacio NO reimprime header).
        Cow::Borrowed("╰─")
    }

    fn render_prompt_right(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, _mode: PromptEditMode) -> Cow<'_, str> {
        Cow::Borrowed(" > ")
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
        Cow::Borrowed("│  ")
    }

    fn render_prompt_history_search_indicator(
        &self,
        _history_search: PromptHistorySearch,
    ) -> Cow<'_, str> {
        Cow::Borrowed("(buscar) ")
    }
}

/// Imprime el header del prompt (linea `╭─ contexto`) manualmente antes de
/// llamar a read_line. Asi controlamos cuando se reimprime.
fn print_prompt_header(ctx: &CwdContext) {
    let header = match ctx {
        CwdContext::KnownProject(p) => format!(
            "╭─ {}  {}  {}",
            "●".bright_green().bold(),
            format!("nexenv({})", p.name).bright_cyan().bold(),
            crate::core::utils::fs::pretty_path(&p.path).dimmed()
        ),
        CwdContext::UnregisteredCandidate { path, .. } => format!(
            "╭─ {}  {}  {}",
            "○".bright_yellow(),
            "nexenv".bright_magenta().bold(),
            format!("{}  (sin registrar)", path).dimmed()
        ),
        CwdContext::Unknown { path } => format!(
            "╭─ {}  {}",
            "nexenv".bright_magenta().bold(),
            path.dimmed()
        ),
    };
    println!("{}", header);
}

// ─── Completer custom ───────────────────────────────────────────────────

struct NexenvCompleter {
    ctx: Arc<Mutex<CwdContext>>,
    project_names: Vec<String>,
}

impl NexenvCompleter {
    fn new(ctx: Arc<Mutex<CwdContext>>) -> Self {
        Self {
            project_names: load_project_names(),
            ctx,
        }
    }
}

impl Completer for NexenvCompleter {
    fn complete(&mut self, line: &str, pos: usize) -> Vec<RLSuggestion> {
        let bound = pos.min(line.len());
        let slice = &line[..bound];

        // Si empieza con `/`, filtramos lo que viene despues del slash.
        let (start_offset, query) = if let Some(rest) = slice.strip_prefix('/') {
            (1usize, rest)
        } else {
            (0usize, slice)
        };

        let in_project = matches!(*self.ctx.lock().unwrap(), CwdContext::KnownProject(_));
        let suggestions = commands::filter(query, &self.project_names, in_project);

        suggestions
            .into_iter()
            .map(|s| RLSuggestion {
                value: s.command.clone(),
                description: Some(s.description),
                extra: None,
                span: Span::new(start_offset, bound),
                append_whitespace: false,
                style: None,
            })
            .collect()
    }
}
