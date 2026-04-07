# Nexenv — Especificacion Tecnica del Manifest

> Documento tecnico del nucleo declarativo. Define el schema, invariantes, validacion, normalizacion y reglas de evolucion del Project Manifest.
>
> Ultima actualizacion: 2026-03-27
>
> Implementacion: `src-tauri/src/core/manifest.rs` (500 lineas)

---

## 1. Que es el manifest

El archivo `.nexenv/manifest.yaml` es el **contrato central** que define que es un proyecto para Nexenv. Todas las capas (workspace, scaffolding, operacion, versionado) leen y escriben sobre el.

Sin manifest, Nexenv no sabe que es el proyecto. Con manifest invalido, Nexenv no lo guarda.

---

## 2. Schema actual (v1)

```yaml
# .nexenv/manifest.yaml
schemaVersion: 1
name: "mi-proyecto"
projectType: "api"
profile: "standard"
runtime: "node"
technologies:
  - "nodejs"
  - "express"
  - "postgresql"
services:
  - name: "postgresql"
    port: 5432
    docker: true
    healthCheck: "pg_isready -U postgres"
envVars:
  required:
    - "DATABASE_URL"
    - "API_SECRET"
  optional:
    - "REDIS_URL"
commands:
  dev: "npm run dev"
  build: "npm run build"
  test: "npm run test"
ports:
  - 3000
  - 5432
recipesApplied:
  - "testing-vitest"
  - "docker"
healthChecks:
  - name: "api"
    command: "curl -f http://localhost:3000/health"
metadata:
  description: "API REST de gestion de ventas"
  createdAt: "2026-03-27T10:00:00Z"
  author: "equipo-backend"
editor: "code"
```

### Campos del schema

| Campo | Tipo | Requerido | Default | Descripcion |
|---|---|---|---|---|
| `schemaVersion` | `u32` | Si | `1` | Version del schema. Permite migraciones futuras |
| `name` | `String` | Si | — | Nombre del proyecto. No puede estar vacio |
| `projectType` | `String` | No | `""` | Tipo: api, frontend, fullstack, cli, etc. |
| `profile` | `String` | No | `""` | Perfil de madurez: rapid, standard, production |
| `runtime` | `String` | No | `""` | Runtime principal: node, python, rust, go |
| `technologies` | `Vec<String>` | No | `[]` | Lista de tecnologias del proyecto |
| `services` | `Vec<ManifestService>` | No | `[]` | Servicios de infraestructura |
| `envVars` | `ManifestEnvVars` | No | vacío | Variables de entorno (solo NOMBRES, nunca valores) |
| `commands` | `HashMap<String, String>` | No | `{}` | Scripts del proyecto |
| `ports` | `Vec<u16>` | No | `[]` | Puertos que usa el proyecto |
| `recipesApplied` | `Vec<String>` | No | `[]` | Recipes aplicadas al proyecto |
| `healthChecks` | `Vec<ManifestHealthCheck>` | No | `[]` | Checks de salud |
| `metadata` | `ManifestMetadata` | No | vacío | Description, createdAt, author |
| `editor` | `Option<String>` | No | `None` | Editor especifico del proyecto (override global) |

### Structs auxiliares

```rust
pub struct ManifestService {
    pub name: String,
    pub docker: bool,
    pub port: u16,
    pub health_check: Option<String>,
}

pub struct ManifestEnvVars {
    pub required: Vec<String>,
    pub optional: Vec<String>,
}

pub struct ManifestHealthCheck {
    pub name: String,
    pub command: Option<String>,
    pub endpoint: Option<String>,
}

pub struct ManifestMetadata {
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub author: Option<String>,
}
```

---

## 3. Invariantes (reglas que SIEMPRE se cumplen)

Estas reglas son aplicadas por `validate_manifest()` y `normalize_manifest()`. Nunca se escribe un manifest al disco que las viole.

### Validacion (`validate_manifest`)

| Regla | Que rechaza | Error |
|---|---|---|
| Name no vacio | `name: ""` | `InvalidManifest("name vacio")` |
| Schema version > 0 | `schemaVersion: 0` | `InvalidManifest("schema_version 0")` |
| Puertos > 0 | `ports: [0, 3000]` | `InvalidManifest("puerto 0")` |
| Sin puertos duplicados | `ports: [3000, 3000]` | `InvalidManifest("puertos duplicados")` |
| Env vars sin valores | `required: ["KEY=value"]` | `InvalidManifest("env var con valor")` |

### Normalizacion (`normalize_manifest`)

| Regla | Que corrige | Automatico |
|---|---|---|
| Schema version 0 → 1 | Manifests sin version | Si |
| Deduplicar puertos | `[3000, 5432, 3000]` → `[3000, 5432]` | Si |
| Deduplicar tecnologias | `["node", "node"]` → `["node"]` | Si |
| Deduplicar recipes | `["docker", "docker"]` → `["docker"]` | Si |
| Trim whitespace en name | `"  mi-app  "` → `"mi-app"` | Si |
| Trim whitespace en technologies | `["  react  "]` → `["react"]` | Si |

### Flujo de guardado

```
save_manifest(path, manifest)
  │
  ├── 1. Clonar manifest (no mutar el original)
  ├── 2. normalize_manifest(&mut clone)
  ├── 3. validate_manifest(&clone)?  ← si falla, NO se escribe
  ├── 4. Serializar a YAML
  └── 5. Escribir a .nexenv/manifest.yaml
```

**Garantia:** Si `save_manifest` retorna `Ok(())`, el manifest en disco es valido y normalizado. Si retorna `Err`, el disco no se toco.

### Flujo de carga

```
load_manifest(path)
  │
  ├── 1. Leer .nexenv/manifest.yaml
  ├── 2. Deserializar
  └── 3. normalize_manifest (upgrade schema si es antiguo)
```

---

## 4. Separacion de datos (por diseno)

| Tipo de dato | Donde vive | Por que |
|---|---|---|
| **Schema del proyecto** (que necesita) | `.nexenv/manifest.yaml` | Declarativo, versionable, portable |
| **Valores de env vars** (secretos) | `~/.local/share/nexenv/envs/{id}.json` | Sensibles, varian por maquina, fuera del repo |
| **Notas** | `~/.local/share/nexenv/notes/{id}.json` | Efimeras, alta frecuencia, no son schema |
| **Snapshots** | `~/.local/share/nexenv/snapshots/{id}/` | Historico, copias del manifest en el tiempo |
| **Configuracion global** | `~/.local/share/nexenv/config.json` | Preferencias del usuario, no del proyecto |

**Regla critica:** El manifest guarda **nombres** de env vars (`DATABASE_URL`), nunca **valores** (`postgresql://localhost:5432/mydb`). Los valores van en el JSON aislado.

---

## 5. Quien lee y escribe el manifest

### Lectores (modulos que leen el manifest)

| Modulo | Para que lo lee |
|---|---|
| `health.rs` | Verificar que servicios, puertos, deps estan funcionando |
| `doctor.rs` | Diagnosticar estado del proyecto |
| `scripts.rs` | Saber que comandos ejecutar |
| `docker.rs` | Saber que servicios levantar |
| `portable.rs` | Incluir manifest en export |
| `versioning.rs` | Guardar snapshot del estado actual |
| `snapshots.rs` | Comparar estado entre momentos |
| `detection.rs` | Comparar con scan actual |
| `cli.rs` | Mostrar manifest al usuario |
| **GUI** (9 tabs) | Renderizar estado del proyecto |

### Escritores (modulos que modifican el manifest)

| Modulo | Que escribe |
|---|---|
| `scaffold.rs` | Genera manifest completo al crear proyecto |
| `recipes/mod.rs` | Agrega recipes a `recipesApplied`, agrega deps |
| `portable.rs` | Restaura manifest desde import |
| `versioning.rs` | Restaura manifest desde rollback |
| `detection.rs` | Genera manifest desde scan de proyecto existente |

**Todos los escritores pasan por `save_manifest()`** — la validacion y normalizacion son obligatorias.

---

## 6. Migraciones de schema

### Actual: v1

La unica version que existe. `normalize_manifest` upgradea `schemaVersion: 0` a `1` automaticamente.

### Cuando agregar v2

Solo si:
- Se agrega un campo que cambia el significado de campos existentes
- Se renombra o elimina un campo existente
- Se cambia el tipo de un campo

**No requiere nueva version:** agregar campos opcionales con `#[serde(default)]`.

### Como migrar

```rust
fn migrate_v1_to_v2(manifest: &mut ProjectManifest) {
    // Transformar campos
    manifest.schema_version = 2;
}

fn normalize_manifest(manifest: &mut ProjectManifest) {
    if manifest.schema_version == 0 {
        manifest.schema_version = 1;
    }
    if manifest.schema_version == 1 {
        migrate_v1_to_v2(manifest);
    }
    // ... normalizacion comun
}
```

---

## 7. Campos prohibidos (lo que NUNCA entra en el manifest)

| Campo | Por que no |
|---|---|
| Valores de env vars | Sensibles. Van en `envs/*.json` |
| Tokens/secrets/passwords | Nunca en un archivo que puede ir al repo |
| Estado observado (runtime instalado, puerto libre) | Eso lo calcula `doctor`/`health` en tiempo real |
| Notas del developer | Efimeras. Van en `notes/*.json` |
| Configuracion del editor (settings.json) | Va en `.vscode/` o equivalente |
| Metricas de uso | Van en `metrics.json` aislado |
| Timestamps de ultima apertura | Va en el registro de proyectos (`projects.json`) |

---

## 8. Regla de admision de campos nuevos

> **"¿Lo leen al menos 2 modulos distintos O es critico para reconstruir/entender el proyecto?"**

Si la respuesta es no, el campo no entra. Va a:
- **Config local** si es preferencia del usuario
- **Notes** si es efimero
- **Otro JSON** si es datos operativos

### Proceso para agregar un campo

1. ¿Cumple la regla de admision? Si no, rechazar
2. ¿Puede ser opcional con `#[serde(default)]`? Siempre que sea posible
3. ¿Rompe manifests existentes? Si si, requiere nueva `schema_version`
4. ¿Necesita validacion? Agregar a `validate_manifest()`
5. ¿Necesita normalizacion? Agregar a `normalize_manifest()`
6. Agregar tests que cubran el campo nuevo
7. Actualizar este documento

---

## 9. on_open — Disciplina futura

El campo `on_open` (ejecutar comandos al abrir proyecto) **no existe aun** en el schema. Cuando se implemente:

- Opt-in explicito (desactivado por defecto)
- Solo comandos escritos a mano por el usuario
- Preview antes de ejecutar
- Separar "preparar contexto" (siempre) de "ejecutar scripts" (solo si el usuario lo pidio)
- Tiempo limite: si on_open tarda >3s, debe poder cancelarse sin afectar el `open`
- Nunca romper la promesa de `open` en <2 segundos

---

## 10. Tests existentes (verificados)

| Test | Que verifica | Archivo |
|---|---|---|
| `test_save_and_load_manifest` | Roundtrip save → load | manifest.rs |
| `test_default_manifest_has_schema_version` | Default tiene schema_version=1 | manifest.rs |
| `test_validate_rejects_empty_name` | Rechaza name vacio | manifest.rs |
| `test_validate_rejects_zero_port` | Rechaza puerto 0 | manifest.rs |
| `test_validate_rejects_duplicate_ports` | Rechaza puertos duplicados | manifest.rs |
| `test_validate_rejects_env_value` | Rechaza `KEY=value` en env vars | manifest.rs |
| `test_normalize_deduplicates` | Deduplica puertos, techs, recipes | manifest.rs |
| `test_normalize_trims_whitespace` | Limpia whitespace en name y techs | manifest.rs |
| `test_normalize_upgrades_schema` | Upgradea schema_version 0→1 | manifest.rs |
| `test_save_validates_before_writing` | save_manifest rechaza invalidos | manifest.rs |
| `test_generate_manifest_from_project` | Genera manifest desde Project | manifest.rs |
| `test_save_and_list_snapshots` | Snapshots preservan manifest | versioning.rs |
| `test_diff_snapshots` | Diff detecta cambios en techs/recipes | versioning.rs |
| `test_rollback_snapshot` | Rollback restaura manifest anterior | versioning.rs |
| `test_export_import_roundtrip` | Export/import preserva manifest | portable.rs |
| `test_export_env_keys_not_values` | Export NO incluye valores de env | portable.rs |

---

*Documento derivado de PLAN.md, seccion 9 (Arquitectura por capas) y seccion 22 (Checklist Capa 0). Verificado contra `src-tauri/src/core/manifest.rs`.*
