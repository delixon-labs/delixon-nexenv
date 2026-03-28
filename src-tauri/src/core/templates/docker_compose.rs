use super::{TemplateFile, TemplateInfo};

pub fn info() -> TemplateInfo {
    TemplateInfo {
        id: "docker-compose",
        name: "Docker Compose Stack",
        runtimes: &[],
        tags: &["docker", "devops", "infra"],
        files,
    }
}

fn files() -> Vec<TemplateFile> {
    vec![
        TemplateFile { path: "docker-compose.yml", content: include_str!("files/docker_compose/docker-compose.yml") },
        TemplateFile { path: ".gitignore", content: include_str!("files/docker_compose/.gitignore") },
        TemplateFile { path: "README.md", content: include_str!("files/docker_compose/README.md") },
        TemplateFile { path: ".env.example", content: include_str!("files/docker_compose/.env.example") },
    ]
}
