use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

const VALIDATION_URL: &str = "https://app.delixon.dev/api/store/license/validate/";

// Solo alfanumerico, guion y underscore, 8-64 chars.
static KEY_FORMAT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[A-Za-z0-9_-]{8,64}$").expect("regex invariante"));

#[derive(Debug, thiserror::Error)]
pub enum LicenseError {
    #[error("Formato de clave invalido")]
    InvalidFormat,
    #[error("Error de red: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Servidor rechazo la licencia (status {0})")]
    Server(reqwest::StatusCode),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LicenseInfo {
    #[serde(default)]
    pub valid: bool,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
}

fn license_key_path() -> PathBuf {
    let home = dirs::home_dir().expect("No se encontro el directorio home");
    home.join(".nexenv").join("license.key")
}

fn read_license_key() -> Option<String> {
    let path = license_key_path();
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

pub fn validate_key_format(key: &str) -> Result<(), LicenseError> {
    if KEY_FORMAT.is_match(key) {
        Ok(())
    } else {
        Err(LicenseError::InvalidFormat)
    }
}

pub async fn validate_license(key: &str) -> Result<LicenseInfo, LicenseError> {
    validate_key_format(key)?;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent(concat!("Nexenv/", env!("CARGO_PKG_VERSION")))
        .build()?;

    let response = client
        .post(VALIDATION_URL)
        .json(&serde_json::json!({ "key": key }))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(LicenseError::Server(response.status()));
    }

    let info: LicenseInfo = response.json().await?;
    Ok(info)
}

pub fn is_enterprise() -> bool {
    let key = match read_license_key() {
        Some(k) if !k.is_empty() => k,
        _ => return false,
    };

    let rt = tokio::runtime::Handle::try_current();
    let result = match rt {
        Ok(handle) => tokio::task::block_in_place(|| handle.block_on(validate_license(&key))),
        Err(_) => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(validate_license(&key))
        }
    };

    matches!(result, Ok(info) if info.valid)
}

pub fn activate_license(key: &str) -> Result<(), String> {
    validate_key_format(key).map_err(|e| e.to_string())?;
    let path = license_key_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Error creando directorio: {}", e))?;
    }
    fs::write(&path, key).map_err(|e| format!("Error guardando licencia: {}", e))?;
    Ok(())
}

pub fn deactivate_license() -> Result<(), String> {
    let path = license_key_path();
    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Error eliminando licencia: {}", e))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_format_rejects_empty() {
        assert!(matches!(
            validate_key_format(""),
            Err(LicenseError::InvalidFormat)
        ));
    }

    #[test]
    fn key_format_rejects_too_short() {
        assert!(matches!(
            validate_key_format("ABC123"),
            Err(LicenseError::InvalidFormat)
        ));
    }

    #[test]
    fn key_format_rejects_too_long() {
        let long = "a".repeat(65);
        assert!(matches!(
            validate_key_format(&long),
            Err(LicenseError::InvalidFormat)
        ));
    }

    #[test]
    fn key_format_rejects_sql_injection() {
        assert!(matches!(
            validate_key_format("FAKE' OR '1'='1"),
            Err(LicenseError::InvalidFormat)
        ));
    }

    #[test]
    fn key_format_rejects_query_string_injection() {
        assert!(matches!(
            validate_key_format("FAKE&admin=true"),
            Err(LicenseError::InvalidFormat)
        ));
    }

    #[test]
    fn key_format_rejects_control_chars() {
        assert!(matches!(
            validate_key_format("ABC\nDEF12345"),
            Err(LicenseError::InvalidFormat)
        ));
    }

    #[test]
    fn key_format_accepts_valid() {
        assert!(validate_key_format("abc-DEF-1234567").is_ok());
        assert!(validate_key_format("LICENSE_KEY_2026").is_ok());
        assert!(validate_key_format("12345678").is_ok());
        assert!(validate_key_format(&"A".repeat(64)).is_ok());
    }

    #[test]
    fn activate_license_rejects_invalid_format() {
        assert!(activate_license("bad key!").is_err());
        assert!(activate_license("").is_err());
    }
}
