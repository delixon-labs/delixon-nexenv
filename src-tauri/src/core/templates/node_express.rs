use super::{TemplateFile, TemplateInfo};

pub fn info() -> TemplateInfo {
    TemplateInfo {
        id: "node-express",
        name: "Node.js + Express",
        runtimes: &["node"],
        tags: &["backend", "api", "rest"],
        files,
    }
}

fn files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "package.json", content: include_str!("files/node_express/package.json") },
        TemplateFile { path: ".gitignore", content: include_str!("files/node_express/.gitignore") },
        TemplateFile { path: "README.md", content: include_str!("files/node_express/README.md") },
        TemplateFile { path: "src/index.js", content: include_str!("files/node_express/src_index.js") },
    ]
}
