# Changelog

All notable changes to Nexenv will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Core source code moved to a private repository (`delixon-labs/nexenv-core`). Installation wrappers (npm, pip) and documentation remain public in this repository. This is a repository-organization change, not a license change — Nexenv remains source-available under FSL-1.1-ALv2 and each version still converts to Apache 2.0 two years after release. See [CONTRIBUTING.md](CONTRIBUTING.md) for the updated contribution flow.

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
