use super::{TemplateFile, TemplateInfo};

pub fn info() -> TemplateInfo {
    TemplateInfo {
        id: "fullstack-react-python",
        name: "React + FastAPI",
        runtimes: &["node", "python"],
        tags: &["fullstack", "monorepo"],
        files,
    }
}

fn files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "README.md", content: include_str!("files/fullstack_react_python/README.md") },
        TemplateFile { path: ".gitignore", content: include_str!("files/fullstack_react_python/.gitignore") },
        TemplateFile { path: "frontend/package.json", content: include_str!("files/fullstack_react_python/frontend_package.json") },
        TemplateFile { path: "frontend/src/App.tsx", content: include_str!("files/fullstack_react_python/frontend_src_App.tsx") },
        TemplateFile { path: "backend/requirements.txt", content: include_str!("files/fullstack_react_python/backend_requirements.txt") },
        TemplateFile { path: "backend/app/main.py", content: include_str!("files/fullstack_react_python/backend_app_main.py") },
    ]
}
