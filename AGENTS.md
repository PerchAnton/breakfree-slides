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
