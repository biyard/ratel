//! Controller for the child-side "my applications" status page.

use crate::common::hooks::{use_infinite_query, InfiniteQuery};
use crate::features::sub_team::controllers::{
    cancel_child_application_handler, get_parent_relationship_handler,
    list_child_applications_handler,
};
use crate::features::sub_team::types::{ParentRelationshipResponse, SubTeamApplicationResponse};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamApplicationStatus {
    pub team_id: ReadSignal<TeamPartition>,
    pub relationship: Loader<ParentRelationshipResponse>,
    pub applications: InfiniteQuery<
        String,
        SubTeamApplicationResponse,
        ListResponse<SubTeamApplicationResponse>,
    >,
    pub handle_cancel: Action<(String,), ()>,
}

#[track_caller]
pub fn use_sub_team_application_status(
) -> std::result::Result<UseSubTeamApplicationStatus, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamApplicationStatus>() {
        return Ok(ctx);
    }

    let team_id: TeamPartition = use_context();
    let team_id_signal: ReadSignal<TeamPartition> = use_signal(|| team_id).into();

    let relationship = use_loader(move || {
        let id = team_id_signal();
        async move { get_parent_relationship_handler(id).await }
    })?;

    let mut applications = use_infinite_query(move |bookmark| {
        let id = team_id_signal();
        async move { list_child_applications_handler(id, bookmark).await }
    })?;

    let team_id_for_cancel = team_id_signal;
    let handle_cancel = use_action(move |application_id: String| async move {
        cancel_child_application_handler(team_id_for_cancel(), application_id).await?;
        applications.refresh();
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseSubTeamApplicationStatus {
        team_id: team_id_signal,
        relationship,
        applications,
        handle_cancel,
    }))
}
