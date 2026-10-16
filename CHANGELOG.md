# Changelog

Follow [the version and approval policy](docs/VERSIONING.md). Entries below describe source work; a published product release requires its own date, tag, immutable commit and artifacts.

## Unreleased — 0.1.0-alpha.2 (2026-10-16)

Routine increment authorized by the owner's per-change rule; candidate acceptance pending. First public product release remains `0.2.0`, subject to separate authorization.

### Changed

- Desktop workspace refined toward the PowerPoint 2024 composition: large Ribbon commands, original vector icons, compact command rows, active-tab underline, quick-access/status controls and notes beneath the canvas.
- Formatting pane now uses a contextual Shape/Picture heading and scrollable properties.
- Seven running-app captures include a native 980 × 680 resize; keyboard control and layout/model invariance bring automated coverage to 29 tests.

- Every completed logical change now increments the current prerelease counter once and updates this changelog. Related version/validation/journal commits share the increment.
- Workspace crates, lockfile, executable version and Office application identity advance to `0.1.0-alpha.2`; document and protocol versions remain unchanged.

The UI changes above were implemented and captured while the binary still reported `alpha.1`; they are assigned to `alpha.2` now. Historical screenshot/test evidence is not relabeled as a new GUI run.

## 0.1.0-alpha.1 — development history (not released)

Version approval: **awaiting explicit owner confirmation**. No published product release or release tag has been created by this work. The first public product release is planned as `0.2.0`, subject to approval.

### Added

- Native Rust editor prototype with Ribbon/Home, real slide thumbnails, notes and a formatting pane.
- Editable text, rectangles and embedded PNG/JPEG/WebP images; slide/object operations, selection, move/resize, stacking and undo/redo.
- Validated native `.bfslide` v1 save/reopen with embedded resources and safe atomic replacement.
- Shared slide-content rendering for editor, thumbnails and basic presentation mode.
- Standalone Office protocol-v1 new/open/version/capabilities adapter, tested against a pinned upstream candidate.
- Native captures, model/persistence/controller/CLI regression tests and Linux/Windows/macOS CI.
- Owner-approved development numbering, version agreement in work reports and explicit release procedure.

### Fixed

- Notes preservation after slide deletion/reordering and in-object text-editor clipping.
- Office capability/version output aligned with the exact pinned v1 schema.

### Verification limits

See [the native verification report](docs/UI_VERIFICATION.md) for exact code/test evidence. Windows desktop DPI, manual interaction/fullscreen and measured PowerPoint 2024 comparison remain pending. Current local tests and framebuffer evidence are recorded separately from historical server CI and unavailable manual/platform checks.
