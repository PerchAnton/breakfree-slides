use crate::model::*;

/// Renderer-neutral display list, consumed identically for canvas, thumbnail and show.
#[derive(Clone, Debug, PartialEq)]
pub struct Primitive {
    pub bounds: Bounds,
    pub content: Content,
}
pub fn scene(deck: &Deck, slide: usize) -> Vec<Primitive> {
    deck.slides
        .get(slide)
        .map(|s| {
            s.objects
                .iter()
                .map(|o| Primitive {
                    bounds: o.bounds,
                    content: o.content.clone(),
                })
                .collect()
        })
        .unwrap_or_default()
}

#[derive(Clone, Copy, Debug)]
pub struct Viewport {
    pub x: f64,
    pub y: f64,
    pub scale: f64,
}
impl Viewport {
    pub fn fit(width: f64, height: f64, available_width: f64, available_height: f64) -> Self {
        let scale = (available_width / width)
            .min(available_height / height)
            .max(0.001);
        Self {
            x: (available_width - width * scale) / 2.0,
            y: (available_height - height * scale) / 2.0,
            scale,
        }
    }
    pub fn project(self, b: Bounds) -> Bounds {
        Bounds {
            x: self.x + b.x * self.scale,
            y: self.y + b.y * self.scale,
            w: b.w * self.scale,
            h: b.h * self.scale,
        }
    }
    pub fn unproject(self, x: f64, y: f64) -> (f64, f64) {
        ((x - self.x) / self.scale, (y - self.y) / self.scale)
    }
}
pub fn hit_test(deck: &Deck, slide: usize, x: f64, y: f64) -> Option<Id> {
    deck.slides
        .get(slide)?
        .objects
        .iter()
        .rev()
        .find(|o| o.bounds.contains(x, y))
        .map(|o| o.id)
}
