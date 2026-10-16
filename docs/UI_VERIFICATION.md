# Native prototype verification

Date: 2026-10-16, Europe/Kyiv. Build: BreakFree Slides 0.1.0-alpha.1, Rust 1.99.1, Apple Silicon macOS. No Windows installation or PowerPoint 2024 installation was available. The project is a prototype for review, not an accepted compatibility/visual-parity release.

## Actual running-app captures

These PNGs come from the native egui/glow framebuffer of the running Rust application using its explicit `--ui-smoke` diagnostic. They are not hand-drawn mockups or screenshots of PowerPoint. The synthetic fixture contains editable model objects and an embedded generated PNG. The capture program prepares UI states directly; it does not certify manual pointer/keyboard interaction.

Requested initial window: 1440 × 900 logical pixels. The host desktop constrained the client to **1440 × 815**, captured at **2880 × 1630** physical pixels (2× Retina). Light theme, English UI, default approximately 202-pixel thumbnails/105-pixel notes/268-pixel formatting pane, expanded 137-pixel Ribbon. Canvas uses Fit; the shown zoom changes when a pane opens. This is not a Windows 100% reference capture.

| State | Actual capture | Observation |
| :--- | :--- | :--- |
| Empty presentation | [01-empty](screenshots/01-empty.png) | Real empty slide, Home groups, rail, notes and status; no decorative placeholder objects |
| Multiple slides | [02-three-slides](screenshots/02-three-slides.png) | Three model-backed thumbnails; text, rectangle and embedded raster on current slide |
| Selected object | [03-selection](screenshots/03-selection.png) | Eight markers follow the rectangle's actual projected geometry; context tab appears |
| In-object text editing | [04-text-edit](screenshots/04-text-edit.png) | Native text widget overlays object bounds; this capture exercises a prepared editing state |
| Formatting pane | [05-format-pane](screenshots/05-format-pane.png) | Real position/size fields, fill and front/back actions; canvas fit adjusts without document-coordinate changes |
| Show composition | [06-slide-show](screenshots/06-slide-show.png) | Same content painter with aspect-preserving fit and black letterboxing; this capture is client-window show content, not a verified fullscreen transition |

All six states were visually inspected. Missing-glyph symbols on several initial buttons were replaced with readable labels and captures regenerated. Text, object stacking, embedded image placement, true thumbnails and selection alignment were checked in the supported states. No pixel similarity score is asserted.

## Differences against the declared PowerPoint 2024 target

- The title/Ribbon/rail/canvas/notes/pane/status composition and Home group order follow the target, but no installed 2024 build or matched screenshot was available for measured comparison.
- Text buttons and egui control styling differ from PowerPoint iconography, Segoe/Aptos metrics, Backstage details and exact spacing. Bundled font fallback and synthetic bold have different text metrics. No Microsoft assets are reused.
- Narrow windows use horizontal Ribbon scrolling rather than PowerPoint's adaptive group collapse. Native OS window decoration differs on macOS.
- Reordering uses Move Up/Down; thumbnail drag/drop, multi-selection, rotation, paragraph controls, rich text, themes, animation, recording and presenter tools are incomplete or unavailable. Unsupported actions are disabled with explanations.

## Checks and limits

| Check | Result / evidence |
| :--- | :--- |
| Workspace build | Passed locally with locked dependencies |
| Regression suite | **27 tests passed**: 19 core tests including full-model save/reopen after deleting original image, undo/redo/stacking, safe failed saves, input/resource/geometry limits, shared scene and view transforms; 4 actual binary CLI tests; 4 headless GUI-controller tests for note preservation, dirty-document guarding and resize geometry |
| CLI | Actual binary capabilities/version print without GUI; malformed argument/document refusal is separately tested |
| Formatting and lint | `cargo fmt --all -- --check` and Clippy with `-D warnings` passed locally |
| Running native framebuffer | Six captures generated successfully and inspected; no mockup substitution |
| Windows 100%, 125%, 150% | **Not run**: host is macOS; mathematical transform tests are not native Windows DPI verification |
| Native keyboard, focus and Save/Discard/Cancel interaction | **Not run manually**: Computer Use reported `Computer Use permissions are not granted`; unbundled executable also could not be selected as an app. Code paths exist; no successful manual claim |
| Native resize, pointer drag/resize, open/save dialogs and fullscreen launch/exit | **Not run manually** for the same permission limitation. Show capture verifies content composition only |
| PowerPoint 2024 measured comparison | **Not run**: installed Windows reference/build unavailable |
| Server CI | Workflow added for Linux/Windows/macOS; local results do not establish server success. Check the PR for actual run status |
| Merge / release | Neither performed; software/asset license decisions remain open |

Next validation: approve a native Windows review environment, record the exact PowerPoint 2024 reference build/parameters in UI_SPEC, execute the DPI/resize/focus/keyboard/dialog/show matrix, then address measured layout differences and richer editing. A preserved model is necessary but does not establish full visual parity.
