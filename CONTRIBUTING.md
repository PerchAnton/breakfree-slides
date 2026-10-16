# Contributing to BreakFree Slides

Read the [shared contribution guide](https://github.com/PerchAnton/breakfree-office/blob/main/CONTRIBUTING.md) and [Code of Conduct](https://github.com/PerchAnton/breakfree-office/blob/main/CODE_OF_CONDUCT.md).

Use this repository for slide models, object editing, layout, persistence, presentation mode and PPTX investigations. Suite-wide identity and policy belong in [BreakFree Office](https://github.com/PerchAnton/breakfree-office).

Start with [ROADMAP.md](ROADMAP.md) and [the first milestones](docs/REQUIREMENTS.md). Discuss rendering architecture, asset/font handling, dependencies and licensing before major implementation work. The software license is an owner decision still to be made.

Use synthetic decks and redistributable visual assets. Never attach confidential presentations or font files without redistribution permission. Explain the checks actually run and identify visual limitations. Follow [the Rust build/test instructions](docs/BUILD.md), [the approved direction and prototype ADR](docs/adr/0001-rust-native-editor.md), and [UI verification](docs/UI_VERIFICATION.md). Rust is mandatory; no JavaScript/TypeScript, Electron or WebView editor. Native Windows/DPI evidence must be reported separately from macOS or headless model tests.

Follow [the version/approval policy](docs/VERSIONING.md) and maintain [the changelog](CHANGELOG.md). The initial alpha/beta/RC line is `0.1.0`; the first authorized public product release is `0.2.0`. End every completed work report with the current version and approval status. Increment the current prerelease counter and update CHANGELOG for every completed logical change, including standalone documentation changes. Related implementation/verification/journal commits share one increment. Routine increments are preauthorized; candidate acceptance, channel promotion, tags, merge and publication remain separate decisions.
