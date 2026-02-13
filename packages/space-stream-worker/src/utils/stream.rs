use aws_lambda_events::dynamodb::EventRecord;
use main_api::types::EntityType;
use serde_dynamo::{AttributeValue as StreamAttr, Item as StreamItem};
use std::str::FromStr;

pub struct StreamIdentifiers {
    pub space_pk: String,
    pub pk: String,
    pub sk: String,
}

pub fn resolve_space_identifiers(record: &EventRecord) -> Option<StreamIdentifiers> {
    let new_pk = attr_string_stream(&record.change.new_image, "pk");
    let new_sk = attr_string_stream(&record.change.new_image, "sk");
    let key_pk = attr_string_stream(&record.change.keys, "pk");
    let key_sk = attr_string_stream(&record.change.keys, "sk");
    let old_pk = attr_string_stream(&record.change.old_image, "pk");
    let old_sk = attr_string_stream(&record.change.old_image, "sk");

    let pk = new_pk.or(key_pk).or(old_pk)?;
    let sk = new_sk
        .or(key_sk)
        .or(old_sk)
        .unwrap_or_else(|| "UNKNOWN".to_string());

    let explicit_space_pk = attr_string_stream(&record.change.new_image, "space_pk")
        .or_else(|| attr_string_stream(&record.change.keys, "space_pk"))
        .or_else(|| attr_string_stream(&record.change.old_image, "space_pk"));

    let _entity = EntityType::from_str(&sk).ok();

    let space_pk = explicit_space_pk
        .or_else(|| parse_space_pk_from_sk(&sk))
        .or_else(|| {
            if pk.starts_with("SPACE#") {
                Some(pk.clone())
            } else {
                None
            }
        });

    let space_pk = space_pk?;

    Some(StreamIdentifiers { space_pk, pk, sk })
}

pub fn is_space_related_pk(pk: &str) -> bool {
    pk.starts_with("SPACE#")
        || pk.starts_with("SPACE_POST#")
        || pk.starts_with("SPACE_POLL_USER_ANSWER#")
}

fn attr_string_stream(image: &StreamItem, key: &str) -> Option<String> {
    image.get(key).and_then(|value| match value {
        StreamAttr::S(v) => Some(v.clone()),
        StreamAttr::N(v) => Some(v.clone()),
        _ => None,
    })
}

fn parse_space_pk_from_sk(sk: &str) -> Option<String> {
    const PREFIX: &str = "SPACE_POLL_USER_ANSWER#SPACE#";
    const POLL_MARKER: &str = "#POLL#";

    if let Some(rest) = sk.strip_prefix(PREFIX) {
        if let Some((space_id, _poll_part)) = rest.split_once(POLL_MARKER) {
            return Some(format!("SPACE#{}", space_id));
        }
    }
    None
}
