PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

-- Control de migraciones
CREATE TABLE IF NOT EXISTS _migrations (
    version     INTEGER PRIMARY KEY,
    name        TEXT NOT NULL,
    applied_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Config global (singleton)
CREATE TABLE IF NOT EXISTS config (
    id                  INTEGER PRIMARY KEY CHECK (id = 1),
    version             TEXT NOT NULL DEFAULT '1.0.0',
    data_dir            TEXT NOT NULL DEFAULT '',
    default_editor      TEXT NOT NULL DEFAULT 'code',
    theme               TEXT NOT NULL DEFAULT 'dark',
    language            TEXT NOT NULL DEFAULT 'es',
    auto_check_updates  INTEGER NOT NULL DEFAULT 1
);
INSERT OR IGNORE INTO config (id) VALUES (1);

-- Proyectos
CREATE TABLE IF NOT EXISTS projects (
    id              TEXT PRIMARY KEY,
    name            TEXT NOT NULL,
    path            TEXT NOT NULL UNIQUE,
    description     TEXT,
    status          TEXT NOT NULL DEFAULT 'active'
                    CHECK (status IN ('active', 'idle', 'archived')),
    created_at      TEXT NOT NULL,
    last_opened_at  TEXT,
    template_id     TEXT
);
CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status);
CREATE INDEX IF NOT EXISTS idx_projects_name ON projects(name);

-- Runtimes por proyecto
CREATE TABLE IF NOT EXISTS project_runtimes (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id  TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    runtime     TEXT NOT NULL,
    version     TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_runtimes_project ON project_runtimes(project_id);

-- Tags por proyecto
CREATE TABLE IF NOT EXISTS project_tags (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id  TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    tag         TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_tags_project ON project_tags(project_id);
CREATE INDEX IF NOT EXISTS idx_tags_tag ON project_tags(tag);
CREATE UNIQUE INDEX IF NOT EXISTS idx_tags_unique ON project_tags(project_id, tag);

-- Variables de entorno
CREATE TABLE IF NOT EXISTS env_vars (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id  TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    key         TEXT NOT NULL,
    value       TEXT NOT NULL DEFAULT ''
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_env_vars_unique ON env_vars(project_id, key);

-- Notas
CREATE TABLE IF NOT EXISTS notes (
    id          TEXT PRIMARY KEY,
    project_id  TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    text        TEXT NOT NULL,
    created_at  TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_notes_project ON notes(project_id);

-- Snapshots de manifest (versionado)
CREATE TABLE IF NOT EXISTS snapshots (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id  TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    version     INTEGER NOT NULL,
    timestamp   TEXT NOT NULL,
    manifest    TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_snapshots_unique ON snapshots(project_id, version);

-- Snapshots de entorno
CREATE TABLE IF NOT EXISTS env_snapshots (
    project_id  TEXT PRIMARY KEY REFERENCES projects(id) ON DELETE CASCADE,
    timestamp   TEXT NOT NULL,
    runtimes    TEXT NOT NULL,
    deps_hash   TEXT NOT NULL
);
