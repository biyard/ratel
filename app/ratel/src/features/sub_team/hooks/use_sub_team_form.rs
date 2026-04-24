//! Controller for the parent-admin application-form editor tab.

use crate::features::sub_team::controllers::{
    create_sub_team_form_field_handler, delete_sub_team_form_field_handler,
    list_sub_team_form_fields_handler, reorder_sub_team_form_fields_handler,
    update_sub_team_form_field_handler,
};
use crate::features::sub_team::types::{
    CreateSubTeamFormFieldRequest, ReorderFormFieldsRequest, SubTeamFormFieldResponse,
    UpdateSubTeamFormFieldRequest,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamForm {
    pub team_id: ReadSignal<TeamPartition>,
    pub fields: Loader<ListResponse<SubTeamFormFieldResponse>>,
    pub handle_create_field: Action<(CreateSubTeamFormFieldRequest,), ()>,
    pub handle_update_field: Action<(String, UpdateSubTeamFormFieldRequest), ()>,
    pub handle_delete_field: Action<(String,), ()>,
    pub handle_reorder: Action<(Vec<String>,), ()>,
}

#[track_caller]
pub fn use_sub_team_form() -> std::result::Result<UseSubTeamForm, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamForm>() {
        return Ok(ctx);
    }

    let team_id: TeamPartition = use_context();
    let team_id_signal: ReadSignal<TeamPartition> = use_signal(|| team_id).into();

    let mut fields = use_loader(move || {
        let id = team_id_signal();
        async move { list_sub_team_form_fields_handler(id).await }
    })?;

    let team_id_for_create = team_id_signal;
    let handle_create_field =
        use_action(move |req: CreateSubTeamFormFieldRequest| async move {
            create_sub_team_form_field_handler(team_id_for_create(), req).await?;
            fields.restart();
            Ok::<(), crate::common::Error>(())
        });

    let team_id_for_update = team_id_signal;
    let handle_update_field = use_action(
        move |field_id: String, req: UpdateSubTeamFormFieldRequest| async move {
            update_sub_team_form_field_handler(team_id_for_update(), field_id, req).await?;
            fields.restart();
            Ok::<(), crate::common::Error>(())
        },
    );

    let team_id_for_delete = team_id_signal;
    let handle_delete_field = use_action(move |field_id: String| async move {
        delete_sub_team_form_field_handler(team_id_for_delete(), field_id).await?;
        fields.restart();
        Ok::<(), crate::common::Error>(())
    });

    let team_id_for_reorder = team_id_signal;
    let handle_reorder = use_action(move |field_ids: Vec<String>| async move {
        reorder_sub_team_form_fields_handler(
            team_id_for_reorder(),
            ReorderFormFieldsRequest { field_ids },
        )
        .await?;
        fields.restart();
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseSubTeamForm {
        team_id: team_id_signal,
        fields,
        handle_create_field,
        handle_update_field,
        handle_delete_field,
        handle_reorder,
    }))
}
