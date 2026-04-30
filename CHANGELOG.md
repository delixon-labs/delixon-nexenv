# Changelog

All notable changes to Nexenv will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Core source code moved to a private repository (`delixon-labs/nexenv-core`). Installation wrappers (npm, pip) and documentation remain public in this repository. This is a repository-organization change, not a license change — Nexenv remains source-available under FSL-1.1-ALv2 and each version still converts to Apache 2.0 two years after release. See [CONTRIBUTING.md](CONTRIBUTING.md) for the updated contribution flow.

## [1.1.2] — 2026-04-30

### Fixed

- **Windows GUI: console window flickers eliminated** on actions that internally spawn subprocesses (folder scan, health checks, `git status`, `docker info`, processes list, "Open in editor", etc.). All non-interactive subprocess calls now use `CREATE_NO_WINDOW` on Windows.
- **CLI shell**: `Home`, `End` and `Delete` keys now work consistently on all Windows terminals (PowerShell, Git Bash, Windows Terminal). `/clear` redraws the welcome banner and project intro instead of leaving an empty screen. `/help` no longer duplicates content when the terminal is too short to fit the help block — switched to in-place rendering with internal scroll (`PgUp` / `PgDn` / `↑↓` / `Home` / `End`), preserving the main shell scrollback.

### Changed

- **Default language is now English** on first install. Existing users keep their previously selected language (no migration needed).
- The static CLI help (`nexenv --help`) and the auto-generated `nexenv.yaml` manifest content are now in English regardless of UI language.

### Added

- Mixed local/CI release flow: `release.yml` accepts `workflow_dispatch` with per-platform inputs (`windows` / `linux` / `macos`); `scripts/build-release.mjs` (in `delixon-labs/nexenv-core`) builds and uploads native bundles from the host OS, saving CI minutes.

## [1.0.0] — 2026-04-21

### Added

- First stable release of Nexenv.
- `nexenv shell` — interactive REPL (reedline-based) with slash-menu navigation, history, per-project actions, and tabbed help.
- `nexenv new` wizard with per-category technology selection (runtime, framework, database, etc.) and project scaffolding.
- Cross-platform desktop application distributed as Windows MSI and NSIS installers, macOS DMG (arm64 and x64) and Linux `.deb` and AppImage packages.
- CLI binaries for Windows, macOS (arm64 and x64) and Linux, distributed through:
  - npm wrapper `@delixon/nexenv`
  - PyPI wrapper `nexenv`
  - GitHub Releases (direct downloads)
- Project detection (languages, frameworks, services), per-project environment variables, Docker service management, health checks, portable configuration export/import.
- Local-first data model (SQLite with JSON fallback) — no telemetry, no remote calls during normal operation.

### Fixed

- `nexenv --version` now reads the version from `env!("CARGO_PKG_VERSION")` instead of a hardcoded literal.
- Paths shown by `list`, `new`, `import` and `manifest` no longer include the Windows `\\?\` extended-length prefix.
- UNIQUE constraint error when creating a project with `--path .` in an existing project directory.

### Security

- All binaries are built reproducibly from tagged commits in the release pipeline.
- Code signing for Windows and macOS binaries is planned for a subsequent release.
