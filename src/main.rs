#![warn(clippy::all, clippy::pedantic)]
use clap::Parser;
use std::path::Path;

mod file_manager;

#[derive(Parser)]
#[command(author,version,about,long_about = None)]
struct Cli {
    /// Path to the directory you want to copy files from.
    #[arg(long)]
    from: String,
    /// Path to the directory to which you want to copy the files.
    #[arg(long)]
    to: String,
}

fn main() {
    let cli = Cli::parse();
    let from_path = Path::new(&cli.from);
    let to_path = Path::new(&cli.to);
    file_manager::start_copying(from_path, to_path);
}
