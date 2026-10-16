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
- End every completed work report with the actual current application version and its owner-approval status, even when the number did not change. Separate a proposed next version from the actual version. The existing `0.1.0-alpha.1` is awaiting explicit owner approval under this policy.
- Obtain explicit owner confirmation for a proposed version change or channel promotion. Record the approved version and confirmation date in `docs/VERSIONING.md` and both project logs. No reply, successful tests, CI, commit or PR publication counts as approval.
- Maintain `CHANGELOG.md`; record unreleased work separately from published releases. Creating tags, publishing artifacts/releases and merging require the corresponding explicit authorization; version approval alone does not authorize those actions. Preserve published tags and artifacts without rewriting them.

## Журнал и приватное зеркало

Maintain **two complete copies** of the BreakFree Slides project log after every meaningful work stage and whenever a check is requested:

- Primary: `PerchAnton/breakfree-slides`, path `docs/PROJECT_LOG.md`, published through the project's branch/PR workflow.
- Private mirror: `PerchAnton/gitwork`, branch `main`, exact path `breakfree/breakfree-slides/PROJECT_LOG.md`.

This requirement replaces the previous instruction not to duplicate this project's log in `gitwork`. Keep this project's records separate from other projects. Before changes, read the current repository instructions and log, and the current `prompts/breakfree-slides.md` in `PerchAnton/gitwork/main`. Fetch remote updates safely and preserve local and other contributors' changes.

Keep the six log sections: «Текущее состояние», «Работа и проверки», «Комментарии и ревью», «Предложения и просьбы о помощи», «Следующие действия» and «История проверок». Use tables for events and tasks. Record Europe/Kyiv timestamps, completed work, commit/PR/issue links, actual verification results and limitations. Separate code readiness, local tests, server CI, merge and release.

For issues, PR conversations, reviews, inline threads and enabled Discussions, record each comment's @author, human/bot account type, date, direct URL, substance, required action and handling status. Record explicit help offers and requests with task, message URL and whether an owner response is needed. An offer is not an assignment; stars and forks are not offers. Record prioritized next actions, their sources, confirmed responsible people only, status and blockers. Preserve history, update an existing event by its canonical URL, and record source coverage, checkpoints and inaccessible sources. Include new replies on older issues and PRs in later checks.

Publish journal updates in dedicated commits in the Slides repository. Then read fresh `gitwork/main` and its existing mirror before copying. Reconcile any additional mirror records into the primary log without losing history or publishing private correspondence or secrets, publish the reconciled source, and copy its **entire text without modification** into the mirror. Never append mirror metadata to `PROJECT_LOG.md` itself.

Store mirror metadata separately at `PerchAnton/gitwork/main:breakfree/breakfree-slides/SYNC.md`: source repository, branch, path, full published source commit SHA, immutable source-file URL and synchronization time in Europe/Kyiv. Publish the mirror and metadata in a dedicated commit to `gitwork/main`; update only this project's row in the Breakfree table of its root `README.md` with brief status, source/mirror links, check time and next step. Preserve every other row and file. On a concurrent update, reread and reconcile the latest state, then retry without force push.

Read both published copies back from GitHub and verify exact text equality. Updating only the Slides repository is incomplete. Report source and mirror commit links separately; do not claim synchronization before publication and verification. If private publication is unavailable, retain a ready local copy and explicitly report «приватная копия не отправлена».

Do not send public replies, invitations or assignments without separate user instructions. No automatic monitoring is configured by this rule; do not promise scheduled checks.
