import json
from pathlib import Path
from bs4 import BeautifulSoup

def extract_links_from_html(file_path):
    """Extracts all href links from the given HTML file."""
    with file_path.open('r', encoding='utf-8') as file:
        soup = BeautifulSoup(file, 'html.parser')
        links = {a['href'].lstrip('./') for a in soup.find_all('a', href=True) if not a['href'].startswith('https://')}
    return links

def build_backlinks(directory):
    """Builds a dictionary with links as keys and their backlinks (HTML files) as values."""
    backlinks = {}
    html_files = directory.rglob('*.html')

    for html_file in html_files:
        links = extract_links_from_html(html_file)
        if html_file.name not in backlinks:
            backlinks[html_file.name] = []
        for link in links:
            if link not in backlinks:
                backlinks[link] = []
            backlinks[link].append(html_file.name)

    return backlinks

if __name__ == "__main__":
    base_dir = Path('./app/src/lib/notes')
    output_file = Path('app/src/lib/backlinks.json')
    backlinks = build_backlinks(base_dir)

    with output_file.open('w', encoding='utf-8') as f:
        json.dump(backlinks, f, indent=4)

    print(f"Backlinks saved to {output_file}")
