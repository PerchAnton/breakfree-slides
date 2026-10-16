# Application versions and release approval

Status: owner-approved numbering policy, recorded 2026-10-16 (Europe/Kyiv). See [ADR 0002](adr/0002-versioning-and-approval.md). This policy applies to BreakFree Slides only.

## Current version

| Field | Value |
| :--- | :--- |
| Actual application version | `0.1.0-alpha.3` |
| Owner approval of this exact version | Increment authorized by standing owner instruction; candidate acceptance pending |
| Published product release | None |
| First public release target | `0.2.0` (short form: 0.2), subject to separate owner approval and publication authorization |

The owner amended the rule on 2026-10-16: every completed logical change must increment the version and update the changelog. `0.1.0-alpha.2` identifies the completed desktop workspace refinement and this policy correction. Historical `alpha.1` captures and test reports retain their actual recorded version. A routine increment is authorized; acceptance of the candidate and readiness for release remain separate.

The owner accepted `0.1.0-alpha.2` on 2026-10-16. The next adaptive Ribbon and formatting-pane change is assigned `0.1.0-alpha.3`, awaiting its own candidate acceptance.

## Numbering

Use the three-part version and prerelease syntax supported by [Cargo](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field) and [Semantic Versioning 2.0.0](https://semver.org/).

| Stage | Form | Examples / progression |
| :--- | :--- | :--- |
| Alpha development | `0.1.0-alpha.N` | `0.1.0-alpha.1`, `0.1.0-alpha.2`, … |
| Beta validation | `0.1.0-beta.N` | `0.1.0-beta.1`, `0.1.0-beta.2`, … |
| Release candidate | `0.1.0-rc.N` | `0.1.0-rc.1`, `0.1.0-rc.2`, … |
| First public product release | `0.2.0` | Start the published product line at 0.2 after owner approval |
| Later releases | Owner-approved three-part versions | Maintenance examples: `0.2.1`, `0.2.2`; a later feature line may be `0.3.0` |

`N` is a positive integer without leading zeroes. Increment it by exactly one for each completed logical change, including code, UI, fixes, configuration and standalone documentation changes. Add a dated changelog entry describing that change. Routine increments in the current channel are authorized by the owner; no additional preapproval is needed. Start at 1 on an explicitly approved channel transition. All initial alpha/beta/RC candidates stay on `0.1.0`; there is no planned public `0.1.0` product release.

A logical change may contain multiple implementation, test, changelog and dedicated journal commits. Those commits share the same assigned version; recording and verifying the same change does not recursively increment it. A new independently completed change receives the next counter. Read-only checks that produce no new substantive changes do not increment the application version.

Publishing source commits or a PR retains the assigned development version. The first public product release means the separately authorized publication of the packaged product at `0.2.0`. Moving to beta, RC or 0.2 requires a readiness review; passing CI alone does not choose the channel. Later release numbers and any later prerelease cycle require owner confirmation.

## Single source of truth

Root `Cargo.toml`, `[workspace.package].version`, supplies the application version. Both crates inherit it with `version.workspace = true`. Cargo.lock must retain matching workspace-package versions after a version change. The executable's `--version` and Office capability field `app_version` derive from `CARGO_PKG_VERSION`.

Do not add an independently maintained VERSION file or hard-code a second runtime version. Update the current-version table here, README and the matching [changelog](../CHANGELOG.md) candidate entry when the assigned version changes. Historical screenshots and verification reports retain the versions actually tested. The native document's `content_version` and Office's `protocol_version` change only for their own reviewed compatibility decisions.

## Work report and approval record

Every completed work report ends with the actual version and one of these explicit statuses: awaiting owner approval, approved by owner (with confirmation date), or previously approved and unchanged. Include a proposed next version separately when needed. Persist confirmed decisions here and in the project log; record an available public approval URL or identify an authorized owner decision without copying private correspondence.

| Decision date (Europe/Kyiv) | Scope | Version / status | Evidence |
| :--- | :--- | :--- | :--- |
| 2026-10-16 | Numbering policy and work-report requirement | Approved policy; exact current `0.1.0-alpha.1` awaits confirmation | Owner instruction in the current task; recorded in this policy and ADR 0002 |
| 2026-10-16 | Mandatory version increment and changelog for every completed change | Routine increments authorized; `0.1.0-alpha.2` assigned, candidate acceptance pending | Owner amendment in the current task; supersedes the earlier per-increment preapproval requirement |
| 2026-10-16 | Candidate acceptance | `0.1.0-alpha.2` accepted; `0.1.0-alpha.3` assigned for the next change, acceptance pending | Explicit owner decision in this task; no merge or release authorization |

## Candidate and publication procedure

1. Prepare the completed changes, changelog, test evidence, GUI/platform limitations and current/proposed version for review. State version approval, code review, CI, merge and release as separate statuses.
2. Increment the current prerelease counter once for the completed change and update the changelog and all current-version references. Present the resulting version for owner acceptance in the final report. Obtain explicit confirmation before channel promotion or changing the numbering scheme. No response is not candidate acceptance.
3. Verify Cargo metadata/lock consistency, actual binary `--version` and Office `app_version`; run checks appropriate to the changed code. Record the exact tested code SHA. Update the project log with results.
4. Before an authorized release, build and review artifacts from the agreed commit, record their names/platforms/checksums and any unavailable platform checks. Keep release notes and redistribution/license decisions explicit.
5. Only with separate publication authorization, create an annotated `v<exact-version>` tag at the agreed commit and publish the corresponding release/artifacts. First public product release: `v0.2.0`. Do not overwrite a published tag/artifact; corrections receive a separately approved new version.

The automatic increment rule supersedes the earlier requirement to wait for approval before every counter increment. It does not authorize a tag, installer, merge or public product release.
