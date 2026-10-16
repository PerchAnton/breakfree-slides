use crate::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashSet},
    io::Cursor,
    sync::Arc,
};

pub type Id = u64;
pub const NATIVE_KIND: &str = "breakfree.slides.native";
pub const CONTENT_VERSION: u32 = 1;
pub const MAX_IMAGE_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_IMAGE_PIXELS: u64 = 16_000_000;
pub const MAX_FILE_BYTES: u64 = 32 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bounds {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}
impl Bounds {
    pub fn contains(self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }
    pub fn valid(self) -> bool {
        [self.x, self.y, self.w, self.h]
            .iter()
            .all(|v| v.is_finite() && v.abs() <= 100_000.0)
            && self.w >= 1.0
            && self.h >= 1.0
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Content {
    Text {
        text: String,
        font_size: f64,
        bold: bool,
        color: [u8; 4],
    },
    Rectangle {
        fill: [u8; 4],
    },
    Image {
        asset: Id,
    },
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Object {
    pub id: Id,
    pub bounds: Bounds,
    pub content: Content,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Slide {
    pub id: Id,
    pub objects: Vec<Object>,
    pub notes: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    pub media_type: String,
    pub width: u32,
    pub height: u32,
    pub data_base64: String,
}
impl Asset {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let (media_type, width, height) = inspect_image(bytes)?;
        Ok(Self {
            media_type,
            width,
            height,
            data_base64: STANDARD.encode(bytes),
        })
    }
    pub fn bytes(&self) -> Result<Vec<u8>> {
        if self.data_base64.len() > MAX_IMAGE_BYTES.div_ceil(3) * 4 {
            return Err("Embedded image exceeds 8 MiB".into());
        }
        STANDARD
            .decode(&self.data_base64)
            .map_err(|_| "Invalid embedded image encoding".into())
    }
    fn validate(&self) -> Result<()> {
        let bytes = self.bytes()?;
        let (mime, w, h) = inspect_image(&bytes)?;
        if (mime, w, h) != (self.media_type.clone(), self.width, self.height) {
            return Err("Image metadata does not match its embedded content".into());
        }
        Ok(())
    }
}
pub fn inspect_image(bytes: &[u8]) -> Result<(String, u32, u32)> {
    if bytes.is_empty() || bytes.len() > MAX_IMAGE_BYTES {
        return Err("Image must be between 1 byte and 8 MiB".into());
    }
    let format = image::guess_format(bytes).map_err(|_| "Unsupported raster image".to_string())?;
    let mime = match format {
        image::ImageFormat::Png => "image/png",
        image::ImageFormat::Jpeg => "image/jpeg",
        image::ImageFormat::WebP => "image/webp",
        _ => return Err("Only PNG, JPEG and WebP images are supported".into()),
    };
    let mut reader = image::ImageReader::with_format(Cursor::new(bytes), format);
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(16_384);
    limits.max_image_height = Some(16_384);
    limits.max_alloc = Some(64 * 1024 * 1024);
    reader.limits(limits);
    let (w, h) = image::ImageReader::with_format(Cursor::new(bytes), format)
        .into_dimensions()
        .map_err(|e| format!("Invalid image dimensions: {e}"))?;
    if w == 0 || h == 0 || u64::from(w) * u64::from(h) > MAX_IMAGE_PIXELS {
        return Err("Image exceeds the 16 megapixel limit".into());
    }
    reader
        .decode()
        .map_err(|e| format!("Invalid image data: {e}"))?;
    Ok((mime.into(), w, h))
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Deck {
    pub kind: String,
    pub content_version: u32,
    pub title: String,
    pub width: f64,
    pub height: f64,
    pub next_id: Id,
    pub slides: Vec<Slide>,
    pub assets: BTreeMap<Id, Arc<Asset>>,
}
impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}
impl Deck {
    pub fn new() -> Self {
        Self {
            kind: NATIVE_KIND.into(),
            content_version: CONTENT_VERSION,
            title: "Presentation1".into(),
            width: 1280.0,
            height: 720.0,
            next_id: 2,
            slides: vec![Slide {
                id: 1,
                objects: vec![],
                notes: String::new(),
            }],
            assets: BTreeMap::new(),
        }
    }
    pub fn fresh_id(&mut self) -> Result<Id> {
        let id = self.next_id;
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or("Identifier space exhausted")?;
        Ok(id)
    }
    pub fn validate(&self) -> Result<()> {
        if self.kind != NATIVE_KIND {
            return Err("This is not a BreakFree Slides presentation".into());
        }
        if self.content_version != CONTENT_VERSION {
            return Err(format!(
                "Unsupported presentation content version {}",
                self.content_version
            ));
        }
        if ![self.width, self.height]
            .iter()
            .all(|v| v.is_finite() && *v >= 100.0 && *v <= 16_384.0)
        {
            return Err("Invalid slide dimensions".into());
        }
        if self.slides.is_empty()
            || self.slides.len() > 1000
            || self.assets.len() > 1000
            || self.title.len() > 4096
        {
            return Err("Document limits exceeded (1–1000 slides, up to 1000 assets)".into());
        }
        let mut ids = HashSet::new();
        let mut max_id = 0;
        let mut count = 0;
        let mut text_bytes = 0usize;
        let mut register = |id| -> Result<()> {
            if id == 0 || !ids.insert(id) {
                return Err("Duplicate or zero identifier".into());
            }
            max_id = max_id.max(id);
            Ok(())
        };
        for (&id, asset) in &self.assets {
            register(id)?;
            asset.validate()?;
        }
        let asset_bytes: usize = self.assets.values().map(|a| a.data_base64.len()).sum();
        if asset_bytes > 24 * 1024 * 1024 {
            return Err("Combined embedded images exceed 24 MiB encoded".into());
        }
        for slide in &self.slides {
            register(slide.id)?;
            text_bytes = text_bytes.saturating_add(slide.notes.len());
            for object in &slide.objects {
                count += 1;
                register(object.id)?;
                if !object.bounds.valid() {
                    return Err("Invalid object geometry".into());
                }
                match &object.content {
                    Content::Text {
                        text, font_size, ..
                    } => {
                        text_bytes = text_bytes.saturating_add(text.len());
                        if text.len() > 65536
                            || !font_size.is_finite()
                            || *font_size < 4.0
                            || *font_size > 400.0
                        {
                            return Err("Invalid text or font size".into());
                        }
                    }
                    Content::Image { asset } => {
                        if !self.assets.contains_key(asset) {
                            return Err("Image references a missing embedded resource".into());
                        }
                    }
                    Content::Rectangle { .. } => {}
                }
            }
        }
        if count > 10_000 || text_bytes > 2 * 1024 * 1024 || self.next_id <= max_id {
            return Err("Document limit or next identifier is invalid".into());
        }
        Ok(())
    }
    pub fn object(&self, slide: usize, id: Id) -> Option<&Object> {
        self.slides.get(slide)?.objects.iter().find(|o| o.id == id)
    }
    pub fn object_mut(&mut self, slide: usize, id: Id) -> Result<&mut Object> {
        self.slides
            .get_mut(slide)
            .and_then(|s| s.objects.iter_mut().find(|o| o.id == id))
            .ok_or("Object no longer exists".into())
    }
}
