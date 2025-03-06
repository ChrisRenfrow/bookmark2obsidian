use std::path::PathBuf;

use anyhow::Result;
use jiff::Timestamp;

use crate::{bookmarks::model::BookmarkNode, cli::Cli};

fn build_tags(tags: Option<Vec<String>>, prefix: Option<String>) -> String {
    match tags {
        Some(tags) => tags
            .into_iter()
            .map(|tag| {
                let tag = tag.to_lowercase();
                match &prefix {
                    Some(prefix) => format!("{prefix}/{tag}"),
                    None => tag,
                }
            })
            .fold("tags:".to_string(), |acc, tag| format!("{acc}\n- {tag}")),
        None => "".to_string(),
    }
}

fn fmt_timestamp(timestamp: Timestamp) -> String {
    timestamp.strftime("%Y-%m-%dT%H:%M:%S").to_string()
}

fn build_bookmark_file_contents(bookmark: BookmarkNode, opts: Option<Cli>) -> String {
    let tag_namespace = match opts {
        Some(Cli { tag_namespace, .. }) => tag_namespace,
        None => None,
    };
    let BookmarkNode::Link {
        title,
        url,
        add_date,
        last_modified,
        tags,
    } = bookmark
    else {
        panic!("Expected BookmarkNode::Link, got BookmarkNode::Folder");
    };

    let tags_list = build_tags(tags, tag_namespace);

    format!(
        r#"---
bookmark_title: {title}
bookmark_url: {url}
bookmark_add_date: {}
bookmark_last_modified: {}
{tags_list}
---"#,
        fmt_timestamp(add_date),
        fmt_timestamp(last_modified)
    )
}

fn write_bookmark_file(bookmark: BookmarkNode, path: PathBuf) -> Result<()> {
    let BookmarkNode::Link {
        title,
        url,
        add_date,
        last_modified,
        tags,
    } = bookmark
    else {
        panic!("Expected BookmarkNode::Link, got BookmarkNode::Folder");
    };

    let file_name = title.replace(|c: char| !c.is_alphanumeric(), "_");
    let mut path = path;
    path.push(format!("{file_name}.md"));

    Ok(())
}

fn export_to_vault(bookmarks: BookmarkNode, opts: Cli) -> Result<()> {
    let Cli { vault, .. } = opts;

    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{bookmarks::model::BookmarkNode, cli::Cli};

    use super::{build_bookmark_file_contents, build_tags};

    #[test]
    fn build_tags_works() {
        // Check basic functionality
        let input = Some(vec!["test".to_string(), "test2".to_string()]);
        let expected = r#"tags:
- test
- test2"#;
        let result = build_tags(input.clone(), None);
        assert_eq!(expected, result);
        // Check prefix
        let expected = r#"tags:
- bookmarks/test
- bookmarks/test2"#;
        let result = build_tags(input, Some("bookmarks".to_string()));
        assert_eq!(expected, result);
        // Check None
        let input = None;
        let expected = "";
        let result = build_tags(input, None);
        assert_eq!(expected, result);
    }

    #[test]
    fn build_bookmark_file_contents_works() {
        let input = BookmarkNode::Link {
            title: "Test".to_string(),
            url: "http://test.website/".to_string(),
            add_date: "2025-03-06T09:06:02Z".parse().unwrap(),
            last_modified: "2025-03-06T09:06:02Z".parse().unwrap(),
            tags: Some(vec!["test".to_string(), "test2".to_string()]),
        };
        let expected = r#"---
bookmark_title: Test
bookmark_url: http://test.website/
bookmark_add_date: 2025-03-06T09:06:02
bookmark_last_modified: 2025-03-06T09:06:02
tags:
- test
- test2
---"#;
        let result = build_bookmark_file_contents(input, None);
        assert_eq!(expected, result);
    }
}
