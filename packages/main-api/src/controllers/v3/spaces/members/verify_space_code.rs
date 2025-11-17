use crate::NoApi;
use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::did::VerifiedAttributes;
use crate::features::spaces::members::{
    InvitationStatus, SpaceEmailVerification, SpaceInvitationMember,
};
use crate::features::spaces::panels::{
    PanelAttribute, SpacePanelParticipant, SpacePanelQuota, SpacePanels,
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

    let panel = check_panel(&dynamo, &space, user.clone()).await;

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

pub async fn check_panel(dynamo: &DynamoClient, space: &SpaceCommon, user: User) -> bool {
    let space_pk = space.pk.clone();
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

    let pk = space_pk.clone();
    // let sk = EntityType::SpacePanels;

    let panel_quota = SpacePanelQuota::query(
        &dynamo.client,
        CompositePartition(pk.clone(), Partition::PanelAttribute),
        SpacePanelQuota::opt_all().sk("SPACE_PANEL_ATTRIBUTE#".to_string()),
    )
    .await
    .unwrap_or_default()
    .0;

    tracing::debug!("panel quota: {:?}", panel_quota.clone());

    if space.remains == 0 {
        return false;
    }

    for p in panel_quota {
        if p.remains == 0 {
            continue;
        }

        if match_by_sk(age.clone(), gender.clone(), &p.sk) {
            let participants = SpacePanelParticipant::new(space_pk.clone(), user);
            let _ = match participants.create(&dynamo.client).await {
                Ok(v) => v,
                Err(_) => return false,
            };
            let pk = p.pk;
            let sk = p.sk;

            let _ = match SpaceCommon::updater(space_pk, EntityType::SpaceCommon)
                .decrease_remains(1)
                .execute(&dynamo.client)
                .await
            {
                Ok(_) => {}
                Err(_) => return false,
            };

            let _ = match SpacePanelQuota::updater(pk, sk)
                .decrease_remains(1)
                .execute(&dynamo.client)
                .await
            {
                Ok(v) => v,
                Err(_) => return false,
            };

            return true;
        }
    }

    return false;
}

pub fn match_by_sk(age: Option<u8>, gender: Option<Gender>, sk: &EntityType) -> bool {
    let (label, value) = match sk {
        EntityType::SpacePanelAttribute(label, value) => (label.as_str(), value.as_str()),
        _ => return true,
    };

    match label {
        "verifiable_attribute" => match value {
            v if v.starts_with("age") => match_age_rule(age, v),
            v if v.starts_with("gender") => match_gender_rule(gender, v),
            _ => true,
        },
        "collective_attribute" => true,

        "none" | _ => true,
    }
}

fn match_age_rule(age: Option<u8>, v: &str) -> bool {
    if v == "age" {
        return age.is_some();
    }

    if let Some(rest) = v.strip_prefix("age:") {
        if let Some((min_s, max_s)) = rest.split_once('-') {
            if let (Ok(min), Ok(max)) = (min_s.trim().parse::<u8>(), max_s.trim().parse::<u8>()) {
                return age.map(|a| a >= min && a <= max).unwrap_or(false);
            }
        } else if let Ok(specific) = rest.trim().parse::<u8>() {
            return age.map(|a| a == specific).unwrap_or(false);
        }
    }

    true
}

fn match_gender_rule(gender: Option<Gender>, v: &str) -> bool {
    if v == "gender" {
        return gender.is_some();
    }

    // "gender:male" | "gender:female"
    if let Some(rest) = v.strip_prefix("gender:") {
        let want = rest.trim().to_ascii_lowercase();
        return match (want.as_str(), gender) {
            ("male", Some(Gender::Male)) => true,
            ("female", Some(Gender::Female)) => true,
            _ => false,
        };
    }

    true
}
