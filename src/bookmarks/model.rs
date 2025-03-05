use anyhow::Result;
use jiff::{civil::Time, Timestamp};

use crate::bookmarks::{
    chrome::{ChromeBookmarkNode, ChromeBookmarks},
    firefox::FirefoxBookmarkNode,
};

/// The base bookmark node type
#[derive(Debug, Clone)]
pub enum BookmarkNode {
    Link {
        title: String,
        url: String,
        add_date: Timestamp,
        last_modified: Timestamp,
        tags: Option<Vec<String>>,
    },
    Folder {
        title: String,
        add_date: Timestamp,
        last_modified: Timestamp,
        children: Vec<BookmarkNode>,
    },
}

impl From<ChromeBookmarks> for BookmarkNode {
    fn from(value: ChromeBookmarks) -> Self {
        let roots = vec![
            value.roots.bookmark_bar,
            value.roots.other,
            value.roots.synced,
        ];

        BookmarkNode::Folder {
            title: "Root".to_string(),
            add_date: Timestamp::now(),
            last_modified: Timestamp::now(),
            children: roots
                .into_iter()
                .map(|root| match root {
                    ChromeBookmarkNode::Folder {
                        name,
                        date_added,
                        date_modified,
                        children,
                    } => BookmarkNode::Folder {
                        title: name,
                        add_date: Timestamp::now(),
                        last_modified: Timestamp::now(),
                        children: children.into_iter().map(|child| child.into()).collect(),
                    },
                    _ => unreachable!(),
                })
                .collect(),
        }
    }
}

impl From<ChromeBookmarkNode> for BookmarkNode {
    fn from(value: ChromeBookmarkNode) -> Self {
        match value {
            ChromeBookmarkNode::Folder {
                name,
                date_added,
                date_modified,
                children,
            } => BookmarkNode::Folder {
                title: name,
                add_date: Timestamp::now(),
                last_modified: Timestamp::now(),
                children: children.into_iter().map(|child| child.into()).collect(),
            },
            ChromeBookmarkNode::Url {
                name,
                date_added,
                date_last_used,
                url,
            } => BookmarkNode::Link {
                title: name,
                url,
                add_date: Timestamp::now(),
                last_modified: Timestamp::now(),
                tags: None,
            },
        }
    }
}

impl From<FirefoxBookmarkNode> for BookmarkNode {
    fn from(value: FirefoxBookmarkNode) -> Self {
        match value {
            FirefoxBookmarkNode::PlaceContainer {
                title,
                date_added,
                last_modified,
                children,
            } => BookmarkNode::Folder {
                title,
                add_date: Timestamp::now(),
                last_modified: Timestamp::now(),
                children: children
                    .unwrap_or_default()
                    .into_iter()
                    .map(|child| child.into())
                    .collect(),
            },
            FirefoxBookmarkNode::Place {
                title,
                date_added,
                last_modified,
                tags,
                uri,
            } => BookmarkNode::Link {
                title,
                url: uri,
                add_date: Timestamp::now(),
                last_modified: Timestamp::now(),
                tags: Some(tags.split(',').map(|tag| tag.to_string()).collect()),
            },
        }
    }
}

// fn timestamp_from_microseconds(microseconds: i64) -> Result<Timestamp> {
//     // WebKit epoch starts at 1601/1/1 UTC
//     let webkit_epoch: Time = "1601-01-01T00:00:00Z".parse()?;
//     let unix_microseconds = microseconds - webkit_epoch.;
//     Ok(Timestamp::from_microsecond(unix_microseconds)?)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chrome_conversion() {
        let input = include_str!("../../example-chromium.json");
        let result: BookmarkNode = serde_json::from_str::<ChromeBookmarks>(input)
            .unwrap()
            .into();
        dbg!(result);
        panic!()
    }
}
