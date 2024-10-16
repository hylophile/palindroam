#![feature(path_add_extension)]

use clap::Parser;
use regex::Regex;
use scraper::{Html, Selector};
use serde::Serialize;
use serde_json::to_string_pretty;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use anyhow::{bail, Context, Result};
use thiserror::Error;

type Filename = String;
type Backlinks = HashMap<PathBuf, HashSet<PathBuf>>;

#[derive(Serialize)]
struct Metadata {
    author: String,
    backlinks: Backlinks,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the folder containing HTML files
    #[arg(short, long)]
    folder: String,

    /// Optional: The file to write the JSON output to (prints to stdout if not provided)
    #[clap(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let base_notes_folder = PathBuf::from(&args.folder);

    let mut backlinks: Backlinks = HashMap::new();

    let mut file_paths = Vec::new();
    for entry in WalkDir::new(&base_notes_folder)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|ext| ext.to_str()) == Some("html"))
    {
        let file_path = entry.path().to_path_buf();
        file_paths.push(file_path);
    }

    for file_path in &file_paths {
        if let Ok(file) = File::open(file_path) {
            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            reader.read_to_string(&mut contents).unwrap();

            let links = collect_links(&contents);

            // dbg!(&links);
            let current_file = file_path
                .strip_prefix(&base_notes_folder)
                .unwrap()
                .to_path_buf();

            // .to_string_lossy()
            // .strip_suffix(".html")
            // .unwrap()
            // .to_string();

            for link in links {
                let resolved_link = resolve_link(&link, &current_file, &base_notes_folder);
                if let Ok(linked_file) = resolved_link {
                    backlinks
                        .entry(linked_file)
                        .or_default()
                        .insert(current_file.clone());
                } else {
                    eprintln!(
                        "WARN: unresolved link: '{link}' (file: {})",
                        current_file.display()
                    );
                }
            }
        }
    }

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
}

/// Collects all links to other notes. (TODO really?)
fn collect_links(html_content: &str) -> Vec<String> {
    let document = Html::parse_document(html_content);
    let selector = Selector::parse("a").unwrap();
    let mut links = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            // dbg!(href);
            let re = Regex::new("^[^#]+").unwrap();
            if let Some(href_without_hash) = re.find(href) {
                let href_without_hash = href_without_hash.as_str();
                let href_without_hash = href_without_hash
                    .strip_prefix("./")
                    .unwrap_or(href_without_hash);
                if href_without_hash.starts_with("https://") // TODO external links might be interesting to handle!
                    || href_without_hash.starts_with("http://")
                {
                    continue;
                }
                links.push(href_without_hash.to_string());
            }
        }
    }

    links
}

/// Resolves relative links such that all links are relative to the `base_notes_folder`.
fn resolve_link(link: &str, current_file: &PathBuf, base_notes_folder: &Path) -> Result<PathBuf> {
    let current_path = base_notes_folder.join(current_file);

    let base_path = current_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_default();

    let decoded_link = urlencoding::decode(link).expect("cannot URL-decode link");
    let u = base_path
        .join(&*decoded_link)
        .with_added_extension("html")
        .canonicalize()?;

    Ok(u.strip_prefix(base_notes_folder.canonicalize()?)?
        .to_path_buf())
}
