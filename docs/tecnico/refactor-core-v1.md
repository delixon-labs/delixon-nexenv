# Refactor Core v1 — Reestructuracion de `src-tauri/src/core/`

## Problema

`core/` tenia 19 archivos sueltos al mismo nivel mezclados con 5 subdirectorios existentes (`models/`, `utils/`, `catalog/`, `templates/`, `recipes/`). A medida que el proyecto crecia, la estructura se volvia inmantenible: archivos de dominios distintos mezclados, codigo duplicado en multiples archivos, y sin agrupacion logica.

## Que se hizo

### 1. Reorganizacion por dominio

Los 19 archivos sueltos se agruparon en 5 nuevos subdirectorios segun su responsabilidad:

| Subdirectorio | Archivos movidos | Responsabilidad |
|---|---|---|
| `project/` | storage, config, manifest, portable, notes | Ciclo de vida del proyecto |
| `analysis/` | detection, health, doctor, rules | Deteccion, diagnostico, validacion |
| `runtime/` | docker, ports, processes, scripts, git | Procesos externos y herramientas |
| `workspace/` | vscode, scaffold | Configuracion de IDE/editor |
| `history/` | env (antes snapshots), versioning | Snapshots y versionado |

Solo `error.rs` quedo en la raiz de `core/` por ser fundacional.

### 2. Backward compatibility via re-exports

`core/mod.rs` re-exporta todos los modulos en sus rutas originales:

```rust
pub use self::project::{config, manifest, notes, portable, storage};
pub use self::analysis::{detection, doctor, health, rules};
// etc.
```

**Cero cambios requeridos** en `commands/` o `bin/cli.rs`. Todos los `use crate::core::X` existentes siguen funcionando.

### 3. Consolidacion de codigo duplicado

**`find_workspace_file()`** — Estaba copiada identica en 3 archivos:
- `commands/projects.rs`
- `commands/shell.rs`
- `bin/cli.rs`

Consolidada en `core/utils/editor.rs`.

**`open_in_editor()`** — Logica de validar editor + buscar en PATH + detectar workspace + abrir, estaba repetida con variaciones en los mismos 3 archivos.

Consolidada en `core/utils/editor.rs` como una sola funcion reutilizable.

### 4. Fix de tests con race conditions

Tests de `storage`, `portable`, `templates` y `scaffold` escribian al mismo `projects.json` en paralelo, causando fallos intermitentes (3 de 133 tests).

Solucion: crate `serial_test` con `#[serial(disk)]` — tests que tocan disco se ejecutan secuencialmente entre modulos, el resto sigue en paralelo.

### 5. Cleanup de tests

Tests de `templates` y `scaffold` creaban proyectos en `projects.json` sin limpiarlos despues, dejando basura. Se agrego `cleanup_by_path()` al final de cada test.

### 6. Comando `unlink` en CLI

Agregado `nexenv-cli unlink <name>` para desvincular proyectos desde la terminal (paridad con el boton "Desvincular" de la GUI).

### 7. Reactividad GUI ante cambios externos

La GUI ahora detecta cambios en `projects.json` hechos por el CLI u otros procesos:
- Refresco silencioso (sin spinner) al pasar el mouse sobre la ventana o al hacerla visible
- Si un proyecto abierto fue eliminado externamente, navega automaticamente al dashboard

## Estructura final

Ver `docs/tecnico/estructura-backend.md` para la estructura completa documentada.

### 8. Migracion de storage a SQLite

Posterior al refactor de estructura, se implemento la capa de abstraccion de persistencia:

- Modulo `core/store/` con 6 traits sync + super-trait `Store`
- `JsonStore` como wrapper de funciones existentes (fallback)
- `SqliteStore` con rusqlite 0.39 bundled (SQLite 3.49.1) como backend principal
- OnceLock global (`store::get()`) para acceso unificado desde GUI, CLI y core
- Migracion automatica JSON → SQLite al primer arranque + backup
- 22 archivos migrados de `storage::X()` a `store::get().X()`
- Tests: jsdom para frontend (localStorage), serial_test para Rust (disco)

Ver `docs/tecnico/storage/MIGRATION_PLAN.md` para el plan original.

## Verificacion

- `cargo check` — compila sin errores
- `cargo test` — 139/139 tests pasan, 0 fallos (133 originales + 6 SqliteStore)
- `npx vitest --run` — 18/18 tests frontend pasan (jsdom environment)
- `npx tsc --noEmit` — frontend compila sin errores
- Ningun import existente se rompio
