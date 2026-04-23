//! Controller for the parent-admin pending-applications queue.

use crate::common::hooks::{use_infinite_query, InfiniteQuery};
use crate::features::sub_team::controllers::{
    approve_application_handler, list_parent_applications_handler, reject_application_handler,
    return_application_handler,
};
use crate::features::sub_team::models::SubTeamApplicationStatus;
use crate::features::sub_team::types::{
    ApplicationDecisionReasonRequest, ApplicationReturnCommentRequest, SubTeamApplicationResponse,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamQueue {
    pub team_id: ReadSignal<TeamPartition>,
    pub status_filter: Signal<SubTeamApplicationStatus>,
    pub queue: InfiniteQuery<
        String,
        SubTeamApplicationResponse,
        ListResponse<SubTeamApplicationResponse>,
    >,
    pub handle_approve: Action<(String,), ()>,
    pub handle_reject: Action<(String, String), ()>,
    pub handle_return: Action<(String, String), ()>,
}

#[track_caller]
pub fn use_sub_team_queue() -> std::result::Result<UseSubTeamQueue, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamQueue>() {
        return Ok(ctx);
    }

    let team_id: TeamPartition = use_context();
    let team_id_signal: ReadSignal<TeamPartition> = use_signal(|| team_id).into();
    let status_filter = use_signal(|| SubTeamApplicationStatus::Pending);

    let mut queue = use_infinite_query(move |bookmark| {
        let id = team_id_signal();
        let status = status_filter();
        async move { list_parent_applications_handler(id, bookmark, Some(status)).await }
    })?;

    let team_id_for_approve = team_id_signal;
    let handle_approve = use_action(move |application_id: String| async move {
        approve_application_handler(team_id_for_approve(), application_id).await?;
        queue.refresh();
        Ok::<(), crate::common::Error>(())
    });

    let team_id_for_reject = team_id_signal;
    let handle_reject =
        use_action(move |application_id: String, reason: String| async move {
            reject_application_handler(
                team_id_for_reject(),
                application_id,
                ApplicationDecisionReasonRequest { reason },
            )
            .await?;
            queue.refresh();
            Ok::<(), crate::common::Error>(())
        });

    let team_id_for_return = team_id_signal;
    let handle_return =
        use_action(move |application_id: String, comment: String| async move {
            return_application_handler(
                team_id_for_return(),
                application_id,
                ApplicationReturnCommentRequest { comment },
            )
            .await?;
            queue.refresh();
            Ok::<(), crate::common::Error>(())
        });

    Ok(use_context_provider(|| UseSubTeamQueue {
        team_id: team_id_signal,
        status_filter,
        queue,
        handle_approve,
        handle_reject,
        handle_return,
    }))
}
