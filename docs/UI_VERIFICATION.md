# Native prototype verification

Date: 2026-10-16, Europe/Kyiv. Latest local refinement check: 2026-10-16 (UTC+03:00), exact publication/check times in the project log. Build: BreakFree Slides 0.1.0-alpha.3, Rust 1.99.1, Apple Silicon macOS. No Windows installation or PowerPoint 2024 installation was available. The project is a prototype for review, not an accepted compatibility/visual-parity release.

## Actual running-app captures

These PNGs come from the native egui/glow framebuffer of the running Rust application using its explicit `--ui-smoke` diagnostic. They are not hand-drawn mockups or screenshots of PowerPoint. The synthetic fixture contains editable model objects and an embedded generated PNG. The capture program prepares UI states directly; it does not certify manual pointer/keyboard interaction.

Requested initial window: 1440 × 900 logical pixels. The host desktop constrained the client to **1440 × 815**, captured at **2880 × 1630** physical pixels (2× Retina). Light theme, English UI, default approximately 202-pixel thumbnails/105-pixel notes/268-pixel formatting pane, expanded 144-pixel Ribbon with 30-pixel tabs and original vector controls. Canvas uses Fit; the shown zoom changes when a pane opens. This is not a Windows 100% reference capture.

| State | Actual capture | Observation |
| :--- | :--- | :--- |
| Empty presentation | [01-empty](screenshots/01-empty.png) | Real empty slide, Home groups, rail, notes and status; no decorative placeholder objects |
| Multiple slides | [02-three-slides](screenshots/02-three-slides.png) | Three model-backed thumbnails; text, rectangle and embedded raster on current slide |
| Selected object | [03-selection](screenshots/03-selection.png) | Eight markers follow the rectangle's actual projected geometry; context tab appears |
| In-object text editing | [04-text-edit](screenshots/04-text-edit.png) | Native text widget overlays object bounds; this capture exercises a prepared editing state |
| Formatting pane | [05-format-pane](screenshots/05-format-pane.png) | Collapsible Size & Position, Fill and Arrange sections with real fields/actions; canvas fit adjusts without document-coordinate changes |
| Narrow workspace | [07-narrow-window](screenshots/07-narrow-window.png) | Actual viewport resized programmatically to 980 × 680 logical / 1960 × 1360 physical pixels; canvas/notes/status retained, Home groups collapse to anchored command popups |
| Compact group popup | [08-group-popup](screenshots/08-group-popup.png) | Drawing commands in an anchored popup at 980 × 680; shared implementations with expanded Home |
| Tabs-only Ribbon | [09-tabs-only](screenshots/09-tabs-only.png) | 38-pixel Ribbon leaves more canvas space; no stored coordinate changes |
| Minimum window and pane | [10-minimum-window](screenshots/10-minimum-window.png) | Native 800 × 560 logical / 1600 × 1120 physical window, selected text and scrollable formatting pane; compact Home groups remain reachable |
| Show composition | [06-slide-show](screenshots/06-slide-show.png) | Same content painter with aspect-preserving fit and black letterboxing; this capture is client-window show content, not a verified fullscreen transition |

The show capture remains 2880 × 1630. Its file is byte-identical to the previous implementation capture; neither the shared content painter nor core model changed in this refinement.

All ten current states were visually inspected after the Ribbon refinement. Initial text-button controls were replaced with original vector glyphs, avoiding font-dependent icon symbols. Text, object stacking, embedded image placement, true thumbnails and selection alignment were checked in the supported states. No pixel similarity score is asserted.

## Differences against the declared PowerPoint 2024 target

- The title/Ribbon/rail/canvas/notes/pane/status composition and Home group order follow the target, but no installed 2024 build or matched screenshot was available for measured comparison.
- Large Paste/New Slide commands, compact icon rows, active-tab underline, quick-access icons and notes below the canvas now follow the desktop composition more closely. Original vector icons and egui controls still differ from PowerPoint iconography, Segoe/Aptos metrics, Backstage details and exact spacing. Bundled font fallback and synthetic bold have different text metrics. No Microsoft assets are reused.
- Home now collapses groups into command popups at narrow widths and supports a tabs-only state. Collapse priorities/thresholds are prototype choices, not measured PowerPoint behavior. Tab overflow and other command rows still scroll horizontally. At 800 × 560, the open side panes leave a small canvas and some tabs require scrolling. Native OS window decoration differs on macOS.
- Reordering uses Move Up/Down; thumbnail drag/drop, multi-selection, rotation, paragraph controls, rich text, themes, animation, recording and presenter tools are incomplete or unavailable. Unsupported actions are disabled with explanations.

## Checks and limits

| Check | Result / evidence |
| :--- | :--- |
| Workspace build | Passed locally with locked dependencies |
| Regression suite | **32 tests passed**: 19 core tests including full-model save/reopen after deleting original image, undo/redo/stacking, safe failed saves, input/resource/geometry limits, shared scene and view transforms; 4 actual binary CLI tests; 9 headless GUI/controller tests for note preservation, dirty-document guarding, resize geometry, keyboard button activation/disabled state, and unchanged document at 1440 × 815 / 980 × 680 / 800 × 560 with different panel states; compact-popup pointer activation creates a real rectangle with undo/redo; popup focus blocks canvas Delete; Ctrl+F1 and tab double-click from both initial states are exercised |
| CLI | Actual binary capabilities/version print without GUI; manifest validated against the exact pinned Office v1 schema, dispatch actions limited to new/open; malformed argument/document refusal tested. Native Office-to-Slides process dispatch not tested |
| Formatting and lint | `cargo fmt --all -- --check` and Clippy with `-D warnings` passed locally |
| Running native framebuffer | Ten current captures generated successfully and inspected; show PNG is byte-identical to the prior capture, providing a bounded fixture regression check |
| Windows 100%, 125%, 150% | **Not run**: host is macOS; mathematical transform tests are not native Windows DPI verification |
| Native keyboard, focus and Save/Discard/Cancel interaction | **Not run manually**: Computer Use reported `Computer Use permissions are not granted`; unbundled executable also could not be selected as an app. Code paths exist; no successful manual claim |
| Native resize | **Programmatic native check passed** at 980 × 680 and 800 × 560 with real framebuffer captures. Manual window dragging/resizing remains unverified |
| Native pointer drag/resize, open/save dialogs and fullscreen launch/exit | **Not run manually** for the same permission limitation. Show capture verifies content composition only |
| PowerPoint 2024 measured comparison | **Not run**: installed Windows reference/build unavailable |
| Earlier server CI | Code commit `0694f99c15ad880dd3f4ef1081c4d621a6b1b379`: [PR run](https://github.com/PerchAnton/breakfree-slides/actions/runs/38876993449) and [push run](https://github.com/PerchAnton/breakfree-slides/actions/runs/38876986573) completed successfully, verified 2026-10-16 12:01 Europe/Kyiv. All three Linux/Windows/macOS jobs passed formatting, Clippy, tests, native build and headless capabilities. This does not establish native desktop/DPI acceptance. Documentation-only revisions are separate from this tested code SHA |
| Merge / release | Neither performed; software/asset license decisions remain open |

Next validation: approve a native Windows review environment, record the exact PowerPoint 2024 reference build/parameters in UI_SPEC, execute the DPI/resize/focus/keyboard/dialog/show matrix, then address measured layout differences and richer editing. A preserved model is necessary but does not establish full visual parity.

## Version assignment follow-up — 2026-10-16

Historical binary identity for the previous stage was **0.1.0-alpha.2**, assigned under the owner's mandatory per-change increment rule. That stage reused alpha.1 screenshots (retained in Git history); the current images above have now been recaptured with alpha.3. No GUI run was claimed for the alpha.2 identity-only update. The renderer, model and document/protocol versions are unchanged.

Repeated locally for alpha.2: all 29 tests, locked workspace build, rustfmt and Clippy with warnings denied passed. Both workspace packages and Cargo.lock agree; actual `--version` and Office `app_version` output are `0.1.0-alpha.2`. New server CI status is tracked by exact commit in [the project log](PROJECT_LOG.md); earlier server evidence above remains historical. The owner accepted alpha.2 on 2026-10-16. Alpha.3 acceptance, merge and publication remain pending.
