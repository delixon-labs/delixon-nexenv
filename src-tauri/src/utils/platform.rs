/// Devuelve la ruta base de datos de Delixon según el SO
pub fn get_data_dir() -> Option<std::path::PathBuf> {
    dirs::data_local_dir().map(|p| p.join("delixon"))
}
