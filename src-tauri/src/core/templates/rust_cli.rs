use super::{TemplateFile, TemplateInfo};

pub fn info() -> TemplateInfo {
    TemplateInfo {
        id: "rust-cli",
        name: "Rust CLI",
        runtimes: &["rust"],
        tags: &["cli", "rust", "tool"],
        files,
    }
}

fn files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "Cargo.toml", content: include_str!("files/rust_cli/Cargo.toml") },
        TemplateFile { path: ".gitignore", content: include_str!("files/rust_cli/.gitignore") },
        TemplateFile { path: "README.md", content: include_str!("files/rust_cli/README.md") },
        TemplateFile { path: "src/main.rs", content: include_str!("files/rust_cli/src_main.rs") },
    ]
}
