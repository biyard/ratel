//! Controller for the child-side "apply to be a sub-team" form.
//!
//! The applying team id (`team_id`) is the CHILD team making the request —
//! submission goes through `POST /api/teams/:team_pk/parent/applications`.
//! The parent team id is user-selected inside the form and lives as a
//! signal on the controller, so the apply page can pick a parent and
//! fetch its public apply-context independently.

use crate::features::sub_team::controllers::{
    get_sub_team_apply_context_handler, submit_application_handler,
};
use crate::features::sub_team::types::{
    ApplyContextResponse, DocAgreementInput, SubmitApplicationRequest,
};
use crate::*;
use std::collections::HashMap;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamApply {
    /// The applying (child) team id from the route.
    pub team_id: ReadSignal<TeamPartition>,
    /// The user-picked parent team id. Empty when nothing is selected —
    /// the loader short-circuits with an empty default context.
    pub parent_team_id: Signal<String>,
    pub apply_context: Loader<ApplyContextResponse>,
    /// Form values keyed by `field_id`. Populated/edited by the UI.
    pub form_values: Signal<HashMap<String, serde_json::Value>>,
    /// Doc ids the user has explicitly agreed to in the current session.
    pub agreed_doc_ids: Signal<HashMap<String, String>>,

    pub handle_submit: Action<(), String>,
}

#[track_caller]
pub fn use_sub_team_apply() -> std::result::Result<UseSubTeamApply, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamApply>() {
        return Ok(ctx);
    }

    let team_id: TeamPartition = use_context();
    let team_id_signal: ReadSignal<TeamPartition> = use_signal(|| team_id).into();
    let parent_team_id: Signal<String> = use_signal(String::new);

    let apply_context = use_loader(move || {
        let parent = parent_team_id();
        async move {
            if parent.is_empty() {
                return Ok(ApplyContextResponse::default());
            }
            get_sub_team_apply_context_handler(TeamPartition(parent)).await
        }
    })?;

    let form_values: Signal<HashMap<String, serde_json::Value>> = use_signal(HashMap::new);
    // doc_id → body_hash the user agreed to.
    let agreed_doc_ids: Signal<HashMap<String, String>> = use_signal(HashMap::new);

    let team_id_for_submit = team_id_signal;
    let handle_submit = use_action(move || async move {
        let parent = parent_team_id();
        let values = form_values.read().clone();
        let agreements: Vec<DocAgreementInput> = agreed_doc_ids
            .read()
            .iter()
            .map(|(doc_id, body_hash)| DocAgreementInput {
                doc_id: doc_id.clone(),
                body_hash: body_hash.clone(),
            })
            .collect();
        let req = SubmitApplicationRequest {
            parent_team_id: parent,
            form_values: values,
            doc_agreements: agreements,
        };
        let resp = submit_application_handler(team_id_for_submit(), req).await?;
        Ok::<String, crate::common::Error>(resp.id)
    });

    Ok(use_context_provider(|| UseSubTeamApply {
        team_id: team_id_signal,
        parent_team_id,
        apply_context,
        form_values,
        agreed_doc_ids,
        handle_submit,
    }))
}
