use crate::NoApi;
use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::did::VerifiedAttributes;
use crate::features::spaces::members::{
    InvitationStatus, SpaceEmailVerification, SpaceInvitationMember,
};
use crate::features::spaces::panels::{
    SpacePanel, SpacePanelParticipant, SpacePanelQueryOption, SpacePanelResponse,
};
use crate::models::{SpaceCommon, User};
use crate::types::{
    Age, Attribute, CompositePartition, EntityType, Gender, Partition, SpacePublishState,
    SpaceStatus, SpaceVisibility,
};
use crate::utils::aws::DynamoClient;
use crate::{
    AppState, Error,
    constants::MAX_ATTEMPT_COUNT,
    models::email::{EmailVerification, EmailVerificationQueryOption},
    utils::time::get_now_timestamp,
};
use bdk::prelude::*;
use by_axum::axum::extract::{Json, Path, State};
use serde::Deserialize;

#[derive(Debug, Clone, serde::Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct VerifySpaceCodeRequest {}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct VerifySpaceCodeResponse {
    #[schemars(description = "Indicates if the verification was successful.")]
    pub success: bool,
}

pub async fn verify_space_code_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(_req): Json<VerifySpaceCodeRequest>,
) -> Result<Json<VerifySpaceCodeResponse>, Error> {
    if user.is_none() {
        return Ok(Json(VerifySpaceCodeResponse { success: false }));
    }

    let user = user.unwrap_or_default();

    let space = SpaceCommon::get(
        &dynamo.client,
        space_pk.clone(),
        Some(EntityType::SpaceCommon),
    )
    .await?
    .ok_or(Error::SpaceNotFound)?;

    if space.status == Some(SpaceStatus::Started) || space.status == Some(SpaceStatus::Finished) {
        return Err(Error::FinishedSpace);
    }

    let panel = check_panel(&dynamo, &space_pk, user.clone()).await;

    if !panel {
        return Ok(Json(VerifySpaceCodeResponse { success: false }));
    }

    if space.visibility != SpaceVisibility::Public {
        let (pk, sk) = SpaceInvitationMember::keys(&space.pk, &user.pk);
        tracing::debug!("verification pk: {:?}, sk: {:?}", pk, sk);

        let member =
            SpaceInvitationMember::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;

        if member.is_none() {
            return Ok(Json(VerifySpaceCodeResponse { success: false }));
        }

        let _ = SpaceInvitationMember::updater(pk, sk)
            .with_status(InvitationStatus::Accepted)
            .execute(&dynamo.client)
            .await
            .unwrap_or_default();

        if space.publish_state != SpacePublishState::Published {
            return Ok(Json(VerifySpaceCodeResponse { success: false }));
        }
    }

    Ok(Json(VerifySpaceCodeResponse { success: true }))
}

pub async fn check_panel(dynamo: &DynamoClient, space_pk: &Partition, user: User) -> bool {
    let res = VerifiedAttributes::get(
        &dynamo.client,
        CompositePartition(user.pk.clone(), Partition::Attributes),
        None::<String>,
    )
    .await
    .unwrap_or_default()
    .unwrap_or(VerifiedAttributes::default());

    let age: Option<u8> = res.age().and_then(|v| u8::try_from(v).ok());
    let gender = res.gender;

    let mut bookmark = None::<String>;

    loop {
        let opt = match &bookmark {
            Some(b) => SpacePanelQueryOption::builder()
                .sk("SPACE_PANEL#".into())
                .bookmark(b.clone()),
            None => SpacePanelQueryOption::builder().sk("SPACE_PANEL#".into()),
        };

        let (panels, next) = match SpacePanel::query(&dynamo.client, space_pk.clone(), opt).await {
            Ok(v) => v,
            Err(_) => return false,
        };

        for p in panels {
            if p.participants >= p.quotas {
                continue;
            }
            if attributes_match(age.clone(), gender.clone(), &p.attributes) {
                let res: SpacePanelResponse = p.into();
                let participants =
                    SpacePanelParticipant::new(space_pk.clone(), res.clone().pk, user);
                let _ = match participants.create(&dynamo.client).await {
                    Ok(v) => v,
                    Err(_) => return false,
                };
                let (pk, sk) = SpacePanel::keys(&space_pk, &res.pk);
                let _ = match SpacePanel::updater(pk, sk)
                    .increase_participants(1)
                    .execute(&dynamo.client)
                    .await
                {
                    Ok(v) => v,
                    Err(_) => return false,
                };

                return true;
            }
        }

        if let Some(b) = next {
            bookmark = Some(b);
        } else {
            break;
        }
    }

    return false;
}

fn attributes_match(age: Option<u8>, gender: Option<Gender>, attrs: &[Attribute]) -> bool {
    if attrs.is_empty() {
        return true;
    }

    let mut age_rules: Vec<&Age> = Vec::new();
    let mut gender_rules: Vec<&Gender> = Vec::new();

    for attr in attrs {
        match attr {
            Attribute::Age(a) => age_rules.push(a),
            Attribute::Gender(g) => gender_rules.push(g),
        }
    }

    let age_ok = if age_rules.is_empty() {
        true
    } else if let Some(a) = age {
        age_rules.iter().any(|rule| match rule {
            Age::Specific(s) => a == *s,
            Age::Range {
                inclusive_min,
                inclusive_max,
            } => a >= *inclusive_min && a <= *inclusive_max,
        })
    } else {
        true
    };

    let gender_ok = if gender_rules.is_empty() {
        true
    } else {
        match gender {
            Some(ref g) => gender_rules.iter().any(|rule| *rule == g),
            None => true,
        }
    };

    age_ok && gender_ok
}
