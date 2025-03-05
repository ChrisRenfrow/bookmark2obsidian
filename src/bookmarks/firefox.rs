use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Firefox exports refer to their bookmarks as "places" and folders as "place containers"
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum FirefoxBookmarkNode {
    #[serde(rename_all = "camelCase")]
    Place {
        title: String,
        date_added: u64,
        last_modified: u64,
        tags: String,
        uri: String,
    },
    #[serde(rename_all = "camelCase")]
    PlaceContainer {
        title: String,
        date_added: u64,
        last_modified: u64,
        children: Option<Vec<FirefoxBookmarkNode>>,
    },
}

/// Attempts to deserialize JSON data representing Firefox bookmarks
pub fn bookmarks_from_ff_json(data: &str) -> Result<FirefoxBookmarkNode> {
    let result: FirefoxBookmarkNode = serde_json::from_str(data)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_parse_ff_place() {
        let expected = FirefoxBookmarkNode::Place {
            title: "Test".to_string(),
            date_added: 1740601105441000,
            last_modified: 1740601275702000,
            tags: "test,test1".to_string(),
            uri: "https://test.website/".to_string(),
        };
        let input = r#"{
    "guid": "mAQ-wHKmGwfH",
    "title": "Test",
    "index": 0,
    "dateAdded": 1740601105441000,
    "lastModified": 1740601275702000,
    "id": 12,
    "typeCode": 1,
    "tags": "test,test1",
    "type": "text/x-moz-place",
    "uri": "https://test.website/",
    "keyword": "test_keyword",
    "postData": null
}"#;
        assert_eq!(
            serde_json::from_str::<FirefoxBookmarkNode>(input).unwrap(),
            expected
        );
    }

    #[test]
    fn deserialize_ff_bookmarks() {
        let input = include_str!("../../example-firefox.json");
        let expected = FirefoxBookmarkNode::PlaceContainer {
            title: "".to_string(),
            date_added: 1740601048186000,
            last_modified: 1740601275702000,
            children: Some(vec![
                FirefoxBookmarkNode::PlaceContainer {
                    title: "menu".to_string(),
                    date_added: 1740601048186000,
                    last_modified: 1740601151666000,
                    children: None,
                },
                FirefoxBookmarkNode::PlaceContainer {
                    title: "toolbar".to_string(),
                    date_added: 1740601048186000,
                    last_modified: 1740601048270000,
                    children: None,
                },
                FirefoxBookmarkNode::PlaceContainer {
                    title: "unfiled".to_string(),
                    date_added: 1740601048186000,
                    last_modified: 1740601275702000,
                    children: Some(vec![FirefoxBookmarkNode::PlaceContainer {
                        title: "Test Folder".to_string(),
                        date_added: 1740601261107000,
                        last_modified: 1740601275702000,
                        children: Some(vec![FirefoxBookmarkNode::Place {
                            title: "Test".to_string(),
                            date_added: 1740601105441000,
                            last_modified: 1740601275702000,
                            tags: "test,test1".to_string(),
                            uri: "https://test.website/".to_string(),
                        }]),
                    }]),
                },
                FirefoxBookmarkNode::PlaceContainer {
                    title: "mobile".to_string(),
                    date_added: 1740601048206000,
                    last_modified: 1740601048270000,
                    children: None,
                },
            ]),
        };
        let result: FirefoxBookmarkNode = serde_json::from_str(input).unwrap();
        assert_eq!(result, expected);
    }
}
