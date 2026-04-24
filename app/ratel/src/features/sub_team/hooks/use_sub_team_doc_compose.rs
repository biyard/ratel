//! Controller for the sub-team document compose page (new + edit).
//!
//! When `doc_id` is `None` the loader yields `None` (creating a new
//! document); when it's `Some`, the loader walks the parent's doc list
//! and finds the matching row. Save routes through the appropriate
//! create/update handler depending on whether the controller is in
//! "new" or "edit" mode.

use crate::features::sub_team::controllers::{
    create_sub_team_doc_handler, delete_sub_team_doc_handler, list_sub_team_docs_handler,
    update_sub_team_doc_handler,
};
use crate::features::sub_team::types::{
    CreateSubTeamDocumentRequest, SubTeamDocumentResponse, UpdateSubTeamDocumentRequest,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamDocCompose {
    pub team_id: ReadSignal<TeamPartition>,
    pub doc_id: ReadSignal<Option<String>>,
    pub doc: Loader<Option<SubTeamDocumentResponse>>,
    pub handle_save_new: Action<(CreateSubTeamDocumentRequest,), String>,
    pub handle_save_existing: Action<(String, UpdateSubTeamDocumentRequest), ()>,
    pub handle_delete: Action<(String,), ()>,
}

#[track_caller]
pub fn use_sub_team_doc_compose() -> std::result::Result<UseSubTeamDocCompose, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamDocCompose>() {
        return Ok(ctx);
    }

    let team_id: TeamPartition = use_context();
    let team_id_signal: ReadSignal<TeamPartition> = use_signal(|| team_id).into();
    // doc_id optional — seeded by the route-level prop via context, or
    // None for "new document" flow.
    let initial_doc_id: Option<String> = try_consume_context().unwrap_or(None);
    let doc_id_signal: ReadSignal<Option<String>> = use_signal(|| initial_doc_id).into();

    let doc = use_loader(move || {
        let id = team_id_signal();
        let doc_id = doc_id_signal();
        async move {
            let Some(doc_id) = doc_id else {
                return Ok::<Option<SubTeamDocumentResponse>, crate::common::Error>(None);
            };
            let list = list_sub_team_docs_handler(id).await?;
            Ok(list.items.into_iter().find(|d| d.id == doc_id))
        }
    })?;

    let team_id_for_new = team_id_signal;
    let handle_save_new =
        use_action(move |req: CreateSubTeamDocumentRequest| async move {
            let created = create_sub_team_doc_handler(team_id_for_new(), req).await?;
            Ok::<String, crate::common::Error>(created.id)
        });

    let team_id_for_update = team_id_signal;
    let handle_save_existing = use_action(
        move |doc_id: String, req: UpdateSubTeamDocumentRequest| async move {
            update_sub_team_doc_handler(team_id_for_update(), doc_id, req).await?;
            Ok::<(), crate::common::Error>(())
        },
    );

    let team_id_for_delete = team_id_signal;
    let handle_delete = use_action(move |doc_id: String| async move {
        delete_sub_team_doc_handler(team_id_for_delete(), doc_id).await?;
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseSubTeamDocCompose {
        team_id: team_id_signal,
        doc_id: doc_id_signal,
        doc,
        handle_save_new,
        handle_save_existing,
        handle_delete,
    }))
}
