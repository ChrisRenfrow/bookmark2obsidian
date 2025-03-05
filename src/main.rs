mod bookmarks;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

/// A simple tool for converting your browser bookmarks into Obsidian-flavored Markdown files with frontmatter properties
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to JSON bookmarks export (Firefox or Chrom(e|ium))
    #[arg(short = 'b', long, value_name = "FILE")]
    bookmarks: PathBuf,
    /// Path to vault/destination directory
    #[arg(short = 'v', long, value_name = "DIRECTORY")]
    vault: PathBuf,
    /// Optional namespace to prefix the Obsidian tag path
    ///
    /// e.g. a namespace of "bookmarks" applied to tag "school" would become `#bookmarks/school` in Obsidian
    #[arg(short = 't', long, value_name = "NAMESPACE")]
    tag_namespace: Option<String>,
    /// Creates base-URL nodes and links child bookmarks
    ///
    /// e.g. all bookmarks that share en.wikipedia.com as a common base-URL would be linked as decendants of a simple list of links in a note titled "en.wikipedia.com"
    #[arg(short = 'u', long)]
    baseurl_linking: bool,
    /// Imports bookmarks as a flat list, disregarding folder hierarchy
    #[arg(short = 'f', long)]
    flatten: bool,
    /// Uses links to represent folder hierarchy instead of folders (overrides `flatten`)
    ///
    /// e.g. each folder would be imported as a single file with links to children
    #[arg(short = 'l', long)]
    link_hierarchy: bool,
}

fn main() {
    let args = Cli::parse();

    // 1. parse the html
    //
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn bookmark_and_folder_struct() {
    //     let input = include_str!("../simple-bookmarks.html");
    //     let expected = Folder {
    //         title: "Test Folder".to_string(),
    //         add_date: 1740601261,
    //         last_modified: 1740601275,
    //         children: vec![BookmarkNode::Link(Bookmark {
    //             title: "Test".to_string(),
    //             url: "https://test.website/".to_string(),
    //             add_date: 1740601105,
    //             last_modified: 1740601275,
    //             tags: Some(vec!["test".to_string(), "test2".to_string()]),
    //         })],
    //     };

    //     let result: Folder = parse_bookmark_file_contents(input).unwrap();
    // }
}
