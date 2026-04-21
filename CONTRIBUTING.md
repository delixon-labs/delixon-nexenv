# Contributing to Nexenv

Thanks for your interest in contributing to Nexenv.

Nexenv is a product of [Delixon Labs](https://delixon.dev), the developer tools division of [XPlus Technologies LLC](https://xplustechnologies.com). It is licensed under [FSL-1.1-ALv2](LICENSE) — source-available, not open source.

Before contributing, please read this document in full.

---

## Where does the code live?

Nexenv is split across two repositories:

- **This repository** (`delixon-labs/delixon-nexenv`, public) hosts the installation wrappers (`npm/cli/`, `pip/cli/`), the user-facing documentation (`docs/`), the license, and the compliance files (SECURITY.md, CONTRIBUTING.md, CODE_OF_CONDUCT.md, CHANGELOG.md). Pull requests to these areas are open and welcome.
- **`delixon-labs/nexenv-core`** (private) hosts the Rust backend, the React frontend, the Tauri configuration and the release workflows. The source code is not publicly browsable while each version is within its first two years (FSL-1.1-ALv2 window). It becomes open source (Apache 2.0) two years after each release.

To contribute to the **core**, follow the flow in the next section. For everything else (wrappers, docs, compliance files), open a PR directly in this repository.

---

## Before you start

### License and contributions

Nexenv is **source-available**, not open source. By contributing code, documentation, or other materials to this project, you agree that your contribution is licensed under the same terms as the rest of the project (FSL-1.1-ALv2 with conversion to Apache 2.0 two years after each version's release).

For non-trivial contributions, we may ask you to sign a Contributor License Agreement (CLA) before merging. This is a lightweight agreement that confirms you have the right to contribute the code and grants us the necessary license to include it. We'll send you the CLA when your PR reaches that point.

### Code of conduct

All contributors are expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md). Be respectful, constructive, and focused on the work.

---

## What kind of contributions are welcome

### ✅ Welcome in this public repository

- **Bug reports** against Nexenv itself (reproduction steps, OS + version, error output)
- **Fixes and improvements to the npm wrapper** (`npm/cli/`)
- **Fixes and improvements to the pip wrapper** (`pip/cli/`)
- **Documentation improvements** (`docs/`, README, CONTRIBUTING, LICENSE-FAQ)
- **Typo fixes** and small polish PRs
- **Translations** of user-facing documentation

### ✅ Welcome in the core — through the invited-access flow

- **Bug fixes** to the Rust or React core
- **New templates**, recipes or catalog entries
- **Performance improvements** with benchmarks
- **Cross-platform fixes** (Windows/macOS/Linux specific issues)

### ⚠️ Discuss first (open an issue in this repository)

- **New commands** or major CLI surface changes
- **Breaking changes** to the manifest format
- **New core features** not already on the roadmap
- **Large refactors** of the Rust core or React frontend
- **Changes to the scan/detect logic** that affect existing projects
- **Additions to the CI/release pipeline**

### ❌ Not welcome

- Contributions that modify the license or its enforcement
- Changes that remove or weaken local-first guarantees
- Telemetry or tracking added without explicit opt-in design
- Commercial-competitor adaptations (prohibited by license)
- Security vulnerability reports in PRs (see [SECURITY.md](SECURITY.md))

---

## How to contribute

### Contributing to the wrappers or documentation (public)

1. **Find or create an issue** in [this repository's issue tracker](https://github.com/delixon-labs/delixon-nexenv/issues). For small fixes (typos, clear bugs), feel free to open a PR directly.
2. **Fork and branch**
   ```bash
   git clone https://github.com/YOUR-USERNAME/delixon-nexenv.git
   cd delixon-nexenv
   git checkout -b fix/description-of-change
   ```
3. **Make your changes** in `npm/cli/`, `pip/cli/`, or the root documentation files.
4. **Test the wrappers** as appropriate (install locally, run `nexenv --version`, etc.).
5. **Open a PR against the `develop` branch** with a clear title and description.

### Contributing to the core (private repository)

1. **Open an issue in this public repository** describing what you'd like to change.
2. If the change is welcome and non-trivial, we'll reply with next steps: CLA (where applicable) and temporary access to `delixon-labs/nexenv-core` for your specific contribution.
3. Your PR gets opened in the private repository and merged under FSL-1.1-ALv2 like the rest of the project.
4. The resulting binaries are published in the Releases section of this public repository on the next tag.

For small bug fixes and suggestions, opening an issue here is always the fastest path.

### Review process

- A maintainer will review within ~7 days for most PRs.
- We may ask for changes or discuss alternative approaches.
- Once approved, a maintainer will merge and include in the next release.

---

## Branching and releases

The public repository uses a three-branch model:

```
feature/* or fix/*  →  desarrollo  →  develop  →  main
```

- `desarrollo`: day-to-day work, fast-moving.
- `develop`: integration branch before release.
- `main`: tracks the latest stable release (tagged).

Contributors should open PRs against `develop`. Never push directly to `develop` or `main`.

Releases are cut by maintainers from the private repository. The release workflow builds the binaries in `nexenv-core` and publishes them as assets on a release in this public repository.

---

## Reporting bugs

Before opening an issue:
1. Search existing issues.
2. Reproduce with the latest version (`nexenv --version` to check).
3. Collect: OS + version, Nexenv version, full error output, minimal reproduction steps.

Issue templates will guide you through this.

---

## Requesting features

Open an issue with:
- The problem you're trying to solve (the *why*).
- How you currently work around it.
- Your proposed solution (if any).
- Why this fits Nexenv's scope ("per-project development environment manager").

Features that don't fit Nexenv's scope will be declined respectfully.

---

## What not to do

- Don't submit PRs for unrelated changes (one topic per PR).
- Don't change the license or remove copyright notices.
- Don't add dependencies without discussion (we keep the dep graph tight).
- Don't submit AI-generated code without review (mention it in the PR description).
- Don't report security issues publicly (see [SECURITY.md](SECURITY.md)).

---

## Getting help

- **Questions about using Nexenv**: [GitHub Discussions](https://github.com/delixon-labs/delixon-nexenv/discussions)
- **Bug reports**: [GitHub Issues](https://github.com/delixon-labs/delixon-nexenv/issues)
- **Security issues**: see [SECURITY.md](SECURITY.md)
- **Licensing questions, code audits under NDA, core contribution requests**: see [LICENSE-FAQ.md](LICENSE-FAQ.md) or email `delirestevez@gmail.com`

---

## Recognition

Contributors are credited in release notes. For significant contributions, you'll be listed in a CONTRIBUTORS.md (coming).

Thank you for helping make Nexenv better.
