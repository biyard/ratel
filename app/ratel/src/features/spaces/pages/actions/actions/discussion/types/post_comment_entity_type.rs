use crate::features::spaces::pages::actions::actions::discussion::*;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::str::FromStr;

#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpacePostCommentTargetEntityType(pub String);

impl std::fmt::Display for SpacePostCommentTargetEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SpacePostCommentTargetEntityType {
    type Err = crate::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = if s.starts_with("SPACE_POST_COMMENT_REPLY#") {
            // After stripping the prefix the remainder is `parent#reply`.
            // The inner `#` would be parsed as a URL fragment delimiter
            // (browsers strip everything after `#` before sending), causing
            // requests like `/comments/parent#reply/likes` to actually hit
            // `/comments/parent` and route to `update_comment` instead of
            // `like_comment` — yielding `missing field content` errors.
            // Switch to the `::` separator that `From<EntityType>` uses so
            // both sides of the conversion produce URL-safe values.
            s.replacen("SPACE_POST_COMMENT_REPLY#", "", 1)
                .replacen('#', "::", 1)
        } else if s.starts_with("SPACE_POST_COMMENT#") {
            s.replacen("SPACE_POST_COMMENT#", "", 1)
        } else {
            s.to_string()
        };

        Ok(Self(s))
    }
}

impl From<EntityType> for SpacePostCommentTargetEntityType {
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::SpacePostComment(id) => Self(id),
            EntityType::SpacePostCommentReply(parent_id, reply_id) => {
                Self(format!("{parent_id}::{reply_id}"))
            }
            _ => Self::default(),
        }
    }
}

impl From<SpacePostCommentTargetEntityType> for EntityType {
    fn from(value: SpacePostCommentTargetEntityType) -> Self {
        let (first, second) = if value.0.contains("::") {
            let mut parts = value.0.splitn(2, "::");
            (
                parts.next().unwrap_or_default().to_string(),
                parts.next().map(str::to_string),
            )
        } else {
            let mut parts = value.0.splitn(2, '#');
            (
                parts.next().unwrap_or_default().to_string(),
                parts.next().map(str::to_string),
            )
        };

        if let Some(reply_id) = second {
            EntityType::SpacePostCommentReply(first, reply_id)
        } else {
            EntityType::SpacePostComment(first)
        }
    }
}
