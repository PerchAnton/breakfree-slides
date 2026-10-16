# ADR 0002: Development versions and owner approval

Date: 2026-10-16 (Europe/Kyiv).

Status: accepted owner decision for numbering and report policy. Approval of an individual candidate version is recorded separately.

## Context

The native Rust prototype already reports `0.1.0-alpha.1`. The owner requires every completed work report to include the current version for agreement. Alpha, beta and RC development belongs to the `0.1.0` line, with the published product line starting at 0.2.

## Decision

Use `0.1.0-alpha.N`, `0.1.0-beta.N` and `0.1.0-rc.N` for the initial development cycle. Use Cargo-compatible `0.2.0` for the first separately authorized public product release. Owner amendment on 2026-10-16 requires a counter increment and changelog entry for every completed logical change. The owner subsequently accepted `0.1.0-alpha.2` on 2026-10-16. The next assignment is `0.1.0-alpha.3`; acceptance of this new candidate remains pending.

Keep root workspace Cargo version as the single runtime source, inherited by both crates and emitted by the binary and Office manifest. Keep application versions independent from native content and Office protocol versions. Maintain an unreleased changelog and an explicit approval record in [VERSIONING.md](../VERSIONING.md).

Every completed work report must end with the actual current version and approval status. Routine prerelease counter increments are preauthorized; obtain explicit owner confirmation before channel promotion or changing the numbering scheme. Version agreement, code review, merge authorization and release publication are separate decisions. Do not infer consent from silence or a passing check.

## Consequences

The owner can review a concrete candidate at each stage. Every completed logical code/UI/fix/configuration/standalone documentation change increments the prerelease counter once and adds a dated changelog entry. Implementation, validation, changelog and dedicated journal commits for the same change share that version to avoid recursive bumps. Read-only checks do not increment it. Historical test reports retain their original version. Future tags/artifacts use an agreed exact version and immutable source commit; published tags/artifacts are preserved. This amendment supersedes the original per-increment preapproval rule and advances the application identity to alpha.2. Document/protocol versions, license, merge and release are unchanged.
