use std::path::PathBuf;

use clap::Parser;

/// A simple tool for converting your browser bookmarks into Obsidian-flavored Markdown files with frontmatter properties
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to JSON bookmarks export (Firefox or Chrom(e|ium))
    #[arg(short = 'b', long, value_name = "FILE")]
    pub bookmarks: PathBuf,
    /// Path to vault/destination directory
    #[arg(short = 'v', long, value_name = "DIRECTORY")]
    pub vault: PathBuf,
    /// Optional namespace to prefix the Obsidian tag path
    ///
    /// e.g. a namespace of "bookmarks" applied to tag "school" would become `#bookmarks/school` in Obsidian
    #[arg(short = 't', long, value_name = "NAMESPACE")]
    pub tag_namespace: Option<String>,
    /// Creates base-URL nodes and links child bookmarks
    ///
    /// e.g. all bookmarks that share en.wikipedia.com as a common base-URL would be linked as decendants of a simple list of links in a note titled "en.wikipedia.com"
    #[arg(short = 'u', long)]
    pub baseurl_linking: bool,
    /// Imports bookmarks as a flat list, disregarding folder hierarchy
    #[arg(short = 'f', long)]
    pub flatten: bool,
    /// Uses links to represent folder hierarchy instead of folders (overrides `flatten`)
    ///
    /// e.g. each folder would be imported as a single file with links to children
    #[arg(short = 'l', long)]
    pub link_hierarchy: bool,
}

impl Cli {
    pub fn start() -> Self {
        Self::parse()
    }
}
