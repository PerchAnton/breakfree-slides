# BreakFree Slides — Roadmap

**Status: planning.** These are proposed milestones, not shipped features. No release dates are committed.

## 0 · Choose the foundation

- [ ] Agree the software license with the owner.
- [ ] Select a first platform, implementation language and UI toolkit.
- [ ] Define slide dimensions, coordinates, object types, text handling and asset storage.
- [ ] Create a buildable shell, reproducible instructions and test runner.

**Exit:** architecture decisions and a minimal application shell are reviewable.

## 1 · A dependable slide editor

- [ ] Create, duplicate, remove and reorder slides.
- [ ] Add and edit text boxes, basic shapes and raster images.
- [ ] Move/resize objects with predictable selection and stacking order.
- [ ] Undo/redo, local save/reopen and unsaved-change handling.
- [ ] Versioned persistence with tests for assets and failed saves.

**Exit:** editing and persistence scenarios in [docs/REQUIREMENTS.md](docs/REQUIREMENTS.md) pass without losing supported objects or their arrangement.

## 2 · Present with confidence

- [ ] Full-screen display, next/previous navigation and an obvious exit.
- [ ] Shared rendering semantics for editing and presentation.
- [ ] Explicit missing-font and missing-asset behavior.
- [ ] Keyboard navigation and accessibility review.

**Exit:** a saved/reopened test deck presents in the intended order and retains supported text, object placement and aspect ratio.

## 3 · Useful structure and interoperability

- [ ] Reusable layouts, themes and alignment tools.
- [ ] Speaker notes and presenter-tools investigation.
- [ ] PDF/image export with visual regression fixtures.
- [ ] A scoped PPTX importer/exporter and compatibility matrix.
- [ ] Warnings or safe refusal for unsupported and potentially lossy operations.

**Exit:** output is compared against defined fixtures; limitations and font substitution are visible and documented. PPTX support is not presented as complete PowerPoint parity.

## 4 · First public preview

- [ ] Verified packaging for the declared platform.
- [ ] Dependency and asset-license review.
- [ ] Private security-reporting route before public binaries.
- [ ] Real installation instructions, release notes and known limitations.

Complex animations, executable content, cloud collaboration and complete PowerPoint parity are outside the first release scope.

[Back to Slides](README.md) · [Suite roadmap](https://github.com/PerchAnton/breakfree-office/blob/main/ROADMAP.md)
