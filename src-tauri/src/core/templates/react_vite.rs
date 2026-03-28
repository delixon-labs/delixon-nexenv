use super::{TemplateFile, TemplateInfo};

pub fn info() -> TemplateInfo {
    TemplateInfo {
        id: "react-vite",
        name: "React + Vite",
        runtimes: &["node"],
        tags: &["frontend", "spa", "react"],
        files,
    }
}

fn files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "package.json", content: include_str!("files/react_vite/package.json") },
        TemplateFile { path: ".gitignore", content: include_str!("files/react_vite/.gitignore") },
        TemplateFile { path: "README.md", content: include_str!("files/react_vite/README.md") },
        TemplateFile { path: "index.html", content: include_str!("files/react_vite/index.html") },
        TemplateFile { path: "src/main.tsx", content: include_str!("files/react_vite/src_main.tsx") },
        TemplateFile { path: "src/App.tsx", content: include_str!("files/react_vite/src_App.tsx") },
        TemplateFile { path: "tsconfig.json", content: include_str!("files/react_vite/tsconfig.json") },
        TemplateFile { path: "vite.config.ts", content: include_str!("files/react_vite/vite.config.ts") },
    ]
}
