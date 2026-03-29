# CI/CD — GitHub Actions

## Workflows activos

| Workflow | Archivo | Trigger | Que hace |
|---|---|---|---|
| Build & Test | `.github/workflows/build.yml` | Push a `develop`, `main` | Tests frontend (vitest) + Rust check/test en Ubuntu y Windows |
| CI | `.github/workflows/ci.yml` | PRs | Lint + build + tests |
| Release | `.github/workflows/release.yml` | Tags `v*` | Build binarios para Windows/Linux/macOS |

## Dependencias de acciones

| Accion | Version actual | Notas |
|---|---|---|
| `actions/checkout` | `@v4` | — |
| `actions/setup-node` | `@v4` | — |
| `actions/cache` | `@v4` | Cache de node_modules |
| `swatinem/rust-cache` | `@v2` | Cache de target/ de Rust |

## Pendientes y recordatorios

### Node.js 20 deprecation (deadline: 2 junio 2026)

GitHub deprecara Node.js 20 en los runners de Actions. Las acciones `@v4` corren en Node 20.
A partir del 2 de junio 2026, se forzara Node 24.

**Que hacer**: cuando se publiquen versiones `@v5` de `actions/checkout`, `actions/setup-node` y `actions/cache`, actualizar los workflows. Mientras tanto el warning es informativo y no rompe nada.

Referencia: https://github.blog/changelog/2025-09-19-deprecation-of-node-20-on-github-actions-runners/

### Cache de Rust en Windows

El cache de `swatinem/rust-cache` en Windows es inestable — a veces falla con "The operation was canceled" al descargar. No es un error de codigo.

**Que hacer**: reintentar el workflow (boton "Re-run jobs" en GitHub). Si falla consistentemente, considerar `sccache` como alternativa.

### Compilacion lenta tras cambiar dependencias

Al modificar `Cargo.toml` (nuevas deps o cambio de version), el cache de Rust se invalida y CI recompila todo desde cero. Esto es normal.

- Primera ejecucion tras cambio: ~5-8 min (incluye compilar SQLite desde C)
- Ejecuciones siguientes con cache: ~1-2 min

### Plataformas de build

| SO | Runner | Estado |
|---|---|---|
| Ubuntu | `ubuntu-latest` | Activo en build + CI |
| Windows | `windows-latest` | Activo en build |
| macOS | `macos-latest` | Solo en release |
