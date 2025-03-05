use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ChromeBookmarkNode {
    Folder {
        name: String,
        date_added: String,
        date_modified: String,
        children: Vec<ChromeBookmarkNode>,
    },
    Url {
        name: String,
        date_added: String,
        date_last_used: String,
        url: String,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChromeBookmarkRoots {
    pub bookmark_bar: ChromeBookmarkNode,
    pub other: ChromeBookmarkNode,
    pub synced: ChromeBookmarkNode,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ChromeBookmarks {
    pub checksum: String,
    pub roots: ChromeBookmarkRoots,
    pub version: u8,
}

pub fn bookmarks_from_chrome_json(data: &str) -> Result<ChromeBookmarks> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    // Makes comparing failed assertions for complex structures like this *really* nice.
    use pretty_assertions::assert_eq;

    #[test]
    fn deserialize_chrome_bookmarks() {
        let input = include_str!("../../example-chromium.json");
        let expected = ChromeBookmarks {
            checksum: "091d7dab6aa5b695a642ef96765ab1ed".to_string(),
            roots: ChromeBookmarkRoots {
                bookmark_bar: ChromeBookmarkNode::Folder {
                    name: "Bookmarks bar".to_string(),
                    date_modified: "0".to_string(),
                    date_added: "13385507687119717".to_string(),
                    children: vec![],
                },
                other: ChromeBookmarkNode::Folder {
                    name: "Other bookmarks".to_string(),
                    date_added: "13385507687119721".to_string(),
                    date_modified: "13385507767527863".to_string(),
                    children: vec![ChromeBookmarkNode::Folder {
                        name: "Test".to_string(),
                        date_added: "13385507709128841".to_string(),
                        date_modified: "13385507771273144".to_string(),
                        children: vec![ChromeBookmarkNode::Url {
                            name: "test.website".to_string(),
                            date_added: "13385507767527863".to_string(),
                            date_last_used: "0".to_string(),
                            url: "http://test.website/".to_string(),
                        }],
                    }],
                },
                synced: ChromeBookmarkNode::Folder {
                    name: "Mobile bookmarks".to_string(),
                    date_added: "13385507687119723".to_string(),
                    date_modified: "0".to_string(),
                    children: vec![],
                },
            },
            version: 1,
        };
        let result: ChromeBookmarks = serde_json::from_str(input).unwrap();
        assert_eq!(expected, result)
    }
}
