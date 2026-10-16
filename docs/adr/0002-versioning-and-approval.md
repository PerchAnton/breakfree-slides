# ADR 0002: Development versions and owner approval

Date: 2026-10-16 (Europe/Kyiv).

Status: accepted owner decision for numbering and report policy. Approval of an individual candidate version is recorded separately.

## Context

The native Rust prototype already reports `0.1.0-alpha.1`. The owner requires every completed work report to include the current version for agreement. Alpha, beta and RC development belongs to the `0.1.0` line, with the published product line starting at 0.2.

## Decision

Use `0.1.0-alpha.N`, `0.1.0-beta.N` and `0.1.0-rc.N` for the initial development cycle. Use Cargo-compatible `0.2.0` for the first separately authorized public product release. The existing current version remains `0.1.0-alpha.1`, awaiting explicit owner confirmation; this policy change does not advance it.

Keep root workspace Cargo version as the single runtime source, inherited by both crates and emitted by the binary and Office manifest. Keep application versions independent from native content and Office protocol versions. Maintain an unreleased changelog and an explicit approval record in [VERSIONING.md](../VERSIONING.md).

Every completed work report must end with the actual current version and approval status. Obtain explicit owner confirmation before changing a candidate number/channel. Version agreement, code review, merge authorization and release publication are separate decisions. Do not infer consent from silence or a passing check.

## Consequences

The owner can review a concrete candidate at each stage. Routine source/documentation commits can retain the same version. Historical test reports retain their original version. Future tags/artifacts use an agreed exact version and immutable source commit; published tags/artifacts are preserved. This ADR changes documentation and policy only; no runtime code, version number, document format, license, merge or release is changed.
