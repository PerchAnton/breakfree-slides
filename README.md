<p align="center">
  <a href="https://github.com/PerchAnton/breakfree-office"><img src="docs/brand/breakfree-slides-transparent.webp" alt="BreakFree Slides" width="160" height="160"></a>
</p>

<h1 align="center">BreakFree Slides</h1>

<p align="center">
  <strong>Your ideas. Your freedom.</strong><br>
  An independent presentation application in the making.<br>
  Give your ideas a clear shape — one slide at a time.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/status-prototype-D86B63?style=flat-square" alt="Status: prototype">
  <img src="https://img.shields.io/badge/focus-presentations-D86B63?style=flat-square" alt="Focus: presentations">
  <img src="https://img.shields.io/badge/suite-BreakFree%20Office-241337?style=flat-square" alt="Part of BreakFree Office">
</p>

<p align="center">
  <a href="#make-your-point-beautifully">Vision</a> ·
  <a href="ROADMAP.md">Roadmap</a> ·
  <a href="docs/REQUIREMENTS.md">First milestone</a> ·
  <a href="CONTRIBUTING.md">Contribute</a> ·
  <a href="https://github.com/PerchAnton/breakfree-office">Office hub</a>
</p>

---

> [!IMPORTANT]
> **Native Rust prototype for review.** Editable text, rectangles and embedded raster images, undo/redo, native save/reopen and a minimal shared-renderer slide show are implemented. Automated checks and running-app captures are documented in [verification](docs/UI_VERIFICATION.md). Native Windows interaction/DPI checks and measured PowerPoint 2024 comparison remain unverified. No installers or PPTX support are available.

## Make your point beautifully

BreakFree Slides aims to help people turn an outline into a clear presentation: arrange slides, place text and visuals, save the deck locally and present it with confidence.

We want a small, understandable editor before a large collection of effects. Readability and dependable layout come first.

## The first useful workflow

> **Create slides → Add content → Arrange → Save → Reopen → Present.**

The prototype includes a native Ribbon workspace, real slide thumbnails, text/rectangle/image editing, object geometry and stacking, undo/redo, local `.bfslide` files with embedded images, notes and a minimal presentation mode. The visual target is PowerPoint 2024 for Windows, with independent BreakFree branding; this is not a claim of visual or file-format parity.

| Prototype scope | Investigate next | Outside the initial scope |
| :--- | :--- | :--- |
| Add, duplicate and reorder slides | Layouts and reusable themes | Full PowerPoint feature parity |
| Text, shapes and image placement | Speaker notes and presenter tools | Complex animation timelines |
| Selection, geometry and object order | PDF/image export | Embedded active content |
| Undo, redo, save and reopen | A documented PPTX subset | Cloud collaboration |

See the [acceptance criteria](docs/REQUIREMENTS.md) and [roadmap](ROADMAP.md) for the proposed sequence.

## Design direction

**Make editing predictable.** Objects should remain where they were placed. The canvas, saved deck and presentation output need a clearly defined coordinate model.

**Let the message lead.** Prefer clear typography, accessible controls and useful alignment tools over visual effects that obscure the content.

**Keep the deck yours.** Prioritize local files and offline essentials. Do not silently upload images, fonts or presentation content to external services.

These are implementation goals, not claims about working software.

## File compatibility

PPTX import and export are development goals. Text, fonts, images, object positions, slide dimensions and theme behavior need separate tests. A file opening successfully does not prove it will look the same when presented.

A future compatibility matrix must describe unsupported elements and font substitution. Macros and other executable document content are outside the first milestone; no automatic execution is planned.

## Development status

| Area | Current position |
| :--- | :--- |
| Product direction | Documented |
| Implementation language and UI toolkit | Rust required by the owner; native egui/eframe prototype proposed for review |
| Slide model and native deck format | Editable objects in logical slide units; versioned `.bfslide` JSON with embedded raster bytes |
| Windows, macOS and Linux | Native targets; local build/captures on macOS, Windows/DPI and other platform evidence still pending |
| Application code and automated tests | Rust workspace, command/persistence/contract regression tests and actual GUI captures present |
| Software license | Owner decision pending |

Build and run independently from this repository:

```text
cargo build --workspace --locked
cargo test --workspace --locked
cargo run --locked -p breakfree-slides -- --new
```

See [Windows and other native launch instructions](docs/BUILD.md), [the owner-approved Rust direction and architecture](docs/adr/0001-rust-native-editor.md), [UI reference parameters](docs/UI_SPEC.md), [actual captures and verification limits](docs/UI_VERIFICATION.md), and [dependency/font notices](docs/DEPENDENCIES.md). `--capabilities` emits JSON without starting the GUI; `--version` reports the build version. Office integration uses an explicitly unverified local fallback snapshot until an upstream contract is published.

## The BreakFree family

| Project | Purpose |
| :--- | :--- |
| [**BreakFree Office**](https://github.com/PerchAnton/breakfree-office) | Suite hub, identity and shared direction |
| [**BreakFree Docs**](https://github.com/PerchAnton/breakfree-docs) | Documents and writing |
| [**BreakFree Sheets**](https://github.com/PerchAnton/breakfree-sheets) | Spreadsheets and calculations |
| **BreakFree Slides** | Presentations — you are here |

## Get involved

Suggest a presentation workflow, accessibility improvement or a focused technical investigation in [Issues](https://github.com/PerchAnton/breakfree-slides/issues). Use synthetic examples and redistributable visuals rather than confidential decks. Read [CONTRIBUTING.md](CONTRIBUTING.md) and [AGENTS.md](AGENTS.md) before preparing changes.

Everyone is welcome, including LGBTQ+ contributors and users. Follow the [shared Code of Conduct](https://github.com/PerchAnton/breakfree-office/blob/main/CODE_OF_CONDUCT.md).

## License and independence

No software license has been selected; public visibility alone is not a reuse license. Licensing will be resolved before substantive third-party implementation contributions are accepted.

BreakFree Slides is independent and is not affiliated with Microsoft. Microsoft PowerPoint and PPTX references describe intended workflows and interoperability goals, not endorsement or a completed replacement.

---

<p align="center"><strong>BreakFree Office · Your work. Your freedom.</strong><br><sub>Created by <a href="https://github.com/PerchAnton">Anton Perchenko</a></sub></p>
