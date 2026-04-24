//! Controller for the sub-team announcement compose page (new + edit).
//!
//! Mirrors `use_sub_team_doc_compose` in shape: a loader that returns
//! `None` for the "new" flow and the existing row for "edit", plus
//! create/update/publish/delete actions.

use crate::features::sub_team::controllers::{
    create_announcement_handler, delete_announcement_handler, get_announcement_handler,
    publish_announcement_handler, update_announcement_handler,
};
use crate::features::sub_team::types::{
    CreateSubTeamAnnouncementRequest, SubTeamAnnouncementResponse,
    UpdateSubTeamAnnouncementRequest,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamBroadcastCompose {
    pub team_id: ReadSignal<TeamPartition>,
    pub announcement_id: ReadSignal<Option<String>>,
    pub announcement: Loader<Option<SubTeamAnnouncementResponse>>,

    pub handle_save_new: Action<(CreateSubTeamAnnouncementRequest,), String>,
    pub handle_save_existing:
        Action<(String, UpdateSubTeamAnnouncementRequest), ()>,
    pub handle_publish: Action<(String,), ()>,
    pub handle_delete: Action<(String,), ()>,
}

#[track_caller]
pub fn use_sub_team_broadcast_compose(
) -> std::result::Result<UseSubTeamBroadcastCompose, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamBroadcastCompose>() {
        return Ok(ctx);
    }

    let team_id: TeamPartition = use_context();
    let team_id_signal: ReadSignal<TeamPartition> = use_signal(|| team_id).into();
    let initial_announcement_id: Option<String> = try_consume_context().unwrap_or(None);
    let announcement_id_signal: ReadSignal<Option<String>> =
        use_signal(|| initial_announcement_id).into();

    let announcement = use_loader(move || {
        let id = team_id_signal();
        let announcement_id = announcement_id_signal();
        async move {
            let Some(announcement_id) = announcement_id else {
                return Ok::<Option<SubTeamAnnouncementResponse>, crate::common::Error>(None);
            };
            let row = get_announcement_handler(id, announcement_id).await?;
            Ok(Some(row))
        }
    })?;

    let team_id_for_new = team_id_signal;
    let handle_save_new =
        use_action(move |req: CreateSubTeamAnnouncementRequest| async move {
            let created = create_announcement_handler(team_id_for_new(), req).await?;
            Ok::<String, crate::common::Error>(created.id)
        });

    let team_id_for_update = team_id_signal;
    let handle_save_existing = use_action(
        move |announcement_id: String,
              req: UpdateSubTeamAnnouncementRequest| async move {
            update_announcement_handler(team_id_for_update(), announcement_id, req).await?;
            Ok::<(), crate::common::Error>(())
        },
    );

    let team_id_for_publish = team_id_signal;
    let handle_publish = use_action(move |announcement_id: String| async move {
        publish_announcement_handler(team_id_for_publish(), announcement_id).await?;
        Ok::<(), crate::common::Error>(())
    });

    let team_id_for_delete = team_id_signal;
    let handle_delete = use_action(move |announcement_id: String| async move {
        delete_announcement_handler(team_id_for_delete(), announcement_id).await?;
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseSubTeamBroadcastCompose {
        team_id: team_id_signal,
        announcement_id: announcement_id_signal,
        announcement,
        handle_save_new,
        handle_save_existing,
        handle_publish,
        handle_delete,
    }))
}
