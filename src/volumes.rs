use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

const VOLUMES_ROOT: &str = "/VOLUMES";

/// Returns Volumes in the VOLUMES directory with AIFF or AIF audio files
pub fn find_audio_volumes() -> Result<Vec<PathBuf>> {
    let mut candidates = Vec::new();

    let entries = std::fs::read_dir(VOLUMES_ROOT).context("Failed to read VOLUMES directory")?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && contains_aiff(&path) {
            candidates.push(path);
        }
    }
    Ok(candidates)
}

/// Returns true if the directory contains (at least one) AIFF or AIF file(s)
fn contains_aiff(dir: &Path) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false; //Unable to read directory -- Skip
    };

    entries.filter_map(|e| e.ok()).any(|e| {
        let path = e.path();
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            ext == "aiff" || ext == "aif"
        } else {
            false
        }
    })
}
