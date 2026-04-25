use crate::cli::{Cli, Commands};
use anyhow::{Result, bail};
use clap::Parser;
use std::io::{self, Write};
use std::path::PathBuf;

mod cli;
mod export;
mod volumes;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Makeflac {
            path,
            output,
            delete,
        } => {
            let resolved = resolve_path(path)?;
            let out = output.unwrap_or_else(|| resolved.clone());
            export::run(&resolved, &out, true, delete)?;
        }
        Commands::Rip {
            output,
            convert,
            delete,
        } => {
            let input = resolve_path(None)?;
            let out = output.unwrap_or_else(|| input.clone());
            export::run(&input, &out, convert, delete)?;
        }
        Commands::View { media } => {
            println!("View not yet implemented: {:?}", media);
        }
    }
    Ok(())
}

/// Use the provided path, or auto-discover from /Volumes.
fn resolve_path(path: Option<PathBuf>) -> Result<PathBuf> {
    // If the user passed --path, use it directly — no discovery needed.
    if let Some(p) = path {
        return Ok(p);
    }

    let mut volumes = volumes::find_audio_volumes()?;

    match volumes.len() {
        0 => bail!("No audio volumes found on this system!"),
        1 => {
            let vol = volumes.remove(0);
            println!("Using {} as the only CD found.", vol.display());
            Ok(vol)
        }
        _ => {
            // Multiple CDs mounted — show a numbered list and prompt.
            println!("Multiple audio volumes found:");
            for (i, vol) in volumes.iter().enumerate() {
                println!("  [{}] {}", i + 1, vol.display());
            }

            print!("Choose one (1-{}): ", volumes.len());
            io::stdout().flush()?; // Ensure the prompt is printed

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let choice: usize = input.trim().parse().unwrap_or(0);

            if choice < 1 || choice > volumes.len() {
                bail!("Invalid selection");
            }

            Ok(volumes.remove(choice - 1))
        }
    }
}
