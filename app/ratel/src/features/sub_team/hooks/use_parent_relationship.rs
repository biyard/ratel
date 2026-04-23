//! Lightweight controller exposing the child-team's parent relationship
//! plus the "leave parent" action. Used by the parent HUD, the leave-parent
//! confirmation page, and anywhere else that needs just the relationship
//! summary without the full application history.

use crate::features::sub_team::controllers::{
    get_parent_relationship_handler, leave_parent_handler,
};
use crate::features::sub_team::types::{LeaveParentRequest, ParentRelationshipResponse};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseParentRelationship {
    pub team_id: ReadSignal<TeamPartition>,
    pub relationship: Loader<ParentRelationshipResponse>,
    pub handle_leave_parent: Action<(LeaveParentRequest,), ()>,
}

#[track_caller]
pub fn use_parent_relationship() -> std::result::Result<UseParentRelationship, RenderError> {
    if let Some(ctx) = try_use_context::<UseParentRelationship>() {
        return Ok(ctx);
    }

    let team_id: TeamPartition = use_context();
    let team_id_signal: ReadSignal<TeamPartition> = use_signal(|| team_id).into();

    let mut relationship = use_loader(move || {
        let id = team_id_signal();
        async move { get_parent_relationship_handler(id).await }
    })?;

    let team_id_for_leave = team_id_signal;
    let handle_leave_parent = use_action(move |req: LeaveParentRequest| async move {
        leave_parent_handler(team_id_for_leave(), req).await?;
        relationship.restart();
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseParentRelationship {
        team_id: team_id_signal,
        relationship,
        handle_leave_parent,
    }))
}
