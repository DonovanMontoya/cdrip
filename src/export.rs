use anyhow::{Context, Result, bail};
use std::fs::{create_dir_all, remove_file};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn run(
    input_path: &Path,
    output_path: &Path,
    convert_to_flac: bool,
    delete_originals: bool,
) -> Result<()> {
    if !output_path.exists() {
        create_dir_all(output_path)
            .with_context(|| format!("Failed to create output path {}", output_path.display()))?;
    }

    check_directory_writable(output_path)?;

    // Check if the input path exists and is a directory.
    if !input_path.exists() {
        bail!("Input path {} does not exist.", input_path.display());
    }
    if !input_path.is_dir() {
        bail!("Input path {} is not a directory.", input_path.display());
    }

    let files = find_aiff_files(input_path)?;

    if files.is_empty() {
        println!("No audio files found in {}", input_path.display());
        return Ok(());
    }

    println!(
        "Found {} audio files in {}",
        files.len(),
        output_path.display()
    );

    if !convert_to_flac {
        for file in &files {
            let output_file =
                output_path.join(file.file_name().context("Failed to get file name")?);
            std::fs::copy(file, &output_file)
                .with_context(|| format!("Failed to copy file {}", file.display()))?;
            if delete_originals {
                remove_file(file)
                    .with_context(|| format!("Failed to delete audio file {}", file.display()))?;
                println!("Deleted audio file {}", file.display());
            }
            println!("Copied {} -> {}", file.display(), output_file.display());
        }
        return Ok(());
    }

    check_ffmpeg()?;
    for file in &files {
        convert_file(file, output_path)?;

        if delete_originals {
            remove_file(file)
                .with_context(|| format!("Failed to delete audio file {}", file.display()))?;
            println!("Deleted audio file {}", file.display());
        }
    }

    println!("Conversion complete");
    Ok(())
}

/// Check if ffmpeg is installed and working properly
fn check_ffmpeg() -> Result<()> {
    let output = Command::new("ffmpeg")
        .arg("-version")
        .output()
        .context("Failed to run ffmpeg. Is it installed?")?;

    if !output.status.success() {
        bail!("ffmpeg is not working properly");
    }

    Ok(())
}

/// Check if a directory is writable
fn check_directory_writable(dir: &Path) -> Result<()> {
    use std::fs::metadata;

    let metadata = metadata(dir)
        .with_context(|| format!("Failed to read directory metadata for {}", dir.display()))?;

    if metadata.permissions().readonly() {
        bail!(
            "Output directory {} is not writable. Please check the directory permissions.",
            dir.display()
        );
    }

    Ok(())
}

/// Find all AIFF files in a directory (non-recursive)
fn find_aiff_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    let entries = std::fs::read_dir(dir)
        .with_context(|| format!("Failed to read dir {:?}", dir.display()))?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Check if it's a file with .aiff or .aif extension
        if path.is_file()
            && path.extension().is_some_and(|ext| {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                ext_lower == "aiff" || ext_lower == "aif"
            })
        {
            files.push(path);
        }
    }

    Ok(files)
}

/// Convert a single AIFF file to FLAC
fn convert_file(input_path: &Path, output_path: &Path) -> Result<PathBuf> {
    // Build output Path: same filename but .flac extension
    let file_stem = input_path
        .file_stem()
        .context("Failed to get file stem from input_path")?;

    let output_file = output_path.join(file_stem).with_extension("flac");

    println!(
        "Converting {} -> {}",
        input_path.display(),
        output_file.display()
    );

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-c:a")
        .arg("flac")
        .arg(&output_file)
        .arg("-y")
        .status()
        .context("Failed to run ffmpeg. Is it installed?")?;

    if !status.success() {
        bail!("FLAC conversion failed");
    }

    Ok(output_file)
}
