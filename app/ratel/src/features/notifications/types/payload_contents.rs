use crate::common::*;
use crate::features::notifications::i18n::NotificationsTranslate;
use dioxus_translate::Language;

impl InboxPayload {
    pub fn get_contents(
        &self,
        tr: &NotificationsTranslate,
        lang: &Language,
    ) -> (String, String, Option<String>) {
        match self {
            InboxPayload::ReplyOnComment {
                replier_name,
                comment_preview,
                replier_profile_url,
                ..
            } => (
                tr.reply_title.replace("{name}", replier_name),
                comment_preview.clone(),
                Some(replier_profile_url.clone()),
            ),
            InboxPayload::MentionInComment {
                mentioned_by_name,
                comment_preview,
                ..
            } => (
                tr.mention_title.replace("{name}", mentioned_by_name),
                comment_preview.clone(),
                None,
            ),
            InboxPayload::SpaceStatusChanged {
                space_title,
                new_status,
                ..
            } => (
                tr.space_status_title
                    .replace("{space}", space_title)
                    .replace("{status}", &new_status.translate(lang)),
                String::new(),
                None,
            ),
            InboxPayload::SpaceInvitation {
                space_title,
                inviter_name,
                ..
            } => (
                tr.space_invite_title
                    .replace("{name}", inviter_name)
                    .replace("{space}", space_title),
                String::new(),
                None,
            ),
            InboxPayload::SpaceActionOngoing {
                space_title,
                action_title,
                ..
            } => (
                tr.action_ongoing_title
                    .replace("{action_title}", action_title),
                tr.action_ongoing_subtitle
                    .replace("{space_title}", space_title),
                None,
            ),
            InboxPayload::SubTeamApplicationSubmitted { sub_team_name, .. } => (
                tr.sub_team_app_submitted_title
                    .replace("{team}", sub_team_name),
                String::new(),
                None,
            ),
            InboxPayload::SubTeamApplicationApproved {
                parent_team_name, ..
            } => (
                tr.sub_team_app_approved_title
                    .replace("{parent}", parent_team_name),
                String::new(),
                None,
            ),
            InboxPayload::SubTeamApplicationRejected {
                parent_team_name,
                reason,
                ..
            } => (
                tr.sub_team_app_rejected_title
                    .replace("{parent}", parent_team_name),
                reason.clone(),
                None,
            ),
            InboxPayload::SubTeamApplicationReturned {
                parent_team_name,
                comment,
                ..
            } => (
                tr.sub_team_app_returned_title
                    .replace("{parent}", parent_team_name),
                comment.clone(),
                None,
            ),
            InboxPayload::SubTeamAnnouncementReceived {
                parent_team_name,
                title,
                ..
            } => (
                tr.sub_team_ann_received_title
                    .replace("{parent}", parent_team_name),
                title.clone(),
                None,
            ),
            InboxPayload::SubTeamAnnouncementComment {
                commenter_name,
                comment_preview,
                ..
            } => (
                tr.sub_team_ann_comment_title
                    .replace("{name}", commenter_name),
                comment_preview.clone(),
                None,
            ),
            InboxPayload::SubTeamDeregistered {
                former_parent_team_name,
                reason,
                ..
            } => (
                tr.sub_team_deregistered_title
                    .replace("{parent}", former_parent_team_name),
                reason.clone(),
                None,
            ),
            InboxPayload::SubTeamLeftParent {
                former_sub_team_name,
                reason,
                ..
            } => (
                tr.sub_team_left_parent_title
                    .replace("{team}", former_sub_team_name),
                reason.clone().unwrap_or_default(),
                None,
            ),
            InboxPayload::SubTeamParentDeleted {
                former_parent_team_name,
                ..
            } => (
                tr.sub_team_parent_deleted_title
                    .replace("{parent}", former_parent_team_name),
                String::new(),
                None,
            ),
        }
    }
}
