use std::fs;
use std::path::PathBuf;

const VALIDATION_URL: &str = "https://app.delixon.dev/api/store/license/validate/";

fn license_key_path() -> PathBuf {
    let home = dirs::home_dir().expect("No se encontro el directorio home");
    home.join(".nexenv").join("license.key")
}

fn read_license_key() -> Option<String> {
    let path = license_key_path();
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

pub async fn validate_license(key: &str) -> bool {
    let url = format!("{}?key={}", VALIDATION_URL, key);
    let client = reqwest::Client::new();

    match client.get(&url).send().await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

pub fn is_enterprise() -> bool {
    let key = match read_license_key() {
        Some(k) if !k.is_empty() => k,
        _ => return false,
    };

    let rt = tokio::runtime::Handle::try_current();
    match rt {
        Ok(handle) => {
            tokio::task::block_in_place(|| handle.block_on(validate_license(&key)))
        }
        Err(_) => {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(validate_license(&key))
        }
    }
}

pub fn activate_license(key: &str) -> Result<(), String> {
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
