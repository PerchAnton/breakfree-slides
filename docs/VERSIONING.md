# Application versions and release approval

Status: owner-approved numbering policy, recorded 2026-10-16 (Europe/Kyiv). See [ADR 0002](adr/0002-versioning-and-approval.md). This policy applies to BreakFree Slides only.

## Current version

| Field | Value |
| :--- | :--- |
| Actual application version | `0.1.0-alpha.1` |
| Owner approval of this exact version | Awaiting explicit confirmation |
| Published product release | None |
| First public release target | `0.2.0` (short form: 0.2), subject to separate owner approval and publication authorization |

The current version already existed before this policy. This documentation stage keeps it unchanged. Approval of the numbering policy does not imply approval of this specific candidate or its readiness for release.

## Numbering

Use the three-part version and prerelease syntax supported by [Cargo](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field) and [Semantic Versioning 2.0.0](https://semver.org/).

| Stage | Form | Examples / progression |
| :--- | :--- | :--- |
| Alpha development | `0.1.0-alpha.N` | `0.1.0-alpha.1`, `0.1.0-alpha.2`, … |
| Beta validation | `0.1.0-beta.N` | `0.1.0-beta.1`, `0.1.0-beta.2`, … |
| Release candidate | `0.1.0-rc.N` | `0.1.0-rc.1`, `0.1.0-rc.2`, … |
| First public product release | `0.2.0` | Start the published product line at 0.2 after owner approval |
| Later releases | Owner-approved three-part versions | Maintenance examples: `0.2.1`, `0.2.2`; a later feature line may be `0.3.0` |

`N` is a positive integer without leading zeroes. Increase it for a distinct candidate within its channel, after the owner confirms the proposed number; start at 1 on an approved channel transition. An ordinary task or documentation commit does not automatically create a new candidate. All alpha/beta/RC candidates in this initial development cycle stay on `0.1.0`; there is no planned public `0.1.0` product release.

Publishing source commits or a PR retains the assigned development version. The first public product release means the separately authorized publication of the packaged product at `0.2.0`. Moving to beta, RC or 0.2 requires a readiness review; passing CI alone does not choose the channel. Later release numbers and any later prerelease cycle require owner confirmation.

## Single source of truth

Root `Cargo.toml`, `[workspace.package].version`, supplies the application version. Both crates inherit it with `version.workspace = true`. Cargo.lock must retain matching workspace-package versions after a version change. The executable's `--version` and Office capability field `app_version` derive from `CARGO_PKG_VERSION`.

Do not add an independently maintained VERSION file or hard-code a second runtime version. Update the current-version table here, README and the matching [changelog](../CHANGELOG.md) candidate entry when an approved version changes. Historical screenshots and verification reports retain the versions actually tested. The native document's `content_version` and Office's `protocol_version` change only for their own reviewed compatibility decisions.

## Work report and approval record

Every completed work report ends with the actual version and one of these explicit statuses: awaiting owner approval, approved by owner (with confirmation date), or previously approved and unchanged. Include a proposed next version separately when needed. Persist confirmed decisions here and in both copies of the project log; record an available public approval URL or identify an authorized owner decision without copying private correspondence.

| Decision date (Europe/Kyiv) | Scope | Version / status | Evidence |
| :--- | :--- | :--- | :--- |
| 2026-10-16 | Numbering policy and work-report requirement | Approved policy; exact current `0.1.0-alpha.1` awaits confirmation | Owner instruction in the current task; recorded in this policy and ADR 0002 |

## Candidate and publication procedure

1. Prepare the completed changes, changelog, test evidence, GUI/platform limitations and current/proposed version for review. State version approval, code review, CI, merge and release as separate statuses.
2. Obtain explicit owner confirmation of the proposed version/channel. Apply the agreed version consistently and record the confirmation. No response is not approval.
3. Verify Cargo metadata/lock consistency, actual binary `--version` and Office `app_version`; run checks appropriate to the changed code. Record the exact tested code SHA. Update both project logs with results.
4. Before an authorized release, build and review artifacts from the agreed commit, record their names/platforms/checksums and any unavailable platform checks. Keep release notes and redistribution/license decisions explicit.
5. Only with separate publication authorization, create an annotated `v<exact-version>` tag at the agreed commit and publish the corresponding release/artifacts. First public product release: `v0.2.0`. Do not overwrite a published tag/artifact; corrections receive a separately approved new version.

This task creates the version policy and an unreleased changelog. It does not create a tag, installer, merge or public product release.
