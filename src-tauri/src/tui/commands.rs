/// Comandos mostrados como sugerencias cuando el input esta vacio
/// y NO hay un proyecto activo.
pub const TOP_COMMANDS: &[(&str, &str)] = &[
    ("menu", "Menu interactivo completo"),
    ("help", "Ayuda"),
    ("list", "Listar proyectos"),
    ("doctor", "Estado del sistema"),
    ("new", "Crear proyecto desde scaffold"),
    ("open", "Abrir proyecto en editor"),
    ("status", "Git status del proyecto"),
];

/// Acciones mostradas cuando estamos DENTRO de un proyecto registrado.
/// Son las "tarjetas" equivalentes en la GUI.
pub const PROJECT_ACTIONS: &[(&str, &str)] = &[
    ("menu", "Menu interactivo del proyecto"),
    ("status", "Estado git del proyecto"),
    ("health", "Health checks (puertos, servicios)"),
    ("doctor", "Estado del sistema"),
    ("manifest", "Ver manifest del proyecto"),
    ("env", "Gestionar variables de entorno"),
    ("docker", "Docker compose (up/down/status/logs)"),
    ("snapshot", "Snapshots (guardar/listar/restaurar)"),
    ("note", "Notas del proyecto"),
    ("ps", "Procesos en puertos del proyecto"),
    ("run", "Ejecutar script del manifest"),
    ("diff", "Diff de env desde ultimo snapshot"),
    ("add", "Aplicar recipe al proyecto"),
    ("export", "Exportar como .nexenv"),
    ("unlink", "Desvincular (no borra archivos)"),
    ("help", "Ayuda del shell"),
    ("list", "Listar todos los proyectos"),
];

/// Catalogo completo de subcomandos disponibles (orden alfabetico para filtrado).
pub const ALL_COMMANDS: &[(&str, &str)] = &[
    ("add", "Aplica una recipe a un proyecto"),
    ("catalog", "Navega el catalogo de tecnologias"),
    ("cd", "Cambia el directorio actual"),
    ("clear", "Limpia la pantalla"),
    ("context", "Muestra el contexto del directorio actual"),
    ("diff", "Muestra cambios de entorno desde el ultimo snapshot"),
    ("docker", "Docker Compose management"),
    ("doctor", "Verifica el estado del sistema"),
    ("env", "Gestiona variables de entorno de un proyecto"),
    ("exit", "Sale del shell"),
    ("export", "Exporta un proyecto como archivo .nexenv"),
    ("health", "Ejecuta health checks para un proyecto"),
    ("help", "Muestra la ayuda del shell"),
    ("import", "Importa un proyecto desde archivo .nexenv"),
    ("list", "Lista todos los proyectos registrados"),
    ("manifest", "Muestra el manifest de un proyecto"),
    ("menu", "Abre el menu navegable"),
    ("new", "Genera un proyecto desde scaffold"),
    ("note", "Gestiona notas de un proyecto"),
    ("open", "Abre un proyecto en el editor configurado"),
    ("ports", "Muestra puertos en uso por proyectos"),
    ("ps", "Lista procesos en puertos del proyecto"),
    ("quit", "Sale del shell"),
    ("recipes", "Lista recipes disponibles"),
    ("run", "Ejecuta un script del manifest"),
    ("scan", "Detecta el stack de un proyecto existente"),
    ("snapshot", "Guarda/lista/restaura snapshots del manifest"),
    ("status", "Muestra estado Git del proyecto"),
    ("unlink", "Desvincula un proyecto"),
    ("validate", "Valida una combinacion de tecnologias"),
];

/// Comandos que aceptan nombre de proyecto como primer argumento.
/// Se usa para sugerir nombres de proyectos tras el comando.
pub const PROJECT_ARG_COMMANDS: &[&str] = &[
    "open", "status", "health", "manifest", "env", "export", "unlink",
    "add", "note", "diff", "run", "ps",
];

/// Devuelve sugerencias filtradas segun el input actual.
///
/// - Input vacio → `TOP_COMMANDS` (los mas usados).
/// - Input con texto pero sin espacio → filtra `ALL_COMMANDS` por prefijo + substring.
/// - Input con espacio (argumento): caller extiende con project names si aplica.
pub fn filter(input: &str, project_names: &[String], in_project: bool) -> Vec<Suggestion> {
    let trimmed = input.trim_start();

    // Input vacio: acciones del proyecto si hay uno activo, si no top commands.
    if trimmed.is_empty() {
        let source = if in_project { PROJECT_ACTIONS } else { TOP_COMMANDS };
        return source
            .iter()
            .map(|(cmd, desc)| Suggestion::new(cmd, desc))
            .collect();
    }

    // Si tiene espacio: estamos completando argumentos.
    if let Some((cmd, rest)) = trimmed.split_once(' ') {
        if PROJECT_ARG_COMMANDS.contains(&cmd) {
            let rest = rest.trim_start();
            return project_names
                .iter()
                .filter(|p| rest.is_empty() || p.to_lowercase().contains(&rest.to_lowercase()))
                .map(|p| Suggestion::new(p, "proyecto"))
                .collect();
        }
        return Vec::new();
    }

    // Solo comando parcial: filtrar por prefijo primero, luego substring.
    let lower = trimmed.to_lowercase();
    let mut prefix: Vec<Suggestion> = Vec::new();
    let mut substr: Vec<Suggestion> = Vec::new();

    for (cmd, desc) in ALL_COMMANDS {
        let c = cmd.to_lowercase();
        if c.starts_with(&lower) {
            prefix.push(Suggestion::new(cmd, desc));
        } else if c.contains(&lower) {
            substr.push(Suggestion::new(cmd, desc));
        }
    }

    prefix.extend(substr);
    prefix
}

#[derive(Debug, Clone)]
pub struct Suggestion {
    pub command: String,
    pub description: String,
}

impl Suggestion {
    fn new(cmd: &str, desc: &str) -> Self {
        Self {
            command: cmd.to_string(),
            description: desc.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_returns_top_commands_when_not_in_project() {
        let s = filter("", &[], false);
        assert!(!s.is_empty());
        assert_eq!(s[0].command, "menu");
    }

    #[test]
    fn empty_input_returns_project_actions_when_in_project() {
        let s = filter("", &[], true);
        assert!(!s.is_empty());
        let ids: Vec<&str> = s.iter().map(|x| x.command.as_str()).collect();
        assert!(ids.contains(&"status"));
        assert!(ids.contains(&"health"));
        assert!(ids.contains(&"env"));
        assert!(ids.contains(&"docker"));
    }

    #[test]
    fn prefix_match_comes_first() {
        let s = filter("lis", &[], false);
        assert!(!s.is_empty());
        assert_eq!(s[0].command, "list");
    }

    #[test]
    fn project_args_suggest_names() {
        let projects = vec!["foo".to_string(), "bar".to_string()];
        let s = filter("open ", &projects, false);
        assert_eq!(s.len(), 2);
    }
}
