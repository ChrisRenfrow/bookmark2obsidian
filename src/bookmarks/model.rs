use jiff::{civil::Date, Timestamp};

use crate::bookmarks::{
    chrome::{ChromeBookmarkNode, ChromeBookmarks},
    firefox::FirefoxBookmarkNode,
};

// TODO: Consider using a Bookmarks wrapper struct to more cleanly unify these bookmark variants
//          (see ChromeBookmarkRoots).
/// The unifying bookmark type between Firefox and Chrom(e|ium) variants.
#[derive(Debug, Clone, PartialEq)]
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
            add_date: chrome_to_unix_timestamp("0".to_string()),
            last_modified: chrome_to_unix_timestamp("0".to_string()),
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
                        add_date: chrome_to_unix_timestamp(date_added),
                        last_modified: chrome_to_unix_timestamp(date_modified),
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
                add_date: chrome_to_unix_timestamp(date_added),
                last_modified: chrome_to_unix_timestamp(date_modified),
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
                add_date: chrome_to_unix_timestamp(date_added),
                last_modified: chrome_to_unix_timestamp(date_last_used),
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
                add_date: Timestamp::from_microsecond(date_added.try_into().unwrap()).unwrap(),
                last_modified: Timestamp::from_microsecond(last_modified.try_into().unwrap())
                    .unwrap(),
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
                add_date: Timestamp::from_microsecond(date_added.try_into().unwrap()).unwrap(),
                last_modified: Timestamp::from_microsecond(last_modified.try_into().unwrap())
                    .unwrap(),
                tags: Some(tags.split(',').map(|tag| tag.to_string()).collect()),
            },
        }
    }
}

/// Converts a chrome-flavored timestamp string (epoch 1601-01-01) and returns it as a Unix-flavored jiff Timestamp.
fn chrome_to_unix_timestamp(chrome_string: String) -> Timestamp {
    let chrome_timestamp =
        Timestamp::from_microsecond(chrome_string.parse::<i64>().unwrap()).unwrap();
    let chrome_epoch: Timestamp = Date::constant(1601, 1, 1).in_tz("UTC").unwrap().into();
    let delta = chrome_timestamp.as_duration();
    chrome_epoch.checked_add(delta).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn date_to_ts(date_str: &str) -> Timestamp {
        date_str.parse::<Timestamp>().unwrap()
    }

    #[test]
    fn chrome_to_unix_timestamp_works() {
        let expected: Timestamp = date_to_ts("2025-03-06T02:51:31Z");
        let input = "13385703091000000".to_string();
        let result = chrome_to_unix_timestamp(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn chrome_conversion_works() {
        let input = include_str!("../../example-chromium.json");
        let expected = BookmarkNode::Folder {
            title: "Root".to_string(),
            add_date: date_to_ts("1601-01-01T00:00:00Z"),
            last_modified: date_to_ts("1601-01-01T00:00:00Z"),
            children: vec![
                BookmarkNode::Folder {
                    title: "Bookmarks bar".to_string(),
                    add_date: date_to_ts("2025-03-03T20:34:47.119717Z"),
                    last_modified: date_to_ts("1601-01-01T00:00:00Z"),
                    children: vec![],
                },
                BookmarkNode::Folder {
                    title: "Other bookmarks".to_string(),
                    add_date: date_to_ts("2025-03-03T20:34:47.119721Z"),
                    last_modified: date_to_ts("2025-03-03T20:36:07.527863Z"),
                    children: vec![BookmarkNode::Folder {
                        title: "Test".to_string(),
                        add_date: date_to_ts("2025-03-03T20:35:09.128841Z"),
                        last_modified: date_to_ts("2025-03-03T20:36:11.273144Z"),
                        children: vec![BookmarkNode::Link {
                            title: "test.website".to_string(),
                            url: "http://test.website/".to_string(),
                            add_date: date_to_ts("2025-03-03T20:36:07.527863Z"),
                            last_modified: date_to_ts("1601-01-01T00:00:00Z"),
                            tags: None,
                        }],
                    }],
                },
                BookmarkNode::Folder {
                    title: "Mobile bookmarks".to_string(),
                    add_date: date_to_ts("2025-03-03T20:34:47.119723Z"),
                    last_modified: date_to_ts("1601-01-01T00:00:00Z"),
                    children: vec![],
                },
            ],
        };
        let result: BookmarkNode = serde_json::from_str::<ChromeBookmarks>(input)
            .unwrap()
            .into();
        assert_eq!(expected, result);
    }

    #[test]
    fn firefox_conversion_works() {
        let input = include_str!("../../example-firefox.json");
        let expected = BookmarkNode::Folder {
            title: "".to_string(),
            add_date: date_to_ts("2025-02-26T20:17:28.186Z"),
            last_modified: date_to_ts("2025-02-26T20:21:15.702Z"),
            children: vec![
                BookmarkNode::Folder {
                    title: "menu".to_string(),
                    add_date: date_to_ts("2025-02-26T20:17:28.186Z"),
                    last_modified: date_to_ts("2025-02-26T20:19:11.666Z"),
                    children: vec![],
                },
                BookmarkNode::Folder {
                    title: "toolbar".to_string(),
                    add_date: date_to_ts("2025-02-26T20:17:28.186Z"),
                    last_modified: date_to_ts("2025-02-26T20:17:28.27Z"),
                    children: vec![],
                },
                BookmarkNode::Folder {
                    title: "unfiled".to_string(),
                    add_date: date_to_ts("2025-02-26T20:17:28.186Z"),
                    last_modified: date_to_ts("2025-02-26T20:21:15.702Z"),
                    children: vec![BookmarkNode::Folder {
                        title: "Test Folder".to_string(),
                        add_date: date_to_ts("2025-02-26T20:21:01.107Z"),
                        last_modified: date_to_ts("2025-02-26T20:21:15.702Z"),
                        children: vec![BookmarkNode::Link {
                            title: "Test".to_string(),
                            url: "https://test.website/".to_string(),
                            add_date: date_to_ts("2025-02-26T20:18:25.441Z"),
                            last_modified: date_to_ts("2025-02-26T20:21:15.702Z"),
                            tags: Some(vec!["test".to_string(), "test1".to_string()]),
                        }],
                    }],
                },
                BookmarkNode::Folder {
                    title: "mobile".to_string(),
                    add_date: date_to_ts("2025-02-26T20:17:28.206Z"),
                    last_modified: date_to_ts("2025-02-26T20:17:28.27Z"),
                    children: vec![],
                },
            ],
        };
        let result: BookmarkNode = serde_json::from_str::<FirefoxBookmarkNode>(input)
            .unwrap()
            .into();
        assert_eq!(expected, result);
    }
}
