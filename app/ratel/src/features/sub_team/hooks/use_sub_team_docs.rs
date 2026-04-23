//! Controller for the parent-admin documents (bylaws/regulations) tab.

use crate::features::sub_team::controllers::{
    create_sub_team_doc_handler, delete_sub_team_doc_handler, list_sub_team_docs_handler,
    reorder_sub_team_docs_handler, update_sub_team_doc_handler,
};
use crate::features::sub_team::types::{
    CreateSubTeamDocumentRequest, ReorderDocumentsRequest, SubTeamDocumentResponse,
    UpdateSubTeamDocumentRequest,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamDocs {
    pub team_id: ReadSignal<TeamPartition>,
    pub docs: Loader<ListResponse<SubTeamDocumentResponse>>,
    pub handle_create: Action<(CreateSubTeamDocumentRequest,), ()>,
    pub handle_update: Action<(String, UpdateSubTeamDocumentRequest), ()>,
    pub handle_delete: Action<(String,), ()>,
    pub handle_reorder: Action<(Vec<String>,), ()>,
}

#[track_caller]
pub fn use_sub_team_docs() -> std::result::Result<UseSubTeamDocs, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamDocs>() {
        return Ok(ctx);
    }

    let team_id: TeamPartition = use_context();
    let team_id_signal: ReadSignal<TeamPartition> = use_signal(|| team_id).into();

    let mut docs = use_loader(move || {
        let id = team_id_signal();
        async move { list_sub_team_docs_handler(id).await }
    })?;

    let team_id_for_create = team_id_signal;
    let handle_create =
        use_action(move |req: CreateSubTeamDocumentRequest| async move {
            create_sub_team_doc_handler(team_id_for_create(), req).await?;
            docs.restart();
            Ok::<(), crate::common::Error>(())
        });

    let team_id_for_update = team_id_signal;
    let handle_update = use_action(
        move |doc_id: String, req: UpdateSubTeamDocumentRequest| async move {
            update_sub_team_doc_handler(team_id_for_update(), doc_id, req).await?;
            docs.restart();
            Ok::<(), crate::common::Error>(())
        },
    );

    let team_id_for_delete = team_id_signal;
    let handle_delete = use_action(move |doc_id: String| async move {
        delete_sub_team_doc_handler(team_id_for_delete(), doc_id).await?;
        docs.restart();
        Ok::<(), crate::common::Error>(())
    });

    let team_id_for_reorder = team_id_signal;
    let handle_reorder = use_action(move |doc_ids: Vec<String>| async move {
        reorder_sub_team_docs_handler(
            team_id_for_reorder(),
            ReorderDocumentsRequest { doc_ids },
        )
        .await?;
        docs.restart();
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseSubTeamDocs {
        team_id: team_id_signal,
        docs,
        handle_create,
        handle_update,
        handle_delete,
        handle_reorder,
    }))
}
