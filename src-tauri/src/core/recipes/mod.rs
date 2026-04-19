use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

use crate::core::error::NexenvError;
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
        recipe_auth_jwt(),
        recipe_database_sqlalchemy(),
        recipe_monitoring(),
    ]
}

pub fn get_recipe(id: &str) -> Option<Recipe> {
    list_recipes().into_iter().find(|r| r.id == id)
}

pub fn preview_recipe(project_path: &str, recipe_id: &str) -> Result<RecipePreview, NexenvError> {
    let recipe = get_recipe(recipe_id).ok_or_else(|| {
        NexenvError::InvalidConfig(format!("Recipe no encontrada: {}", recipe_id))
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

pub fn apply_recipe(project_path: &str, recipe_id: &str) -> Result<RecipeApplyResult, NexenvError> {
    let recipe = get_recipe(recipe_id).ok_or_else(|| {
        NexenvError::InvalidConfig(format!("Recipe no encontrada: {}", recipe_id))
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

fn recipe_auth_jwt() -> Recipe {
    Recipe {
        id: "auth-jwt".to_string(),
        name: "JWT Auth".to_string(),
        description: "Autenticacion JWT con refresh tokens (Express/Fastify)".to_string(),
        category: "auth".to_string(),
        files_to_create: vec![
            RecipeFile {
                path: "src/auth/jwt.ts".to_string(),
                content: r#"import jwt from 'jsonwebtoken';

const ACCESS_SECRET = process.env.JWT_ACCESS_SECRET || 'change-me-access';
const REFRESH_SECRET = process.env.JWT_REFRESH_SECRET || 'change-me-refresh';
const ACCESS_TTL = process.env.JWT_ACCESS_TTL || '15m';
const REFRESH_TTL = process.env.JWT_REFRESH_TTL || '7d';

export interface AccessPayload { sub: string; email?: string; }
export interface RefreshPayload { sub: string; jti: string; }

export function signAccess(payload: AccessPayload): string {
  return jwt.sign(payload, ACCESS_SECRET, { expiresIn: ACCESS_TTL });
}

export function signRefresh(payload: RefreshPayload): string {
  return jwt.sign(payload, REFRESH_SECRET, { expiresIn: REFRESH_TTL });
}

export function verifyAccess(token: string): AccessPayload {
  return jwt.verify(token, ACCESS_SECRET) as AccessPayload;
}

export function verifyRefresh(token: string): RefreshPayload {
  return jwt.verify(token, REFRESH_SECRET) as RefreshPayload;
}
"#.to_string(),
            },
            RecipeFile {
                path: "src/auth/middleware.ts".to_string(),
                content: r#"import type { Request, Response, NextFunction } from 'express';
import { verifyAccess } from './jwt';

declare module 'express-serve-static-core' {
  interface Request { user?: { sub: string; email?: string }; }
}

export function requireAuth(req: Request, res: Response, next: NextFunction) {
  const header = req.headers.authorization || '';
  const token = header.startsWith('Bearer ') ? header.slice(7) : null;
  if (!token) return res.status(401).json({ error: 'missing token' });
  try {
    req.user = verifyAccess(token);
    next();
  } catch {
    return res.status(401).json({ error: 'invalid or expired token' });
  }
}
"#.to_string(),
            },
            RecipeFile {
                path: "src/auth/routes.ts".to_string(),
                content: r#"import { Router } from 'express';
import { randomUUID } from 'node:crypto';
import { signAccess, signRefresh, verifyRefresh } from './jwt';

export const authRouter = Router();

// POST /auth/login — sustituir validacion por la propia
authRouter.post('/login', (req, res) => {
  const { email, password } = req.body ?? {};
  if (!email || !password) return res.status(400).json({ error: 'missing credentials' });
  // TODO: validar contra base de datos
  const userId = 'user-id-from-db';
  const accessToken = signAccess({ sub: userId, email });
  const refreshToken = signRefresh({ sub: userId, jti: randomUUID() });
  res.json({ accessToken, refreshToken });
});

// POST /auth/refresh — emite nuevo access token
authRouter.post('/refresh', (req, res) => {
  const { refreshToken } = req.body ?? {};
  if (!refreshToken) return res.status(400).json({ error: 'missing refresh token' });
  try {
    const payload = verifyRefresh(refreshToken);
    const accessToken = signAccess({ sub: payload.sub });
    res.json({ accessToken });
  } catch {
    res.status(401).json({ error: 'invalid refresh token' });
  }
});

// POST /auth/logout — el cliente debe descartar tokens; aqui se podria revocar en una blacklist
authRouter.post('/logout', (_req, res) => {
  res.status(204).send();
});
"#.to_string(),
            },
        ],
        deps_to_install: vec!["jsonwebtoken".to_string(), "express".to_string()],
        dev_deps_to_install: vec![
            "@types/jsonwebtoken".to_string(),
            "@types/express".to_string(),
        ],
        env_vars_to_add: [
            ("JWT_ACCESS_SECRET".to_string(), "change-me-access".to_string()),
            ("JWT_REFRESH_SECRET".to_string(), "change-me-refresh".to_string()),
            ("JWT_ACCESS_TTL".to_string(), "15m".to_string()),
            ("JWT_REFRESH_TTL".to_string(), "7d".to_string()),
        ]
        .into_iter()
        .collect(),
        scripts_to_add: HashMap::new(),
    }
}

fn recipe_database_sqlalchemy() -> Recipe {
    Recipe {
        id: "database-sqlalchemy".to_string(),
        name: "SQLAlchemy + Alembic".to_string(),
        description: "ORM SQLAlchemy con Postgres y migraciones Alembic".to_string(),
        category: "database".to_string(),
        files_to_create: vec![
            RecipeFile {
                path: "app/db/__init__.py".to_string(),
                content: String::new(),
            },
            RecipeFile {
                path: "app/db/session.py".to_string(),
                content: r#"import os
from sqlalchemy import create_engine
from sqlalchemy.orm import declarative_base, sessionmaker

DATABASE_URL = os.environ.get(
    "DATABASE_URL",
    "postgresql+psycopg://user:password@localhost:5432/mydb",
)

engine = create_engine(DATABASE_URL, pool_pre_ping=True, future=True)
SessionLocal = sessionmaker(bind=engine, autoflush=False, expire_on_commit=False, future=True)
Base = declarative_base()


def get_db():
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()
"#.to_string(),
            },
            RecipeFile {
                path: "app/db/models.py".to_string(),
                content: r#"from datetime import datetime
from sqlalchemy import Column, DateTime, String
from sqlalchemy.dialects.postgresql import UUID
import uuid

from .session import Base


class User(Base):
    __tablename__ = "users"

    id = Column(UUID(as_uuid=True), primary_key=True, default=uuid.uuid4)
    email = Column(String, unique=True, nullable=False, index=True)
    name = Column(String, nullable=True)
    created_at = Column(DateTime, default=datetime.utcnow, nullable=False)
"#.to_string(),
            },
            RecipeFile {
                path: "alembic.ini".to_string(),
                content: r#"[alembic]
script_location = alembic
sqlalchemy.url = postgresql+psycopg://user:password@localhost:5432/mydb

[loggers]
keys = root,sqlalchemy,alembic

[handlers]
keys = console

[formatters]
keys = generic

[logger_root]
level = WARN
handlers = console
qualname =

[logger_sqlalchemy]
level = WARN
handlers =
qualname = sqlalchemy.engine

[logger_alembic]
level = INFO
handlers =
qualname = alembic

[handler_console]
class = StreamHandler
args = (sys.stderr,)
level = NOTSET
formatter = generic

[formatter_generic]
format = %(levelname)-5.5s [%(name)s] %(message)s
datefmt = %H:%M:%S
"#.to_string(),
            },
            RecipeFile {
                path: "alembic/env.py".to_string(),
                content: r#"from logging.config import fileConfig
from sqlalchemy import engine_from_config, pool
from alembic import context

from app.db.session import Base
from app.db import models  # noqa: F401  asegura que los modelos se registren

config = context.config

if config.config_file_name is not None:
    fileConfig(config.config_file_name)

target_metadata = Base.metadata


def run_migrations_offline():
    url = config.get_main_option("sqlalchemy.url")
    context.configure(url=url, target_metadata=target_metadata, literal_binds=True)
    with context.begin_transaction():
        context.run_migrations()


def run_migrations_online():
    connectable = engine_from_config(
        config.get_section(config.config_ini_section),
        prefix="sqlalchemy.",
        poolclass=pool.NullPool,
    )
    with connectable.connect() as connection:
        context.configure(connection=connection, target_metadata=target_metadata)
        with context.begin_transaction():
            context.run_migrations()


if context.is_offline_mode():
    run_migrations_offline()
else:
    run_migrations_online()
"#.to_string(),
            },
            RecipeFile {
                path: "alembic/script.py.mako".to_string(),
                content: r#""""${message}

Revision ID: ${up_revision}
Revises: ${down_revision | comma,n}
Create Date: ${create_date}

"""
from alembic import op
import sqlalchemy as sa
${imports if imports else ""}

revision = ${repr(up_revision)}
down_revision = ${repr(down_revision)}
branch_labels = ${repr(branch_labels)}
depends_on = ${repr(depends_on)}


def upgrade() -> None:
    ${upgrades if upgrades else "pass"}


def downgrade() -> None:
    ${downgrades if downgrades else "pass"}
"#.to_string(),
            },
            RecipeFile {
                path: "alembic/versions/.gitkeep".to_string(),
                content: String::new(),
            },
        ],
        deps_to_install: vec![
            "sqlalchemy>=2.0".to_string(),
            "psycopg[binary]>=3.1".to_string(),
            "alembic>=1.13".to_string(),
        ],
        dev_deps_to_install: vec![],
        env_vars_to_add: [(
            "DATABASE_URL".to_string(),
            "postgresql+psycopg://user:password@localhost:5432/mydb".to_string(),
        )]
        .into_iter()
        .collect(),
        scripts_to_add: HashMap::new(),
    }
}

fn recipe_monitoring() -> Recipe {
    Recipe {
        id: "monitoring".to_string(),
        name: "Health + Structured Logging".to_string(),
        description: "Endpoint /health y logging estructurado JSON (pino/structlog)".to_string(),
        category: "monitoring".to_string(),
        files_to_create: vec![
            RecipeFile {
                path: "src/monitoring/health.ts".to_string(),
                content: r#"import type { Request, Response } from 'express';

const STARTED_AT = Date.now();
const VERSION = process.env.APP_VERSION || '0.0.0';

export function healthHandler(_req: Request, res: Response) {
  res.json({
    status: 'ok',
    version: VERSION,
    uptimeSeconds: Math.floor((Date.now() - STARTED_AT) / 1000),
    db: true,
  });
}
"#.to_string(),
            },
            RecipeFile {
                path: "src/monitoring/logger.ts".to_string(),
                content: r#"import pino from 'pino';

export const logger = pino({
  level: process.env.LOG_LEVEL || 'info',
  base: { service: process.env.SERVICE_NAME || 'app' },
  timestamp: pino.stdTimeFunctions.isoTime,
});

export type Logger = typeof logger;
"#.to_string(),
            },
            RecipeFile {
                path: "src/monitoring/README.md".to_string(),
                content: r#"# Monitoring

## Endpoints
- `GET /health` — devuelve `{ status, version, uptimeSeconds, db }`. Conectalo con `app.get('/health', healthHandler)`.

## Logger
- Importa `logger` desde `src/monitoring/logger.ts`.
- Usa `logger.info({ event: 'user.login', userId })` con campos estructurados, no strings concatenados.
- Configurable con `LOG_LEVEL` (default `info`) y `SERVICE_NAME`.
"#.to_string(),
            },
        ],
        deps_to_install: vec!["pino".to_string()],
        dev_deps_to_install: vec![],
        env_vars_to_add: [
            ("APP_VERSION".to_string(), "0.0.0".to_string()),
            ("LOG_LEVEL".to_string(), "info".to_string()),
            ("SERVICE_NAME".to_string(), "app".to_string()),
        ]
        .into_iter()
        .collect(),
        scripts_to_add: HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_recipes() {
        let recipes = list_recipes();
        assert_eq!(recipes.len(), 9);
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

    // --- 3 recipes nuevas (G4 del plan v1.0.0) ---

    #[test]
    fn test_recipe_auth_jwt_apply_creates_files_and_env() {
        let dir = tempfile::tempdir().unwrap();
        let result = apply_recipe(dir.path().to_str().unwrap(), "auth-jwt").unwrap();
        assert!(result.files_created.contains(&"src/auth/jwt.ts".to_string()));
        assert!(result.files_created.contains(&"src/auth/middleware.ts".to_string()));
        assert!(result.files_created.contains(&"src/auth/routes.ts".to_string()));
        assert!(result.env_vars_added.contains(&"JWT_ACCESS_SECRET".to_string()));
        assert!(result.env_vars_added.contains(&"JWT_REFRESH_SECRET".to_string()));

        let routes = std::fs::read_to_string(dir.path().join("src/auth/routes.ts")).unwrap();
        assert!(routes.contains("/login"));
        assert!(routes.contains("/refresh"));
        assert!(routes.contains("/logout"));
    }

    #[test]
    fn test_recipe_database_sqlalchemy_apply_creates_alembic_skeleton() {
        let dir = tempfile::tempdir().unwrap();
        let result = apply_recipe(dir.path().to_str().unwrap(), "database-sqlalchemy").unwrap();
        assert!(result.files_created.contains(&"alembic.ini".to_string()));
        assert!(result.files_created.contains(&"alembic/env.py".to_string()));
        assert!(result.files_created.contains(&"app/db/session.py".to_string()));
        assert!(result.files_created.contains(&"app/db/models.py".to_string()));
        assert!(result.env_vars_added.contains(&"DATABASE_URL".to_string()));

        let session = std::fs::read_to_string(dir.path().join("app/db/session.py")).unwrap();
        assert!(session.contains("create_engine"));
        assert!(session.contains("DATABASE_URL"));
        let env_py = std::fs::read_to_string(dir.path().join("alembic/env.py")).unwrap();
        assert!(env_py.contains("target_metadata = Base.metadata"));
    }

    #[test]
    fn test_recipe_monitoring_apply_creates_health_and_logger() {
        let dir = tempfile::tempdir().unwrap();
        let result = apply_recipe(dir.path().to_str().unwrap(), "monitoring").unwrap();
        assert!(result.files_created.contains(&"src/monitoring/health.ts".to_string()));
        assert!(result.files_created.contains(&"src/monitoring/logger.ts".to_string()));
        assert!(result.env_vars_added.contains(&"LOG_LEVEL".to_string()));

        let health = std::fs::read_to_string(dir.path().join("src/monitoring/health.ts")).unwrap();
        assert!(health.contains("status"));
        assert!(health.contains("uptimeSeconds"));
    }

    #[test]
    fn test_new_recipes_visible_via_get() {
        assert!(get_recipe("auth-jwt").is_some());
        assert!(get_recipe("database-sqlalchemy").is_some());
        assert!(get_recipe("monitoring").is_some());
    }
}
