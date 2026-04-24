//! Controller for the parent-admin broadcast (announcements) tab.

use crate::common::hooks::{use_infinite_query, InfiniteQuery};
use crate::features::sub_team::controllers::{
    create_announcement_handler, delete_announcement_handler, list_announcements_handler,
    publish_announcement_handler, update_announcement_handler,
};
use crate::features::sub_team::models::SubTeamAnnouncementStatus;
use crate::features::sub_team::types::{
    CreateSubTeamAnnouncementRequest, SubTeamAnnouncementResponse,
    UpdateSubTeamAnnouncementRequest,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamBroadcast {
    pub team_id: ReadSignal<TeamPartition>,
    pub status_filter: Signal<Option<SubTeamAnnouncementStatus>>,
    pub announcements: InfiniteQuery<
        String,
        SubTeamAnnouncementResponse,
        ListResponse<SubTeamAnnouncementResponse>,
    >,
    pub handle_create_draft: Action<(CreateSubTeamAnnouncementRequest,), String>,
    pub handle_update_draft: Action<(String, UpdateSubTeamAnnouncementRequest), ()>,
    pub handle_publish: Action<(String,), ()>,
    pub handle_delete: Action<(String,), ()>,
}

#[track_caller]
pub fn use_sub_team_broadcast() -> std::result::Result<UseSubTeamBroadcast, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamBroadcast>() {
        return Ok(ctx);
    }

    let team_id: TeamPartition = use_context();
    let team_id_signal: ReadSignal<TeamPartition> = use_signal(|| team_id).into();
    let status_filter: Signal<Option<SubTeamAnnouncementStatus>> = use_signal(|| None);

    let mut announcements = use_infinite_query(move |bookmark| {
        let id = team_id_signal();
        let status = status_filter();
        async move { list_announcements_handler(id, bookmark, status).await }
    })?;

    let team_id_for_create = team_id_signal;
    let handle_create_draft =
        use_action(move |req: CreateSubTeamAnnouncementRequest| async move {
            let created = create_announcement_handler(team_id_for_create(), req).await?;
            announcements.refresh();
            Ok::<String, crate::common::Error>(created.id)
        });

    let team_id_for_update = team_id_signal;
    let handle_update_draft = use_action(
        move |announcement_id: String, req: UpdateSubTeamAnnouncementRequest| async move {
            update_announcement_handler(team_id_for_update(), announcement_id, req).await?;
            announcements.refresh();
            Ok::<(), crate::common::Error>(())
        },
    );

    let team_id_for_publish = team_id_signal;
    let handle_publish = use_action(move |announcement_id: String| async move {
        publish_announcement_handler(team_id_for_publish(), announcement_id).await?;
        announcements.refresh();
        Ok::<(), crate::common::Error>(())
    });

    let team_id_for_delete = team_id_signal;
    let handle_delete = use_action(move |announcement_id: String| async move {
        delete_announcement_handler(team_id_for_delete(), announcement_id).await?;
        announcements.refresh();
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseSubTeamBroadcast {
        team_id: team_id_signal,
        status_filter,
        announcements,
        handle_create_draft,
        handle_update_draft,
        handle_publish,
        handle_delete,
    }))
}
