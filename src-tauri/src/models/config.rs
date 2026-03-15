use serde::{Deserialize, Serialize};

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
            version: "0.1.0".to_string(),
            data_dir: String::new(),
            default_editor: "code".to_string(),
            theme: "dark".to_string(),
            language: "es".to_string(),
            auto_check_updates: true,
        }
    }
}
