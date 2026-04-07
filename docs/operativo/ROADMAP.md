# Nexenv — Roadmap Operativo

> Documento de ejecucion. Se actualiza cada sprint. El plan maestro (PLAN.md) no se toca.
>
> Ultima actualizacion: 2026-03-27

---

## Sprint 1 — "Que se pueda usar de verdad" (30 dias)

> **Objetivo:** Un developer descarga Nexenv, instala en <5 min, abre su primer proyecto con el entorno correcto, y vuelve al dia siguiente.

### 1.1 `open` perfecto (<2 segundos)

| Tarea | Estado | Criterio de done |
|---|---|---|
| Medir tiempo actual de `open` en Windows/Linux/macOS | [ ] Pendiente | Baseline documentado |
| Optimizar apertura de editor (eliminar delays innecesarios) | [ ] Pendiente | <2s medido en los 3 SO |
| Activar runtime correcto al hacer `open` (nvm/fnm, pyenv, rustup) | [ ] Pendiente | `open` activa Node/Python/Rust automaticamente |
| Si runtime no esta instalado, sugerir instalacion | [ ] Pendiente | Mensaje claro con instrucciones |
| Si algo falla, error con los 4 campos (intento/detecto/fallo/hacer) | [ ] Pendiente | 0 errores sin "que hacer" |

**Dependencias:** Requiere detectar nvm/fnm/pyenv/rustup en el sistema.

### 1.2 Instalacion global del CLI

| Tarea | Estado | Criterio de done |
|---|---|---|
| Build del binario `nexenv` (no `nexenv-cli`) para release | [ ] Pendiente | Binario unico funcional |
| Instalador Windows (scoop o winget manifest) | [ ] Pendiente | `scoop install nexenv` funciona |
| Instalador macOS (brew tap) | [ ] Pendiente | `brew install nexenv` funciona |
| Instalador Linux (apt/snap o binario en releases) | [ ] Pendiente | Descarga + PATH funciona |
| La GUI instala el CLI automaticamente al instalarse | [ ] Pendiente | CLI disponible post-install GUI |
| `nexenv doctor` funciona desde cualquier terminal en cualquier SO | [ ] Pendiente | Test manual en 3 SO |

### 1.3 CI/CD multi-SO minimo

| Tarea | Estado | Criterio de done |
|---|---|---|
| GitHub Actions: build en Windows, Ubuntu, macOS | [ ] Pendiente | Los 3 builds pasan |
| Ejecutar tests de Rust (`cargo test`) en los 3 SO | [ ] Pendiente | 0 fallos en CI |
| Ejecutar tests de frontend (`vitest`) en CI | [ ] Pendiente | 0 fallos en CI |
| Build de binarios release en CI (artifacts) | [ ] Pendiente | Binarios descargables |

### 1.4 Errores con los 4 campos

| Tarea | Estado | Criterio de done |
|---|---|---|
| Auditar errores en `open` — los 4 campos | [ ] Pendiente | Todos los errores de open cubren intento/detecto/fallo/hacer |
| Auditar errores en `doctor` — los 4 campos | [ ] Pendiente | Idem |
| Auditar errores en `scan` — los 4 campos | [ ] Pendiente | Idem |
| Auditar errores en `add` (recipes) — los 4 campos | [ ] Pendiente | Idem |

**Entregable Sprint 1:** Un developer puede instalar Nexenv globalmente, hacer `nexenv open mi-proyecto` y tener su entorno activado en <2 segundos. CI corre en los 3 SO.

---

## Sprint 2 — "Que sea completo" (30 dias)

> **Objetivo:** Las capas 0-3 quedan blindadas. La propuesta de valor de recipes se completa.

### 2.1 Tests automatizados de templates

| Tarea | Estado | Criterio de done |
|---|---|---|
| Test: `node-express` se genera y `npm install` pasa | [ ] Pendiente | CI verde |
| Test: `react-vite` se genera y `npm run build` pasa | [ ] Pendiente | CI verde |
| Test: `python-fastapi` se genera y `pip install` pasa | [ ] Pendiente | CI verde |
| Test: `python-django` se genera y `pip install` pasa | [ ] Pendiente | CI verde |
| Test: `fullstack-react-python` se genera sin errores | [ ] Pendiente | CI verde |
| Test: `rust-cli` se genera y `cargo build` pasa | [ ] Pendiente | CI verde |
| Test: `docker-compose` se genera con YAML valido | [ ] Pendiente | CI verde |

### 2.2 3 Recipes criticas

| Recipe | Estado | Criterio de done |
|---|---|---|
| **auth** (NextAuth/Clerk/JWT) | [ ] Pendiente | Se aplica a proyecto Node, genera archivos, actualiza manifest, preview funciona |
| **database-orm** (PostgreSQL+Prisma / SQLAlchemy) | [ ] Pendiente | Se aplica a proyecto Node o Python, genera schema/models, docker-compose, env vars |
| **monitoring** (health endpoint + logging) | [ ] Pendiente | Se aplica a proyecto Node o Python, genera endpoint + logger config |

**Nota:** Las recipes existentes (vitest, pytest, docker, ci-github, biome, prisma) ya funcionan con preview. Las nuevas deben seguir el mismo patron.

### 2.3 Patron preview/diff/confirm en rutas principales

| Comando | Preview implementado | Confirm implementado | Estado |
|---|---|---|---|
| `add` (recipes) | [x] Si (`--preview`) | [ ] Pendiente | Falta confirm interactivo |
| `scaffold` / `new` | [x] Si (`preview_scaffold`) | [ ] Pendiente | Falta confirm en CLI |
| `snapshot rollback` | [ ] Pendiente | [ ] Pendiente | Debe mostrar diff antes de restaurar |
| `import` | [ ] Pendiente | [ ] Pendiente | Debe mostrar que proyecto va a crear |

### 2.4 Errores restantes con los 4 campos

| Tarea | Estado | Criterio de done |
|---|---|---|
| Auditar TODOS los mensajes de error del CLI | [ ] Pendiente | 0 errores sin "que hacer" |
| Auditar errores en la GUI (toasts, modales) | [ ] Pendiente | Cada error guia al usuario |

**Entregable Sprint 2:** 9 recipes totales (6 existentes + 3 nuevas). Todos los templates pasan tests en CI. Patron preview/confirm en todas las rutas destructivas.

---

## Sprint 3 — "Beta gate" (15 dias)

> **Objetivo:** Verificar que todo lo anterior funciona como un todo. Preparar para beta testers.

### 3.1 Beta gate checklist

> **No se lanza beta si alguno de estos es rojo.**

| Criterio | Estado | Verificado |
|---|---|---|
| `nexenv` instalable globalmente en Windows | [ ] | [ ] |
| `nexenv` instalable globalmente en macOS | [ ] | [ ] |
| `nexenv` instalable globalmente en Linux | [ ] | [ ] |
| `nexenv open` activa runtime en <2s | [ ] | [ ] |
| CI pasa en los 3 SO | [ ] | [ ] |
| 7 templates se generan sin errores en CI | [ ] | [ ] |
| 9 recipes se aplican sin errores | [ ] | [ ] |
| `doctor` detecta problemas reales y sugiere fix | [ ] | [ ] |
| `health` cubre los checks criticos | [ ] | [ ] |
| Export/import transporta manifest completo | [ ] | [ ] |
| 0 errores sin "que hacer" en rutas principales | [ ] | [ ] |
| GUI funciona en Windows (campo de batalla principal) | [ ] | [ ] |
| GUI funciona en macOS | [ ] | [ ] |
| GUI funciona en Linux | [ ] | [ ] |

### 3.2 Metricas locales de primera semana

| Tarea | Estado | Criterio de done |
|---|---|---|
| Crear `metrics.json` en data dir | [ ] Pendiente | Se crea al primer uso |
| Trackear: tiempo hasta primer `open` | [ ] Pendiente | Registrado localmente |
| Trackear: proyectos registrados en 7 dias | [ ] Pendiente | Registrado localmente |
| Trackear: uso de `doctor`/`health` | [ ] Pendiente | Registrado localmente |
| Trackear: recipes aplicadas | [ ] Pendiente | Registrado localmente |
| Trackear: errores encontrados | [ ] Pendiente | Registrado localmente |
| Pantalla en settings para ver metricas propias | [ ] Pendiente | Solo lectura, datos locales |

### 3.3 Smoke test manual completo

| Flujo | Estado |
|---|---|
| Instalar Nexenv desde cero en Windows limpio | [ ] |
| Crear proyecto con scaffold wizard (GUI) | [ ] |
| Crear proyecto con `nexenv new` (CLI) | [ ] |
| Scan de proyecto existente real | [ ] |
| Aplicar recipe a proyecto existente | [ ] |
| Export → Import en otra maquina/carpeta | [ ] |
| `open` abre editor con runtime correcto | [ ] |
| `doctor` en maquina sin Node instalado | [ ] |
| `health` en proyecto sin .gitignore | [ ] |
| Snapshot save → modificar → diff → rollback | [ ] |

**Entregable Sprint 3:** Beta lista para primeros 100 testers. Metricas locales activas. Smoke test completo en 3 SO.

---

## Estado real verificado (2026-03-27)

> Verificado contra el codigo fuente, no contra el plan.

### Lo que SI esta implementado y funciona

| Area | Detalle | Verificado |
|---|---|---|
| **CLI** | 23 comandos funcionales (0 stubs) | [x] cli.rs revisado |
| **Core modules** | 17 modulos Rust con implementacion real | [x] Todos >90 lineas |
| **Manifest** | schema_version, metadata, editor, validate, normalize | [x] manifest.rs:500 lineas |
| **GUI** | 6 paginas + 9 tabs funcionales. Paleta semantica (info/success/warning/error/dlx-grays). Colores de marca por tecnologia via CSS (@theme). PathInput reutilizable. i18n parcial (es/en) | [x] Componentes revisados |
| **Templates** | 7 templates con generacion real (include_str), modularizados (1 archivo .rs por template + registry.rs) | [x] templates/*.rs |
| **Recipes** | 6 recipes con preview y apply | [x] recipes/mod.rs |
| **Catalogo** | 30 YAML con metadatos completos | [x] 30 archivos .yaml |
| **RulesEngine** | Validacion, dependencias, conflictos, puertos | [x] rules.rs:247 lineas |
| **Scaffold** | Generacion real de proyecto + preview | [x] scaffold.rs:626 lineas |
| **Detection** | Scan de 15+ aspectos del stack | [x] detection.rs:1030 lineas |
| **Docker** | up/down/status/logs cross-platform | [x] docker.rs:145 lineas |
| **Git** | status + log | [x] git.rs:181 lineas |
| **Versioning** | save/list/diff/rollback | [x] versioning.rs:272 lineas |

### Lo que NO esta implementado aun

| Area | Que falta |
|---|---|
| **Activacion de runtimes** | `open` abre editor pero no activa nvm/fnm/pyenv/rustup |
| **Instalacion global CLI** | Solo disponible via `cargo run` |
| **CI/CD multi-SO** | No hay GitHub Actions configurado |
| **Terminal integrada** | No existe panel de terminal embebido |
| **Autocompletado shell** | No hay completions para bash/zsh/fish/PowerShell |
| **Tests de templates** | No hay tests automatizados de generacion |
| **Recipes auth/db/monitoring** | Solo existen las 6 iniciales |
| **Metricas locales** | No hay tracking de primera semana |
| **Historial terminal aislado** | No implementado |
| **Gestion de runtimes** | Solo deteccion, no instalacion/cambio |

---

## Reglas de ejecucion

1. **Cada tarea tiene criterio de done** — si no lo tiene, no se empieza
2. **Windows primero** — cada feature se prueba primero en Windows
3. **Paridad GUI + CLI** — si solo funciona en una interfaz, no se considera completada
4. **No tocar el plan maestro** — este documento es el operativo
5. **Marcar completado inmediatamente** — no acumular tareas hechas sin marcar

---

*Documento derivado de PLAN.md. No duplica contenido — referencia al plan maestro para contexto.*
