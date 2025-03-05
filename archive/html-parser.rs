use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::multispace0,
    multi::{fold_many0, fold_many1, many_till},
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};

#[derive(Debug, PartialEq)]
pub enum BookmarkNode<'a> {
    Folder {
        title: &'a str,
        add_date: &'a str,
        last_modified: &'a str,
        children: Vec<BookmarkNode<'a>>,
    },
    Link {
        href: &'a str,
        title: &'a str,
        add_date: &'a str,
        last_modified: &'a str,
        tags: Option<&'a str>,
    },
}

fn parse_attribute(input: &str) -> IResult<&str, (&str, &str)> {
    preceded(
        multispace0,
        pair(
            take_until("="),
            delimited(tag("=\""), take_until("\""), tag("\"")),
        ),
    )
    .parse(input)
}

fn parse_attributes(input: &str) -> IResult<&str, HashMap<&str, &str>> {
    let (input, (attrs, _)) = many_till(parse_attribute, tag(">")).parse(input)?;

    Ok((input, attrs.into_iter().collect()))
}

fn parse_folder<'a>(input: &'a str) -> IResult<&str, BookmarkNode> {
    let (input, _) = tag("<DT><H3")(input)?;
    let (input, attrs) = parse_attributes(input)?;
    dbg!(&attrs);
    let (input, title) = take_until("</H3>")(input)?;
    let (input, _) = tag("</H3>\n")(input)?;
    dbg!(&input);
    let (input, _) = tag("<DL><p>")(input)?;
    dbg!(&input);
    let (input, children) = parse_bookmarks(input)?;
    let (input, _) = tag("</DL><p>")(input)?;

    Ok((
        input,
        BookmarkNode::Folder {
            title,
            add_date: attrs.get("ADD_DATE").unwrap(),
            last_modified: attrs.get("LAST_MODIFIED").unwrap(),
            children,
        },
    ))
}

fn parse_link<'a>(input: &'a str) -> IResult<&str, BookmarkNode> {
    let (input, _) = tag("<DT><A")(input)?;
    let (input, attrs) = parse_attributes(input)?;
    let (input, title) = take_until("<")(input)?;
    let (input, _) = tag("</A>")(input)?;

    Ok((
        input,
        BookmarkNode::Link {
            href: attrs.get("HREF").unwrap(),
            title,
            add_date: attrs.get("ADD_DATE").unwrap(),
            last_modified: attrs.get("LAST_MODIFIED").unwrap(),
            tags: attrs.get("TAGS").copied(),
        },
    ))
}
fn parse_bookmarks(input: &str) -> IResult<&str, Vec<BookmarkNode>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_attribute() {
        assert_eq!(
            parse_attribute("  TEST_ATTR=\"foo\""),
            Ok(("", ("TEST_ATTR", "foo")))
        );
    }

    #[test]
    fn test_parse_link() {
        let input = r#"<DT><A HREF="https://test.website/" ADD_DATE="1740601105" LAST_MODIFIED="1740601275" SHORTCUTURL="test_keyword" TAGS="test,test2">Test</A>"#;
        assert_eq!(
            parse_link(input),
            Ok((
                "",
                BookmarkNode::Link {
                    href: "https://test.website/",
                    title: "Test",
                    add_date: "1740601105",
                    last_modified: "1740601275",
                    tags: Some("test,test2"),
                }
            ))
        )
    }

    #[test]
    fn test_parse_folder() {
        let input = r#"<DT><H3 ADD_DATE="1740601048" LAST_MODIFIED="1740601275" UNFILED_BOOKMARKS_FOLDER="true">Other Bookmarks</H3>
<DL><p>
    <DT><H3 ADD_DATE="1740601261" LAST_MODIFIED="1740601275">Test Folder</H3>
    <DL><p>
        <DT><A HREF="https://test.website/" ADD_DATE="1740601105" LAST_MODIFIED="1740601275" SHORTCUTURL="test_keyword" TAGS="test,test2">Test</A>
    </DL><p>
</DL><p>"#;
        assert_eq!(
            parse_folder(input),
            Ok((
                "",
                BookmarkNode::Folder {
                    title: "Test Folder",
                    add_date: "1740601261",
                    last_modified: "1740601275",
                    children: vec![BookmarkNode::Link {
                        href: "https://test.website/",
                        title: "Test",
                        add_date: "1740601105",
                        last_modified: "1740601275",
                        tags: Some("test,test2"),
                    }],
                }
            ))
        )
    }
}
