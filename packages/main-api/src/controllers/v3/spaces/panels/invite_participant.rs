use crate::aide::NoApi;
use crate::controllers::v3::spaces::SpacePanelPath;
use crate::controllers::v3::spaces::SpacePanelPathParam;
use crate::features::spaces::panels::SpacePanel;
use crate::features::spaces::panels::SpacePanelParticipant;
use crate::features::spaces::panels::SpacePanelParticipantResponse;
use crate::features::spaces::panels::SpacePanelRequest;
use crate::features::spaces::panels::SpacePanelResponse;
use crate::models::SpaceCommon;
use crate::models::User;
use crate::types::EntityType;
use crate::types::Partition;
use crate::types::TeamGroupPermission;
use crate::{AppState, Error};
use bdk::prelude::axum::extract::{Json, Path, State};

pub async fn participate_panel_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePanelPathParam { space_pk, panel_pk }): SpacePanelPath,
) -> Result<Json<SpacePanelParticipantResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let (pk, sk) = SpacePanel::keys(&space_pk, &panel_pk);

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    let panel = SpacePanel::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;
    if panel.is_none() {
        return Err(Error::NotFoundPanel);
    }

    let panel = panel.unwrap();
    if panel.quotas == panel.participants {
        return Err(Error::AlreadyFullPanel);
    }

    let panel_pk = match panel.sk {
        EntityType::SpacePanel(v) => Partition::Panel(v.to_string()),
        _ => Partition::Panel("".to_string()),
    };

    let (pk, sk) = SpacePanelParticipant::keys(&panel_pk, &user.pk);

    let participant =
        SpacePanelParticipant::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;
    if participant.is_some() {
        return Err(Error::AlreadyParticipateUser);
    }

    // TODO: authorization users with DID

    let participant = SpacePanelParticipant::new(panel_pk.clone(), user);
    participant.create(&dynamo.client).await?;

    let (pk, sk) = SpacePanel::keys(&space_pk, &panel_pk);
    let _panel = SpacePanel::updater(&pk, sk)
        .increase_participants(1)
        .execute(&dynamo.client)
        .await?;

    let participant = participant.into();

    Ok(Json(participant))
}
