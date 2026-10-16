# First milestones — edit, save and present

**Acceptance requirements for the Rust prototype.** Model/editing/persistence and a shared-renderer show are implemented; automated results, native captures and incomplete GUI/platform acceptance are separated in [verification](UI_VERIFICATION.md). Milestone 1 verifies editing/persistence; milestone 2 adds presentation mode. A partial native GUI check is not completion of every acceptance scenario.

## User story

A person creates a short deck, adds text and images, arranges objects, saves it locally, reopens it and presents the slides in the intended order.

## In scope

Slide list and canvas, text boxes, basic shapes, raster images, selection, movement, resizing, stacking order, undo/redo and versioned local persistence. A minimal full-screen presentation mode is a separate follow-on milestone.

## Acceptance scenarios

| ID | Scenario | Observable result |
| :--- | :--- | :--- |
| P01 | Add, duplicate, remove and reorder slides | The deck model and displayed order agree. |
| P02 | Add and edit text, a shape and a supported image | Each object can be selected and edited independently. |
| P03 | Move, resize and reorder overlapping objects | Coordinates, dimensions and stacking order are predictable and persisted. |
| P04 | Undo and redo edits and slide operations | Both the deck model and canvas return to the intended states. |
| P05 | Save, close, reopen and continue editing | Supported objects, text, dimensions, assets and ordering survive. |
| P06 | Reopen when an original imported image has moved | The saved deck retains the image according to the documented embedding policy, or reports a missing asset explicitly; it does not silently lose content. |
| P07 | Close with unsaved changes or simulate a failed save | Choices/errors are clear; a good saved deck is not silently destroyed. |
| P08 | Open malformed input | The operation fails safely without modifying the input file. |
| P09 | Present a saved/reopened deck after milestone 2 | Slides advance in order, preserve the defined aspect ratio and retain supported placement. |
| P10 | Navigate and exit presentation mode with the keyboard | Next, previous and exit behave consistently and return to editing without altering content. |

## Required decisions

Specify coordinates and units, slide dimensions, text layout and font fallback, image embedding, native-format versioning and rendering consistency between edit and presentation modes. Record alternatives and limitations. Do not redistribute fonts without permission.

Rust and the PowerPoint 2024 Windows desktop visual target are owner-approved requirements. The prototype's explicit units, IDs, image policy, format and shared rendering are in [ADR 0001](adr/0001-rust-native-editor.md); [UI_SPEC](UI_SPEC.md) defines the Ribbon workspace and honest reference/verification boundaries. The license remains undecided. This application must build and run independently of other BreakFree repositories.

Do not execute macros, active embedded objects or imported scripts. Do not silently fetch remote assets or upload the deck.

## Not included yet

PPTX round-trip compatibility, complex animations, video playback, speaker notes, theme inheritance, cloud accounts and collaboration. These require later, separately testable milestones.

## Completion evidence

Reproducible build/test commands; synthetic decks; automated model, undo and persistence tests; defined visual fixtures; manual keyboard and presentation checks on the declared operating system. Presenting one sample is not proof of PowerPoint compatibility.

[Back to roadmap](../ROADMAP.md)
