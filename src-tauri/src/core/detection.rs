use serde::Serialize;
use std::path::Path;

use crate::core::error::DelixonError;
use crate::core::models::project::RuntimeConfig;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DetectedStack {
    pub runtimes: Vec<RuntimeConfig>,
    pub tags: Vec<String>,
    pub package_manager: Option<String>,
    pub orm: Option<String>,
    pub auth: Option<String>,
    pub ci: Option<String>,
    pub testing: Option<String>,
    pub docker: Option<DockerInfo>,
    pub linter: Option<String>,
    pub is_fullstack: bool,
    pub has_env_example: bool,
    pub has_readme: bool,
    pub has_types: bool,
    pub readiness_score: ReadinessScore,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DockerInfo {
    pub has_dockerfile: bool,
    pub has_compose: bool,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadinessScore {
    pub total: u8,
    pub max: u8,
    pub breakdown: Vec<ScoreItem>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScoreItem {
    pub name: String,
    pub points: u8,
    pub max_points: u8,
    pub present: bool,
}

pub fn detect_stack(project_path: &str) -> Result<DetectedStack, DelixonError> {
    let path = Path::new(project_path);
    if !path.exists() || !path.is_dir() {
        return Err(DelixonError::InvalidPath(format!(
            "La ruta no existe o no es un directorio: {}",
            project_path
        )));
    }

    let mut runtimes = Vec::new();
    let mut tags = Vec::new();

    // --- Node.js ---
    let pkg_json = path.join("package.json");
    let mut pkg_data: Option<serde_json::Value> = None;
    if pkg_json.exists() {
        let mut version = String::new();
        let mut detected_tags: Vec<String> = Vec::new();

        if let Ok(data) = std::fs::read_to_string(&pkg_json) {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&data) {
                if let Some(engines) = parsed.get("engines") {
                    if let Some(node_ver) = engines.get("node").and_then(|v| v.as_str()) {
                        version = node_ver.to_string();
                    }
                }

                let deps = [
                    parsed.get("dependencies"),
                    parsed.get("devDependencies"),
                ];
                for dep_obj in deps.iter().flatten() {
                    if let Some(obj) = dep_obj.as_object() {
                        for key in obj.keys() {
                            match key.as_str() {
                                "react" | "react-dom" => {
                                    if !detected_tags.contains(&"react".to_string()) {
                                        detected_tags.push("react".to_string());
                                    }
                                }
                                "next" => detected_tags.push("nextjs".to_string()),
                                "vue" => detected_tags.push("vue".to_string()),
                                "nuxt" => detected_tags.push("nuxt".to_string()),
                                "svelte" => detected_tags.push("svelte".to_string()),
                                "express" => detected_tags.push("express".to_string()),
                                "fastify" => detected_tags.push("fastify".to_string()),
                                "@nestjs/core" => detected_tags.push("nestjs".to_string()),
                                "@tauri-apps/api" => detected_tags.push("tauri".to_string()),
                                "electron" => detected_tags.push("electron".to_string()),
                                "vite" => detected_tags.push("vite".to_string()),
                                "tailwindcss" => detected_tags.push("tailwind".to_string()),
                                "typescript" => detected_tags.push("typescript".to_string()),
                                _ => {}
                            }
                        }
                    }
                }

                pkg_data = Some(parsed);
            }
        }

        runtimes.push(RuntimeConfig {
            runtime: "node".to_string(),
            version,
        });
        tags.extend(detected_tags);
    }

    // --- Rust ---
    let cargo_toml = path.join("Cargo.toml");
    if cargo_toml.exists() {
        runtimes.push(RuntimeConfig {
            runtime: "rust".to_string(),
            version: String::new(),
        });
        tags.push("rust".to_string());

        if let Ok(data) = std::fs::read_to_string(&cargo_toml) {
            if data.contains("[[bin]]") || data.contains("[bin]") {
                tags.push("cli".to_string());
            }
        }
    }

    // --- Python ---
    let pyproject = path.join("pyproject.toml");
    let requirements = path.join("requirements.txt");
    let setup_py = path.join("setup.py");
    if pyproject.exists() || requirements.exists() || setup_py.exists() {
        runtimes.push(RuntimeConfig {
            runtime: "python".to_string(),
            version: String::new(),
        });

        let files_to_scan = [&pyproject, &requirements];
        for file in &files_to_scan {
            if file.exists() {
                if let Ok(data) = std::fs::read_to_string(file) {
                    let lower = data.to_lowercase();
                    if lower.contains("fastapi") {
                        tags.push("fastapi".to_string());
                    }
                    if lower.contains("django") {
                        tags.push("django".to_string());
                    }
                    if lower.contains("flask") {
                        tags.push("flask".to_string());
                    }
                }
            }
        }
    }

    // --- Go ---
    if path.join("go.mod").exists() {
        runtimes.push(RuntimeConfig {
            runtime: "go".to_string(),
            version: String::new(),
        });
        tags.push("go".to_string());
    }

    // --- .NET ---
    if has_file_with_extension(path, "csproj") || has_file_with_extension(path, "sln") {
        runtimes.push(RuntimeConfig {
            runtime: "dotnet".to_string(),
            version: String::new(),
        });
        tags.push("dotnet".to_string());
    }

    // --- PHP ---
    if path.join("composer.json").exists() {
        runtimes.push(RuntimeConfig {
            runtime: "php".to_string(),
            version: String::new(),
        });
        if let Ok(data) = std::fs::read_to_string(path.join("composer.json")) {
            let lower = data.to_lowercase();
            if lower.contains("laravel") {
                tags.push("laravel".to_string());
            }
            if lower.contains("symfony") {
                tags.push("symfony".to_string());
            }
        }
    }

    // --- Ruby ---
    if path.join("Gemfile").exists() {
        runtimes.push(RuntimeConfig {
            runtime: "ruby".to_string(),
            version: String::new(),
        });
        if let Ok(data) = std::fs::read_to_string(path.join("Gemfile")) {
            if data.contains("rails") {
                tags.push("rails".to_string());
            }
        }
    }

    // --- Docker ---
    let has_dockerfile = path.join("Dockerfile").exists();
    let has_compose = path.join("docker-compose.yml").exists()
        || path.join("docker-compose.yaml").exists()
        || path.join("compose.yml").exists()
        || path.join("compose.yaml").exists();
    let docker = if has_dockerfile || has_compose {
        tags.push("docker".to_string());
        Some(DockerInfo {
            has_dockerfile,
            has_compose,
        })
    } else {
        None
    };

    // --- CI ---
    let ci = detect_ci(path);
    if ci.is_some() {
        tags.push("ci".to_string());
    }

    // --- Git ---
    if path.join(".git").exists() {
        tags.push("git".to_string());
    }

    // --- Package Manager ---
    let package_manager = detect_package_manager(path);

    // --- ORM ---
    let orm = detect_orm(path, pkg_data.as_ref());

    // --- Auth ---
    let auth = detect_auth(path, pkg_data.as_ref());

    // --- Testing ---
    let testing = detect_testing(path, pkg_data.as_ref());

    // --- Linter ---
    let linter = detect_linter(path, pkg_data.as_ref());

    // --- Fullstack detection ---
    let is_fullstack = detect_fullstack(path, &tags);

    // --- Quality signals ---
    let has_env_example = path.join(".env.example").exists() || path.join(".env.local.example").exists();
    let has_readme = path.join("README.md").exists() || path.join("readme.md").exists();
    let has_types = tags.contains(&"typescript".to_string())
        || path.join("tsconfig.json").exists()
        || path.join("py.typed").exists()
        || pyproject.exists(); // Python type hints via pyproject

    // --- Readiness Score ---
    let readiness_score = calculate_readiness_score(&ReadinessInput {
        testing: &testing,
        ci: &ci,
        docker: &docker,
        linter: &linter,
        has_env_example,
        has_readme,
        has_types,
        tags: &tags,
        runtimes: &runtimes,
    });

    Ok(DetectedStack {
        runtimes,
        tags,
        package_manager,
        orm,
        auth,
        ci,
        testing,
        docker,
        linter,
        is_fullstack,
        has_env_example,
        has_readme,
        has_types,
        readiness_score,
    })
}

fn detect_package_manager(path: &Path) -> Option<String> {
    if path.join("bun.lockb").exists() || path.join("bun.lock").exists() {
        Some("bun".to_string())
    } else if path.join("pnpm-lock.yaml").exists() {
        Some("pnpm".to_string())
    } else if path.join("yarn.lock").exists() {
        Some("yarn".to_string())
    } else if path.join("package-lock.json").exists() {
        Some("npm".to_string())
    } else if path.join("Pipfile.lock").exists() {
        Some("pipenv".to_string())
    } else if path.join("poetry.lock").exists() {
        Some("poetry".to_string())
    } else if path.join("uv.lock").exists() {
        Some("uv".to_string())
    } else if path.join("Cargo.lock").exists() {
        Some("cargo".to_string())
    } else if path.join("go.sum").exists() {
        Some("go".to_string())
    } else {
        None
    }
}

fn detect_orm(path: &Path, pkg_data: Option<&serde_json::Value>) -> Option<String> {
    // Prisma
    if path.join("prisma").is_dir() || path.join("schema.prisma").exists() {
        return Some("prisma".to_string());
    }

    // Drizzle
    if path.join("drizzle.config.ts").exists() || path.join("drizzle.config.js").exists() {
        return Some("drizzle".to_string());
    }

    // TypeORM
    if path.join("ormconfig.json").exists() || path.join("ormconfig.ts").exists() {
        return Some("typeorm".to_string());
    }

    // Check package.json dependencies
    if let Some(parsed) = pkg_data {
        let deps = [
            parsed.get("dependencies"),
            parsed.get("devDependencies"),
        ];
        for dep_obj in deps.iter().flatten() {
            if let Some(obj) = dep_obj.as_object() {
                if obj.contains_key("prisma") || obj.contains_key("@prisma/client") {
                    return Some("prisma".to_string());
                }
                if obj.contains_key("drizzle-orm") {
                    return Some("drizzle".to_string());
                }
                if obj.contains_key("typeorm") {
                    return Some("typeorm".to_string());
                }
                if obj.contains_key("sequelize") {
                    return Some("sequelize".to_string());
                }
                if obj.contains_key("knex") {
                    return Some("knex".to_string());
                }
                if obj.contains_key("mongoose") {
                    return Some("mongoose".to_string());
                }
            }
        }
    }

    // Python ORMs
    let py_files = [
        path.join("pyproject.toml"),
        path.join("requirements.txt"),
    ];
    for file in &py_files {
        if file.exists() {
            if let Ok(data) = std::fs::read_to_string(file) {
                let lower = data.to_lowercase();
                if lower.contains("sqlalchemy") {
                    return Some("sqlalchemy".to_string());
                }
                if lower.contains("tortoise-orm") {
                    return Some("tortoise".to_string());
                }
                if lower.contains("peewee") {
                    return Some("peewee".to_string());
                }
            }
        }
    }

    None
}

fn detect_auth(path: &Path, pkg_data: Option<&serde_json::Value>) -> Option<String> {
    // Check env files for auth clues
    let env_files = [".env", ".env.example", ".env.local"];
    for env_file in &env_files {
        let env_path = path.join(env_file);
        if env_path.exists() {
            if let Ok(data) = std::fs::read_to_string(&env_path) {
                let upper = data.to_uppercase();
                if upper.contains("CLERK_") || upper.contains("NEXT_PUBLIC_CLERK") {
                    return Some("clerk".to_string());
                }
                if upper.contains("NEXTAUTH_") || upper.contains("AUTH_SECRET") {
                    return Some("nextauth".to_string());
                }
                if upper.contains("SUPABASE_") {
                    return Some("supabase".to_string());
                }
                if upper.contains("FIREBASE_") || upper.contains("GOOGLE_APPLICATION_CREDENTIALS") {
                    return Some("firebase".to_string());
                }
                if upper.contains("AUTH0_") {
                    return Some("auth0".to_string());
                }
            }
        }
    }

    // Check package.json
    if let Some(parsed) = pkg_data {
        let deps = [
            parsed.get("dependencies"),
            parsed.get("devDependencies"),
        ];
        for dep_obj in deps.iter().flatten() {
            if let Some(obj) = dep_obj.as_object() {
                if obj.contains_key("@clerk/nextjs") || obj.contains_key("@clerk/clerk-react") {
                    return Some("clerk".to_string());
                }
                if obj.contains_key("next-auth") || obj.contains_key("@auth/core") {
                    return Some("nextauth".to_string());
                }
                if obj.contains_key("passport") {
                    return Some("passport".to_string());
                }
                if obj.contains_key("@supabase/supabase-js") {
                    return Some("supabase".to_string());
                }
                if obj.contains_key("firebase") {
                    return Some("firebase".to_string());
                }
            }
        }
    }

    // Python auth
    let py_files = [
        path.join("pyproject.toml"),
        path.join("requirements.txt"),
    ];
    for file in &py_files {
        if file.exists() {
            if let Ok(data) = std::fs::read_to_string(file) {
                let lower = data.to_lowercase();
                if lower.contains("python-jose") || lower.contains("pyjwt") {
                    return Some("jwt".to_string());
                }
                if lower.contains("django-allauth") {
                    return Some("django-allauth".to_string());
                }
            }
        }
    }

    None
}

fn detect_ci(path: &Path) -> Option<String> {
    if path.join(".github/workflows").is_dir() {
        return Some("github-actions".to_string());
    }
    if path.join(".gitlab-ci.yml").exists() {
        return Some("gitlab-ci".to_string());
    }
    if path.join("Jenkinsfile").exists() {
        return Some("jenkins".to_string());
    }
    if path.join(".circleci").is_dir() {
        return Some("circleci".to_string());
    }
    if path.join("bitbucket-pipelines.yml").exists() {
        return Some("bitbucket".to_string());
    }
    if path.join(".travis.yml").exists() {
        return Some("travis".to_string());
    }
    None
}

fn detect_testing(path: &Path, pkg_data: Option<&serde_json::Value>) -> Option<String> {
    // Config files
    if path.join("vitest.config.ts").exists() || path.join("vitest.config.js").exists() || path.join("vitest.config.mts").exists() {
        return Some("vitest".to_string());
    }
    if path.join("jest.config.js").exists() || path.join("jest.config.ts").exists() || path.join("jest.config.mjs").exists() {
        return Some("jest".to_string());
    }
    if path.join("playwright.config.ts").exists() || path.join("playwright.config.js").exists() {
        return Some("playwright".to_string());
    }
    if path.join("cypress.config.ts").exists() || path.join("cypress.config.js").exists() || path.join("cypress").is_dir() {
        return Some("cypress".to_string());
    }
    if path.join("pytest.ini").exists() || path.join("conftest.py").exists() || path.join("setup.cfg").exists() {
        return Some("pytest".to_string());
    }

    // Check pyproject.toml for pytest
    let pyproject = path.join("pyproject.toml");
    if pyproject.exists() {
        if let Ok(data) = std::fs::read_to_string(&pyproject) {
            if data.contains("[tool.pytest") {
                return Some("pytest".to_string());
            }
        }
    }

    // Cargo.toml tests dir
    if path.join("Cargo.toml").exists() && path.join("tests").is_dir() {
        return Some("cargo-test".to_string());
    }

    // Go tests
    if path.join("go.mod").exists() {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with("_test.go") {
                        return Some("go-test".to_string());
                    }
                }
            }
        }
    }

    // Check package.json scripts / deps
    if let Some(parsed) = pkg_data {
        let deps = [
            parsed.get("dependencies"),
            parsed.get("devDependencies"),
        ];
        for dep_obj in deps.iter().flatten() {
            if let Some(obj) = dep_obj.as_object() {
                if obj.contains_key("vitest") {
                    return Some("vitest".to_string());
                }
                if obj.contains_key("jest") {
                    return Some("jest".to_string());
                }
                if obj.contains_key("mocha") {
                    return Some("mocha".to_string());
                }
            }
        }
    }

    None
}

fn detect_linter(path: &Path, pkg_data: Option<&serde_json::Value>) -> Option<String> {
    if path.join("biome.json").exists() || path.join("biome.jsonc").exists() {
        return Some("biome".to_string());
    }
    if path.join("eslint.config.js").exists()
        || path.join("eslint.config.mjs").exists()
        || path.join(".eslintrc.json").exists()
        || path.join(".eslintrc.js").exists()
        || path.join(".eslintrc.yml").exists()
    {
        return Some("eslint".to_string());
    }
    if path.join(".pylintrc").exists() || path.join("setup.cfg").exists() {
        // setup.cfg might not be pylint, but often is
        return Some("pylint".to_string());
    }
    if path.join("ruff.toml").exists() {
        return Some("ruff".to_string());
    }

    // Check pyproject.toml for ruff
    let pyproject = path.join("pyproject.toml");
    if pyproject.exists() {
        if let Ok(data) = std::fs::read_to_string(&pyproject) {
            if data.contains("[tool.ruff") {
                return Some("ruff".to_string());
            }
        }
    }

    // Check package.json
    if let Some(parsed) = pkg_data {
        let deps = [
            parsed.get("dependencies"),
            parsed.get("devDependencies"),
        ];
        for dep_obj in deps.iter().flatten() {
            if let Some(obj) = dep_obj.as_object() {
                if obj.contains_key("@biomejs/biome") {
                    return Some("biome".to_string());
                }
                if obj.contains_key("eslint") {
                    return Some("eslint".to_string());
                }
            }
        }
    }

    // Rust clippy (always available with rustc)
    if path.join("Cargo.toml").exists() {
        return Some("clippy".to_string());
    }

    None
}

fn detect_fullstack(path: &Path, tags: &[String]) -> bool {
    // Explicit fullstack directories
    let has_frontend_dir = path.join("frontend").is_dir()
        || path.join("client").is_dir()
        || path.join("web").is_dir()
        || path.join("app").is_dir();
    let has_backend_dir = path.join("backend").is_dir()
        || path.join("server").is_dir()
        || path.join("api").is_dir();

    if has_frontend_dir && has_backend_dir {
        return true;
    }

    // Mixed framework tags
    let frontend_tags = ["react", "vue", "svelte", "nextjs", "nuxt"];
    let backend_tags = ["express", "fastify", "nestjs", "fastapi", "django", "flask"];

    let has_frontend = tags.iter().any(|t| frontend_tags.contains(&t.as_str()));
    let has_backend = tags.iter().any(|t| backend_tags.contains(&t.as_str()));

    has_frontend && has_backend
}

struct ReadinessInput<'a> {
    testing: &'a Option<String>,
    ci: &'a Option<String>,
    docker: &'a Option<DockerInfo>,
    linter: &'a Option<String>,
    has_env_example: bool,
    has_readme: bool,
    has_types: bool,
    tags: &'a [String],
    runtimes: &'a [RuntimeConfig],
}

fn calculate_readiness_score(input: &ReadinessInput) -> ReadinessScore {
    let mut breakdown = Vec::new();
    let mut suggestions = Vec::new();
    let mut total: u8 = 0;

    // 1. Testing (+2)
    let testing_present = input.testing.is_some();
    breakdown.push(ScoreItem {
        name: "Testing".to_string(),
        points: if testing_present { 2 } else { 0 },
        max_points: 2,
        present: testing_present,
    });
    if testing_present {
        total += 2;
    } else {
        suggestions.push("testing".to_string());
    }

    // 2. CI (+1)
    let ci_present = input.ci.is_some();
    breakdown.push(ScoreItem {
        name: "CI/CD".to_string(),
        points: if ci_present { 1 } else { 0 },
        max_points: 1,
        present: ci_present,
    });
    if ci_present {
        total += 1;
    } else {
        suggestions.push("ci".to_string());
    }

    // 3. Docker (+1)
    let docker_present = input.docker.is_some();
    breakdown.push(ScoreItem {
        name: "Docker".to_string(),
        points: if docker_present { 1 } else { 0 },
        max_points: 1,
        present: docker_present,
    });
    if docker_present {
        total += 1;
    } else {
        suggestions.push("docker".to_string());
    }

    // 4. Linter (+1)
    let linter_present = input.linter.is_some();
    breakdown.push(ScoreItem {
        name: "Linter".to_string(),
        points: if linter_present { 1 } else { 0 },
        max_points: 1,
        present: linter_present,
    });
    if linter_present {
        total += 1;
    } else {
        suggestions.push("linting".to_string());
    }

    // 5. .env.example (+1)
    breakdown.push(ScoreItem {
        name: "Env example".to_string(),
        points: if input.has_env_example { 1 } else { 0 },
        max_points: 1,
        present: input.has_env_example,
    });
    if input.has_env_example {
        total += 1;
    }

    // 6. README (+1)
    breakdown.push(ScoreItem {
        name: "README".to_string(),
        points: if input.has_readme { 1 } else { 0 },
        max_points: 1,
        present: input.has_readme,
    });
    if input.has_readme {
        total += 1;
    }

    // 7. Type system (+1)
    breakdown.push(ScoreItem {
        name: "Types".to_string(),
        points: if input.has_types { 1 } else { 0 },
        max_points: 1,
        present: input.has_types,
    });
    if input.has_types {
        total += 1;
    }

    // 8. Git (+1)
    let has_git = input.tags.contains(&"git".to_string());
    breakdown.push(ScoreItem {
        name: "Git".to_string(),
        points: if has_git { 1 } else { 0 },
        max_points: 1,
        present: has_git,
    });
    if has_git {
        total += 1;
    }

    // 9. Has runtime detected (+1)
    let has_runtime = !input.runtimes.is_empty();
    breakdown.push(ScoreItem {
        name: "Runtime".to_string(),
        points: if has_runtime { 1 } else { 0 },
        max_points: 1,
        present: has_runtime,
    });
    if has_runtime {
        total += 1;
    }

    ReadinessScore {
        total,
        max: 10,
        breakdown,
        suggestions,
    }
}

fn has_file_with_extension(dir: &Path, ext: &str) -> bool {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Some(file_ext) = entry.path().extension() {
                if file_ext == ext {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_stack_node() {
        let dir = tempfile::tempdir().unwrap();
        let pkg = dir.path().join("package.json");
        std::fs::write(
            &pkg,
            r#"{"name":"test","dependencies":{"express":"^4.0"},"devDependencies":{"typescript":"^5"}}"#,
        )
        .unwrap();

        let result = detect_stack(dir.path().to_str().unwrap()).unwrap();
        assert!(result.runtimes.iter().any(|r| r.runtime == "node"));
        assert!(result.tags.contains(&"express".to_string()));
        assert!(result.tags.contains(&"typescript".to_string()));
    }

    #[test]
    fn test_detect_stack_rust() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("Cargo.toml"), "[package]\nname = \"test\"\n").unwrap();

        let result = detect_stack(dir.path().to_str().unwrap()).unwrap();
        assert!(result.runtimes.iter().any(|r| r.runtime == "rust"));
    }

    #[test]
    fn test_detect_stack_python() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("requirements.txt"), "fastapi\nuvicorn\n").unwrap();

        let result = detect_stack(dir.path().to_str().unwrap()).unwrap();
        assert!(result.runtimes.iter().any(|r| r.runtime == "python"));
        assert!(result.tags.contains(&"fastapi".to_string()));
    }

    #[test]
    fn test_detect_stack_docker() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("Dockerfile"), "FROM node:20\n").unwrap();

        let result = detect_stack(dir.path().to_str().unwrap()).unwrap();
        assert!(result.tags.contains(&"docker".to_string()));
        assert!(result.docker.is_some());
        assert!(result.docker.as_ref().unwrap().has_dockerfile);
    }

    #[test]
    fn test_detect_stack_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let result = detect_stack(dir.path().to_str().unwrap()).unwrap();
        assert!(result.runtimes.is_empty());
        assert!(result.tags.is_empty());
    }

    #[test]
    fn test_detect_stack_invalid_path() {
        let result = detect_stack("/nonexistent/path/12345");
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_package_manager_npm() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("package-lock.json"), "{}").unwrap();
        assert_eq!(detect_package_manager(dir.path()), Some("npm".to_string()));
    }

    #[test]
    fn test_detect_package_manager_pnpm() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("pnpm-lock.yaml"), "").unwrap();
        assert_eq!(detect_package_manager(dir.path()), Some("pnpm".to_string()));
    }

    #[test]
    fn test_detect_package_manager_yarn() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("yarn.lock"), "").unwrap();
        assert_eq!(detect_package_manager(dir.path()), Some("yarn".to_string()));
    }

    #[test]
    fn test_detect_orm_prisma() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("prisma")).unwrap();
        assert_eq!(detect_orm(dir.path(), None), Some("prisma".to_string()));
    }

    #[test]
    fn test_detect_orm_from_package_json() {
        let pkg: serde_json::Value = serde_json::from_str(
            r#"{"dependencies":{"drizzle-orm":"^0.30"}}"#,
        )
        .unwrap();
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(detect_orm(dir.path(), Some(&pkg)), Some("drizzle".to_string()));
    }

    #[test]
    fn test_detect_ci_github() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".github/workflows")).unwrap();
        assert_eq!(detect_ci(dir.path()), Some("github-actions".to_string()));
    }

    #[test]
    fn test_detect_testing_vitest() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("vitest.config.ts"), "").unwrap();
        assert_eq!(detect_testing(dir.path(), None), Some("vitest".to_string()));
    }

    #[test]
    fn test_detect_linter_eslint() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("eslint.config.js"), "").unwrap();
        assert_eq!(detect_linter(dir.path(), None), Some("eslint".to_string()));
    }

    #[test]
    fn test_detect_fullstack() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join("frontend")).unwrap();
        std::fs::create_dir(dir.path().join("backend")).unwrap();
        assert!(detect_fullstack(dir.path(), &[]));
    }

    #[test]
    fn test_readiness_score_full() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path();
        // Create all the quality signals
        std::fs::write(path.join("package.json"), r#"{"name":"t","devDependencies":{"vitest":"^1","eslint":"^8","typescript":"^5"}}"#).unwrap();
        std::fs::write(path.join("vitest.config.ts"), "").unwrap();
        std::fs::create_dir_all(path.join(".github/workflows")).unwrap();
        std::fs::write(path.join("Dockerfile"), "FROM node:20").unwrap();
        std::fs::write(path.join("eslint.config.js"), "").unwrap();
        std::fs::write(path.join(".env.example"), "PORT=3000").unwrap();
        std::fs::write(path.join("README.md"), "# Test").unwrap();
        std::fs::write(path.join("tsconfig.json"), "{}").unwrap();
        std::fs::create_dir(path.join(".git")).unwrap();

        let result = detect_stack(path.to_str().unwrap()).unwrap();
        assert!(result.readiness_score.total >= 8);
    }

    #[test]
    fn test_readiness_score_empty() {
        let dir = tempfile::tempdir().unwrap();
        let result = detect_stack(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(result.readiness_score.total, 0);
    }

    #[test]
    fn test_detect_auth_clerk_env() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(".env.example"), "CLERK_SECRET_KEY=xxx").unwrap();
        assert_eq!(detect_auth(dir.path(), None), Some("clerk".to_string()));
    }

    #[test]
    fn test_detect_auth_from_deps() {
        let pkg: serde_json::Value = serde_json::from_str(
            r#"{"dependencies":{"next-auth":"^5"}}"#,
        )
        .unwrap();
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(detect_auth(dir.path(), Some(&pkg)), Some("nextauth".to_string()));
    }
}
