use crate::common::attribute::Gender;
use crate::common::models::auth::{OptionalUser, User};
use crate::features::spaces::models::{
    CollectiveAttribute, PanelAttribute, SpacePanelQuota, VerifiableAttribute,
};
use crate::features::spaces::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::features::spaces::models::verified_attributes::UserAttributesExt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PanelRequirementKind {
    Age,
    Gender,
    University,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PanelRequirementStatus {
    pub kind: PanelRequirementKind,
    pub satisfied: bool,
}

#[get("/api/spaces/{space_id}/panel-requirements", user: OptionalUser)]
pub async fn get_panel_requirements(
    space_id: SpacePartition,
) -> crate::common::Result<Vec<PanelRequirementStatus>> {
    let config = crate::features::spaces::config::get();
    let dynamo = config.common.dynamodb();
    let space_pk: Partition = space_id.into();
    let panel_quotas = list_panel_quotas(dynamo, &space_pk)
        .await?
        .into_iter()
        .filter(|panel| panel.remains > 0)
        .collect::<Vec<_>>();

    if panel_quotas.is_empty() {
        return Ok(vec![]);
    }

    let user: Option<User> = user.into();
    let (age, gender, has_university) = if let Some(user) = user {
        let user_attributes = user.get_attributes(dynamo).await?;
        (
            user_attributes
                .age()
                .and_then(|value| u8::try_from(value).ok()),
            user_attributes.gender,
            user_attributes
                .university
                .as_ref()
                .map(|value| !value.is_empty())
                .unwrap_or(false),
        )
    } else {
        (None, None, false)
    };

    let mut statuses = vec![];
    for kind in [
        PanelRequirementKind::Age,
        PanelRequirementKind::Gender,
        PanelRequirementKind::University,
    ] {
        let active_attributes = panel_quotas
            .iter()
            .flat_map(panel_attributes)
            .filter(|attribute| panel_requirement_kind(attribute) == Some(kind))
            .collect::<Vec<_>>();

        if active_attributes.is_empty() {
            continue;
        }

        let satisfied = active_attributes
            .iter()
            .any(|attribute| matches_panel_attribute(age, gender, has_university, attribute));

        statuses.push(PanelRequirementStatus { kind, satisfied });
    }

    Ok(statuses)
}

#[cfg(feature = "server")]
pub(crate) async fn list_panel_quotas(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> crate::common::Result<Vec<SpacePanelQuota>> {
    let (panel_quotas, _) = SpacePanelQuota::query(
        cli,
        CompositePartition(space_pk.clone(), Partition::PanelAttribute),
        SpacePanelQuota::opt_all().sk("SPACE_PANEL_ATTRIBUTE#".to_string()),
    )
    .await?;

    Ok(panel_quotas
        .into_iter()
        .filter(|panel| !matches!(panel.attributes, PanelAttribute::None))
        .collect())
}

#[cfg(feature = "server")]
pub(crate) fn panel_matches_user(
    age: Option<u8>,
    gender: Option<Gender>,
    has_university: bool,
    panel: &SpacePanelQuota,
) -> bool {
    panel_attributes(panel)
        .into_iter()
        .all(|attribute| matches_panel_attribute(age, gender, has_university, &attribute))
}

#[cfg(feature = "server")]
pub(crate) fn panel_attributes(panel: &SpacePanelQuota) -> Vec<PanelAttribute> {
    if panel.attributes_vec.is_empty() && !matches!(panel.attributes, PanelAttribute::None) {
        vec![panel.attributes]
    } else {
        panel.attributes_vec.clone()
    }
}

#[cfg(feature = "server")]
fn panel_requirement_kind(attribute: &PanelAttribute) -> Option<PanelRequirementKind> {
    match attribute {
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age)
        | PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_)) => {
            Some(PanelRequirementKind::Age)
        }
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender)
        | PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_)) => {
            Some(PanelRequirementKind::Gender)
        }
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::University) => {
            Some(PanelRequirementKind::University)
        }
        _ => None,
    }
}

#[cfg(feature = "server")]
fn matches_panel_attribute(
    age: Option<u8>,
    gender: Option<Gender>,
    has_university: bool,
    attribute: &PanelAttribute,
) -> bool {
    match attribute {
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::University) => has_university,
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age) => age.is_some(),
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender) => gender.is_some(),
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(
            crate::common::attribute::Age::Specific(value),
        )) => age.map(|age| age == *value).unwrap_or(false),
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(
            crate::common::attribute::Age::Range {
                inclusive_min,
                inclusive_max,
            },
        )) => age
            .map(|age| age >= *inclusive_min && age <= *inclusive_max)
            .unwrap_or(false),
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(expected)) => {
            gender.map(|value| value == *expected).unwrap_or(false)
        }
        _ => false,
    }
}
