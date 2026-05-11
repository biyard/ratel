use super::*;

#[test]
fn legacy_raw_string_deserializes_as_html_content() {
    let json = r#""<p>hello</p>""#;
    let body: ContentBody = serde_json::from_str(json).unwrap();
    assert_eq!(body, ContentBody::HtmlContent("<p>hello</p>".to_string()));
}

#[test]
fn tagged_html_content_deserializes() {
    let json = r#"{"content_type":"html_content","data":"<p>hi</p>"}"#;
    let body: ContentBody = serde_json::from_str(json).unwrap();
    assert_eq!(body, ContentBody::HtmlContent("<p>hi</p>".to_string()));
}

#[test]
fn tagged_structured_content_deserializes() {
    let json = r#"{
        "content_type":"structured_content",
        "data":{"schema_version":1,"blocks":[],"meta":{}}
    }"#;
    let body: ContentBody = serde_json::from_str(json).unwrap();
    match body {
        ContentBody::StructuredContent(d) => assert_eq!(d.schema_version, 1),
        _ => panic!("wrong variant"),
    }
}

#[test]
fn html_content_serializes_as_tagged() {
    let body = ContentBody::HtmlContent("<p>x</p>".to_string());
    let json = serde_json::to_value(&body).unwrap();
    assert_eq!(
        json,
        serde_json::json!({"content_type":"html_content","data":"<p>x</p>"})
    );
}

#[test]
fn structured_content_serializes_as_tagged() {
    let doc = ContentDocument {
        schema_version: 1,
        ..Default::default()
    };
    let body = ContentBody::StructuredContent(doc);
    let json = serde_json::to_value(&body).unwrap();
    assert_eq!(json["content_type"], "structured_content");
    assert!(json["data"].is_object());
}

#[test]
fn round_trip_legacy_string() {
    let legacy = r#""<p>legacy</p>""#;
    let body1: ContentBody = serde_json::from_str(legacy).unwrap();
    let canonical = serde_json::to_string(&body1).unwrap();
    let body2: ContentBody = serde_json::from_str(&canonical).unwrap();
    assert_eq!(body1, body2);
}

#[test]
fn default_is_empty_html() {
    let body: ContentBody = ContentBody::default();
    assert_eq!(body, ContentBody::HtmlContent(String::new()));
}

#[test]
fn html_content_to_plain_text_strips_tags() {
    let body = ContentBody::html("<p>Hello <b>world</b> <a href='x'>link</a></p>");
    assert_eq!(body.to_plain_text(), "Hello world link");
}

#[test]
fn html_content_to_html_returns_string() {
    let body = ContentBody::html("<p>raw</p>");
    assert_eq!(body.to_html(), "<p>raw</p>");
}

#[test]
fn structured_content_to_plain_text_walks_blocks() {
    let doc = ContentDocument {
        schema_version: 1,
        blocks: vec![Block {
            id: "b1".into(),
            kind: BlockKind::Paragraph(TextBlock {
                rich_text: RichText(vec![InlineNode::Text(TextRun {
                    content: "Hello world".into(),
                    annotations: Annotations::default(),
                    link: None,
                })]),
                color: Color::Default,
            }),
            children: vec![],
            created_at: 0,
            updated_at: 0,
        }],
        meta: serde_json::Map::new(),
    };
    let body = ContentBody::structured(doc);
    assert_eq!(body.to_plain_text(), "Hello world");
}

#[test]
fn char_count_counts_unicode_chars() {
    let body = ContentBody::html("<p>가나다</p>");
    assert_eq!(body.char_count(), 3);
}
