//! Native workspace controls. Original vector glyphs, independent of slide rendering.
use eframe::egui::*;

pub const ACCENT: Color32 = Color32::from_rgb(184, 76, 65);
pub const INK: Color32 = Color32::from_rgb(57, 57, 57);
pub const LINE: Color32 = Color32::from_rgb(216, 216, 216);

#[derive(Clone, Copy)]
pub enum Icon {
    Expand,
    Collapse,
    Save,
    Undo,
    Redo,
    Paste,
    Cut,
    Copy,
    Slide,
    Layout,
    Delete,
    Text,
    Shape,
    Picture,
    Format,
    Find,
    Align,
    List,
    Front,
    Back,
    Show,
    Normal,
    Grid,
    Fit,
    Notes,
}

pub fn glyph(p: &Painter, r: Rect, icon: Icon, color: Color32) {
    let p = p.with_clip_rect(r.expand(1.0));
    let pt = |x: f32, y: f32| {
        pos2(
            r.left() + r.width() * x / 24.0,
            r.top() + r.height() * y / 24.0,
        )
    };
    let stroke = Stroke::new(1.2_f32, color);
    let line = |a: (f32, f32), b: (f32, f32)| {
        p.line_segment([pt(a.0, a.1), pt(b.0, b.1)], stroke);
    };
    let rect = |a: (f32, f32), b: (f32, f32)| {
        p.rect_stroke(
            Rect::from_two_pos(pt(a.0, a.1), pt(b.0, b.1)),
            0.0,
            stroke,
            StrokeKind::Inside,
        );
    };
    match icon {
        Icon::Expand | Icon::Collapse => {
            let y = if matches!(icon, Icon::Expand) {
                15.0
            } else {
                9.0
            };
            p.add(Shape::line(
                vec![pt(5., 24. - y), pt(12., y), pt(19., 24. - y)],
                stroke,
            ));
        }
        Icon::Save => {
            rect((4., 3.), (20., 21.));
            rect((8., 3.), (16., 10.));
            rect((8., 14.), (17., 21.));
        }
        Icon::Undo | Icon::Redo => {
            let flip = matches!(icon, Icon::Redo);
            let q = |x: f32, y: f32| pt(if flip { 24. - x } else { x }, y);
            p.add(Shape::line(
                vec![q(5., 8.), q(14., 8.), q(19., 12.), q(19., 17.), q(16., 20.)],
                stroke,
            ));
            p.add(Shape::line(vec![q(9., 3.), q(4., 8.), q(9., 13.)], stroke));
        }
        Icon::Paste => {
            rect((5., 5.), (19., 22.));
            p.rect_filled(Rect::from_two_pos(pt(8., 2.), pt(16., 7.)), 1., color);
            line((8., 12.), (16., 12.));
            line((8., 16.), (16., 16.));
        }
        Icon::Cut => {
            p.circle_stroke(pt(6., 18.), r.width() / 8., stroke);
            p.circle_stroke(pt(17., 18.), r.width() / 8., stroke);
            line((8., 16.), (18., 3.));
            line((15., 16.), (5., 3.));
        }
        Icon::Copy => {
            rect((4., 3.), (15., 17.));
            rect((9., 7.), (20., 21.));
        }
        Icon::Slide | Icon::Layout => {
            rect((2., 5.), (22., 20.));
            line((5., 9.), (18., 9.));
            if matches!(icon, Icon::Slide) {
                line((19., 1.), (19., 8.));
                line((15.5, 4.5), (22.5, 4.5));
            } else {
                rect((5., 12.), (11., 17.));
                rect((13., 12.), (19., 17.));
            }
        }
        Icon::Delete => {
            rect((6., 6.), (18., 22.));
            line((4., 5.), (20., 5.));
            line((9., 2.), (15., 2.));
            line((10., 9.), (10., 18.));
            line((14., 9.), (14., 18.));
        }
        Icon::Text => {
            rect((2., 4.), (22., 20.));
            line((7., 8.), (17., 8.));
            line((12., 8.), (12., 17.));
            line((9., 17.), (15., 17.));
        }
        Icon::Shape => {
            p.rect_filled(
                Rect::from_two_pos(pt(3., 5.), pt(21., 20.)),
                1.,
                Color32::from_rgb(228, 237, 247),
            );
            rect((3., 5.), (21., 20.));
        }
        Icon::Picture => {
            rect((2., 4.), (22., 21.));
            p.circle_stroke(pt(16., 9.), r.width() / 10., stroke);
            p.add(Shape::line(
                vec![
                    pt(3., 19.),
                    pt(9., 12.),
                    pt(14., 17.),
                    pt(18., 14.),
                    pt(21., 18.),
                ],
                stroke,
            ));
        }
        Icon::Format => {
            line((5., 19.), (19., 5.));
            line((7., 21.), (21., 7.));
            line((5., 19.), (4., 22.));
            line((4., 22.), (7., 21.));
            rect((3., 3.), (11., 10.));
        }
        Icon::Find => {
            p.circle_stroke(pt(10., 10.), r.width() * 0.27, stroke);
            line((15., 15.), (22., 22.));
        }
        Icon::Align | Icon::List | Icon::Notes => {
            for (i, y) in [5., 10., 15., 20.].into_iter().enumerate() {
                let left = if matches!(icon, Icon::List) { 8. } else { 3. };
                line((left, y), (if i % 2 == 0 { 21. } else { 17. }, y));
                if matches!(icon, Icon::List) {
                    p.circle_filled(pt(3., y), 1., color);
                }
            }
        }
        Icon::Front | Icon::Back => {
            rect((2., 3.), (15., 15.));
            let small = Rect::from_two_pos(pt(9., 10.), pt(22., 22.));
            p.rect_filled(small, 0., Color32::WHITE);
            p.rect_stroke(small, 0., stroke, StrokeKind::Inside);
            if matches!(icon, Icon::Front) {
                p.rect_filled(Rect::from_two_pos(pt(3., 4.), pt(14., 14.)), 0., color);
            }
        }
        Icon::Show => {
            rect((2., 3.), (22., 17.));
            line((12., 17.), (12., 22.));
            line((7., 22.), (17., 22.));
            p.add(Shape::convex_polygon(
                vec![pt(10., 7.), pt(10., 13.), pt(15., 10.)],
                color,
                Stroke::NONE,
            ));
        }
        Icon::Normal => {
            rect((2., 3.), (22., 21.));
            line((8., 3.), (8., 21.));
            line((8., 17.), (22., 17.));
        }
        Icon::Grid => {
            for x in [3., 14.] {
                for y in [4., 14.] {
                    rect((x, y), (x + 7., y + 6.));
                }
            }
        }
        Icon::Fit => {
            rect((5., 7.), (19., 17.));
            for (x, y, dx, dy) in [
                (2., 3., 1., 1.),
                (22., 3., -1., 1.),
                (2., 21., 1., -1.),
                (22., 21., -1., -1.),
            ] {
                line((x, y), (x + 4. * dx, y));
                line((x, y), (x, y + 4. * dy));
            }
        }
    }
}

/// Uses egui's button interaction (including focus/keyboard/accessibility), with a vector label.
pub fn command(ui: &mut Ui, label: &str, icon: Icon, large: bool, enabled: bool) -> Response {
    let font = FontId::proportional(12.0);
    let label_width = ui
        .painter()
        .layout_no_wrap(label.into(), font.clone(), INK)
        .size()
        .x;
    let size = if large {
        vec2(label_width.max(42.0) + 14.0, 70.0)
    } else {
        vec2(label_width + 31.0, 22.0)
    };
    let response = ui.add_enabled(enabled, Button::new("").min_size(size).frame(false));
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, label));
    let color = if enabled && ui.is_enabled() {
        INK
    } else {
        Color32::from_gray(161)
    };
    let icon_rect = if large {
        Rect::from_center_size(
            pos2(response.rect.center().x, response.rect.top() + 22.0),
            vec2(29., 29.),
        )
    } else {
        Rect::from_min_size(response.rect.min + vec2(4., 3.), vec2(16., 16.))
    };
    glyph(ui.painter(), icon_rect, icon, color);
    let (pos, align) = if large {
        (
            pos2(response.rect.center().x, response.rect.bottom() - 15.),
            Align2::CENTER_CENTER,
        )
    } else {
        (
            pos2(response.rect.left() + 25., response.rect.center().y),
            Align2::LEFT_CENTER,
        )
    };
    ui.painter().text(pos, align, label, font, color);
    response.on_hover_text(label)
}

pub fn icon_button(ui: &mut Ui, label: &str, icon: Icon, enabled: bool) -> Response {
    let r = ui.add_enabled(
        enabled,
        Button::new("").min_size(vec2(28., 24.)).frame(false),
    );
    r.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, label));
    glyph(
        ui.painter(),
        Rect::from_center_size(r.rect.center(), vec2(17., 17.)),
        icon,
        if enabled {
            INK
        } else {
            Color32::from_gray(165)
        },
    );
    r.on_hover_text(label)
}

pub fn tab(ui: &mut Ui, label: &str, selected: bool, enabled: bool, accent: bool) -> Response {
    let width = ui
        .painter()
        .layout_no_wrap(label.into(), FontId::proportional(13.0), INK)
        .size()
        .x
        + 20.0;
    let color = if accent || selected { ACCENT } else { INK };
    let r = ui.add_enabled(
        enabled,
        Button::new(RichText::new(label).size(13.).color(if enabled {
            color
        } else {
            Color32::from_gray(155)
        }))
        .frame(false)
        .min_size(vec2(width, 30.)),
    );
    if selected {
        ui.painter().line_segment(
            [
                pos2(r.rect.left() + 5., r.rect.bottom()),
                pos2(r.rect.right() - 5., r.rect.bottom()),
            ],
            Stroke::new(2.0_f32, ACCENT),
        );
    }
    r
}

pub const HOME_GROUPS: [(&str, f32, Icon); 6] = [
    ("Clipboard", 157.0, Icon::Paste),
    ("Slides", 166.0, Icon::Slide),
    ("Font", 190.0, Icon::Text),
    ("Paragraph", 145.0, Icon::Align),
    ("Drawing", 266.0, Icon::Shape),
    ("Editing", 100.0, Icon::Find),
];

pub fn compact_home_groups(available_width: f32) -> [bool; 6] {
    let mut compact = [false; 6];
    let mut width: f32 = HOME_GROUPS.iter().map(|(_, w, _)| w + 16.0).sum();
    // Retain slide and font tools longest. A small reserve covers theme spacing.
    for index in [5, 3, 4, 0, 2, 1] {
        if width + 24.0 <= available_width {
            break;
        }
        compact[index] = true;
        width -= HOME_GROUPS[index].1 - 82.0;
    }
    compact
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_command_preserves_keyboard_activation_and_disabled_state() {
        for enabled in [true, false] {
            let ctx = Context::default();
            let mut activated = false;
            let mut draw = |ctx: &Context| {
                CentralPanel::default().show(ctx, |ui| {
                    let r = command(ui, "New Slide", Icon::Slide, true, enabled);
                    r.request_focus();
                    activated |= r.clicked();
                });
            };
            let _ = ctx.run(RawInput::default(), &mut draw);
            let input = RawInput {
                events: vec![Event::Key {
                    key: Key::Enter,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: Modifiers::NONE,
                }],
                ..Default::default()
            };
            let _ = ctx.run(input, &mut draw);
            assert_eq!(
                activated, enabled,
                "Keyboard activation must respect disabled state"
            );
        }
    }
}

// Dimensions are app layout choices, not measurements of an installed Office build.
