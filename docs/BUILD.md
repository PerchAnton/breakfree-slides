# Build and run the native Rust prototype

The application is standalone. Office, Docs, Sheets, Node.js, a web server and a WebView runtime are not needed. The project software license remains undecided; this is source for review, not a released installer.

## Requirements

Install stable Rust using [rustup](https://rust-lang.org/tools/install/), including rustfmt and Clippy. The prototype was tested with Rust 1.99.1 on Apple Silicon macOS. The eframe baseline requires Rust 1.88 or later; the minimum compiler has not been separately verified for the full locked dependency graph. Use Cargo.lock.

Windows: use the MSVC Rust toolchain and Visual Studio Build Tools with **Desktop development with C++** and a Windows SDK. Windows compilation and headless tests passed in CI; native desktop file dialogs, fullscreen and DPI still require verification. macOS: Xcode Command Line Tools are needed. Linux: install native X11/Wayland/OpenGL development packages; the CI workflow lists them. Building requires initial crates.io access; editing, local files and presenting require no network access.

## Commands

From the repository root (same Cargo commands in PowerShell, Terminal or a Linux shell):

```text
cargo build --workspace --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo run --locked -p breakfree-slides -- --new
```

After building, Windows PowerShell:

```powershell
.\target\debug\breakfree-slides.exe --capabilities
.\target\debug\breakfree-slides.exe --version
.\target\debug\breakfree-slides.exe --new
.\target\debug\breakfree-slides.exe --open "C:\Presentations\Example.bfslide"
```

macOS/Linux:

```sh
./target/debug/breakfree-slides --capabilities
./target/debug/breakfree-slides --version
./target/debug/breakfree-slides --new
./target/debug/breakfree-slides --open "/absolute/path/Example.bfslide"
```

`--capabilities` prints protocol-v1 JSON to stdout without opening a window; `--version` also exits without GUI startup. Unknown/ambiguous arguments, relative open paths and invalid documents exit with a diagnostic and nonzero status. Run GUI commands in a native desktop session; an OS command sandbox can prevent app registration. Use a specifically approved native launch context rather than retrying a failing restricted GUI launch.

## First workflow

Home/Insert adds editable text, rectangles and local PNG/JPEG/WebP images. Click to select, drag to move, use one of eight handles to resize, or open Format to edit geometry. Double-click text (or Enter/F2 on selection) to edit inside its bounds; leave focus or Ctrl/Cmd+Enter to commit, Escape to cancel the text draft. Use arrange controls to change stacking order. Add, duplicate, delete and reorder slides in the left rail; reordering currently uses Move Up/Down buttons.

At narrower widths, Home groups collapse to buttons that open their actual commands. Ctrl+F1, the right Ribbon toggle or a supported-tab double-click switches between commands and tabs only. Formatting sections can collapse independently.

Ctrl/Cmd+Z undoes; Ctrl/Cmd+Shift+Z or Ctrl/Cmd+Y redoes. Ctrl/Cmd+M adds a slide; Ctrl/Cmd+D duplicates the selected object, or the current slide when no object is selected. The object clipboard supports Ctrl/Cmd+C/X/V within the deck. Save with Ctrl/Cmd+S or File, reopen through File/Ctrl/Cmd+O or `--open`. Native files embed imported image bytes. Save/Discard/Cancel guards new/open/close; canceled or failed saving does not execute the pending action.

F5 starts at the beginning, Shift+F5 at the current slide. Arrows, PageUp/PageDown and Space navigate; Escape returns to editing. The show uses the same slide-content painter as editor and thumbnails. No transition is applied.

## GUI evidence

```text
cargo run --locked -p breakfree-slides -- --ui-smoke <absolute_output_directory>
```

This explicit diagnostic opens the real application, constructs a synthetic editable three-slide fixture, captures ten framebuffer states and exits without saving the fixture as a user document. It is separate from ordinary launch behavior. See [UI_SPEC](UI_SPEC.md) and [UI verification](UI_VERIFICATION.md) for reference settings, captures, differences and unavailable checks. The later states resize the actual native window to 980 × 680 and 800 × 560, open a compact Home group, collapse the Ribbon and open formatting. Core/headless view-transform tests do not certify native Windows scaling.

Not implemented: PPTX, themes/layouts, complex animations, multimedia, AI, cloud collaboration, arbitrary fonts, rich text, rotation, multi-selection and presenter tools. Unimplemented commands are disabled with explanatory hover text.
