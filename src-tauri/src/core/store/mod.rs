mod traits;
mod migrations;
pub mod json_store;
pub mod migration;
pub mod sqlite_store;

pub use traits::*;

use std::sync::{Arc, OnceLock};

static GLOBAL_STORE: OnceLock<Arc<dyn Store>> = OnceLock::new();

/// Inicializa el store global. Llamar una sola vez al arranque.
pub fn init(store: Arc<dyn Store>) {
    let _ = GLOBAL_STORE.set(store);
}

/// Obtiene referencia al store global.
pub fn get() -> &'static dyn Store {
    GLOBAL_STORE
        .get()
        .expect("Store no inicializado. Llamar store::init() primero.")
        .as_ref()
}

/// Inicializa el store con JsonStore si no esta inicializado aun.
/// Util para tests que necesitan el store global.
#[cfg(test)]
pub fn init_test_store() {
    let _ = GLOBAL_STORE.set(Arc::new(json_store::JsonStore::new()));
}
