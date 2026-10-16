# Working in BreakFree Slides

Read README.md, ROADMAP.md, docs/REQUIREMENTS.md and CONTRIBUTING.md first. The repository is currently documentation-only; language, toolkit, license and native format are not selected.

## Scope

Prioritize predictable object editing and safe save/reopen behavior, then basic presentation mode. Suite-wide branding lives in `PerchAnton/breakfree-office`.

## Rules

- Preserve the approved name and shared office-document icon.
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

For now, check Markdown links, image references and YAML syntax. Add real build/test commands with the first implementation. Report checks actually run, manual rendering checks and remaining limitations separately.

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
