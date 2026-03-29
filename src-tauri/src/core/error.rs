use thiserror::Error;

#[derive(Debug, Error)]
pub enum DelixonError {
    #[error("Proyecto no encontrado: {0}")]
    ProjectNotFound(String),

    #[error("Ruta invalida: {0}")]
    InvalidPath(String),

    #[error("Template no encontrado: {0}")]
    TemplateNotFound(String),

    #[error("Configuracion invalida: {0}")]
    InvalidConfig(String),

    #[error("Manifest invalido: {0}")]
    InvalidManifest(String),

    #[error("Error de IO: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error de serializacion: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Error de base de datos: {0}")]
    Database(String),
}

impl From<DelixonError> for String {
    fn from(e: DelixonError) -> String {
        e.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_not_found_display() {
        let err = DelixonError::ProjectNotFound("abc-123".to_string());
        let msg = err.to_string();
        assert!(msg.contains("abc-123"), "display should contain the id");
    }

    #[test]
    fn test_invalid_path_display() {
        let err = DelixonError::InvalidPath("/bad/path".to_string());
        let msg = err.to_string();
        assert!(msg.contains("/bad/path"), "display should contain the path");
    }

    #[test]
    fn test_template_not_found_display() {
        let err = DelixonError::TemplateNotFound("my-template".to_string());
        let msg = err.to_string();
        assert!(msg.contains("my-template"), "display should contain template name");
    }

    #[test]
    fn test_invalid_config_display() {
        let err = DelixonError::InvalidConfig("missing field".to_string());
        let msg = err.to_string();
        assert!(msg.contains("missing field"), "display should contain the message");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file gone");
        let delixon_err: DelixonError = io_err.into();
        match delixon_err {
            DelixonError::Io(_) => {} // expected
            other => panic!("expected Io variant, got: {:?}", other),
        }
    }

    #[test]
    fn test_from_serde_error() {
        let serde_err = serde_json::from_str::<String>("not valid json").unwrap_err();
        let delixon_err: DelixonError = serde_err.into();
        match delixon_err {
            DelixonError::Serialization(_) => {} // expected
            other => panic!("expected Serialization variant, got: {:?}", other),
        }
    }

    #[test]
    fn test_into_string() {
        let err = DelixonError::ProjectNotFound("xyz".to_string());
        let s: String = err.into();
        assert!(!s.is_empty(), "string conversion should produce non-empty string");
        assert!(s.contains("xyz"));
    }
}
