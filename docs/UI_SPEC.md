# BreakFree Slides — desktop UI specification

## Reference and evidence boundaries

Owner-approved target: **Microsoft PowerPoint 2024, Windows desktop**, own BreakFree branding. Target reference configuration: English (United States), White/light theme, expanded classic Ribbon, Normal view, 1440 × 900 logical window, 100% Windows display scaling initially, 16:9 slide, left thumbnails approximately 200 logical pixels, notes approximately 105 pixels when expanded, formatting pane approximately 268 pixels when open. Reference slide zoom is Fit to Window; compare at the same viewport rather than changing document coordinates.

The exact PowerPoint 2024 build number and measured reference capture are **not available in this macOS environment**. These are declared target parameters, not measurements from an installed PowerPoint instance. No pixel-perfect or measured similarity score is claimed. Before visual sign-off, record the installed PowerPoint build, Windows version, actual theme/font settings, native/client window dimensions, DPI and the captured Ribbon/pane sizes. A different Office update/channel can alter its layout.

Primary documentation: Microsoft's [Ribbon and contextual-tab guide](https://support.microsoft.com/en-us/powerpoint/where-are-the-menus-and-toolbars) explicitly applies to PowerPoint 2024. It confirms task-grouped tabs and context-dependent formatting. The owner's requested composition remains the implementation target. No Microsoft screenshots, logos, icons, fonts or code have been copied into the application.

## Composition and supported behavior

| Zone | Prototype target / behavior | Current scope |
| :--- | :--- | :--- |
| Title and quick access | 37 logical pixels; approved 26-pixel transparent Slides icon, own name, document name, Save, Undo, Redo | Functional; native OS title decoration is additional on macOS |
| File and tabs | File; Home, Insert, Draw, Design, Transitions, Animations, Slide Show, Record, Review, View, Help | Unsupported tabs disabled with tooltips; Record added per reference documentation |
| Compact expanded Ribbon | 137 pixels including tabs; white background, compact controls, group separators | Horizontal scrolling at narrow widths; no floating toolbar substitution |
| Home | Clipboard, Slides, Font, Paragraph, Drawing, Editing, in that order | Real object clipboard, slide actions, font size/color/synthetic bold, text/rectangle/image insertion, selection formatting and delete; unsupported commands disabled with explanations |
| Context tabs | Shape Format for text/rectangles; Picture Format for images; hidden without selection | Real size/position/color and arrange controls, no fake effects |
| Thumbnails | Resizable left rail, default 202 pixels | Actual shared-renderer slide content; select, duplicate, delete, move up/down; keyboard Ctrl/Cmd+M and Ctrl/Cmd+D |
| Workspace | Neutral gray, centered white 16:9 slide, bounded canvas with scrolling at high zoom | Selection, eight resize markers, drag move/resize, arrow nudge (Shift for 10 units), delete and in-object text editing |
| Notes | Collapsible/resizable lower panel, default 105 pixels | Editable notes preserved in native files; presenter tools are not implemented |
| Formatting | Contextual right pane, default 268 pixels | Numeric real model geometry, supported color/text controls, front/back order |
| Status | Slide number/count, Notes, Normal, show launch, zoom and Fit | Unimplemented view buttons disabled; zoom affects only viewport |
| Slide show | F5 beginning; Shift+F5 current; arrows/Space/Page keys; Escape | Same scene/painter, letterboxed fit, no transitions; show navigation does not edit the deck |
| File safety | File view and native dialogs; local .bfslide only | Save/Discard/Cancel for new/open/close; canceled/failed save leaves the pending operation unexecuted |

Object clipboard is internal to the current presentation; copying text to/from external applications uses the text editor's native clipboard. Slide reordering currently uses explicit Move Up/Down buttons rather than thumbnail drag/drop. The initial renderer has axis-aligned objects, one selected object, plain wrapped text and stretch-to-bounds images. Synthetic bold, bundled fonts, control styling, adaptive Ribbon collapse, accessibility and rich text differ from PowerPoint. No cloud/account/sharing status is fabricated.

## Running-app captures

Run `cargo run --locked -p breakfree-slides -- --ui-smoke <absolute_output_directory>` in an approved native desktop execution context. This opens the real application, builds an explicitly synthetic editable fixture, captures its actual rendered framebuffer and exits. It does not simulate a PowerPoint screenshot or certify manual pointer interactions. Ordinary `--new` starts with an empty deck; the fixture is only for the explicit smoke-test flag.

Required captures: empty presentation, three slides, selected rectangle, in-object text editing, open formatting pane. An additional shared-renderer show capture is included. Capture links and actual dimensions/results are recorded after execution in [UI verification](UI_VERIFICATION.md).

## Comparison and verification matrix

Compare only implemented states. Check title/Ribbon/rail/canvas/notes/pane/status composition, real thumbnail content, geometry/handles, text clipping, image persistence and identical object ordering in editor/show. Known differences must remain documented alongside the captures, not hidden behind a compatibility claim.

| Check | Required evidence | Availability |
| :--- | :--- | :--- |
| Windows 100%, 125%, 150% | Native captures and pointer/keyboard/focus at each scale | Not available on this macOS host; not run |
| PowerPoint 2024 comparison | Exact build, measured screenshots and matched parameters | No installed Windows reference; not run |
| Resize | Working native window at 1440 × 900 and smaller sizes; no unreachable essential actions | Native verification tracked separately |
| Focus/keyboard | Text edits, commands, selection, dialogs, show/exit | Track actual manual/automated coverage separately |
| Coordinate/DPI invariant | Model unchanged under view transform and show navigation | Core tests cover transform math; not Windows DPI certification |
| Regression | Undo/redo, assets after source removal, safe save/reopen, shared scene | Cargo integration tests; results in verification report |
