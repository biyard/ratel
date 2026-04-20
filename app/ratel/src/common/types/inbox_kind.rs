use crate::common::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum InboxKind {
    ReplyOnComment,
    MentionInComment,
    SpaceStatusChanged,
    SpaceInvitation,
}

impl Default for InboxKind {
    fn default() -> Self {
        Self::ReplyOnComment
    }
}

impl InboxKind {
    pub fn as_prefix(&self) -> &'static str {
        match self {
            InboxKind::ReplyOnComment => "REPLY",
            InboxKind::MentionInComment => "MENTION",
            InboxKind::SpaceStatusChanged => "SPACE_STATUS",
            InboxKind::SpaceInvitation => "SPACE_INV",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum InboxPayload {
    ReplyOnComment {
        space_id: Option<SpacePartition>,
        post_id: Option<FeedPartition>,
        comment_preview: String,
        replier_name: String,
        replier_profile_url: String,
        cta_url: String,
    },
    MentionInComment {
        comment_preview: String,
        mentioned_by_name: String,
        cta_url: String,
    },
    SpaceStatusChanged {
        space_id: SpacePartition,
        space_title: String,
        new_status: SpaceStatus,
        cta_url: String,
    },
    SpaceInvitation {
        space_id: SpacePartition,
        space_title: String,
        inviter_name: String,
        cta_url: String,
    },
}

impl Default for InboxPayload {
    fn default() -> Self {
        Self::ReplyOnComment {
            space_id: None,
            post_id: None,
            comment_preview: String::new(),
            replier_name: String::new(),
            replier_profile_url: String::new(),
            cta_url: String::new(),
        }
    }
}

impl InboxPayload {
    pub fn kind(&self) -> InboxKind {
        match self {
            InboxPayload::ReplyOnComment { .. } => InboxKind::ReplyOnComment,
            InboxPayload::MentionInComment { .. } => InboxKind::MentionInComment,
            InboxPayload::SpaceStatusChanged { .. } => InboxKind::SpaceStatusChanged,
            InboxPayload::SpaceInvitation { .. } => InboxKind::SpaceInvitation,
        }
    }
}
