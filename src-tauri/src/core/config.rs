use serde::{Deserialize, Serialize};

use crate::core::error::DelixonError;
use crate::core::storage;
use crate::core::utils::platform::get_data_dir;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DelixonConfig {
    pub version: String,
    pub data_dir: String,
    pub default_editor: String,
    pub theme: String,
    pub language: String,
    pub auto_check_updates: bool,
}

impl Default for DelixonConfig {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            data_dir: String::new(),
            default_editor: "code".to_string(),
            theme: "dark".to_string(),
            language: "es".to_string(),
            auto_check_updates: true,
        }
    }
}

fn config_file() -> Result<std::path::PathBuf, DelixonError> {
    let dir = get_data_dir().ok_or_else(|| {
        DelixonError::InvalidConfig("No se pudo determinar el directorio de datos".to_string())
    })?;
    Ok(dir.join("config.json"))
}

pub fn load_config() -> Result<DelixonConfig, DelixonError> {
    storage::init_data_dir()?;
    let path = config_file()?;
    if !path.exists() {
        let config = DelixonConfig::default();
        save_config(&config)?;
        return Ok(config);
    }
    let data = std::fs::read_to_string(&path)?;
    let config: DelixonConfig = serde_json::from_str(&data)?;
    Ok(config)
}

pub fn save_config(config: &DelixonConfig) -> Result<(), DelixonError> {
    storage::init_data_dir()?;
    let path = config_file()?;
    let data = serde_json::to_string_pretty(config)?;

    std::fs::write(&path, &data)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(&path, perms)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DelixonConfig::default();
        assert_eq!(config.default_editor, "code");
        assert_eq!(config.theme, "dark");
        assert_eq!(config.language, "es");
    }

    #[test]
    fn test_config_serialization() {
        let config = DelixonConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let parsed: DelixonConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.default_editor, config.default_editor);
        assert_eq!(parsed.theme, config.theme);
    }
}
