use crate::common::*;
use crate::features::spaces::pages::actions::types::SpaceActionType;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum InboxKind {
    ReplyOnComment,
    MentionInComment,
    SpaceStatusChanged,
    SpaceInvitation,
    SpaceActionOngoing,
    SubTeamApplicationSubmitted,
    SubTeamApplicationApproved,
    SubTeamApplicationRejected,
    SubTeamApplicationReturned,
    SubTeamAnnouncementReceived,
    SubTeamAnnouncementComment,
    SubTeamDeregistered,
    SubTeamLeftParent,
    SubTeamParentDeleted,
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
            InboxKind::SpaceActionOngoing => "SPACE_ACT_ON",
            InboxKind::SubTeamApplicationSubmitted => "STAPP_SUB",
            InboxKind::SubTeamApplicationApproved => "STAPP_APR",
            InboxKind::SubTeamApplicationRejected => "STAPP_REJ",
            InboxKind::SubTeamApplicationReturned => "STAPP_RET",
            InboxKind::SubTeamAnnouncementReceived => "STANN_RCV",
            InboxKind::SubTeamAnnouncementComment => "STANN_CMT",
            InboxKind::SubTeamDeregistered => "STTERM_DREG",
            InboxKind::SubTeamLeftParent => "STTERM_LEAVE",
            InboxKind::SubTeamParentDeleted => "STTERM_PDEL",
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
    SpaceActionOngoing {
        space_id: SpacePartition,
        space_title: String,
        action_id: String,
        action_type: SpaceActionType,
        action_title: String,
        cta_url: String,
    },
    SubTeamApplicationSubmitted {
        parent_team_id: String,
        application_id: String,
        sub_team_id: String,
        sub_team_name: String,
        cta_url: String,
    },
    SubTeamApplicationApproved {
        parent_team_id: String,
        parent_team_name: String,
        sub_team_id: String,
        cta_url: String,
    },
    SubTeamApplicationRejected {
        parent_team_id: String,
        parent_team_name: String,
        sub_team_id: String,
        reason: String,
        cta_url: String,
    },
    SubTeamApplicationReturned {
        parent_team_id: String,
        parent_team_name: String,
        sub_team_id: String,
        comment: String,
        cta_url: String,
    },
    SubTeamAnnouncementReceived {
        parent_team_id: String,
        parent_team_name: String,
        announcement_id: String,
        title: String,
        post_id: String,
        post_pk: String,
        cta_url: String,
    },
    SubTeamAnnouncementComment {
        parent_team_id: String,
        post_id: String,
        post_pk: String,
        commenter_user_id: String,
        commenter_name: String,
        comment_preview: String,
        cta_url: String,
    },
    SubTeamDeregistered {
        former_parent_team_id: String,
        former_parent_team_name: String,
        sub_team_id: String,
        reason: String,
        cta_url: String,
    },
    SubTeamLeftParent {
        former_parent_team_id: String,
        former_sub_team_id: String,
        former_sub_team_name: String,
        reason: Option<String>,
        cta_url: String,
    },
    SubTeamParentDeleted {
        former_parent_team_id: String,
        former_parent_team_name: String,
        cta_url: String,
    },
}

impl InboxPayload {
    pub fn url(&self) -> &str {
        match self {
            InboxPayload::ReplyOnComment { cta_url, .. } => cta_url,
            InboxPayload::MentionInComment { cta_url, .. } => cta_url,
            InboxPayload::SpaceStatusChanged { cta_url, .. } => cta_url,
            InboxPayload::SpaceInvitation { cta_url, .. } => cta_url,
            InboxPayload::SpaceActionOngoing { cta_url, .. } => cta_url,
            InboxPayload::SubTeamApplicationSubmitted { cta_url, .. } => cta_url,
            InboxPayload::SubTeamApplicationApproved { cta_url, .. } => cta_url,
            InboxPayload::SubTeamApplicationRejected { cta_url, .. } => cta_url,
            InboxPayload::SubTeamApplicationReturned { cta_url, .. } => cta_url,
            InboxPayload::SubTeamAnnouncementReceived { cta_url, .. } => cta_url,
            InboxPayload::SubTeamAnnouncementComment { cta_url, .. } => cta_url,
            InboxPayload::SubTeamDeregistered { cta_url, .. } => cta_url,
            InboxPayload::SubTeamLeftParent { cta_url, .. } => cta_url,
            InboxPayload::SubTeamParentDeleted { cta_url, .. } => cta_url,
        }
    }
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
            InboxPayload::SpaceActionOngoing { .. } => InboxKind::SpaceActionOngoing,
            InboxPayload::SubTeamApplicationSubmitted { .. } => {
                InboxKind::SubTeamApplicationSubmitted
            }
            InboxPayload::SubTeamApplicationApproved { .. } => {
                InboxKind::SubTeamApplicationApproved
            }
            InboxPayload::SubTeamApplicationRejected { .. } => {
                InboxKind::SubTeamApplicationRejected
            }
            InboxPayload::SubTeamApplicationReturned { .. } => {
                InboxKind::SubTeamApplicationReturned
            }
            InboxPayload::SubTeamAnnouncementReceived { .. } => {
                InboxKind::SubTeamAnnouncementReceived
            }
            InboxPayload::SubTeamAnnouncementComment { .. } => {
                InboxKind::SubTeamAnnouncementComment
            }
            InboxPayload::SubTeamDeregistered { .. } => InboxKind::SubTeamDeregistered,
            InboxPayload::SubTeamLeftParent { .. } => InboxKind::SubTeamLeftParent,
            InboxPayload::SubTeamParentDeleted { .. } => InboxKind::SubTeamParentDeleted,
        }
    }
}
