# Working in BreakFree Slides

Read README.md, ROADMAP.md, docs/REQUIREMENTS.md, docs/UI_SPEC.md, docs/VERSIONING.md, CONTRIBUTING.md and the ADRs first. Rust is the owner-approved mandatory implementation language. The software license remains an owner decision.

## Scope

Prioritize predictable object editing and safe save/reopen behavior, then basic presentation mode. Suite-wide branding lives in `PerchAnton/breakfree-office`.

## Rules

- Preserve the approved BreakFree Slides name and dedicated transparent Slides icon.
- Keep public documentation in English; separate plans from verified features.
- Obtain owner review for license, architecture and major dependency choices.
- Define coordinates, text/font fallback, asset storage and rendering semantics explicitly.
- Preserve supported content and object placement across undo and persistence.
- Do not silently fetch remote assets, upload decks or execute embedded active content.
- Use synthetic decks and legally redistributable visuals; do not commit confidential slides or restricted fonts.
- Test model changes and save/reopen behavior; use defined visual fixtures once rendering exists.
- Do not invent PPTX compatibility, platform support, passing tests or working installers.
- Update development status only after features are implemented and verified.

## Validation

Run the documented Rust formatting, lint and test commands, plus Markdown links, image references and YAML checks. Report automated model/persistence tests, native GUI evidence and unavailable Windows/DPI checks separately. Do not infer Windows verification from macOS screenshots.

## Approved language and desktop UI direction

- Implement the model, commands, persistence, rendering, native editor, slide show and Office adapter in Rust. JavaScript/TypeScript, Electron and a WebView-hosted editor are not allowed.
- Follow the Windows desktop Microsoft PowerPoint 2024 workspace as closely as possible in composition, proportions, panel sizes, command groups and interactions, while retaining BreakFree branding. Record reference parameters and honest comparison limits in `docs/UI_SPEC.md`; do not invent a measured build or screenshot reference.
- Preserve a title/document bar, File and compact Ribbon, the specified main tabs and Home groups, left slide thumbnails, neutral central workspace, collapsible notes, contextual right formatting pane and status/view/zoom controls. Unimplemented actions must be disabled with an explanation. No fake cloud functions or Microsoft logo/name substitution.
- Keep the GUI separate from the presentation model and engine. Editing, thumbnails and slide show must share the same content rendering path and slide geometry. Selection handles follow object geometry; viewport/DPI zoom never changes stored coordinates.
- Capture actual running-app states and maintain explicit Windows 100%, 125%, 150%, resize, focus and keyboard verification status. Owner approval of Rust and the visual target is final; the provisional Rust GUI toolkit remains reviewable, and the license is unchanged. See `docs/adr/0001-rust-native-editor.md`.

## Versions and owner approval

- Follow `docs/VERSIONING.md` and ADR 0002. Keep all alpha, beta and release-candidate builds in the `0.1.0` line: `0.1.0-alpha.N`, `0.1.0-beta.N`, `0.1.0-rc.N`. The first owner-authorized public product release starts at `0.2.0` (display shorthand: 0.2).
- Use `[workspace.package].version` in root `Cargo.toml` as the application-version source of truth. Keep inherited package/lock versions, executable `--version`, Office `app_version` and release notes consistent. Native document content and Office protocol versions are independent.
- Increment the current prerelease counter by one for every completed logical change (code, UI, fixes, configuration or standalone documentation) and add a matching `CHANGELOG.md` entry. The owner has authorized these routine increments; do not wait for another permission request before applying them. Version/changelog/log/verification commits belonging to that same change share one version and do not recursively trigger another bump. Read-only checks do not increment the version.
- End every completed work report with the actual current application version and its owner-approval status for review. The current `0.1.0-alpha.2` is assigned under the automatic increment rule; explicit owner acceptance of the candidate remains pending.
- Obtain explicit owner confirmation for channel promotion or a change to the versioning scheme. Record candidate acceptance and its date in `docs/VERSIONING.md` and `docs/PROJECT_LOG.md`. Routine counter increments are preauthorized; silence, successful tests, CI, commits and PR publication do not mean acceptance of the candidate or authorization to release.
- Maintain `CHANGELOG.md`; record unreleased work separately from published releases. Creating tags, publishing artifacts/releases and merging require the corresponding explicit authorization; version approval alone does not authorize those actions. Preserve published tags and artifacts without rewriting them.

## Project log

Maintain `docs/PROJECT_LOG.md` after each meaningful work stage and whenever a check is requested. Keep the six sections: «Текущее состояние», «Работа и проверки», «Комментарии и ревью», «Предложения и просьбы о помощи», «Следующие действия» and «История проверок». Use English entries and tables for events/tasks, with Europe/Kyiv timestamps, completed work, source links, results and limitations.

Track issue/PR conversations, reviews, inline threads and enabled Discussions, including new replies on older objects. Record each event's @author, human/bot account type, date, direct URL, required action and handling status. Record explicit help offers/requests without inferring assignments from offers, stars or forks. Assign responsible people only when confirmed.

Preserve project history and update existing events by canonical URL. Record source coverage, checkpoints and inaccessible sources. Separate code readiness, local tests, server CI, merge and release. Fetch safely before writing, preserve local/contributor changes, and publish journal updates in dedicated commits through the branch/PR workflow. Do not force push.

Do not post public replies, invitations or assignments without separate owner instructions. No automatic monitoring is configured; do not promise scheduled checks.
