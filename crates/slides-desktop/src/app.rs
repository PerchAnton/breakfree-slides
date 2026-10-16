use crate::chrome::{self, Icon, command, icon_button};
use crate::painting::{self, Textures};
use breakfree_slides_core::model::Id;
use breakfree_slides_core::{commands::Editor, model::*, render, storage};
use eframe::egui::*;
use std::{io::Cursor, path::PathBuf};

const ACCENT: Color32 = chrome::ACCENT;
const TABS: [&str; 12] = [
    "Home",
    "Insert",
    "Draw",
    "Design",
    "Transitions",
    "Animations",
    "Slide Show",
    "Record",
    "Review",
    "View",
    "Help",
    "File",
];
enum Pending {
    New,
    Open(PathBuf),
    Quit,
}
struct TextEditing {
    id: Id,
    text: String,
    focus: bool,
}
struct Drag {
    id: Id,
    original: Bounds,
    start: Pos2,
    handle: Option<usize>,
}
struct Capture {
    dir: PathBuf,
    stage: usize,
    frames: u32,
    pending: bool,
}
const SHOTS: [&str; 7] = [
    "01-empty",
    "02-three-slides",
    "03-selection",
    "04-text-edit",
    "05-format-pane",
    "06-slide-show",
    "07-narrow-window",
];

pub struct SlidesApp {
    editor: Editor,
    path: Option<PathBuf>,
    slide: usize,
    selected: Option<Id>,
    tab: String,
    textures: Textures,
    logo: TextureHandle,
    format: bool,
    notes: bool,
    notes_draft: String,
    editing: Option<TextEditing>,
    drag: Option<Drag>,
    preview: Option<Bounds>,
    clipboard: Option<Object>,
    zoom: f64,
    fit: bool,
    show: bool,
    pending: Option<Pending>,
    allow_close: bool,
    error: Option<String>,
    capture: Option<Capture>,
}
impl SlidesApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        deck: Deck,
        path: Option<PathBuf>,
        capture: Option<PathBuf>,
    ) -> Self {
        Self::with_context(&cc.egui_ctx, deck, path, capture)
    }
    fn with_context(
        ctx: &Context,
        deck: Deck,
        path: Option<PathBuf>,
        capture: Option<PathBuf>,
    ) -> Self {
        ctx.set_visuals(Visuals::light());
        ctx.style_mut(|s| {
            s.spacing.item_spacing = vec2(5.0, 3.0);
            s.spacing.button_padding = vec2(6.0, 3.0);
            s.text_styles
                .insert(TextStyle::Body, FontId::proportional(12.0));
            s.text_styles
                .insert(TextStyle::Button, FontId::proportional(12.0));
            s.visuals.override_text_color = Some(chrome::INK);
            s.visuals.panel_fill = Color32::from_gray(250);
            s.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, chrome::LINE);
            s.visuals.widgets.inactive.weak_bg_fill = Color32::from_gray(253);
            s.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, chrome::LINE);
            s.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(244, 232, 228);
            s.visuals.widgets.hovered.bg_stroke =
                Stroke::new(1.0_f32, Color32::from_rgb(225, 190, 183));
            for w in [
                &mut s.visuals.widgets.inactive,
                &mut s.visuals.widgets.hovered,
                &mut s.visuals.widgets.active,
            ] {
                w.corner_radius = CornerRadius::same(2);
            }
            s.visuals.selection.bg_fill = Color32::from_rgb(250, 225, 221);
            s.visuals.selection.stroke = Stroke::new(1.0_f32, ACCENT);
            s.visuals.widgets.active.bg_fill = Color32::from_rgb(246, 217, 212);
        });
        let rgba = image::load_from_memory(include_bytes!(
            "../../../docs/brand/breakfree-slides-transparent.webp"
        ))
        .expect("Approved embedded icon")
        .into_rgba8();
        let logo = ctx.load_texture(
            "breakfree-slides-brand",
            ColorImage::from_rgba_unmultiplied(
                [rgba.width() as usize, rgba.height() as usize],
                &rgba,
            ),
            TextureOptions::LINEAR,
        );
        Self {
            notes_draft: deck.slides[0].notes.clone(),
            editor: Editor::new(deck),
            path,
            slide: 0,
            selected: None,
            tab: "Home".into(),
            textures: Textures::new(),
            logo,
            format: false,
            notes: true,
            editing: None,
            drag: None,
            preview: None,
            clipboard: None,
            zoom: 80.0,
            fit: true,
            show: false,
            pending: None,
            allow_close: false,
            error: None,
            capture: capture.map(|dir| Capture {
                dir,
                stage: 0,
                frames: 0,
                pending: false,
            }),
        }
    }
    fn result<T>(&mut self, r: Result<T, String>) -> Option<T> {
        match r {
            Ok(v) => Some(v),
            Err(e) => {
                self.error = Some(e);
                None
            }
        }
    }
    fn finish_text(&mut self) {
        if let Some(edit) = self.editing.take() {
            let r = self.editor.set_text(self.slide, edit.id, edit.text);
            self.result(r);
        }
    }
    fn finish_notes(&mut self) {
        let text = self.notes_draft.clone();
        let index = self.slide;
        let r = self.editor.apply(|d| {
            d.slides.get_mut(index).ok_or("Missing slide")?.notes = text;
            Ok(())
        });
        self.result(r);
    }
    fn finish_edits(&mut self) {
        self.finish_text();
        self.finish_notes();
    }
    fn choose_slide(&mut self, index: usize) {
        self.finish_edits();
        self.activate_slide(index);
    }
    // The caller already flushed the outgoing draft before changing slide order.
    fn activate_slide(&mut self, index: usize) {
        self.slide = index.min(self.editor.deck.slides.len() - 1);
        self.notes_draft = self.editor.deck.slides[self.slide].notes.clone();
        self.selected = None;
        self.drag = None;
        self.preview = None;
    }
    fn request(&mut self, p: Pending, ctx: &Context) {
        self.finish_edits();
        if self.editor.dirty() {
            self.pending = Some(p);
        } else {
            self.perform(p, ctx);
        }
    }
    fn perform(&mut self, p: Pending, ctx: &Context) {
        match p {
            Pending::Quit => {
                self.allow_close = true;
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
            Pending::New => self.replace(Deck::new(), None),
            Pending::Open(path) => {
                let r = storage::load(&path);
                if let Some(d) = self.result(r) {
                    self.replace(d, Some(path));
                }
            }
        }
    }
    fn replace(&mut self, d: Deck, path: Option<PathBuf>) {
        self.editor = Editor::new(d);
        self.path = path;
        self.slide = 0;
        self.selected = None;
        self.editing = None;
        self.drag = None;
        self.preview = None;
        self.textures.clear();
        self.clipboard = None;
        self.notes_draft = self.editor.deck.slides[0].notes.clone();
        self.tab = "Home".into();
    }
    fn save(&mut self, as_new: bool) -> bool {
        self.finish_edits();
        let path = if as_new || self.path.is_none() {
            rfd::FileDialog::new()
                .add_filter("BreakFree Slides", &["bfslide"])
                .set_file_name("Presentation.bfslide")
                .save_file()
        } else {
            self.path.clone()
        };
        let Some(path) = path else {
            return false;
        };
        let r = storage::save(&self.editor.deck, &path);
        if self.result(r).is_some() {
            self.path = Some(path);
            self.editor.mark_saved();
            true
        } else {
            false
        }
    }
    fn open(&mut self, ctx: &Context) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("BreakFree Slides", &["bfslide"])
            .pick_file()
        {
            self.request(Pending::Open(path), ctx);
        }
    }
    fn add_text(&mut self) {
        self.finish_edits();
        let r = self.editor.add_object(
            self.slide,
            Content::Text {
                text: "Your ideas, clearly.".into(),
                font_size: 36.0,
                bold: false,
                color: [36, 32, 42, 255],
            },
            Bounds {
                x: 100.0,
                y: 100.0,
                w: 700.0,
                h: 110.0,
            },
        );
        if let Some(id) = self.result(r) {
            self.selected = Some(id);
        }
    }
    fn add_rectangle(&mut self) {
        self.finish_edits();
        let r = self.editor.add_object(
            self.slide,
            Content::Rectangle {
                fill: [215, 115, 103, 255],
            },
            Bounds {
                x: 180.0,
                y: 270.0,
                w: 330.0,
                h: 190.0,
            },
        );
        if let Some(id) = self.result(r) {
            self.selected = Some(id);
        }
    }
    fn add_image(&mut self) {
        self.finish_edits();
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Raster images", &["png", "jpg", "jpeg", "webp"])
            .pick_file()
        {
            let r = storage::read_image(&path).and_then(|b| self.editor.add_image(self.slide, &b));
            if let Some(id) = self.result(r) {
                self.selected = Some(id);
            }
        }
    }
    fn add_slide(&mut self) {
        self.finish_edits();
        let r = self.editor.add_slide(self.slide);
        if let Some(i) = self.result(r) {
            self.activate_slide(i);
        }
    }
    fn duplicate_slide(&mut self) {
        self.finish_edits();
        let r = self.editor.duplicate_slide(self.slide);
        if let Some(i) = self.result(r) {
            self.activate_slide(i);
        }
    }
    fn remove_slide(&mut self) {
        self.finish_edits();
        let r = self.editor.remove_slide(self.slide);
        if self.result(r).is_some() {
            self.activate_slide(self.slide);
        }
    }
    fn move_slide(&mut self, to: usize) {
        self.finish_edits();
        let r = self.editor.reorder_slide(self.slide, to);
        if self.result(r).is_some() {
            self.activate_slide(to);
        }
    }
    fn copy(&mut self, cut: bool) {
        self.finish_text();
        self.clipboard = self
            .selected
            .and_then(|id| self.editor.deck.object(self.slide, id).cloned());
        if cut {
            self.remove_object();
        }
    }
    fn paste(&mut self) {
        self.finish_text();
        if let Some(o) = self.clipboard.clone() {
            let mut b = o.bounds;
            b.x += 20.0;
            b.y += 20.0;
            let r = self.editor.add_object(self.slide, o.content, b);
            if let Some(id) = self.result(r) {
                self.selected = Some(id);
            }
        }
    }
    fn remove_object(&mut self) {
        self.finish_text();
        if let Some(id) = self.selected.take() {
            let r = self.editor.remove_object(self.slide, id);
            self.result(r);
        }
    }
    fn history(&mut self, redo: bool) {
        self.finish_edits();
        if redo {
            self.editor.redo()
        } else {
            self.editor.undo()
        }
        self.slide = self.slide.min(self.editor.deck.slides.len() - 1);
        self.notes_draft = self.editor.deck.slides[self.slide].notes.clone();
        self.selected = None;
        self.drag = None;
        self.preview = None;
    }
    fn start_show(&mut self, from_start: bool, ctx: &Context) {
        self.finish_edits();
        if from_start {
            self.slide = 0;
        }
        self.show = true;
        ctx.send_viewport_cmd(ViewportCommand::Fullscreen(true));
    }
    fn stop_show(&mut self, ctx: &Context) {
        self.show = false;
        self.notes_draft = self.editor.deck.slides[self.slide].notes.clone();
        ctx.send_viewport_cmd(ViewportCommand::Fullscreen(false));
    }
    fn keyboard(&mut self, ctx: &Context) {
        if self.show {
            let (exit, next, prev) = ctx.input(|i| {
                (
                    i.key_pressed(Key::Escape),
                    i.key_pressed(Key::ArrowRight)
                        || i.key_pressed(Key::ArrowDown)
                        || i.key_pressed(Key::Space)
                        || i.key_pressed(Key::PageDown),
                    i.key_pressed(Key::ArrowLeft)
                        || i.key_pressed(Key::ArrowUp)
                        || i.key_pressed(Key::PageUp),
                )
            });
            if exit {
                self.stop_show(ctx);
            } else if next {
                self.slide = (self.slide + 1).min(self.editor.deck.slides.len() - 1);
            } else if prev {
                self.slide = self.slide.saturating_sub(1);
            }
            return;
        }
        if self.pending.is_some() {
            return;
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::S)) {
            self.save(false);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::O)) {
            self.open(ctx);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::N)) {
            self.request(Pending::New, ctx);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::SHIFT, Key::F5)) {
            self.start_show(false, ctx);
        } else if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::F5)) {
            self.start_show(true, ctx);
        }
        if ctx.wants_keyboard_input() {
            return;
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::COMMAND | Modifiers::SHIFT, Key::Z))
            || ctx.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::Y))
        {
            self.history(true);
        } else if ctx.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::Z)) {
            self.history(false);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::C)) {
            self.copy(false);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::X)) {
            self.copy(true);
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::V)) {
            self.paste();
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::M)) {
            self.add_slide();
        }
        if ctx.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::D)) {
            if self.selected.is_some() {
                self.copy(false);
                self.paste();
            } else {
                self.duplicate_slide();
            }
        }
        if ctx.input(|i| i.key_pressed(Key::Delete) || i.key_pressed(Key::Backspace)) {
            self.remove_object();
        }
        if let Some(id) = self.selected {
            let (dx, dy) = ctx.input(|i| {
                let step = if i.modifiers.shift { 10.0 } else { 1.0 };
                (
                    (if i.key_pressed(Key::ArrowRight) {
                        step
                    } else {
                        0.0
                    }) - (if i.key_pressed(Key::ArrowLeft) {
                        step
                    } else {
                        0.0
                    }),
                    (if i.key_pressed(Key::ArrowDown) {
                        step
                    } else {
                        0.0
                    }) - (if i.key_pressed(Key::ArrowUp) {
                        step
                    } else {
                        0.0
                    }),
                )
            });
            if (dx != 0.0 || dy != 0.0)
                && let Some(o) = self.editor.deck.object(self.slide, id)
            {
                let mut b = o.bounds;
                b.x += dx;
                b.y += dy;
                let r = self.editor.transform(self.slide, id, b);
                self.result(r);
            }
            if ctx.input(|i| i.key_pressed(Key::Enter) || i.key_pressed(Key::F2)) {
                self.begin_text(id);
            }
        }
    }
    fn begin_text(&mut self, id: Id) {
        if let Some(Object {
            content: Content::Text { text, .. },
            ..
        }) = self.editor.deck.object(self.slide, id)
        {
            self.editing = Some(TextEditing {
                id,
                text: text.clone(),
                focus: true,
            });
        }
    }
    fn title_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("document-bar")
            .exact_height(40.0)
            .frame(
                Frame::new()
                    .fill(Color32::from_rgb(247, 240, 238))
                    .inner_margin(Margin::symmetric(12, 6)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.image((self.logo.id(), vec2(25.0, 25.0)));
                    ui.add_space(6.0);
                    if icon_button(ui, "Save (Ctrl/Cmd+S)", Icon::Save, true).clicked() {
                        self.save(false);
                    }
                    if icon_button(ui, "Undo (Ctrl/Cmd+Z)", Icon::Undo, self.editor.can_undo())
                        .clicked()
                    {
                        self.history(false);
                    }
                    if icon_button(ui, "Redo (Ctrl/Cmd+Y)", Icon::Redo, self.editor.can_redo())
                        .clicked()
                    {
                        self.history(true);
                    }
                    ui.separator();
                    ui.add_space(8.0);
                    let name = self
                        .path
                        .as_ref()
                        .and_then(|p| p.file_name())
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or(self.editor.deck.title.clone());
                    ui.add(
                        Label::new(
                            RichText::new(format!(
                                "{}{}",
                                name,
                                if self.editor.dirty() { " •" } else { "" }
                            ))
                            .size(13.0),
                        )
                        .truncate(),
                    );
                    ui.label(
                        RichText::new("— BreakFree Slides")
                            .size(12.0)
                            .color(Color32::from_gray(106)),
                    );
                });
            });
    }
    fn ribbon(&mut self, ctx: &Context) {
        TopBottomPanel::top("ribbon")
            .exact_height(144.0)
            .frame(Frame::new().fill(Color32::from_gray(252)).inner_margin(Margin::symmetric(10, 2)))
            .show(ctx, |ui| {
                ScrollArea::horizontal().id_salt("tabs").show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if chrome::tab(ui, "File", self.tab == "File", true, true).clicked() {
                            self.finish_text(); self.tab = "File".into();
                        }
                        for name in &TABS[..11] {
                            let supported = matches!(*name, "Home" | "Insert" | "Slide Show" | "View" | "Help");
                            let r = chrome::tab(ui, name, self.tab == *name, supported, false);
                            if r.clicked() { self.finish_text(); self.tab = (*name).into(); }
                            if !supported { r.on_disabled_hover_text("Not implemented: this tab's commands are not available yet."); }
                        }
                        if let Some(o) = self.selected.and_then(|id| self.editor.deck.object(self.slide, id)) {
                            let label = if matches!(o.content, Content::Image { .. }) { "Picture Format" } else { "Shape Format" };
                            if chrome::tab(ui, label, self.tab == "Format", true, true).clicked() {
                                self.finish_text(); self.tab = "Format".into(); self.format = true;
                            }
                        }
                    });
                });
                ui.separator();
                ScrollArea::horizontal().id_salt("ribbon-commands").show(ui, |ui| {
                    ui.horizontal_top(|ui| match self.tab.as_str() {
                        "Home" => self.home(ui),
                        "Insert" => {
                            group(ui,"Slides",85.0,|ui| { if command(ui,"New Slide",Icon::Slide,true,true).clicked() { self.add_slide(); } });
                            group(ui,"Images",90.0,|ui| { if command(ui,"Pictures…",Icon::Picture,true,true).clicked() { self.add_image(); } });
                            group(ui,"Illustrations",100.0,|ui| { if command(ui,"Rectangle",Icon::Shape,true,true).clicked() { self.add_rectangle(); } });
                            group(ui,"Text",85.0,|ui| { if command(ui,"Text Box",Icon::Text,true,true).clicked() { self.add_text(); } });
                            group(ui,"Media",140.0,|ui| { disabled(ui,"Video / Audio","Multimedia is not implemented."); });
                        }
                        "Slide Show" => {
                            group(ui,"Start Slide Show",260.0,|ui| { ui.horizontal_top(|ui| {
                                if command(ui,"From Beginning",Icon::Show,true,true).on_hover_text("F5").clicked() { self.start_show(true,ctx); }
                                if command(ui,"From Current Slide",Icon::Show,true,true).on_hover_text("Shift+F5").clicked() { self.start_show(false,ctx); }
                            }); });
                            group(ui,"Set Up",240.0,|ui| { disabled(ui,"Presenter View","Presenter tools and multiple displays are not implemented."); ui.label("Arrow keys / Space · Esc to exit"); });
                        }
                        "View" => {
                            group(ui,"Presentation Views",250.0,|ui| { ui.label("Normal view"); disabled(ui,"Slide Sorter / Reading View","Additional views are not implemented."); });
                            group(ui,"Show",160.0,|ui| { ui.checkbox(&mut self.notes,"Notes"); ui.checkbox(&mut self.format,"Format pane"); });
                            group(ui,"Zoom",130.0,|ui| { if command(ui,"Fit to Window",Icon::Fit,true,true).clicked() { self.fit=true; } });
                        }
                        "Format" => {
                            group(ui,"Format",140.0,|ui| { if command(ui,"Format Selection",Icon::Format,true,self.selected.is_some()).clicked() { self.format=true; } });
                            self.arrange(ui);
                        }
                        "Help" => { group(ui,"About",450.0,|ui| {
                            ui.label(format!("BreakFree Slides {} · native Rust",env!("CARGO_PKG_VERSION")));
                            ui.label("Local .bfslide files · PNG / JPEG / WebP");
                            ui.label("Prototype: no PPTX, cloud, transitions or animations");
                        }); }
                        "File" => { ui.label("Local presentation files — choose an action below."); }
                        _ => {}
                    });
                });
            });
    }
    fn home(&mut self, ui: &mut Ui) {
        group(ui, "Clipboard", 157.0, |ui| {
            ui.horizontal_top(|ui| {
                if command(ui, "Paste", Icon::Paste, true, self.clipboard.is_some())
                    .on_disabled_hover_text("Copy an object in this presentation first.")
                    .clicked()
                {
                    self.paste();
                }
                ui.vertical(|ui| {
                    if command(ui, "Cut", Icon::Cut, false, self.selected.is_some()).clicked() {
                        self.copy(true);
                    }
                    if command(ui, "Copy", Icon::Copy, false, self.selected.is_some()).clicked() {
                        self.copy(false);
                    }
                    command(ui, "Format Painter", Icon::Format, false, false)
                        .on_disabled_hover_text("Copying formatting is not implemented.");
                });
            });
        });
        group(ui, "Slides", 166.0, |ui| {
            ui.horizontal_top(|ui| {
                if command(ui, "New Slide", Icon::Slide, true, true).clicked() {
                    self.add_slide();
                }
                ui.vertical(|ui| {
                    command(ui, "Layout", Icon::Layout, false, false).on_disabled_hover_text(
                        "Slide layouts are not implemented. New Slide creates a blank slide.",
                    );
                    if command(ui, "Duplicate", Icon::Copy, false, true).clicked() {
                        self.duplicate_slide();
                    }
                    if command(
                        ui,
                        "Delete",
                        Icon::Delete,
                        false,
                        self.editor.deck.slides.len() > 1,
                    )
                    .clicked()
                    {
                        self.remove_slide();
                    }
                });
            });
        });
        group(ui, "Font", 190.0, |ui| {
            let text = self
                .selected
                .and_then(|id| self.editor.deck.object(self.slide, id))
                .and_then(|o| {
                    if let Content::Text {
                        font_size,
                        bold,
                        color,
                        ..
                    } = o.content
                    {
                        Some((font_size, bold, color))
                    } else {
                        None
                    }
                });
            ui.add_enabled_ui(text.is_some(), |ui| {
                let (mut size, mut bold, mut color) =
                    text.unwrap_or((24.0, false, [36, 32, 42, 255]));
                ui.horizontal(|ui| {
                    ui.add_enabled(
                        false,
                        Button::new("Bundled Sans").min_size(vec2(116.0, 23.0)),
                    )
                    .on_disabled_hover_text(
                        "Fixed bundled fallback; system font selection is not implemented.",
                    );
                    if ui
                        .add(DragValue::new(&mut size).range(4.0..=400.0).speed(1.0))
                        .changed()
                    {
                        self.text_style(Some(size), None, None);
                    }
                });
                ui.horizontal(|ui| {
                    if ui
                        .toggle_value(&mut bold, RichText::new("B").strong())
                        .on_hover_text("Synthetic bold; shared renderer in edit and show")
                        .changed()
                    {
                        self.text_style(None, Some(bold), None);
                    }
                    disabled(ui, "I", "Italic font face is not implemented.");
                    disabled(ui, "U", "Underline is not implemented.");
                    if ui
                        .color_edit_button_srgba_unmultiplied(&mut color)
                        .changed()
                    {
                        self.text_style(None, None, Some(color));
                    }
                });
            });
        });
        group(ui, "Paragraph", 145.0, |ui| {
            ui.horizontal(|ui| {
                icon_button(ui, "Bullets — not implemented", Icon::List, false)
                    .on_disabled_hover_text("Lists are not implemented.");
                icon_button(
                    ui,
                    "Paragraph alignment — not implemented",
                    Icon::Align,
                    false,
                )
                .on_disabled_hover_text("Only left-aligned text is currently supported.");
            });
            command(ui, "Text Direction", Icon::Text, false, false)
                .on_disabled_hover_text("Only horizontal text is implemented.");
            command(ui, "Align Text", Icon::Align, false, false)
                .on_disabled_hover_text("Only top-left text alignment is implemented.");
        });
        group(ui, "Drawing", 266.0, |ui| {
            ui.horizontal_top(|ui| {
                Frame::new()
                    .stroke(Stroke::new(1.0_f32, chrome::LINE))
                    .inner_margin(3)
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            if command(ui, "Text Box", Icon::Text, false, true).clicked() {
                                self.add_text();
                            }
                            if command(ui, "Rectangle", Icon::Shape, false, true).clicked() {
                                self.add_rectangle();
                            }
                            if command(ui, "Picture…", Icon::Picture, false, true).clicked() {
                                self.add_image();
                            }
                        });
                    });
                ui.vertical(|ui| {
                    if command(
                        ui,
                        "Format Selection",
                        Icon::Format,
                        false,
                        self.selected.is_some(),
                    )
                    .clicked()
                    {
                        self.finish_text();
                        self.format = true;
                    }
                    if command(
                        ui,
                        "Bring to Front",
                        Icon::Front,
                        false,
                        self.selected.is_some(),
                    )
                    .clicked()
                        && let Some(id) = self.selected
                    {
                        let r = self.editor.stack(self.slide, id, true);
                        self.result(r);
                    }
                    if command(
                        ui,
                        "Send to Back",
                        Icon::Back,
                        false,
                        self.selected.is_some(),
                    )
                    .clicked()
                        && let Some(id) = self.selected
                    {
                        let r = self.editor.stack(self.slide, id, false);
                        self.result(r);
                    }
                });
            });
        });
        group(ui, "Editing", 100.0, |ui| {
            command(ui, "Find", Icon::Find, false, false)
                .on_disabled_hover_text("Text search is not implemented.");
            command(ui, "Replace", Icon::Text, false, false)
                .on_disabled_hover_text("Text replacement is not implemented.");
            if command(
                ui,
                "Delete Object",
                Icon::Delete,
                false,
                self.selected.is_some(),
            )
            .clicked()
            {
                self.remove_object();
            }
        });
    }
    fn text_style(&mut self, size: Option<f64>, weight: Option<bool>, new_color: Option<[u8; 4]>) {
        self.finish_text();
        if let Some(id) = self.selected {
            let index = self.slide;
            let r = self.editor.apply(|d| {
                if let Content::Text {
                    font_size,
                    bold,
                    color,
                    ..
                } = &mut d.object_mut(index, id)?.content
                {
                    if let Some(v) = size {
                        *font_size = v;
                    }
                    if let Some(v) = weight {
                        *bold = v;
                    }
                    if let Some(v) = new_color {
                        *color = v;
                    }
                }
                Ok(())
            });
            self.result(r);
        }
    }
    fn arrange(&mut self, ui: &mut Ui) {
        group(ui, "Arrange", 200.0, |ui| {
            for (label, front) in [("Bring to Front", true), ("Send to Back", false)] {
                if ui
                    .add_enabled(self.selected.is_some(), Button::new(label))
                    .clicked()
                    && let Some(id) = self.selected
                {
                    let r = self.editor.stack(self.slide, id, front);
                    self.result(r);
                }
            }
        });
    }
    fn status(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("status")
            .exact_height(29.0)
            .frame(
                Frame::new()
                    .fill(Color32::from_rgb(250, 250, 251))
                    .inner_margin(Margin::symmetric(12, 3)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Slide {} of {}",
                        self.slide + 1,
                        self.editor.deck.slides.len()
                    ));
                    ui.separator();
                    if command(ui, "Notes", Icon::Notes, false, true).clicked() {
                        self.notes = !self.notes;
                    }
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if icon_button(ui, "Fit slide to window", Icon::Fit, true).clicked() {
                            self.fit = true;
                        }
                        if ui
                            .add(
                                Slider::new(&mut self.zoom, 10.0..=200.0)
                                    .suffix("%")
                                    .show_value(true),
                            )
                            .changed()
                        {
                            self.fit = false;
                        }
                        if icon_button(ui, "Slide show from current slide", Icon::Show, true)
                            .clicked()
                        {
                            self.start_show(false, ctx);
                        }
                        icon_button(ui, "Slide Sorter", Icon::Grid, false)
                            .on_disabled_hover_text("Slide Sorter is not implemented.");
                        icon_button(ui, "Normal view", Icon::Normal, true);
                        ui.separator();
                    });
                });
            });
    }
    fn thumbnails(&mut self, ctx: &Context) {
        SidePanel::left("thumbnails")
            .resizable(true)
            .default_width(202.0)
            .width_range(140.0..=320.0)
            .frame(
                Frame::new()
                    .fill(Color32::from_rgb(246, 246, 247))
                    .inner_margin(10),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Slides").strong());
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.small_button("+").on_hover_text("New slide").clicked() {
                            self.add_slide();
                        }
                    });
                });
                ui.separator();
                ScrollArea::vertical()
                    .id_salt("slides-scroll")
                    .show(ui, |ui| {
                        for index in 0..self.editor.deck.slides.len() {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(format!("{}", index + 1)).size(11.0).color(
                                    if index == self.slide {
                                        ACCENT
                                    } else {
                                        Color32::GRAY
                                    },
                                ));
                                let w = (ui.available_width() - 7.0).max(80.0);
                                let (r, response) = ui.allocate_exact_size(
                                    vec2(
                                        w,
                                        w * (self.editor.deck.height / self.editor.deck.width)
                                            as f32,
                                    ),
                                    Sense::click(),
                                );
                                painting::paint(
                                    ui,
                                    r,
                                    &self.editor.deck,
                                    index,
                                    &mut self.textures,
                                );
                                ui.painter().rect_stroke(
                                    r.expand(2.0),
                                    0.0,
                                    Stroke::new(
                                        if index == self.slide {
                                            2.0_f32
                                        } else {
                                            1.0_f32
                                        },
                                        if index == self.slide {
                                            ACCENT
                                        } else {
                                            Color32::from_gray(200)
                                        },
                                    ),
                                    StrokeKind::Outside,
                                );
                                if response.clicked() {
                                    self.choose_slide(index);
                                }
                                response.context_menu(|ui| {
                                    if ui.button("Duplicate slide").clicked() {
                                        self.choose_slide(index);
                                        self.duplicate_slide();
                                        ui.close();
                                    }
                                    if ui
                                        .add_enabled(
                                            self.editor.deck.slides.len() > 1,
                                            Button::new("Delete slide"),
                                        )
                                        .clicked()
                                    {
                                        self.choose_slide(index);
                                        self.remove_slide();
                                        ui.close();
                                    }
                                });
                            });
                            ui.add_space(13.0);
                        }
                    });
                ui.separator();
                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(self.slide > 0, Button::new("Move Up"))
                        .clicked()
                    {
                        self.move_slide(self.slide - 1);
                    }
                    if ui
                        .add_enabled(
                            self.slide + 1 < self.editor.deck.slides.len(),
                            Button::new("Move Down"),
                        )
                        .clicked()
                    {
                        self.move_slide(self.slide + 1);
                    }
                });
            });
    }
    fn formatting(&mut self, ctx: &Context) {
        if !self.format {
            return;
        }
        SidePanel::right("format")
            .resizable(true)
            .default_width(268.0)
            .width_range(230.0..=400.0)
            .frame(
                Frame::new()
                    .fill(Color32::from_rgb(251, 251, 252))
                    .inner_margin(16),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let title = self
                        .selected
                        .and_then(|id| self.editor.deck.object(self.slide, id))
                        .map(|o| {
                            if matches!(o.content, Content::Image { .. }) {
                                "Format Picture"
                            } else {
                                "Format Shape"
                            }
                        })
                        .unwrap_or("Format Selection");
                    ui.heading(RichText::new(title).size(19.0));
                    if ui.small_button("×").clicked() {
                        self.format = false;
                    }
                });
                ui.separator();
                ScrollArea::vertical()
                    .id_salt("format-properties")
                    .show(ui, |ui| {
                        if let Some(o) = self
                            .selected
                            .and_then(|id| self.editor.deck.object(self.slide, id))
                            .cloned()
                        {
                            ui.add_space(10.0);
                            ui.label(RichText::new("Size & Properties").strong());
                            ui.add_space(8.0);
                            let mut b = o.bounds;
                            let mut changed = false;
                            Grid::new("geometry")
                                .num_columns(2)
                                .spacing([12.0, 12.0])
                                .show(ui, |ui| {
                                    for (label, value) in [
                                        ("Horizontal", &mut b.x),
                                        ("Vertical", &mut b.y),
                                        ("Width", &mut b.w),
                                        ("Height", &mut b.h),
                                    ] {
                                        ui.label(label);
                                        changed |= ui
                                            .add(DragValue::new(value).speed(1.0).suffix(" u"))
                                            .changed();
                                        ui.end_row();
                                    }
                                });
                            if changed {
                                b.w = b.w.max(1.0);
                                b.h = b.h.max(1.0);
                                let r = self.editor.transform(self.slide, o.id, b);
                                self.result(r);
                            }
                            ui.add_space(18.0);
                            ui.separator();
                            ui.label(RichText::new("Fill & Text").strong());
                            match o.content {
                                Content::Rectangle { mut fill } => {
                                    if ui.color_edit_button_srgba_unmultiplied(&mut fill).changed()
                                    {
                                        let index = self.slide;
                                        let r = self.editor.apply(|d| {
                                            d.object_mut(index, o.id)?.content =
                                                Content::Rectangle { fill };
                                            Ok(())
                                        });
                                        self.result(r);
                                    }
                                }
                                Content::Text {
                                    mut font_size,
                                    mut bold,
                                    mut color,
                                    ..
                                } => {
                                    if ui
                                        .add(
                                            DragValue::new(&mut font_size)
                                                .range(4.0..=400.0)
                                                .suffix(" u"),
                                        )
                                        .changed()
                                    {
                                        self.text_style(Some(font_size), None, None);
                                    }
                                    if ui.checkbox(&mut bold, "Synthetic bold").changed() {
                                        self.text_style(None, Some(bold), None);
                                    }
                                    if ui
                                        .color_edit_button_srgba_unmultiplied(&mut color)
                                        .changed()
                                    {
                                        self.text_style(None, None, Some(color));
                                    }
                                }
                                Content::Image { .. } => {
                                    ui.label("Embedded picture");
                                    disabled(
                                        ui,
                                        "Crop / Effects",
                                        "Image cropping and effects are not implemented.",
                                    );
                                }
                            }
                            ui.add_space(18.0);
                            ui.separator();
                            ui.label(RichText::new("Arrange").strong());
                            for (label, icon, front) in [
                                ("Bring to Front", Icon::Front, true),
                                ("Send to Back", Icon::Back, false),
                            ] {
                                if command(ui, label, icon, false, true).clicked() {
                                    let r = self.editor.stack(self.slide, o.id, front);
                                    self.result(r);
                                }
                            }
                            ui.add_space(12.0);
                            ui.small("Position and size use slide units.");
                        } else {
                            ui.label(
                        "Select an object to change its size, position or supported formatting.",
                    );
                        }
                    });
            });
    }
    fn notes_panel(&mut self, ctx: &Context) {
        if self.notes {
            TopBottomPanel::bottom("notes")
                .resizable(true)
                .default_height(105.0)
                .height_range(60.0..=260.0)
                .frame(
                    Frame::new()
                        .fill(Color32::from_rgb(253, 253, 253))
                        .inner_margin(Margin::symmetric(20, 8)),
                )
                .show(ctx, |ui| {
                    let r = ui.add_sized(
                        ui.available_size(),
                        TextEdit::multiline(&mut self.notes_draft)
                            .hint_text("Click to add notes")
                            .frame(false),
                    );
                    if r.lost_focus() {
                        self.finish_notes();
                    }
                });
        }
    }
    fn canvas(&mut self, ctx: &Context) {
        CentralPanel::default()
            .frame(Frame::new().fill(Color32::from_rgb(231, 232, 234)))
            .show(ctx, |ui| {
                if self.tab == "File" {
                    self.backstage(ui, ctx);
                    return;
                }
                let available = ui.available_size();
                let (w, h) = (self.editor.deck.width, self.editor.deck.height);
                let fit = render::Viewport::fit(
                    w,
                    h,
                    (available.x - 72.0).max(50.0) as f64,
                    (available.y - 72.0).max(50.0) as f64,
                )
                .scale;
                let scale = if self.fit { fit } else { self.zoom / 100.0 };
                if self.fit {
                    self.zoom = (fit * 100.0).round();
                }
                ScrollArea::both().id_salt("workspace").show(ui, |ui| {
                    let size = vec2(
                        available.x.max((w * scale) as f32 + 72.0),
                        available.y.max((h * scale) as f32 + 72.0),
                    );
                    let (area, response) = ui.allocate_exact_size(size, Sense::click_and_drag());
                    let r = Rect::from_center_size(
                        area.center(),
                        vec2((w * scale) as f32, (h * scale) as f32),
                    );
                    let mut displayed = self.editor.deck.clone();
                    if let (Some(drag), Some(b)) = (&self.drag, self.preview)
                        && let Ok(o) = displayed.object_mut(self.slide, drag.id)
                    {
                        o.bounds = b;
                    }
                    // The live text widget replaces only this object's text while editing.
                    if let Some(edit) = &self.editing
                        && let Ok(o) = displayed.object_mut(self.slide, edit.id)
                        && let Content::Text { text, .. } = &mut o.content
                    {
                        text.clear();
                    }
                    ui.painter().rect_filled(
                        r.translate(vec2(3.0, 4.0)),
                        0.0,
                        Color32::from_black_alpha(24),
                    );
                    let view = painting::paint(ui, r, &displayed, self.slide, &mut self.textures);
                    if let Some(id) = self.selected
                        && let Some(o) = displayed.object(self.slide, id)
                    {
                        let b = painting::projected(view, o.bounds);
                        ui.painter().rect_stroke(
                            b,
                            0.0,
                            Stroke::new(1.4_f32, ACCENT),
                            StrokeKind::Outside,
                        );
                        for pos in painting::handles(b) {
                            ui.painter().rect(
                                Rect::from_center_size(pos, vec2(7.0, 7.0)),
                                0.0,
                                Color32::WHITE,
                                Stroke::new(1.2_f32, ACCENT),
                                StrokeKind::Middle,
                            );
                        }
                    }
                    if self.editing.is_none() {
                        if response.clicked()
                            && let Some(pos) = response.interact_pointer_pos()
                        {
                            self.finish_text();
                            let (x, y) = view.unproject(pos.x as f64, pos.y as f64);
                            self.selected = render::hit_test(&displayed, self.slide, x, y);
                        }
                        if response.double_clicked()
                            && let Some(id) = self.selected
                        {
                            self.begin_text(id);
                        }
                        if response.drag_started()
                            && let Some(start) = ctx.input(|i| i.pointer.press_origin())
                        {
                            let handle = self
                                .selected
                                .and_then(|id| displayed.object(self.slide, id))
                                .and_then(|o| {
                                    painting::handles(painting::projected(view, o.bounds))
                                        .iter()
                                        .position(|p| p.distance(start) < 9.0)
                                });
                            let (x, y) = view.unproject(start.x as f64, start.y as f64);
                            let id = if handle.is_some() {
                                self.selected
                            } else {
                                render::hit_test(&displayed, self.slide, x, y)
                            };
                            self.selected = id;
                            if let Some(id) = id
                                && let Some(o) = displayed.object(self.slide, id)
                            {
                                self.drag = Some(Drag {
                                    id,
                                    original: o.bounds,
                                    start,
                                    handle,
                                });
                            }
                        }
                        if response.dragged()
                            && let Some(pos) = response.interact_pointer_pos()
                            && let Some(d) = &self.drag
                        {
                            self.preview = Some(drag_bounds(
                                d,
                                (pos.x - d.start.x) as f64 / view.scale,
                                (pos.y - d.start.y) as f64 / view.scale,
                            ));
                            ctx.request_repaint();
                        }
                        if response.drag_stopped()
                            && let (Some(d), Some(b)) = (self.drag.take(), self.preview.take())
                        {
                            let result = self.editor.transform(self.slide, d.id, b);
                            self.result(result);
                        }
                    }
                    if response.secondary_clicked()
                        && let Some(pos) = response.interact_pointer_pos()
                    {
                        self.finish_text();
                        let (x, y) = view.unproject(pos.x as f64, pos.y as f64);
                        self.selected = render::hit_test(&displayed, self.slide, x, y);
                    }
                    response.context_menu(|ui| {
                        if ui
                            .add_enabled(self.selected.is_some(), Button::new("Format Selection"))
                            .clicked()
                        {
                            self.format = true;
                            ui.close();
                        }
                        if ui
                            .add_enabled(self.selected.is_some(), Button::new("Delete Object"))
                            .clicked()
                        {
                            self.remove_object();
                            ui.close();
                        }
                    });
                    let mut finish = false;
                    let mut cancel = false;
                    if let Some(edit) = &mut self.editing
                        && let Some(o) = displayed.object(self.slide, edit.id)
                    {
                        let b = painting::projected(view, o.bounds).shrink(1.0);
                        let size = if let Content::Text { font_size, .. } = o.content {
                            font_size
                        } else {
                            24.0
                        };
                        ui.scope_builder(UiBuilder::new().max_rect(b), |ui| {
                            ui.set_clip_rect(ui.clip_rect().intersect(b));
                            let text_color = if let Content::Text { color, .. } = o.content {
                                painting::color(color)
                            } else {
                                Color32::BLACK
                            };
                            let response = ui.add_sized(
                                b.size(),
                                TextEdit::multiline(&mut edit.text)
                                    .desired_rows(1)
                                    .text_color(text_color)
                                    .font(FontId::proportional((size * view.scale) as f32))
                                    .margin(8.0 * view.scale as f32)
                                    .background_color(Color32::TRANSPARENT),
                            );
                            if edit.focus {
                                response.request_focus();
                                edit.focus = false;
                            } else if response.lost_focus() {
                                finish = true;
                            }
                            if ctx.input(|i| i.key_pressed(Key::Escape)) {
                                cancel = true;
                            }
                            if ctx.input(|i| i.modifiers.command && i.key_pressed(Key::Enter)) {
                                finish = true;
                            }
                        });
                    }
                    if cancel {
                        self.editing = None;
                    } else if finish {
                        self.finish_text();
                    }
                });
            });
    }
    fn backstage(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.add_space(28.0);
        ui.horizontal(|ui| {
            ui.add_space(34.0);
            ui.vertical(|ui| {
                ui.heading("Local presentation");
                ui.label("Your ideas. Your freedom.");
                ui.add_space(25.0);
                if ui.button("← Back to editor").clicked() {
                    self.tab = "Home".into();
                }
                ui.add_space(16.0);
                if ui.button("New presentation").clicked() {
                    self.request(Pending::New, ctx);
                }
                if ui.button("Open…").clicked() {
                    self.open(ctx);
                }
                if ui.button("Save").clicked() {
                    self.save(false);
                }
                if ui.button("Save As…").clicked() {
                    self.save(true);
                }
                ui.add_space(16.0);
                disabled(
                    ui,
                    "Export / Print",
                    "PDF, image export and printing are not implemented.",
                );
                ui.label("Native .bfslide format · local files only");
                ui.label("PPTX import/export is not available.");
            });
        });
    }
    fn dialogs(&mut self, ctx: &Context) {
        if self.pending.is_some() {
            let mut answer = None;
            Modal::new(eframe::egui::Id::new("save-changes")).show(ctx, |ui| {
                ui.heading("Save changes?");
                ui.label("Save changes to this presentation before continuing?");
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        answer = Some(0);
                    }
                    if ui.button("Discard").clicked() {
                        answer = Some(1);
                    }
                    if ui.button("Cancel").clicked() {
                        answer = Some(2);
                    }
                });
            });
            match answer {
                Some(0) => {
                    if self.save(false)
                        && let Some(p) = self.pending.take()
                    {
                        self.perform(p, ctx);
                    }
                }
                Some(1) => {
                    if let Some(p) = self.pending.take() {
                        self.perform(p, ctx);
                    }
                }
                Some(2) => self.pending = None,
                _ => {}
            }
        }
        if let Some(error) = self.error.clone() {
            Window::new("BreakFree Slides — action failed")
                .collapsible(false)
                .resizable(true)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label(error);
                    ui.label("The current document has been kept.");
                    if ui.button("OK").clicked() {
                        self.error = None;
                    }
                });
        }
    }
    fn capture_tick(&mut self, ctx: &Context) {
        let events = ctx.input(|i| i.events.clone());
        for e in events {
            if let Event::Screenshot { image, .. } = e
                && let Some(c) = &mut self.capture
                && c.pending
            {
                let bytes: Vec<u8> = image.pixels.iter().flat_map(|p| p.to_array()).collect();
                let path = c.dir.join(format!("{}.png", SHOTS[c.stage]));
                let r = std::fs::create_dir_all(&c.dir)
                    .map_err(|e| e.to_string())
                    .and_then(|_| {
                        image::save_buffer(
                            &path,
                            &bytes,
                            image.width() as u32,
                            image.height() as u32,
                            image::ColorType::Rgba8,
                        )
                        .map_err(|e| e.to_string())
                    });
                match r {
                    Ok(()) => eprintln!("Captured {}", path.display()),
                    Err(e) => self.error = Some(e),
                }
                c.stage += 1;
                c.frames = 0;
                c.pending = false;
            }
        }
        let Some(c) = &self.capture else {
            return;
        };
        if c.stage >= SHOTS.len() {
            self.allow_close = true;
            ctx.send_viewport_cmd(ViewportCommand::Close);
            return;
        }
        let stage = c.stage;
        let initialize = c.frames == 0;
        if initialize {
            self.editing = None;
            self.drag = None;
            self.preview = None;
            self.show = false;
            self.selected = None;
            self.format = false;
            match stage {
                0 => self.replace(Deck::new(), None),
                1 => self.replace(demo_deck(), None),
                2 | 4 => {
                    self.selected = self.editor.deck.slides[0]
                        .objects
                        .iter()
                        .find(|o| matches!(o.content, Content::Rectangle { .. }))
                        .map(|o| o.id);
                    self.format = stage == 4;
                }
                3 => {
                    if let Some(id) = self.editor.deck.slides[0]
                        .objects
                        .iter()
                        .find(|o| matches!(o.content, Content::Text { .. }))
                        .map(|o| o.id)
                    {
                        self.selected = Some(id);
                        self.begin_text(id);
                    }
                }
                5 => {
                    self.show = true;
                }
                6 => {
                    ctx.send_viewport_cmd(ViewportCommand::InnerSize(vec2(980.0, 680.0)));
                    self.tab = "Home".into();
                }
                _ => {}
            }
        }
        if let Some(c) = &mut self.capture {
            c.frames += 1;
            if c.frames == 8 && !c.pending {
                ctx.send_viewport_cmd(ViewportCommand::Screenshot(Default::default()));
                c.pending = true;
            }
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(45));
    }
}
impl eframe::App for SlidesApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.capture_tick(ctx);
        self.slide = self.slide.min(self.editor.deck.slides.len() - 1);
        if self
            .selected
            .is_some_and(|id| self.editor.deck.object(self.slide, id).is_none())
        {
            self.selected = None;
            self.editing = None;
        }
        if self.tab == "Format" && self.selected.is_none() {
            self.tab = "Home".into();
        }
        ctx.send_viewport_cmd(ViewportCommand::Title(format!(
            "{}{} — BreakFree Slides",
            self.path
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or(self.editor.deck.title.clone()),
            if self.editor.dirty() { " *" } else { "" }
        )));
        self.keyboard(ctx);
        if ctx.input(|i| i.viewport().close_requested()) && !self.allow_close {
            ctx.send_viewport_cmd(ViewportCommand::CancelClose);
            self.request(Pending::Quit, ctx);
        }
        if self.show {
            CentralPanel::default()
                .frame(Frame::new().fill(Color32::BLACK))
                .show(ctx, |ui| {
                    let r = painting::fit(
                        ui.available_rect_before_wrap(),
                        self.editor.deck.width,
                        self.editor.deck.height,
                    );
                    painting::paint(ui, r, &self.editor.deck, self.slide, &mut self.textures);
                });
            return;
        }
        self.title_bar(ctx);
        self.ribbon(ctx);
        self.status(ctx);
        self.thumbnails(ctx);
        self.formatting(ctx);
        self.notes_panel(ctx);
        self.canvas(ctx);
        self.dialogs(ctx);
    }
}
fn group(ui: &mut Ui, label: &str, width: f32, body: impl FnOnce(&mut Ui)) {
    ui.allocate_ui_with_layout(vec2(width, 94.0), Layout::top_down(Align::Min), |ui| {
        ui.set_min_width(width);
        body(ui);
        ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
            ui.label(
                RichText::new(label)
                    .size(11.0)
                    .color(Color32::from_gray(100)),
            );
        });
    });
    ui.separator();
}
fn disabled(ui: &mut Ui, label: &str, reason: &str) {
    ui.add_enabled(false, Button::new(label))
        .on_disabled_hover_text(reason);
}
fn drag_bounds(d: &Drag, dx: f64, dy: f64) -> Bounds {
    let mut b = d.original;
    if let Some(h) = d.handle {
        let left = matches!(h, 0 | 6 | 7);
        let right = matches!(h, 2..=4);
        let top = matches!(h, 0..=2);
        let bottom = matches!(h, 4..=6);
        if left {
            let shift = dx.min(b.w - 16.0);
            b.x += shift;
            b.w -= shift;
        }
        if right {
            b.w = (b.w + dx).max(16.0);
        }
        if top {
            let shift = dy.min(b.h - 16.0);
            b.y += shift;
            b.h -= shift;
        }
        if bottom {
            b.h = (b.h + dy).max(16.0);
        }
    } else {
        b.x += dx;
        b.y += dy;
    }
    b
}
fn demo_deck() -> Deck {
    let mut editor = Editor::default();
    editor.deck.title = "Ideas in motion".into();
    editor
        .add_object(
            0,
            Content::Text {
                text: "Ideas in motion".into(),
                font_size: 52.0,
                bold: false,
                color: [45, 29, 61, 255],
            },
            Bounds {
                x: 90.0,
                y: 70.0,
                w: 1000.0,
                h: 100.0,
            },
        )
        .unwrap();
    editor
        .add_object(
            0,
            Content::Text {
                text:
                    "Give your ideas a clear shape.\nBuild it. Save it. Share your point of view."
                        .into(),
                font_size: 28.0,
                bold: false,
                color: [82, 78, 89, 255],
            },
            Bounds {
                x: 94.0,
                y: 190.0,
                w: 660.0,
                h: 130.0,
            },
        )
        .unwrap();
    editor
        .add_object(
            0,
            Content::Rectangle {
                fill: [215, 115, 103, 255],
            },
            Bounds {
                x: 100.0,
                y: 395.0,
                w: 360.0,
                h: 180.0,
            },
        )
        .unwrap();
    editor
        .add_object(
            0,
            Content::Text {
                text: "Your ideas.\nYour freedom.".into(),
                font_size: 30.0,
                bold: false,
                color: [255, 255, 255, 255],
            },
            Bounds {
                x: 124.0,
                y: 422.0,
                w: 310.0,
                h: 120.0,
            },
        )
        .unwrap();
    let mut image = image::RgbaImage::new(480, 320);
    for (x, y, p) in image.enumerate_pixels_mut() {
        let circle = (x as i32 - 300).pow(2) + (y as i32 - 115).pow(2) < 75 * 75;
        *p = if circle {
            image::Rgba([244, 203, 168, 255])
        } else if y > 220 - (x / 5) {
            image::Rgba([90, 104, 146, 255])
        } else {
            image::Rgba([217, 226, 234, 255])
        };
    }
    let mut png = Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(image)
        .write_to(&mut png, image::ImageFormat::Png)
        .unwrap();
    let id = editor.add_image(0, png.get_ref()).unwrap();
    editor
        .transform(
            0,
            id,
            Bounds {
                x: 820.0,
                y: 220.0,
                w: 350.0,
                h: 233.0,
            },
        )
        .unwrap();
    for (title, body) in [
        (
            "A clear structure",
            "Outline your message.\nArrange the slide objects.\nKeep the deck yours.",
        ),
        (
            "Ready to present",
            "Text, shapes and images stay editable.\nOne shared renderer. One composition.",
        ),
    ] {
        let index = editor.add_slide(editor.deck.slides.len() - 1).unwrap();
        editor
            .add_object(
                index,
                Content::Text {
                    text: title.into(),
                    font_size: 48.0,
                    bold: false,
                    color: [45, 29, 61, 255],
                },
                Bounds {
                    x: 90.0,
                    y: 85.0,
                    w: 1050.0,
                    h: 100.0,
                },
            )
            .unwrap();
        editor
            .add_object(
                index,
                Content::Text {
                    text: body.into(),
                    font_size: 32.0,
                    bold: false,
                    color: [82, 78, 89, 255],
                },
                Bounds {
                    x: 100.0,
                    y: 250.0,
                    w: 1050.0,
                    h: 260.0,
                },
            )
            .unwrap();
    }
    editor.deck
}

#[cfg(test)]
mod tests {
    use super::*;
    fn noted_app() -> SlidesApp {
        let mut e = Editor::default();
        e.add_slide(0).unwrap();
        e.add_slide(1).unwrap();
        for (s, note) in e.deck.slides.iter_mut().zip(["A", "B", "C"]) {
            s.notes = note.into();
        }
        SlidesApp::with_context(&Context::default(), e.deck, None, None)
    }
    #[test]
    fn resizing_workspace_and_opening_panes_preserves_the_document() {
        let ctx = Context::default();
        let deck = demo_deck();
        let mut app = SlidesApp::with_context(&ctx, deck.clone(), None, None);
        app.selected = app.editor.deck.slides[0].objects.first().map(|o| o.id);
        for (w, h, pane, notes) in [
            (1440.0, 815.0, false, true),
            (980.0, 680.0, true, true),
            (800.0, 560.0, true, false),
        ] {
            app.format = pane;
            app.notes = notes;
            let input = RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(w, h))),
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                app.title_bar(ctx);
                app.ribbon(ctx);
                app.status(ctx);
                app.thumbnails(ctx);
                app.formatting(ctx);
                app.notes_panel(ctx);
                app.canvas(ctx);
            });
            assert_eq!(app.editor.deck, deck);
            assert!(!app.editor.dirty());
            assert!(app.zoom.is_finite() && app.zoom > 0.0);
        }
    }
    #[test]
    fn moving_active_slide_commits_its_notes_without_overwriting_other_slides() {
        let mut app = noted_app();
        app.notes_draft = "A edited".into();
        app.move_slide(2);
        assert_eq!(
            app.editor
                .deck
                .slides
                .iter()
                .map(|s| s.notes.as_str())
                .collect::<Vec<_>>(),
            ["B", "C", "A edited"]
        );
        assert_eq!(app.notes_draft, "A edited");
        assert!(app.error.is_none());
        app.history(false);
        assert_eq!(
            app.editor
                .deck
                .slides
                .iter()
                .map(|s| s.notes.as_str())
                .collect::<Vec<_>>(),
            ["A edited", "B", "C"]
        );
    }
    #[test]
    fn deleting_active_slide_does_not_copy_its_notes_to_the_successor() {
        let mut app = noted_app();
        app.choose_slide(1);
        app.notes_draft = "B edited".into();
        app.remove_slide();
        assert_eq!(
            app.editor
                .deck
                .slides
                .iter()
                .map(|s| s.notes.as_str())
                .collect::<Vec<_>>(),
            ["A", "C"]
        );
        assert_eq!(app.notes_draft, "C");
        assert!(app.error.is_none());
        app.remove_slide();
        assert_eq!(app.notes_draft, "A");
        assert!(app.error.is_none());
    }
    #[test]
    fn dirty_new_request_waits_without_replacing_the_model() {
        let ctx = Context::default();
        let mut app = noted_app();
        app.notes_draft = "Unsaved notes".into();
        app.request(Pending::New, &ctx);
        assert!(app.pending.is_some());
        assert_eq!(app.editor.deck.slides.len(), 3);
        assert_eq!(app.editor.deck.slides[0].notes, "Unsaved notes");
    }
    #[test]
    fn resize_handles_preserve_the_opposite_edge_and_enforce_minimum_size() {
        for handle in 0..8 {
            let d = Drag {
                id: 1,
                original: Bounds {
                    x: 30.0,
                    y: 40.0,
                    w: 200.0,
                    h: 100.0,
                },
                start: Pos2::ZERO,
                handle: Some(handle),
            };
            let b = drag_bounds(&d, 500.0, 500.0);
            assert!(b.valid());
            assert!(b.w >= 16.0 && b.h >= 16.0);
            if matches!(handle, 0 | 6 | 7) {
                assert!((b.x + b.w - 230.0).abs() < 1e-9);
            }
            if matches!(handle, 0..=2) {
                assert!((b.y + b.h - 140.0).abs() < 1e-9);
            }
        }
    }
}
