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

type Filename = String;
type Backlinks = HashMap<Filename, HashSet<Filename>>;

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
    let folder_path = PathBuf::from(&args.folder);

    let mut backlinks: Backlinks = HashMap::new();

    let mut file_paths = Vec::new();
    for entry in WalkDir::new(&folder_path)
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
                .strip_prefix(&folder_path)
                .unwrap()
                .to_string_lossy()
                .strip_suffix(".html")
                .unwrap()
                .to_string();

            for link in links {
                let normalized_link =
                    normalize_link(&link, &current_file, &file_paths, &folder_path);
                if let Some(linked_file) = normalized_link {
                    backlinks
                        .entry(linked_file)
                        .or_default()
                        .insert(current_file.clone());
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

fn collect_links(html_content: &str) -> Vec<String> {
    let document = Html::parse_document(html_content);
    let selector = Selector::parse("a").unwrap();
    let mut links = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            dbg!(href);
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

// Function to normalize the link (resolve relative paths)
fn normalize_link(
    link: &str,
    current_file: &str,
    all_files: &[PathBuf],
    base_folder: &Path,
) -> Option<Filename> {
    // Handle relative links by converting them into absolute paths
    let current_path = base_folder.join(current_file);

    let base_path = current_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_default();

    dbg!(current_path, &base_path, &link);

    // TODO url decode link
    let dec = urlencoding::decode(link).expect("utf8");
    dbg!(&dec);
    dbg!(base_path.join(&*dec));
    dbg!(base_path.join(&*dec).join(".html")); // TODO this is readme.md/.html, but we want to append .html
    panic!();
    let absolute_link = base_path.join(link).canonicalize().ok()?;

    // Check if the resolved path is one of the files in our list
    for file in all_files {
        dbg!(file.canonicalize());
        if absolute_link == file.canonicalize().ok()? {
            // Make the path relative to the base folder
            return file
                .strip_prefix(base_folder)
                .ok()
                .map(|path| path.to_string_lossy().to_string());
        }
    }

    panic!();

    None
}
