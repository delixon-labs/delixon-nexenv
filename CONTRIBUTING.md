# Contributing to Nexenv

Thanks for your interest in contributing to Nexenv.

Nexenv is a product of [Delixon Labs](https://delixon.dev), the developer tools division of [XPlus Technologies LLC](https://xplustechnologies.com). It is licensed under [FSL-1.1-ALv2](LICENSE) — source-available, not open source.

Before contributing, please read this document in full.

---

## Before you start

### License and contributions

Nexenv is **source-available**, not open source. By contributing code, documentation, or other materials to this project, you agree that your contribution is licensed under the same terms as the rest of the project (FSL-1.1-ALv2 with conversion to Apache 2.0 two years after each version's release).

For non-trivial contributions, we may ask you to sign a Contributor License Agreement (CLA) before merging. This is a lightweight agreement that confirms you have the right to contribute the code and grants us the necessary license to include it. We'll send you the CLA when your PR reaches that point.

### Code of conduct

All contributors are expected to follow the [Code of Conduct](CODE_OF_CONDUCT.md). Be respectful, constructive, and focused on the work.

---

## What kind of contributions are welcome

### ✅ Welcome

- **Bug reports** with clear reproduction steps
- **Bug fixes** with tests
- **Documentation improvements** (README, docs/, inline comments)
- **Typo fixes** and small polish PRs
- **New templates** (add an entry under `src-tauri/src/core/templates/`)
- **New recipes** (add under `src-tauri/src/core/recipes/`)
- **New technology entries** in the catalog (`src-tauri/src/core/catalog/technologies/*.yaml`)
- **Translations** of the UI
- **Performance improvements** with benchmarks
- **Cross-platform bug fixes** (Windows/macOS/Linux specific issues)

### ⚠️ Discuss first (open an issue)

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

### 1. Find or create an issue

- Check [existing issues](https://github.com/delixon-labs/delixon-nexenv/issues).
- For small fixes (typos, clear bugs), feel free to open a PR directly.
- For larger changes, **open an issue first** to discuss the approach. This saves you time and helps us align.

### 2. Fork and branch

```bash
# Fork via the GitHub UI, then:
git clone https://github.com/YOUR-USERNAME/delixon-nexenv.git
cd delixon-nexenv
git checkout -b fix/description-of-change
```

### 3. Set up your development environment

See the [README Development section](README.md#development) for requirements and setup.

Minimum versions:
- Rust 1.77+
- Node.js 22+
- npm 10+

### 4. Make your changes

- Follow existing code style (rustfmt for Rust, Prettier for TS/TSX).
- Add tests for new functionality.
- Update documentation if user-facing behavior changes.
- Keep commits focused — one logical change per commit.

### 5. Test

```bash
# Rust core
cd src-tauri && cargo test

# Frontend
npm run test
npm run lint

# Build
npm run tauri build
```

### 6. Submit a pull request

- Target the `develop` branch, not `main`.
- Write a clear title and description.
- Link the issue if one exists.
- Explain the why, not just the what.
- Include screenshots or GIFs for UI changes.

### 7. Review process

- A maintainer will review within ~7 days for most PRs.
- We may ask for changes or discuss alternative approaches.
- Once approved, a maintainer will merge and include in the next release.

---

## Branching and releases

Nexenv uses a three-branch model:

```
feature/* or fix/*  →  desarrollo  →  develop  →  main
```

- `desarrollo`: day-to-day work, fast-moving.
- `develop`: integration branch before release.
- `main`: tracks the latest stable release (tagged).

Releases are cut by maintainers. Contributors should never push directly to `develop` or `main`.

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
- **Licensing questions**: see [LICENSE-FAQ.md](LICENSE-FAQ.md) or email `hello@delixon.dev`

---

## Recognition

Contributors are credited in release notes. For significant contributions, you'll be listed in a CONTRIBUTORS.md (coming).

Thank you for helping make Nexenv better.
