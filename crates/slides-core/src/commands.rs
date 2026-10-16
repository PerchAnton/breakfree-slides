use crate::{Result, model::*};
use std::sync::Arc;

/// Snapshots share immutable raster resources. A gesture/text edit creates one undo step.
pub struct Editor {
    pub deck: Deck,
    undo: Vec<Deck>,
    redo: Vec<Deck>,
    saved: Deck,
}
impl Default for Editor {
    fn default() -> Self {
        Self::new(Deck::new())
    }
}
impl Editor {
    pub fn new(deck: Deck) -> Self {
        Self {
            saved: deck.clone(),
            deck,
            undo: vec![],
            redo: vec![],
        }
    }
    pub fn dirty(&self) -> bool {
        self.deck != self.saved
    }
    pub fn mark_saved(&mut self) {
        self.saved = self.deck.clone();
    }
    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }
    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }
    pub fn undo(&mut self) {
        if let Some(d) = self.undo.pop() {
            self.redo.push(std::mem::replace(&mut self.deck, d));
        }
    }
    pub fn redo(&mut self) {
        if let Some(d) = self.redo.pop() {
            self.undo.push(std::mem::replace(&mut self.deck, d));
        }
    }
    pub fn apply(&mut self, f: impl FnOnce(&mut Deck) -> Result<()>) -> Result<()> {
        let mut candidate = self.deck.clone();
        f(&mut candidate)?;
        candidate.validate()?;
        if candidate != self.deck {
            if self.undo.len() >= 32 {
                self.undo.remove(0);
            }
            self.undo.push(std::mem::replace(&mut self.deck, candidate));
            self.redo.clear();
        }
        Ok(())
    }
    pub fn add_slide(&mut self, after: usize) -> Result<usize> {
        let index = (after + 1).min(self.deck.slides.len());
        self.apply(|d| {
            let id = d.fresh_id()?;
            d.slides.insert(
                index,
                Slide {
                    id,
                    objects: vec![],
                    notes: String::new(),
                },
            );
            Ok(())
        })?;
        Ok(index)
    }
    pub fn remove_slide(&mut self, index: usize) -> Result<()> {
        self.apply(|d| {
            if d.slides.len() == 1 {
                return Err("Keep at least one slide".into());
            }
            if index >= d.slides.len() {
                return Err("Invalid slide index".into());
            }
            d.slides.remove(index);
            Ok(())
        })
    }
    pub fn duplicate_slide(&mut self, index: usize) -> Result<usize> {
        self.apply(|d| {
            let mut copy = d.slides.get(index).ok_or("Invalid slide index")?.clone();
            copy.id = d.fresh_id()?;
            for object in &mut copy.objects {
                object.id = d.fresh_id()?;
            }
            d.slides.insert(index + 1, copy);
            Ok(())
        })?;
        Ok(index + 1)
    }
    pub fn reorder_slide(&mut self, from: usize, to: usize) -> Result<()> {
        self.apply(|d| {
            if from >= d.slides.len() || to >= d.slides.len() {
                return Err("Invalid slide index".into());
            }
            let s = d.slides.remove(from);
            d.slides.insert(to, s);
            Ok(())
        })
    }
    pub fn add_object(&mut self, slide: usize, content: Content, bounds: Bounds) -> Result<Id> {
        let mut id = 0;
        self.apply(|d| {
            id = d.fresh_id()?;
            d.slides
                .get_mut(slide)
                .ok_or("Invalid slide index")?
                .objects
                .push(Object {
                    id,
                    bounds,
                    content,
                });
            Ok(())
        })?;
        Ok(id)
    }
    pub fn add_image(&mut self, slide: usize, bytes: &[u8]) -> Result<Id> {
        let asset = Arc::new(Asset::from_bytes(bytes)?);
        let mut object_id = 0;
        self.apply(|d| {
            let asset_id = d.fresh_id()?;
            object_id = d.fresh_id()?;
            let w = 400.0;
            let h = w * f64::from(asset.height) / f64::from(asset.width);
            d.assets.insert(asset_id, asset);
            d.slides
                .get_mut(slide)
                .ok_or("Invalid slide index")?
                .objects
                .push(Object {
                    id: object_id,
                    bounds: Bounds {
                        x: 320.0,
                        y: 200.0,
                        w,
                        h,
                    },
                    content: Content::Image { asset: asset_id },
                });
            Ok(())
        })?;
        Ok(object_id)
    }
    pub fn transform(&mut self, slide: usize, id: Id, bounds: Bounds) -> Result<()> {
        self.apply(|d| {
            d.object_mut(slide, id)?.bounds = bounds;
            Ok(())
        })
    }
    pub fn set_text(&mut self, slide: usize, id: Id, text: String) -> Result<()> {
        self.apply(|d| {
            if let Content::Text { text: t, .. } = &mut d.object_mut(slide, id)?.content {
                *t = text;
                Ok(())
            } else {
                Err("Selection is not text".into())
            }
        })
    }
    pub fn remove_object(&mut self, slide: usize, id: Id) -> Result<()> {
        self.apply(|d| {
            let s = d.slides.get_mut(slide).ok_or("Invalid slide index")?;
            let i = s
                .objects
                .iter()
                .position(|o| o.id == id)
                .ok_or("Missing object")?;
            s.objects.remove(i);
            Ok(())
        })
    }
    pub fn stack(&mut self, slide: usize, id: Id, front: bool) -> Result<()> {
        self.apply(|d| {
            let s = d.slides.get_mut(slide).ok_or("Invalid slide index")?;
            let i = s
                .objects
                .iter()
                .position(|o| o.id == id)
                .ok_or("Missing object")?;
            let o = s.objects.remove(i);
            if front {
                s.objects.push(o);
            } else {
                s.objects.insert(0, o);
            }
            Ok(())
        })
    }
}
