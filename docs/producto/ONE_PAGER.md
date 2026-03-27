# Delixon — One-Pager Ejecutivo

> Ultima actualizacion: 2026-03-27

---

## Que es

Delixon es una **app de escritorio + CLI** que gestiona el ciclo de vida completo de proyectos de desarrollo: crear, configurar, aislar, ejecutar, evolucionar, diagnosticar y reparar — todo desde una sola herramienta.

**GUI** (Tauri + React) para usuarios visuales. **CLI** (Rust + clap) para power users. Ambas comparten el mismo core.

---

## El problema

Un developer con 3+ proyectos pierde **2-5 horas/semana** en:
- Configurar entornos desde cero
- Mezclar variables de entorno entre proyectos
- Recordar que version de runtime usa cada proyecto
- Retomar un proyecto olvidado ("¿como se arrancaba esto?")
- Onboarding de nuevos devs (4-8 horas por persona)

No hay ninguna herramienta que integre todo esto en una sola experiencia.

---

## La solucion

Un **Project Manifest** (`.delixon/manifest.yaml`) como columna vertebral. Todo lo demas lee y escribe sobre el.

4 verbos que resumen todo lo que Delixon hace:

| Verbo | Que hace |
|---|---|
| **scan** | Entender que hay en un proyecto |
| **open** | Activar el entorno correcto en <2s |
| **doctor** | Diagnosticar que falta o falla |
| **evolve** | Cambiar el proyecto sin romperlo |

---

## Que funciona HOY (verificado en codigo)

### Nucleo (Capa 0)
- [x] Project Manifest con schema_version, metadata, validacion y normalizacion obligatoria
- [x] Nunca se guarda un manifest invalido al disco

### Workspace (Capa 1)
- [x] App de escritorio Tauri 2 + React (6 paginas, 9 tabs)
- [x] CRUD de proyectos con dashboard, busqueda y filtros
- [x] Env vars aisladas por proyecto
- [x] Deteccion de 7 runtimes (Node, Python, Rust, Go, .NET, PHP, Ruby)
- [x] Apertura en editor configurado (VS Code, Cursor, Zed, Neovim, Sublime)
- [x] Export/import portable (.delixon) con manifest incluido
- [x] Settings: editor, tema, idioma, runtimes

### Scaffolding (Capa 2)
- [x] 30 tecnologias en catalogo YAML con metadatos completos
- [x] RulesEngine: dependencias auto, incompatibilidades, puertos, sugerencias
- [x] 7 templates funcionales (Node+Express, React+Vite, FastAPI, Django, Fullstack, Rust CLI, Docker Compose)
- [x] 6 recipes con preview (vitest, pytest, docker, ci-github, biome, prisma)
- [x] Scaffold wizard de 4 pasos en GUI + `new`/`create` en CLI
- [x] Scan de proyectos existentes (1030 lineas de deteccion)
- [x] Perfiles de madurez: rapid/standard/production

### Operacion diaria (Capa 3)
- [x] 23 comandos CLI funcionales (28 contando sub-acciones)
- [x] Docker management: up/down/status/logs (GUI + CLI)
- [x] Git integration: rama, cambios, remoto, commits (GUI + CLI)
- [x] Health checks con sugerencias de fix (GUI + CLI)
- [x] Doctor del sistema (GUI + CLI)
- [x] Versionado de stacks: save/diff/rollback (GUI + CLI)
- [x] Notas, scripts, puertos, procesos (GUI + CLI)

---

## Que falta antes de beta

| # | Que | Por que es critico |
|---|---|---|
| 1 | `open` perfecto (<2s, runtime activado) | Es la feature que crea el habito diario |
| 2 | Instalacion global del CLI | No podemos pedir `cargo run --manifest-path...` |
| 3 | CI/CD multi-SO (Win + Linux + macOS) | Sin esto, vamos a ciegas en 2 de 3 plataformas |
| 4 | 3 recipes criticas (auth, db+orm, monitoring) | Completan la propuesta de valor |
| 5 | Activacion de runtimes (nvm/fnm, pyenv, rustup) | Convierte `open` de "abre editor" a "activa entorno" |
| 6 | Tests automatizados de templates | Cada template debe generarse y arrancar sin errores |

---

## Por que gana

| vs | Delixon gana porque |
|---|---|
| **DevContainers** | Sin Docker obligatorio, 5MB vs 200MB, dashboard visual |
| **mise** | GUI + scaffolding + health checks + recipes + scan + versionado |
| **direnv** | Solo carga env vars. Delixon gestiona el ciclo de vida completo |
| **Scripts manuales** | Cada dev reinventa la rueda. Delixon estandariza |

**Posicionamiento:** Delixon integra en una sola experiencia lo que hoy requiere combinar 4-5 herramientas. No es que cada competidor sea malo — es que el developer tiene que ensamblar la solucion a mano.

---

## Numeros clave

- **5 MB** de instalador (Tauri, no Electron)
- **30** tecnologias en catalogo
- **7** templates funcionales
- **6** recipes aplicables
- **23** comandos CLI implementados
- **9** tabs en la GUI
- **0** dependencia de Docker para el developer
- **100%** offline, todo local

---

## Stack

**Backend:** Rust (Tauri 2) — serde, tokio, clap, chrono, uuid
**Frontend:** React 19, TypeScript, TailwindCSS 4, Zustand 5, shadcn/ui
**Plataformas:** Windows (primario), Linux, macOS

---

## Modelo de negocio

- **Gratis** para developer individual
- **Pro** para equipos: catalogos corporativos, templates privadas, secrets vault, onboarding automatizado
- **Servidor** (futuro): Delixon headless para entornos compartidos

---

*Delixon — Deja de configurar. Empieza a construir.*
