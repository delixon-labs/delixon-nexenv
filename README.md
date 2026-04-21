<p align="center">
  <img src="src/assets/logos/Log-bt.png" width="120" alt="Nexenv" />
</p>

<h1 align="center">Nexenv</h1>

<p align="center">
  <strong>Per-project development environment manager.</strong><br>
  Detects stack, isolates variables, opens projects ready to work.
</p>

<p align="center">
  <a href="https://delixon.dev/nexenv">Website</a> ·
  <a href="https://github.com/delixon-labs/delixon-nexenv/releases">Releases</a> ·
  <a href="docs/cli/CLI_REFERENCE.md">CLI Reference</a> ·
  <a href="LICENSE">License</a>
</p>

---

Nexenv is a per-project development environment manager for Windows, macOS and Linux. Each project declares a manifest (`./.nexenv/manifest.yaml`) with its runtime version, env vars, services (Docker), health checks and editor. One command activates everything and opens the project ready to work.

**Source-available** under [FSL-1.1-ALv2](LICENSE) — not open source yet. Each version converts to Apache 2.0 two years after release.

## Features

- **Project isolation** — each project has its own environment, variables, runtime versions and terminal history
- **Automatic stack detection** — scans your project and identifies languages, frameworks and tools instantly
- **Smart dependency management** — detects what you already have installed and links it instead of duplicating
- **Project templates** — start any project in minutes with the right structure and best practices from day one
- **Portable configuration** — export and import project setups across machines with a single file
- **Health checks** — diagnose missing tools, outdated runtimes and misconfigured environments
- **Instant launch** — one click opens your editor, terminal and environment, all configured correctly

## Stack

| Layer | Technologies |
|-------|-------------|
| Frontend | React 19 · TypeScript · Tailwind CSS |
| Backend | Rust · Tauri 2 |
| Data | SQLite (local) · JSON fallback |
| Platforms | Windows · macOS · Linux |
| Distribution | npm (`@delixon/nexenv`) · NSIS · MSI |

## Install

### Desktop (recommended)

Download the latest installer from [Releases](https://github.com/delixon-labs/delixon-nexenv/releases).

### CLI via npm

```bash
npm install -g @delixon/nexenv
nexenv --version
```

## Development

```bash
# Requirements: Node 22+, Rust 1.77+
git clone https://github.com/delixon-labs/delixon-nexenv.git
cd delixon-nexenv
npm install
cp .env.example .env.local
npm run tauri dev
```

### Build installers

```bash
npm run tauri build
```

Output: `src-tauri/target/release/bundle/`

## Documentation

| Document | Description |
|----------|-------------|
| [CLI Reference](docs/cli/CLI_REFERENCE.md) | Complete command-line interface documentation |
| [Manifest Spec](docs/tecnico/MANIFEST_SPEC.md) | Project manifest format specification |
| [Product Plan](docs/producto/PLAN.md) | Product vision and feature roadmap |
| [Roadmap](docs/operativo/ROADMAP.md) | Release timeline and milestones |

## License

**Nexenv is source-available, not open source.**

Licensed under [FSL-1.1-ALv2](LICENSE) — the Functional Source License with Apache 2.0 Future License. In plain English:

- ✅ Use Nexenv for free — personal, commercial, enterprise
- ✅ Read, study, modify and redistribute the code
- ✅ Each version auto-converts to Apache 2.0 **two years** after its release (v1.0.0 → Apache 2.0 on 2028-04-21)
- ❌ Don't build a commercial product or service that competes with Nexenv

See [LICENSE-FAQ.md](LICENSE-FAQ.md) for the full FAQ, including enterprise use, private forks, and why we chose this model.

---

<p align="center">
  <strong>Nexenv</strong> is a product of <a href="https://delixon.dev">Delixon Labs</a><br>
  <sub>Delixon Labs is the developer tools division of <a href="https://xplustechnologies.com">XPlus Technologies LLC</a></sub><br>
  <sub>© 2026 XPlus Technologies LLC. All rights reserved.</sub>
</p>
