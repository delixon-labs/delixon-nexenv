use super::{TemplateFile, TemplateInfo};

pub fn info() -> TemplateInfo {
    TemplateInfo {
        id: "python-django",
        name: "Python + Django",
        runtimes: &["python"],
        tags: &["backend", "fullstack", "python"],
        files,
    }
}

fn files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "requirements.txt", content: include_str!("files/python_django/requirements.txt") },
        TemplateFile { path: ".gitignore", content: include_str!("files/python_django/.gitignore") },
        TemplateFile { path: "README.md", content: include_str!("files/python_django/README.md") },
        TemplateFile { path: "manage.py", content: include_str!("files/python_django/manage.py") },
    ]
}
