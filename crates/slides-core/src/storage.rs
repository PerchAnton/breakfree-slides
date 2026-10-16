use crate::{
    Result,
    model::{Deck, MAX_FILE_BYTES, MAX_IMAGE_BYTES},
};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub fn decode(bytes: &[u8]) -> Result<Deck> {
    if bytes.len() as u64 > MAX_FILE_BYTES {
        return Err("Presentation exceeds the 32 MiB file limit".into());
    }
    let deck: Deck =
        serde_json::from_slice(bytes).map_err(|e| format!("Invalid presentation: {e}"))?;
    deck.validate()?;
    Ok(deck)
}
pub fn load(path: &Path) -> Result<Deck> {
    let file = File::open(path).map_err(|e| format!("Cannot open presentation: {e}"))?;
    if file.metadata().map_err(|e| e.to_string())?.len() > MAX_FILE_BYTES {
        return Err("Presentation exceeds the 32 MiB file limit".into());
    }
    let mut bytes = Vec::new();
    file.take(MAX_FILE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    decode(&bytes)
}
pub fn read_image(path: &Path) -> Result<Vec<u8>> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut bytes = Vec::new();
    file.take(MAX_IMAGE_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > MAX_IMAGE_BYTES {
        return Err("Image exceeds 8 MiB".into());
    }
    Ok(bytes)
}
pub fn save(deck: &Deck, path: &Path) -> Result<()> {
    deck.validate()?;
    let bytes = serde_json::to_vec_pretty(deck).map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX_FILE_BYTES {
        return Err("Presentation exceeds the 32 MiB file limit".into());
    }
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut temp = tempfile::NamedTempFile::new_in(parent)
        .map_err(|e| format!("Cannot create temporary presentation: {e}"))?;
    temp.write_all(&bytes)
        .and_then(|_| temp.flush())
        .and_then(|_| temp.as_file().sync_all())
        .map_err(|e| format!("Save failed; previous file preserved: {e}"))?;
    temp.persist(path).map_err(|e| {
        format!(
            "Cannot replace presentation; previous file preserved: {}",
            e.error
        )
    })?;
    #[cfg(unix)]
    if let Ok(dir) = File::open(parent) {
        let _ = dir.sync_all();
    }
    Ok(())
}
