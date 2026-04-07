# Migracion de Storage: JSON → SQLite + Arquitectura por Capas

## Contexto

Nexenv almacena actualmente toda su informacion (proyectos, config, env vars, snapshots, notas) en archivos JSON planos en `%LOCALAPPDATA%/nexenv/`. Esta decision fue correcta para la v1, pero se queda corta para:

- **Filtrado entre proyectos** (ej: "todos los proyectos con React que usan puerto 3000")
- **Deteccion de inconsistencias** (extensiones/tecnologias conflictivas)
- **Contexto para IA** (consultar N proyectos en una sola query)
- **Escalabilidad futura** a servidor multi-usuario, NAS empresarial, PostgreSQL

La solucion es migrar a **SQLite** con una **arquitectura de capas abstracta** que permita cambiar el backend sin reescribir la app.

---

## Arquitectura Propuesta

```
┌─────────────────────────────────────────────────────────┐
│                    Tauri Commands                        │
│  (projects.rs, environments.rs, config.rs, notes.rs)    │
│              API sin cambios → frontend intacto          │
├─────────────────────────────────────────────────────────┤
│          Commands reciben State<Arc<dyn Store>>          │
├─────────────────────────────────────────────────────────┤
│                    Store Traits                          │
│  ProjectStore · ConfigStore · NoteStore · EnvVarStore    │
│          SnapshotStore · EnvSnapshotStore                │
├────────────┬────────────────────┬───────────────────────┤
│ JsonStore  │    SqliteStore     │   (PostgresStore)     │
│ (legacy/   │    (nueva impl)    │   (futuro)            │
│  fallback) │                    │                       │
└────────────┴────────────────────┴───────────────────────┘
```

**Principio clave**: los Tauri commands nunca llaman directamente a funciones de I/O. Reciben un `State<Arc<dyn Store>>` inyectado, lo que permite cambiar el backend de almacenamiento sin tocar ni un solo command ni el frontend.

---

## Store Traits (Capa de Abstraccion)

Seis traits pequenos (Interface Segregation Principle) + un super-trait compuesto:

```rust
// src-tauri/src/core/store/traits.rs

#[async_trait]
pub trait ProjectStore: Send + Sync {
    async fn list_projects(&self) -> Result<Vec<Project>, NexenvError>;
    async fn get_project(&self, id: &str) -> Result<Option<Project>, NexenvError>;
    async fn create_project(&self, project: &Project) -> Result<(), NexenvError>;
    async fn update_project(&self, project: &Project) -> Result<(), NexenvError>;
    async fn delete_project(&self, id: &str) -> Result<(), NexenvError>;
    // Queries avanzadas (para IA y deteccion)
    async fn find_projects_by_tag(&self, tag: &str) -> Result<Vec<Project>, NexenvError>;
    async fn find_projects_by_status(&self, status: &str) -> Result<Vec<Project>, NexenvError>;
    async fn search_projects(&self, query: &str) -> Result<Vec<Project>, NexenvError>;
}

#[async_trait]
pub trait ConfigStore: Send + Sync {
    async fn load_config(&self) -> Result<NexenvConfig, NexenvError>;
    async fn save_config(&self, config: &NexenvConfig) -> Result<(), NexenvError>;
}

#[async_trait]
pub trait NoteStore: Send + Sync {
    async fn get_notes(&self, project_id: &str) -> Result<Vec<ProjectNote>, NexenvError>;
    async fn add_note(&self, project_id: &str, note: &ProjectNote) -> Result<(), NexenvError>;
    async fn delete_note(&self, project_id: &str, note_id: &str) -> Result<(), NexenvError>;
}

#[async_trait]
pub trait EnvVarStore: Send + Sync {
    async fn load_env_vars(&self, project_id: &str) -> Result<HashMap<String, String>, NexenvError>;
    async fn save_env_vars(&self, project_id: &str, vars: &HashMap<String, String>) -> Result<(), NexenvError>;
    async fn delete_env_vars(&self, project_id: &str) -> Result<(), NexenvError>;
}

#[async_trait]
pub trait SnapshotStore: Send + Sync {
    async fn save_snapshot(&self, project_id: &str, snapshot: &Snapshot) -> Result<(), NexenvError>;
    async fn list_snapshots(&self, project_id: &str) -> Result<Vec<Snapshot>, NexenvError>;
    async fn get_snapshot(&self, project_id: &str, version: u32) -> Result<Option<Snapshot>, NexenvError>;
    async fn next_version(&self, project_id: &str) -> Result<u32, NexenvError>;
}

#[async_trait]
pub trait EnvSnapshotStore: Send + Sync {
    async fn save_env_snapshot(&self, project_id: &str, snapshot: &EnvSnapshot) -> Result<(), NexenvError>;
    async fn load_env_snapshot(&self, project_id: &str) -> Result<Option<EnvSnapshot>, NexenvError>;
}

// Super-trait compuesto
pub trait Store:
    ProjectStore + ConfigStore + NoteStore + EnvVarStore + SnapshotStore + EnvSnapshotStore {}

impl<T> Store for T where
    T: ProjectStore + ConfigStore + NoteStore + EnvVarStore + SnapshotStore + EnvSnapshotStore {}
```

### Por que `rusqlite` y no `sqlx`

- SQLite es sincrono por naturaleza
- `rusqlite` con feature `bundled` compila SQLite desde fuente (no requiere libsqlite3 en Windows)
- Se usa `tokio::task::spawn_blocking` dentro de cada metodo async
- Cuando se implemente `PostgresStore`, ese usara `sqlx` con async nativo
- Los traits ya son async, el cambio sera transparente

---

## Schema SQLite

```sql
-- migrations/001_initial.sql

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
    manifest    TEXT NOT NULL  -- JSON serializado de ProjectManifest
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_snapshots_unique ON snapshots(project_id, version);

-- Snapshots de entorno
CREATE TABLE IF NOT EXISTS env_snapshots (
    project_id  TEXT PRIMARY KEY REFERENCES projects(id) ON DELETE CASCADE,
    timestamp   TEXT NOT NULL,
    runtimes    TEXT NOT NULL,  -- JSON array de RuntimeSnapshot
    deps_hash   TEXT NOT NULL
);

-- Full-text search (para IA y busqueda avanzada)
CREATE VIRTUAL TABLE IF NOT EXISTS projects_fts USING fts5(
    name, description, tags, content='projects', content_rowid='rowid'
);
```

### Decisiones del schema

| Decision | Razon |
|----------|-------|
| `manifest` como JSON blob en snapshots | Es complejo (12+ campos), solo se consulta como unidad |
| `runtimes` y `tags` normalizados | Permiten queries con JOIN y filtros por tag/runtime |
| `ON DELETE CASCADE` | Borrar un proyecto limpia automaticamente tablas hijas |
| FTS5 | Busqueda full-text para `search_projects` y contexto IA |
| WAL journal mode | Mejor rendimiento en lecturas concurrentes |
| `config` con CHECK id=1 | Garantiza singleton a nivel de DB |

---

## Plan de Migracion de Datos (JSON → SQLite)

La migracion es **automatica al primer arranque post-actualizacion**:

1. La app intenta abrir/crear `{LOCALAPPDATA}/nexenv/nexenv.db`
2. Si la DB no tiene tablas: aplica schema inicial
3. Detecta si existen archivos JSON legacy
4. Si existen: ejecuta migracion **dentro de una transaccion SQL**
5. Si la transaccion tiene exito: mueve JSON a `{LOCALAPPDATA}/nexenv/json_backup/`
6. Si falla: ROLLBACK, log del error, fallback a JsonStore

### Mapeo de datos

| Origen JSON | Destino SQLite |
|---|---|
| `config.json` | tabla `config` |
| `projects.json` (array) | `projects` + `project_runtimes` + `project_tags` |
| `envs/{id}.json` | tabla `env_vars` |
| `notes/{id}.json` | tabla `notes` |
| `snapshots/{id}/v*.json` | tabla `snapshots` |
| `env_snapshots/{id}.json` | tabla `env_snapshots` |

---

## Fases de Implementacion

### FASE 0 — Preparacion (sin cambios funcionales)

**Objetivo**: crear la estructura de modulos sin romper nada.

- Crear modulo `core/store/` con `mod.rs` y `traits.rs`
- Anadir `async-trait` a Cargo.toml
- Anadir variante `Database(String)` a `NexenvError`

**Verificacion**: `cargo check` compila, `cargo test` pasa todo.

### FASE 1 — JsonStore como wrapper de traits

**Objetivo**: implementar los traits usando las funciones JSON existentes.

- Crear `core/store/json_store.rs`
- Cada metodo usa `tokio::task::spawn_blocking` para llamar a las funciones actuales
- Metodos de query avanzada hacen filtrado en memoria (temporalmente)

**Verificacion**: tests de JsonStore producen mismos resultados.

### FASE 2 — Inyeccion de Store en Commands

**Objetivo**: los commands pasan a usar `State<Arc<dyn Store>>`.

- En `lib.rs`: instanciar `JsonStore`, wrappear en `Arc<dyn Store>`, registrar con `.manage()`
- Modificar los commands para recibir `State<Arc<dyn Store>>`
- La logica de negocio en commands (validacion, apertura de editor) no cambia

**Verificacion**: `cargo tauri dev` arranca, todas las operaciones del frontend funcionan identicas.

### FASE 3 — Implementacion SqliteStore

**Objetivo**: backend SQLite completo.

- Anadir `rusqlite = { version = "0.39", features = ["bundled"] }`
- Crear `core/store/sqlite_store.rs` con `SqliteStore { conn: Mutex<Connection> }`
- Crear sistema de migraciones versionadas
- Implementar los 6 traits con queries SQL

**Verificacion**: tests con `:memory:` DB, CRUD completo para cada entidad.

### FASE 4 — Migracion automatica JSON → SQLite

**Objetivo**: upgrade transparente para usuarios existentes.

- Crear funcion `migrate_json_to_sqlite()`
- Ejecutar en transaccion, backup de JSON si exito

**Verificacion**: test con fixtures JSON → verificar datos en SQLite.

### FASE 5 — Switch a SQLite como default

**Objetivo**: SQLite es el backend principal.

- En `lib.rs`: intentar `SqliteStore::new_with_migration()`, fallback a `JsonStore` si falla

**Verificacion**: instalacion limpia y upgrade desde JSON funcionan.

### FASE 6 — Queries avanzadas

**Objetivo**: habilitar las capacidades que motivaron la migracion.

- `search_projects` con FTS5
- `find_projects_by_tag` con JOIN SQL
- Nuevos Tauri commands si se necesitan

**Verificacion**: busqueda full-text funciona, queries de filtrado correctas.

---

## Archivos a Crear

| Archivo | Proposito |
|---------|-----------|
| `src-tauri/src/core/store/mod.rs` | Modulo principal del store |
| `src-tauri/src/core/store/traits.rs` | Definicion de los 6 traits |
| `src-tauri/src/core/store/json_store.rs` | Implementacion JSON (wrapper) |
| `src-tauri/src/core/store/sqlite_store.rs` | Implementacion SQLite |
| `src-tauri/src/core/store/migration.rs` | Migracion JSON → SQLite |
| `src-tauri/src/core/store/migrations.rs` | Sistema de migraciones SQL |
| `src-tauri/migrations/001_initial.sql` | Schema SQL inicial |

## Archivos a Modificar

| Archivo | Cambio |
|---------|--------|
| `src-tauri/Cargo.toml` | +rusqlite, +async-trait |
| `src-tauri/src/core/mod.rs` | +pub mod store |
| `src-tauri/src/core/error.rs` | +variante Database |
| `src-tauri/src/lib.rs` | Inicializacion store + managed state |
| `src-tauri/src/commands/projects.rs` | Recibir State<Arc<dyn Store>> |
| `src-tauri/src/commands/environments.rs` | Recibir State<Arc<dyn Store>> |
| `src-tauri/src/commands/config.rs` | Recibir State<Arc<dyn Store>> |
| `src-tauri/src/commands/notes.rs` | Recibir State<Arc<dyn Store>> |
| `src-tauri/src/commands/snapshots.rs` | Recibir State<Arc<dyn Store>> |
| `src-tauri/src/commands/versioning.rs` | Recibir State<Arc<dyn Store>> |
| `src-tauri/src/commands/portable.rs` | Recibir State<Arc<dyn Store>> |

## Sin Cambios

- `src-tauri/src/core/project/manifest.rs` — sigue en `.nexenv/manifest.yaml` (es del proyecto)
- `src-tauri/src/core/catalog/` — YAML read-only embebido
- `src-tauri/src/core/recipes/` — hardcoded
- `src-tauri/src/core/runtime/` — no usa storage
- `src-tauri/src/core/analysis/` — lee del filesystem del proyecto
- `src/` — **frontend React completo, cero cambios**

## Dependencias Rust a Anadir

```toml
rusqlite = { version = "0.39", features = ["bundled"] }
async-trait = "0.1"
```

`bundled` compila SQLite 3.49.1 desde fuente — no requiere libsqlite3 instalada en ningun SO. Anade ~1.5MB al binario.

---

## Preparacion para PostgreSQL Futuro

Los traits estan disenados para que `PostgresStore` sea drop-in:

- Todos los metodos son `async` — `sqlx` con postgres es async nativo
- Los tipos de retorno son modelos de dominio (`Project`, `ProjectNote`, etc.)
- No hay nada especifico de SQLite en las signatures
- Para habilitar: anadir `sqlx`, implementar los 6 traits, leer connection string de config

```
Ruta de crecimiento:

  v1 (hoy)        → JSON files
  v2 (esta migracion) → SQLite local
  v3 (futuro)      → PostgreSQL + API server + Nexenv como cliente
```

---

## Estructura final de disco post-migracion

```
{LOCALAPPDATA}/nexenv/
├── nexenv.db                      # SQLite (todo centralizado)
├── config.json                     # Solo si falla SQLite (fallback)
└── json_backup/                    # Backup automatico post-migracion
    ├── projects.json
    ├── config.json
    ├── envs/
    ├── notes/
    ├── snapshots/
    └── env_snapshots/

{PROJECT_PATH}/
└── .nexenv/
    └── manifest.yaml               # Sigue en el proyecto (sin cambios)
```
