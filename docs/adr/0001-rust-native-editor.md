# ADR 0001 — Rust native editor and shared slide engine

Date: 2026-10-16. Status: **Rust and visual direction approved by the owner; toolkit and prototype architecture proposed for review.** Software licensing remains undecided.

## Decision and scope

Rust is mandatory for the presentation model, editing commands, persistence, renderer, desktop GUI, slide show and Office adapter. JavaScript/TypeScript, Electron and a WebView editor are excluded. Preserve BreakFree Slides branding while pursuing the Windows desktop PowerPoint 2024 workspace described in [UI_SPEC](../UI_SPEC.md). Visual similarity does not imply file compatibility.

Use an independent Cargo workspace: `breakfree-slides-core` has no GUI dependency; `breakfree-slides` supplies the native editor and CLI. A reversible prototype uses **eframe/egui 0.33.3**, its native winit/glow backend, and rfd native file dialogs. No browser runtime, web server, JavaScript or neighboring repository is required. Cargo.lock pins the dependency graph. The pinned API was checked against the [eframe documentation](https://docs.rs/eframe/0.33.3/eframe/trait.App.html); this is a prototype choice, not a permanent toolkit commitment.

## Model and editing

Slides use independent logical units: default 1280 × 720, origin at top left, x right, y down. Geometry and font sizes use finite logical slide units; device pixels, physical print points and view zoom are separate concepts. Objects are axis-aligned text, rectangles or embedded raster images. Vector order defines stacking, back to front. Nonzero monotonic document-local u64 IDs identify slides, objects and assets in one namespace. Duplicate slides receive fresh object/slide IDs and share immutable resources.

Commands edit a candidate deck, validate it and replace the current model only on success. Undo/redo stores up to 32 document snapshots with shared immutable image allocations. A pointer gesture commits once on release; in-object text commits when focus leaves or Ctrl/Cmd+Enter, and Escape cancels the text draft. Dirty state compares to the last successfully saved model, so undo can return to the saved state. The UI resolves Save / Discard / Cancel before replacing or closing a dirty document; canceling Save As keeps the pending action unexecuted.

## Persistence and resources

The native `.bfslide` document is bounded UTF-8 JSON with kind `breakfree.slides.native` and `content_version: 1`. This version is independent of Office protocol version 1. The JSON contains editable object records and all raster bytes in base64; it is not a screenshot. Imported source paths are not stored or fetched. Moving/deleting an original image does not break an embedded image. PNG, JPEG and WebP are decoded and checked, with limits of 8 MiB per image, 16 megapixels, 64 MiB decode allocation, 24 MiB combined base64 resources, 32 MiB document, 1000 slides/assets, 10,000 objects and 2 MiB text/notes. Unknown fields/kinds/versions, malformed resources, inconsistent dimensions and invalid IDs/geometry are rejected.

Save validates and serializes before touching the target, writes a sibling temporary file, flushes/syncs it and atomically persists it over the target. Unix directory sync is best effort. Cross-platform replacement is delegated to tempfile; Windows behavior still needs native verification. Failed validation/write/replace must retain a previously valid target. Loading never writes its source. Resources remain available for undo and are not garbage-collected in this prototype.

## Rendering and fonts

The core builds one renderer-neutral scene in model coordinates. A single Rust painter consumes it for the editor, live thumbnails and slide show; only viewport fitting and editor selection overlays differ. Text wraps and clips within its editable bounds, images scale into their rectangles, and show mode preserves slide aspect ratio with letterboxing. View scale never mutates model coordinates.

This iteration uses egui's bundled proportional font with bundled fallback glyphs. The native format does not promise arbitrary font-family fidelity or store restricted fonts. Bold is synthetic emboldening in the same painter. PowerPoint/Aptos/Segoe typography, advanced text shaping, rotation, rich spans and exact text metrics are not implemented. Missing glyph coverage can produce fallback glyphs; no automatic font downloads occur. Dependencies' font notices remain applicable (see [dependency notices](../DEPENDENCIES.md)).

## Office boundary

Initially, Office main at `cfaaf419e84d9981acd421d130113ca2265061fd` had no contracts; the [owner fallback snapshot](../../contracts/office-v1-baseline.json) preserves that provenance. During implementation, Office draft PR #2 published protocol v1 and a Rust architecture ADR at `edfa845a3b5de902df988933416b7e4bf20caf67`. The [exact pinned local candidate](../../contracts/office-v1/README.md) now governs adapter tests; it is explicitly unmerged, not an approved main-branch contract. The actual binary's JSON is validated against its schema.

The standalone executable provides `--new`, `--open <absolute_path>`, `--capabilities` (JSON stdout without GUI), and `--version` (actual build version, one line). Protocol v1 dispatch capabilities are exactly `new` and `open`; the native format's `read`/`write` flags describe persistence. Editing/save/present are UI features, not extra launcher actions. No PPTX support is advertised. Independent application and manifest tests do not claim successful native dispatch from a separately built Office process; that check remains pending. Reconcile future revisions explicitly, without runtime dependencies on mutable main or a neighboring repository.

## Consequences and follow-up

The renderer/persistence can be tested without a window or Office. Native custom controls enable a compact Ribbon but require further accessibility and keyboard work. Initial platform evidence is macOS only; Windows 100/125/150% scaling and the measured PowerPoint 2024 reference remain explicit verification gaps. Approval of Rust does not select a software license. No release or automatic merge is authorized.
