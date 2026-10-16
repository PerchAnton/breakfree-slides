# Changelog

Follow [the version and approval policy](docs/VERSIONING.md). Entries below describe source work; a published product release requires its own date, tag, immutable commit and artifacts.

## Unreleased — 0.1.0-alpha.1

Version approval: **awaiting explicit owner confirmation**. No published product release or release tag has been created by this work. The first public product release is planned as `0.2.0`, subject to approval.

### Added

- Native Rust editor prototype with Ribbon/Home, real slide thumbnails, notes and a formatting pane.
- Editable text, rectangles and embedded PNG/JPEG/WebP images; slide/object operations, selection, move/resize, stacking and undo/redo.
- Validated native `.bfslide` v1 save/reopen with embedded resources and safe atomic replacement.
- Shared slide-content rendering for editor, thumbnails and basic presentation mode.
- Standalone Office protocol-v1 new/open/version/capabilities adapter, tested against a pinned upstream candidate.
- Native captures, model/persistence/controller/CLI regression tests and Linux/Windows/macOS CI.
- Owner-approved development numbering, version agreement in work reports and explicit release procedure.

### Changed

- Desktop workspace refined toward the PowerPoint 2024 composition: large Ribbon commands, original vector icons, compact command rows, active-tab underline, quick-access/status controls and notes beneath the canvas.
- Formatting pane now uses a contextual Shape/Picture heading and scrollable properties.
- Seven running-app captures include a native 980 × 680 resize; keyboard control and layout/model invariance bring automated coverage to 29 tests.

### Fixed

- Notes preservation after slide deletion/reordering and in-object text-editor clipping.
- Office capability/version output aligned with the exact pinned v1 schema.

### Verification limits

See [the native verification report](docs/UI_VERIFICATION.md) for exact code/test evidence. Windows desktop DPI, manual interaction/fullscreen and measured PowerPoint 2024 comparison remain pending. Current local tests and framebuffer evidence are recorded separately from historical server CI and unavailable manual/platform checks.
