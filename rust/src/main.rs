#![feature(path_add_extension)]

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use serde::Serialize;
use serde_json::to_string_pretty;

mod backlinks;

use backlinks::Backlinks;

#[derive(Serialize)]
struct Metadata {
    author: String,
    backlinks: Backlinks,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the directory containing HTML notes
    #[arg(short, long)]
    notes_dir: String,

    /// Optional: The file to write the JSON output to (prints to stdout if not provided)
    #[clap(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let notes_dir = PathBuf::from(&args.notes_dir).canonicalize()?;
    let backlinks = backlinks::collect(&notes_dir)?;
    let metadata = Metadata {
        author: "rando".to_string(),
        backlinks,
    };

    let json_output = to_string_pretty(&metadata).unwrap();

    match args.output {
        Some(output) => {
            if let Err(e) = std::fs::write(output, json_output) {
                eprintln!("Error writing to file: {e}");
                std::process::exit(1);
            }
        }
        None => {
            println!("{json_output}");
        }
    }

    Ok(())
}
