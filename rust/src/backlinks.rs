use anyhow::Result;
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub type Backlinks = HashMap<PathBuf, HashSet<PathBuf>>;

pub fn collect(notes_dir: &Path) -> Result<Backlinks> {
    let mut backlinks: Backlinks = HashMap::new();

    let mut file_paths = Vec::new();
    for entry in WalkDir::new(notes_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|ext| ext.to_str()) == Some("html"))
    {
        let file_path = entry.path().to_path_buf();
        file_paths.push(file_path);
    }

    for file_path in &file_paths {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;

        let links = collect_links(&contents);
        let current_file = file_path.strip_prefix(notes_dir)?.to_path_buf();
        let current_file_dir = notes_dir
            .join(&current_file)
            .parent()
            .map(Path::to_path_buf)
            .unwrap();

        for link in links {
            let resolved_link = resolve_link(&link, &current_file_dir, notes_dir);
            match resolved_link {
                Ok(linked_file) => {
                    backlinks
                        .entry(linked_file)
                        .or_default()
                        .insert(current_file.clone());
                }
                Err(err) => {
                    eprintln!(
                        "WARN: {err} (link: {link}, file: {})",
                        current_file.display()
                    );
                }
            }
        }
    }
    Ok(backlinks)
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

/// Resolves relative links such that all links are relative to the `notes_dir`.
fn resolve_link(link: &str, current_file_dir: &Path, notes_dir: &Path) -> Result<PathBuf> {
    let link_decoded = urlencoding::decode(link)?;
    let link_decoded = link_decoded.as_ref();

    let link_absolute = current_file_dir
        .join(link_decoded)
        .with_added_extension("html")
        .canonicalize()?;

    Ok(link_absolute.strip_prefix(notes_dir)?.to_path_buf())
}
