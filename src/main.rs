use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;

mod cli;
mod makeflac;
mod volumes;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Rip { path } => {
            println!("Rip: output path: {:?}", path);
        }
        Commands::View { media } => {
            println!("View: device: {:?}", media);
        }
        Commands::Makeflac {
            path,
            output,
            delete,
        } => {
            let output_path = output.as_deref().unwrap_or(&path);
            makeflac::run(&path, output_path, delete)?;
        }
    }

    Ok(())
}
