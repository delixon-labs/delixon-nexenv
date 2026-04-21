# Security Policy

## Supported versions

We provide security updates for the latest stable release of Nexenv.

| Version | Supported |
|---------|-----------|
| 1.0.x   | ✅ Yes    |
| < 1.0.0 | ❌ No     |

## Reporting a vulnerability

If you find a security vulnerability in Nexenv, please **do not open a public GitHub issue**.

Instead, report it privately via one of the following channels:

- **Email:** `security@delixon.dev`
- **GitHub Security Advisory:** [Report privately](https://github.com/delixon-labs/delixon-nexenv/security/advisories/new)

### What to include

Please include as much of the following as possible:

- A clear description of the vulnerability
- Steps to reproduce (proof of concept if available)
- The version of Nexenv affected (`nexenv --version`)
- The operating system and version
- The potential impact (e.g. "reads env vars of other projects", "executes arbitrary code")
- Whether you have already disclosed this to anyone else

### What happens next

- We will acknowledge receipt within **72 hours**.
- We will investigate and assess severity within **7 days**.
- We will keep you updated on progress toward a fix.
- Once a fix is ready, we will coordinate disclosure with you.
- You will be credited in the release notes (unless you prefer to remain anonymous).

## Scope

The following are considered in-scope for security reports:

- Arbitrary code execution via Nexenv
- Reading files outside the user's project directories
- Leaking environment variables across projects
- Tampering with the manifest format to cause unintended behavior
- Privilege escalation
- Memory safety bugs in the Rust core
- Supply-chain integrity of binaries and packages distributed via npm/PyPI

The following are **out of scope**:

- Vulnerabilities in third-party dependencies (please report them upstream; we monitor advisories and update)
- Issues that require local physical access to an unlocked machine
- Social engineering or phishing

## Security practices

Nexenv is designed with these principles:

- **Local-first**: no data leaves your machine by default.
- **No telemetry by default**: any opt-in analytics would be clearly disclosed.
- **Isolated storage**: project data in SQLite local (`~/.nexenv/`).
- **Cross-platform binaries**: built in CI with reproducible pipelines.
- **Dependency updates**: Dependabot monitors deps in CI.

## Disclosure timeline

We aim for **90 days** from report to public disclosure, with flexibility depending on severity and complexity.

---

*Last updated: 2026-04-21*
