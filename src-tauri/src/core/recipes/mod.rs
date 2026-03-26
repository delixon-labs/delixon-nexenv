use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

use crate::core::error::DelixonError;
use crate::core::manifest;
use crate::core::utils::fs::ensure_dir;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub files_to_create: Vec<RecipeFile>,
    pub deps_to_install: Vec<String>,
    pub dev_deps_to_install: Vec<String>,
    pub env_vars_to_add: HashMap<String, String>,
    pub scripts_to_add: HashMap<String, String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecipeFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecipePreview {
    pub recipe: Recipe,
    pub files_that_exist: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecipeApplyResult {
    pub recipe_id: String,
    pub files_created: Vec<String>,
    pub files_skipped: Vec<String>,
    pub env_vars_added: Vec<String>,
}

pub fn list_recipes() -> Vec<Recipe> {
    vec![
        recipe_testing_vitest(),
        recipe_testing_pytest(),
        recipe_docker(),
        recipe_ci_github(),
        recipe_linting_biome(),
        recipe_database_prisma(),
    ]
}

pub fn get_recipe(id: &str) -> Option<Recipe> {
    list_recipes().into_iter().find(|r| r.id == id)
}

pub fn preview_recipe(project_path: &str, recipe_id: &str) -> Result<RecipePreview, DelixonError> {
    let recipe = get_recipe(recipe_id).ok_or_else(|| {
        DelixonError::InvalidConfig(format!("Recipe no encontrada: {}", recipe_id))
    })?;

    let path = Path::new(project_path);
    let files_that_exist: Vec<String> = recipe
        .files_to_create
        .iter()
        .filter(|f| path.join(&f.path).exists())
        .map(|f| f.path.clone())
        .collect();

    Ok(RecipePreview {
        recipe,
        files_that_exist,
    })
}

pub fn apply_recipe(project_path: &str, recipe_id: &str) -> Result<RecipeApplyResult, DelixonError> {
    let recipe = get_recipe(recipe_id).ok_or_else(|| {
        DelixonError::InvalidConfig(format!("Recipe no encontrada: {}", recipe_id))
    })?;

    let path = Path::new(project_path);
    let mut files_created = Vec::new();
    let mut files_skipped = Vec::new();

    for file in &recipe.files_to_create {
        let full_path = path.join(&file.path);
        if full_path.exists() {
            files_skipped.push(file.path.clone());
            continue;
        }
        if let Some(parent) = full_path.parent() {
            ensure_dir(parent)?;
        }
        std::fs::write(&full_path, &file.content)?;
        files_created.push(file.path.clone());
    }

    // Add env vars to .env.example
    let mut env_vars_added = Vec::new();
    if !recipe.env_vars_to_add.is_empty() {
        let env_example_path = path.join(".env.example");
        let mut content = if env_example_path.exists() {
            std::fs::read_to_string(&env_example_path).unwrap_or_default()
        } else {
            String::new()
        };

        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(&format!("\n# Added by recipe: {}\n", recipe.name));

        let mut sorted: Vec<_> = recipe.env_vars_to_add.iter().collect();
        sorted.sort_by_key(|(k, _)| (*k).clone());
        for (key, val) in sorted {
            if !content.contains(key.as_str()) {
                content.push_str(&format!("{}={}\n", key, val));
                env_vars_added.push(key.clone());
            }
        }
        std::fs::write(&env_example_path, content)?;
    }

    // Update manifest to record applied recipe
    if let Ok(Some(mut m)) = manifest::load_manifest(project_path) {
        if !m.recipes_applied.contains(&recipe_id.to_string()) {
            m.recipes_applied.push(recipe_id.to_string());
            let _ = manifest::save_manifest(project_path, &m);
        }
    }

    Ok(RecipeApplyResult {
        recipe_id: recipe_id.to_string(),
        files_created,
        files_skipped,
        env_vars_added,
    })
}

fn recipe_testing_vitest() -> Recipe {
    Recipe {
        id: "testing-vitest".to_string(),
        name: "Vitest Testing".to_string(),
        description: "Configura Vitest para testing unitario e integracion".to_string(),
        category: "testing".to_string(),
        files_to_create: vec![
            RecipeFile {
                path: "vitest.config.ts".to_string(),
                content: r#"import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
    },
  },
});
"#.to_string(),
            },
            RecipeFile {
                path: "tests/example.test.ts".to_string(),
                content: r#"import { describe, it, expect } from 'vitest';

describe('Example', () => {
  it('should pass', () => {
    expect(1 + 1).toBe(2);
  });
});
"#.to_string(),
            },
        ],
        deps_to_install: vec![],
        dev_deps_to_install: vec!["vitest".to_string(), "@vitest/coverage-v8".to_string()],
        env_vars_to_add: HashMap::new(),
        scripts_to_add: [
            ("test".to_string(), "vitest".to_string()),
            ("test:run".to_string(), "vitest run".to_string()),
            ("test:coverage".to_string(), "vitest run --coverage".to_string()),
        ]
        .into_iter()
        .collect(),
    }
}

fn recipe_testing_pytest() -> Recipe {
    Recipe {
        id: "testing-pytest".to_string(),
        name: "Pytest Testing".to_string(),
        description: "Configura pytest para testing Python".to_string(),
        category: "testing".to_string(),
        files_to_create: vec![
            RecipeFile {
                path: "conftest.py".to_string(),
                content: "import pytest\n".to_string(),
            },
            RecipeFile {
                path: "tests/__init__.py".to_string(),
                content: String::new(),
            },
            RecipeFile {
                path: "tests/test_example.py".to_string(),
                content: r#"def test_example():
    assert 1 + 1 == 2
"#.to_string(),
            },
        ],
        deps_to_install: vec!["pytest".to_string(), "pytest-cov".to_string()],
        dev_deps_to_install: vec![],
        env_vars_to_add: HashMap::new(),
        scripts_to_add: HashMap::new(),
    }
}

fn recipe_docker() -> Recipe {
    Recipe {
        id: "docker".to_string(),
        name: "Docker".to_string(),
        description: "Agrega Dockerfile y docker-compose.yml".to_string(),
        category: "devops".to_string(),
        files_to_create: vec![
            RecipeFile {
                path: "Dockerfile".to_string(),
                content: r#"FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM node:20-alpine
WORKDIR /app
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./
EXPOSE 3000
CMD ["node", "dist/index.js"]
"#.to_string(),
            },
            RecipeFile {
                path: ".dockerignore".to_string(),
                content: "node_modules\ndist\n.git\n.env\n*.log\n".to_string(),
            },
        ],
        deps_to_install: vec![],
        dev_deps_to_install: vec![],
        env_vars_to_add: HashMap::new(),
        scripts_to_add: [
            ("docker:build".to_string(), "docker build -t app .".to_string()),
            ("docker:run".to_string(), "docker run -p 3000:3000 app".to_string()),
        ]
        .into_iter()
        .collect(),
    }
}

fn recipe_ci_github() -> Recipe {
    Recipe {
        id: "ci-github".to_string(),
        name: "GitHub Actions CI".to_string(),
        description: "Configura CI con GitHub Actions".to_string(),
        category: "ci".to_string(),
        files_to_create: vec![RecipeFile {
            path: ".github/workflows/ci.yml".to_string(),
            content: r#"name: CI
on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [develop]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: npm
      - run: npm ci
      - run: npm run lint
      - run: npm run test -- --run
      - run: npm run build
"#.to_string(),
        }],
        deps_to_install: vec![],
        dev_deps_to_install: vec![],
        env_vars_to_add: HashMap::new(),
        scripts_to_add: HashMap::new(),
    }
}

fn recipe_linting_biome() -> Recipe {
    Recipe {
        id: "linting-biome".to_string(),
        name: "Biome Linting".to_string(),
        description: "Configura Biome para linting y formateo".to_string(),
        category: "linting".to_string(),
        files_to_create: vec![RecipeFile {
            path: "biome.json".to_string(),
            content: r#"{
  "$schema": "https://biomejs.dev/schemas/1.9.0/schema.json",
  "organizeImports": { "enabled": true },
  "linter": {
    "enabled": true,
    "rules": { "recommended": true }
  },
  "formatter": {
    "enabled": true,
    "indentStyle": "space",
    "indentWidth": 2
  }
}
"#.to_string(),
        }],
        deps_to_install: vec![],
        dev_deps_to_install: vec!["@biomejs/biome".to_string()],
        env_vars_to_add: HashMap::new(),
        scripts_to_add: [
            ("lint".to_string(), "biome check .".to_string()),
            ("format".to_string(), "biome format --write .".to_string()),
        ]
        .into_iter()
        .collect(),
    }
}

fn recipe_database_prisma() -> Recipe {
    Recipe {
        id: "database-prisma".to_string(),
        name: "Prisma ORM".to_string(),
        description: "Configura Prisma con PostgreSQL".to_string(),
        category: "database".to_string(),
        files_to_create: vec![RecipeFile {
            path: "prisma/schema.prisma".to_string(),
            content: r#"generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

model User {
  id        String   @id @default(uuid())
  email     String   @unique
  name      String?
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
}
"#.to_string(),
        }],
        deps_to_install: vec!["@prisma/client".to_string()],
        dev_deps_to_install: vec!["prisma".to_string()],
        env_vars_to_add: [("DATABASE_URL".to_string(), "postgresql://user:password@localhost:5432/mydb".to_string())]
            .into_iter()
            .collect(),
        scripts_to_add: [
            ("db:push".to_string(), "prisma db push".to_string()),
            ("db:studio".to_string(), "prisma studio".to_string()),
            ("db:generate".to_string(), "prisma generate".to_string()),
        ]
        .into_iter()
        .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_recipes() {
        let recipes = list_recipes();
        assert_eq!(recipes.len(), 6);
    }

    #[test]
    fn test_get_recipe() {
        assert!(get_recipe("testing-vitest").is_some());
        assert!(get_recipe("docker").is_some());
        assert!(get_recipe("nonexistent").is_none());
    }

    #[test]
    fn test_preview_recipe() {
        let dir = tempfile::tempdir().unwrap();
        let preview = preview_recipe(dir.path().to_str().unwrap(), "testing-vitest").unwrap();
        assert_eq!(preview.recipe.id, "testing-vitest");
        assert!(preview.files_that_exist.is_empty());
    }

    #[test]
    fn test_apply_recipe() {
        let dir = tempfile::tempdir().unwrap();
        let result = apply_recipe(dir.path().to_str().unwrap(), "testing-vitest").unwrap();
        assert_eq!(result.recipe_id, "testing-vitest");
        assert!(result.files_created.contains(&"vitest.config.ts".to_string()));
        assert!(dir.path().join("vitest.config.ts").exists());
        assert!(dir.path().join("tests/example.test.ts").exists());
    }

    #[test]
    fn test_apply_recipe_skips_existing() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("vitest.config.ts"), "existing").unwrap();
        let result = apply_recipe(dir.path().to_str().unwrap(), "testing-vitest").unwrap();
        assert!(result.files_skipped.contains(&"vitest.config.ts".to_string()));
        // Should not overwrite
        let content = std::fs::read_to_string(dir.path().join("vitest.config.ts")).unwrap();
        assert_eq!(content, "existing");
    }

    #[test]
    fn test_apply_recipe_adds_env_vars() {
        let dir = tempfile::tempdir().unwrap();
        let result = apply_recipe(dir.path().to_str().unwrap(), "database-prisma").unwrap();
        assert!(result.env_vars_added.contains(&"DATABASE_URL".to_string()));
        assert!(dir.path().join(".env.example").exists());
    }

    #[test]
    fn test_apply_recipe_nonexistent() {
        let dir = tempfile::tempdir().unwrap();
        let result = apply_recipe(dir.path().to_str().unwrap(), "nonexistent");
        assert!(result.is_err());
    }
}
