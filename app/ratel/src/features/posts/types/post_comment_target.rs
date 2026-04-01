use crate::features::posts::*;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::str::FromStr;

/// A type that can represent both PostComment and PostCommentReply EntityTypes.
/// Used for like/unlike operations that apply to both comments and replies.
#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct PostCommentTargetEntityType(pub String);

impl std::fmt::Display for PostCommentTargetEntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PostCommentTargetEntityType {
    type Err = crate::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = if s.starts_with("POST_COMMENT_REPLY#") {
            s.replacen("POST_COMMENT_REPLY#", "", 1)
        } else if s.starts_with("POST_COMMENT#") {
            s.replacen("POST_COMMENT#", "", 1)
        } else {
            s.to_string()
        };
        Ok(Self(s))
    }
}

impl From<EntityType> for PostCommentTargetEntityType {
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::PostComment(id) => Self(id),
            EntityType::PostCommentReply(parent_id, reply_id) => {
                Self(format!("{parent_id}::{reply_id}"))
            }
            _ => Self::default(),
        }
    }
}

impl From<PostCommentTargetEntityType> for EntityType {
    fn from(value: PostCommentTargetEntityType) -> Self {
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
            EntityType::PostCommentReply(first, reply_id)
        } else {
            EntityType::PostComment(first)
        }
    }
}

impl From<PostCommentEntityType> for PostCommentTargetEntityType {
    fn from(value: PostCommentEntityType) -> Self {
        let et: EntityType = value.into();
        et.into()
    }
}
