//! Controller for the child-side "my application status" page.
//!
//! Route: `/{parent_username}/sub-teams/application`. The URL's
//! `:username` is the **parent** team — semantically "the status of my
//! application TO this parent team". The hook reads the parent's
//! `TeamPartition` from context (provided by the page after a
//! `find_team_handler` lookup) and calls
//! `find_my_application_for_parent_handler`, which walks the viewer's
//! admin/owner teams to locate the one application targeting this
//! parent. Cancel still mutates by the applicant team's pk that lives
//! on the returned application DTO.

use crate::features::sub_team::controllers::{
    cancel_child_application_handler, find_my_application_for_parent_handler,
};
use crate::features::sub_team::types::SubTeamApplicationResponse;
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamApplicationStatus {
    /// Parent team's pk — taken from URL via the page-provided context.
    pub parent_team_id: ReadSignal<TeamPartition>,
    /// The viewer's application targeting this parent. `None` means
    /// the viewer's admin teams have never applied here.
    pub application: Loader<Option<SubTeamApplicationResponse>>,
    /// Cancels the in-flight application. Idempotent — succeeds even
    /// if the application is already in a terminal state.
    pub handle_cancel: Action<(), ()>,
}

#[track_caller]
pub fn use_sub_team_application_status(
) -> std::result::Result<UseSubTeamApplicationStatus, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamApplicationStatus>() {
        return Ok(ctx);
    }

    let parent_team_id: TeamPartition = use_context();
    let parent_team_id_signal: ReadSignal<TeamPartition> = use_signal(|| parent_team_id).into();

    let application = use_loader(move || {
        let parent_pk = parent_team_id_signal();
        async move { find_my_application_for_parent_handler(parent_pk).await }
    })?;

    // Cancel uses the applicant team's pk (carried on the returned
    // application DTO via `sub_team_id`) — the parent only stores
    // its own perspective. If there's no application yet, the action
    // is a no-op.
    let mut application_for_cancel = application;
    let handle_cancel = use_action(move || async move {
        let Some(app) = application_for_cancel.read().clone() else {
            return Ok::<(), crate::common::Error>(());
        };
        let applicant_pk = TeamPartition(app.sub_team_id.clone());
        cancel_child_application_handler(applicant_pk, app.id.clone()).await?;
        application_for_cancel.restart();
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseSubTeamApplicationStatus {
        parent_team_id: parent_team_id_signal,
        application,
        handle_cancel,
    }))
}
