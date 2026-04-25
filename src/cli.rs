use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cdrip")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Locate and Rip aiff files from CD
    Rip {
        /// Output directory for ripped files (e.g. ~/Desktop/AlbumName)
        output: Option<PathBuf>,

        /// Convert ripped files to FLAC (default: copy as AIFF)
        #[arg(short, long, default_value_t = false)]
        convert: bool,

        /// Delete original AIFF files from the CD volume after export
        #[arg(short, long, default_value_t = false)]
        delete: bool,
    },

    /// Shows contents of CD
    View {
        /// Show contents for the specific cd Dir
        #[arg(short, long)]
        media: Option<String>,
    },
    /// Make flac files from aiff files (Requires ffmpeg)
    Makeflac {
        /// Path to directory containing AIFF files (auto-detects from /Volumes if omitted)
        path: Option<PathBuf>,

        /// Output directory for FLAC files (defaults to input path if omitted)
        output: Option<PathBuf>,

        /// Delete original AIFF files after conversion
        #[arg(short, long, default_value_t = false)]
        delete: bool,
    },
}
