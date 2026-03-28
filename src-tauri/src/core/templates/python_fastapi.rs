use super::{TemplateFile, TemplateInfo};

pub fn info() -> TemplateInfo {
    TemplateInfo {
        id: "python-fastapi",
        name: "Python + FastAPI",
        runtimes: &["python"],
        tags: &["backend", "api", "python"],
        files,
    }
}

fn files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "requirements.txt", content: include_str!("files/python_fastapi/requirements.txt") },
        TemplateFile { path: ".gitignore", content: include_str!("files/python_fastapi/.gitignore") },
        TemplateFile { path: "README.md", content: include_str!("files/python_fastapi/README.md") },
        TemplateFile { path: "app/main.py", content: include_str!("files/python_fastapi/app_main.py") },
    ]
}
