use clap::Parser;
use regex::Regex;
use scraper::{Html, Selector};
use serde::Serialize;
use serde_json::to_string_pretty;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};
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
}

fn main() {
    let args = Args::parse();
    let folder_path = &args.folder;

    let mut backlinks: Backlinks = HashMap::new();

    let mut file_paths = Vec::new();
    for entry in WalkDir::new(folder_path)
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

            let current_file = file_path
                .strip_prefix(folder_path)
                .unwrap()
                .to_string_lossy()
                .strip_suffix(".html")
                .unwrap()
                .to_string();

            for link in links {
                backlinks
                    .entry(link)
                    .or_default()
                    .insert(current_file.clone());
                // }
            }
        }
    }

    let metadata = Metadata {
        author: "rando".to_string(),
        backlinks,
    };
    let json_output = to_string_pretty(&metadata).unwrap();
    println!("{json_output}");
}

fn collect_links(html_content: &str) -> Vec<String> {
    let document = Html::parse_document(html_content);
    let selector = Selector::parse("a").unwrap();
    let mut links = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            let re = Regex::new("^[^#]+").unwrap();
            if let Some(href_without_hash) = re.find(href) {
                let href_without_hash = href_without_hash.as_str();
                let href_without_hash = href_without_hash
                    .strip_prefix("./")
                    .unwrap_or(href_without_hash);
                links.push(href_without_hash.to_string());
            }
        }
    }

    links
}
