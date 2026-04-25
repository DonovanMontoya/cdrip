use anyhow::Result;
use std::path::{Path, PathBuf};

#[cfg(target_os = "macos")]
fn mount_roots() -> Vec<PathBuf> {
    vec![PathBuf::from("/Volumes")]
}

#[cfg(target_os = "linux")]
fn mount_roots() -> Vec<PathBuf> {
    let mut roots = vec![PathBuf::from("/mnt")];
    for base in ["/run/media", "/media"] {
        if let Ok(users) = std::fs::read_dir(base) {
            for entry in users.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    roots.push(path);
                }
            }
        }
    }
    roots
}

/// Returns Volumes in the VOLUMES directory with AIFF or AIF audio files
pub fn find_audio_volumes() -> Result<Vec<PathBuf>> {
    let mut candidates = Vec::new();

    for root in mount_roots() {
        let Ok(entries) = std::fs::read_dir(&root) else {
            continue;
        };
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() && contains_aiff(&path) {
                candidates.push(path);
            }
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
