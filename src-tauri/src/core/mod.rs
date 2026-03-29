// Fundacional
pub mod error;
pub mod models;
pub mod utils;

// Existentes
pub mod catalog;
pub mod templates;
pub mod recipes;

// Nuevos subdirectorios
pub mod analysis;
pub mod history;
pub mod project;
pub mod runtime;
pub mod workspace;

// Re-exports para backward compat: crate::core::X sigue funcionando
pub use self::project::{config, manifest, notes, portable, storage};
pub use self::analysis::{detection, doctor, health, rules};
pub use self::runtime::{docker, git, ports, processes, scripts};
pub use self::workspace::{scaffold, vscode};
pub use self::history::env as snapshots;
pub use self::history::versioning;
