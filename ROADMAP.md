# BreakFree Slides — Roadmap

**Status: native Rust prototype for review.** Rust and the Windows desktop PowerPoint 2024 visual direction are owner-approved. The BreakFree brand and independent build remain mandatory. No license, supported Windows build, compatibility promise, release date or automatic merge is implied.

## 0 · Choose the foundation

Status: Rust approved; prototype architecture/toolkit reviewable in [ADR 0001](docs/adr/0001-rust-native-editor.md). License decision remains open. Core, native GUI, rendering, storage and Office adapter are separate Rust modules/crates. Native .bfslide v1 and a local, explicitly unverified Office fallback are defined.

Dependencies: owner language/direction decision, native toolchain and approved brand assets. Exit: independent build, locked dependencies, documented units/IDs/font/resource policies and reviewable shell. Checks: format/lint, contract CLI and core tests; native Windows build still requires evidence. See [build instructions](docs/BUILD.md).

## 1 · A dependable slide editor

Status: implemented prototype; automated regression tests and macOS GUI evidence are tracked in [verification](docs/UI_VERIFICATION.md). Create/delete/duplicate/reorder slides; text, rectangle and embedded image objects; selection, move/resize, stacking; undo/redo; atomic save/reopen and unsaved-change choices. Home groups and real thumbnails use the native Ribbon workspace. The visual refinement adds original vector command icons, large Paste/New Slide, underlined tabs, quick access/status controls and notes between full-height side panes. The alpha.3 refinement adds adaptive Home group popups, a tabs-only Ribbon and collapsible formatting sections. Ten actual captures include native 980 × 680 and 800 × 560 resize states; exact Windows reference matching remains pending.

Dependencies: milestone 0. Exit: P01–P08 in [requirements](docs/REQUIREMENTS.md), including three slides and image persistence after source removal. Tests: full-model equality after save/reopen, command rollback/undo/redo, resource/ID/geometry limits, malformed input, failed replacement and view-coordinate invariants. Remaining review: native Windows interaction/file dialogs and accessibility; thumbnail drag/drop and richer text are follow-up work.

## 2 · Present with confidence

Status: minimal shared-renderer show implemented; no transitions or presenter tools. Dependencies: milestone 1. Exit: P09–P10, retained aspect ratio/order/content, keyboard launch/navigation/Escape and unchanged document on return. Tests: common display-list/painter, geometry invariants, actual show capture and explicit native keyboard/fullscreen verification. Record unavailable environments separately.

## 3 · Layouts and themes

Status: planned. Dependencies: stable model/native versioning and text metrics. Exit: reusable layouts, placeholders, theme colors/fonts and alignment tools with clear overrides. Tests: theme inheritance, undo and persistence fixtures; editor/show equivalence. Avoid announcing arbitrary PowerPoint theme compatibility.

## 4 · Speaker notes and presenter tools

Status: basic editable notes already persist; presenter workflow planned. Dependencies: milestone 2, multi-display design and accessibility. Exit: presenter-only notes, timing/navigation and clear separation from the audience screen. Tests: notes round-trip, keyboard/focus and native dual-display checks.

## 5 · PDF and image export

Status: planned. Dependencies: shared scene and documented font/text semantics. Exit: deterministic, bounded exports matching supported rendered content. Tests: synthetic fixtures, dimensions/page order and visual comparisons; failed-export handling.

## 6 · PPTX interoperability

Status: planned, no PPTX import/export now. Dependencies: milestones 3 and 5 and an explicitly scoped compatibility matrix. Exit: supported text/images/geometry/themes round-trip with visible unsupported/lossy warnings. Tests: independent fixtures, font substitutions, malformed package/resource limits and visual comparisons in actual PowerPoint versions. Opening one file is not PowerPoint parity.

## 7 · Multimedia and animation

Status: planned, outside the first iteration. Dependencies: stable shared renderer, timing model and security/resource design. Exit: separately specified media/transition/animation subsets with editor/show consistency. Tests: timing, cancellation, unsupported codecs, persistence and bounded resource use. Macros and embedded executable content remain excluded.

## 8 · Automation and Office integration

Status: basic launch/capability adapter implemented and the actual binary validated against the [pinned unmerged Office v1 candidate](contracts/office-v1/README.md); original fallback provenance retained. Wider automation and native cross-process dispatch verification are planned. Dependencies: accepted upstream contract and stable command model. Exit: pinned interoperability tests and bounded automation without mutable-main/runtime coupling. Tests: headless capabilities/version, schema/semantic rules, invalid arguments and actual format claims. No silent uploads or remote execution.

## 9 · Accessibility, performance and releases

Status: planned. Dependencies: declared supported workflows and owner license decision. Exit: native platform evidence, keyboard/focus/accessibility review, Windows 100/125/150% checks, packaging, private security reporting and release notes. Tests: bounded large decks/history/assets, font/glyph coverage, rendering/performance fixtures and native install/open/save/show. Follow the accepted [version policy](docs/VERSIONING.md): initial alpha/beta/RC candidates stay on `0.1.0`; first public product release starts at `0.2.0`. Increment the current prerelease counter and update the changelog for every completed logical change; related verification/journal commits share that version. Record candidate acceptance, tested commit and changelog before release; every work report includes current version and agreement status. Version agreement, release publication and merge require their corresponding explicit authorization.

[Back to Slides](README.md) · [Suite roadmap](https://github.com/PerchAnton/breakfree-office/blob/main/ROADMAP.md)
