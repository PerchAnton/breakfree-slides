# Pinned Office protocol v1 candidate

Protocol version: **1**. Source repository: [PerchAnton/breakfree-office](https://github.com/PerchAnton/breakfree-office). Immutable source commit: **edfa845a3b5de902df988933416b7e4bf20caf67**, from [draft PR #2](https://github.com/PerchAnton/breakfree-office/pull/2), inspected 2026-10-16 11:51:42 Europe/Kyiv. This is a published, unmerged candidate; it is not claimed to be an approved main-branch contract.

`integration.md` and `app-manifest.schema.json` are exact local copies of `contracts/v1/` at that commit. The Office [Rust ADR](https://github.com/PerchAnton/breakfree-office/blob/edfa845a3b5de902df988933416b7e4bf20caf67/docs/adr/0002-rust-native-shell.md) was also read. No runtime/build dependency on Office, a neighboring checkout or mutable main is introduced.

Slides tests validate its actual binary capability response against this schema. Advertised dispatch actions are only `new` and `open`; native saving is represented by the format's `write` flag, not a launcher action. The actual build version is emitted. No fictional example version or PPTX support is copied.

The earlier [owner-provided fallback snapshot](../office-v1-baseline.json) remains as provenance for the initial implementation when Office main had no contracts at commit cfaaf419e84d9981acd421d130113ca2265061fd. This candidate supersedes the fallback for current adapter testing. Reconcile future upstream revisions explicitly; do not silently follow main. End-to-end dispatch from the separately built Office process still requires native verification.
