use breakfree_slides_core::model::Id;
use breakfree_slides_core::{model::*, render};
use eframe::egui::*;
use std::collections::HashMap;

pub type Textures = HashMap<Id, TextureHandle>;
pub fn color(c: [u8; 4]) -> Color32 {
    Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3])
}
pub fn projected(v: render::Viewport, b: Bounds) -> Rect {
    let b = v.project(b);
    Rect::from_min_size(pos2(b.x as f32, b.y as f32), vec2(b.w as f32, b.h as f32))
}
/// The only slide-content painter: editor, real thumbnails and show all call this.
pub fn paint(
    ui: &Ui,
    rect: Rect,
    deck: &Deck,
    index: usize,
    textures: &mut Textures,
) -> render::Viewport {
    let v = render::Viewport {
        x: rect.min.x as f64,
        y: rect.min.y as f64,
        scale: rect.width() as f64 / deck.width,
    };
    let p = ui.painter().with_clip_rect(ui.clip_rect().intersect(rect));
    p.rect_filled(rect, 0.0, Color32::WHITE);
    for primitive in render::scene(deck, index) {
        let bounds = projected(v, primitive.bounds);
        let clip = p.with_clip_rect(p.clip_rect().intersect(bounds));
        match primitive.content {
            Content::Rectangle { fill } => {
                clip.rect_filled(bounds, 0.0, color(fill));
            }
            Content::Text {
                text,
                font_size,
                color: c,
                bold,
            } => {
                let padding = 8.0 * v.scale as f32;
                let galley = clip.layout(
                    text,
                    FontId::proportional((font_size * v.scale) as f32),
                    color(c),
                    (bounds.width() - padding * 2.0).max(1.0),
                );
                clip.galley(
                    bounds.min + vec2(padding, padding),
                    galley.clone(),
                    color(c),
                );
                if bold {
                    clip.galley(
                        bounds.min + vec2(padding + 0.45 * v.scale as f32, padding),
                        galley,
                        color(c),
                    );
                }
            }
            Content::Image { asset } => {
                if !textures.contains_key(&asset)
                    && let Some(a) = deck.assets.get(&asset)
                    && let Ok(bytes) = a.bytes()
                    && let Ok(image) = image::load_from_memory(&bytes)
                {
                    let rgba = image.into_rgba8();
                    let size = [rgba.width() as usize, rgba.height() as usize];
                    textures.insert(
                        asset,
                        ui.ctx().load_texture(
                            format!("asset-{asset}"),
                            ColorImage::from_rgba_unmultiplied(size, &rgba),
                            TextureOptions::LINEAR,
                        ),
                    );
                }
                if let Some(texture) = textures.get(&asset) {
                    clip.image(
                        texture.id(),
                        bounds,
                        Rect::from_min_max(Pos2::ZERO, pos2(1.0, 1.0)),
                        Color32::WHITE,
                    );
                }
            }
        }
    }
    v
}
pub fn fit(available: Rect, width: f64, height: f64) -> Rect {
    let v = render::Viewport::fit(
        width,
        height,
        available.width() as f64,
        available.height() as f64,
    );
    Rect::from_min_size(
        available.min + vec2(v.x as f32, v.y as f32),
        vec2((width * v.scale) as f32, (height * v.scale) as f32),
    )
}
pub fn handles(r: Rect) -> [Pos2; 8] {
    [
        r.left_top(),
        r.center_top(),
        r.right_top(),
        r.right_center(),
        r.right_bottom(),
        r.center_bottom(),
        r.left_bottom(),
        r.left_center(),
    ]
}
