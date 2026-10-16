# Dependencies and notices

No software license has been selected for BreakFree Slides. Dependency licenses and preserved upstream notices do not choose the project's license. No Microsoft artwork, fonts or code have been added. Brand-asset licensing remains an owner decision.

The locked Cargo graph is recorded in Cargo.lock. Direct dependency purposes: serde/serde_json for the bounded native document and capability JSON; base64 for embedded rasters; image for validated PNG/JPEG/WebP decoding and diagnostic captures; tempfile for sibling temporary files and atomic replacement; eframe/egui for native Rust controls and shared GPU painting; rfd for native local-file dialogs. Only native compilation is configured; browser/WebAssembly target dependencies in Cargo.lock are not an embedded web editor.

Dependency source packages retain their own manifests/license notices in the Cargo registry. eframe/egui use MIT OR Apache-2.0 terms. The default-font package also contains font-specific notices, preserved here because fonts are embedded by the dependency:

- [Ubuntu Font Licence](third-party/fonts/UFL.txt)
- [SIL Open Font License](third-party/fonts/OFL.txt)
- [Hack font notices](third-party/fonts/Hack-Regular.txt)
- [Emoji icon font MIT notice](third-party/fonts/emoji-icon-font-mit-license.txt)

Sources: epaint_default_fonts 0.33.3, files under its `fonts/` directory, copied without modification. Application text currently uses bundled proportional/fallback faces; arbitrary font selection and PowerPoint font fidelity are not claimed. Synthetic bold is applied by the shared painter. No remote font download occurs. Review the complete transitive license/notice inventory and the owner's software/asset license decisions before redistributing binaries.
