# Nexenv — License FAQ

Nexenv is **source-available** software under the [Functional Source License (FSL-1.1-ALv2)](LICENSE).

This is not open source in the traditional sense. The source code is publicly available and usable, with one restriction: you cannot use it to build a competing commercial product. After two years, each version automatically converts to Apache 2.0.

---

## What you CAN do

- Use Nexenv internally at your company, regardless of size
- Read, study and learn from the source code
- Modify the code for your personal or internal use
- Use it for non-commercial education and research
- Provide professional services to others who use Nexenv
- Contribute improvements back to the project

## What you CANNOT do

- Sell or offer a commercial product or service that competes with Nexenv
- Offer the same or substantially similar functionality as a commercial service
- Repackage or rebrand it as your own product

---

## Common questions

**Can I use it at my company internally?**
Yes. Internal use is explicitly permitted, regardless of company size.

**Can a consultant customize it for a client?**
Yes, as long as the client uses it internally and the consultant is not selling a competing product.

**Does a private fork for internal use count as competing?**
No. Internal use is always permitted.

**Can I build a SaaS that does the same thing?**
No. That would be a Competing Use under the license terms.

**When does it become Apache 2.0?**
Two years after each version's release date. After that date, you may use that version under the full terms of the Apache License 2.0, including commercial use. Example: Nexenv v1.0.0 released 2026-04-21 becomes Apache 2.0 on 2028-04-21 — automatically, without any action from us.

**Is Nexenv open source?**
No, not during the first two years of each release. Nexenv is *source-available*: the source code is viewable and usable with the restrictions listed above. After two years per version, each release becomes open source under Apache 2.0 automatically.

**Why not Apache 2.0 from day one?**
Building Nexenv takes resources. FSL lets us share the code with the community while protecting the project during its growth phase. After two years, every version becomes open source regardless of what we do — it's a commitment coded into the license, not a promise.

**Can I audit the code before using Nexenv?**
Yes. The source is publicly viewable (see the repository). For enterprise compliance requirements (SOC2, HIPAA, etc.), we also offer formal code audits under NDA. Contact: hello@delixon.dev.

**How do I know Nexenv isn't sending my data somewhere?**
Nexenv is local-first by design. All data lives in SQLite on your machine (`~/.nexenv/`). There is no telemetry by default, no required account, no cloud component in the core product. You can verify this with standard tools (Wireshark, Little Snitch, lsof).

**What happens if Delixon disappears?**
Three things protect you:
1. Your data lives on your machine. Manifests are plain YAML — readable and portable without Nexenv.
2. The binary you have already downloaded keeps working.
3. Every version converts to Apache 2.0 after two years — v1.0.0 becomes fully open source in 2028 regardless of what happens to us.

**Can I contribute?**
Yes. For bug fixes, feature suggestions and small improvements, PRs and issues are welcome in the public repository. For larger contributions touching the core, we may require a Contributor License Agreement (CLA). See [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Legal entity

| Role | Entity |
|------|--------|
| Copyright holder and licensor | XPlus Technologies LLC |
| Public brand | Delixon Labs |
| Product | Nexenv |

Delixon Labs is the developer tools division of [XPlus Technologies LLC](https://xplustechnologies.com).

For licensing inquiries: hello@delixon.dev
