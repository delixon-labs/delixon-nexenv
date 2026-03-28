use super::TemplateInfo;

/// Registro central de todas las plantillas disponibles.
/// Para agregar una nueva: crear su modulo .rs y añadir una entrada aqui.
pub fn all_templates() -> Vec<TemplateInfo> {
    vec![
        super::node_express::info(),
        super::react_vite::info(),
        super::python_fastapi::info(),
        super::python_django::info(),
        super::fullstack_react_python::info(),
        super::rust_cli::info(),
        super::docker_compose::info(),
    ]
}
