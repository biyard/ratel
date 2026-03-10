use crate::common::attribute::Gender;
use crate::common::models::auth::{OptionalUser, User};
use crate::features::spaces::models::{
    CollectiveAttribute, PanelAttribute, SpacePanelQuota, VerifiableAttribute,
};
use crate::features::spaces::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use std::collections::BTreeSet;

#[cfg(feature = "server")]
use crate::features::spaces::models::verified_attributes::UserAttributesExt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PanelRequirementAttribute {
    Age,
    Gender,
    University,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PanelRequirementStatus {
    pub attribute: PanelRequirementAttribute,
    pub satisfied: bool,
    pub required_values: Vec<String>,
    pub current_value: Option<String>,
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
    let (age, gender, university, has_university) = if let Some(user) = user {
        let user_attributes = user.get_attributes(dynamo).await?;
        (
            user_attributes
                .age()
                .and_then(|value| u8::try_from(value).ok()),
            user_attributes.gender,
            user_attributes.university.clone(),
            user_attributes
                .university
                .as_ref()
                .map(|value| !value.is_empty())
                .unwrap_or(false),
        )
    } else {
        (None, None, None, false)
    };

    let mut statuses = vec![];
    for attribute in [
        PanelRequirementAttribute::Age,
        PanelRequirementAttribute::Gender,
        PanelRequirementAttribute::University,
    ] {
        let active_attributes = panel_quotas
            .iter()
            .flat_map(panel_attributes)
            .filter(|panel_attribute| {
                panel_requirement_attribute(panel_attribute) == Some(attribute)
            })
            .collect::<Vec<_>>();

        if active_attributes.is_empty() {
            continue;
        }

        let required_values = active_attributes
            .iter()
            .filter_map(panel_requirement_value)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let satisfied = active_attributes
            .iter()
            .any(|attribute| matches_panel_attribute(age, gender, has_university, attribute));

        let current_value = if satisfied {
            active_attributes
                .iter()
                .find(|attr| matches_panel_attribute(age, gender, has_university, attr))
                .and_then(panel_requirement_value)
        } else {
            current_value_for_attribute(attribute, age, gender, university.clone())
        };

        statuses.push(PanelRequirementStatus {
            attribute,
            satisfied,
            required_values,
            current_value,
        });
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
fn panel_requirement_attribute(attribute: &PanelAttribute) -> Option<PanelRequirementAttribute> {
    match attribute {
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age)
        | PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_)) => {
            Some(PanelRequirementAttribute::Age)
        }
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender)
        | PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_)) => {
            Some(PanelRequirementAttribute::Gender)
        }
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::University) => {
            Some(PanelRequirementAttribute::University)
        }
        _ => None,
    }
}

#[cfg(feature = "server")]
fn panel_requirement_value(attribute: &PanelAttribute) -> Option<String> {
    match attribute {
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::University)
        | PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age)
        | PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender) => {
            Some("Verified".to_string())
        }
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(
            crate::common::attribute::Age::Specific(value),
        )) => Some(value.to_string()),
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(
            crate::common::attribute::Age::Range {
                inclusive_min,
                inclusive_max,
            },
        )) if *inclusive_max == u8::MAX => Some(format!("{inclusive_min}+")),
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(
            crate::common::attribute::Age::Range {
                inclusive_min,
                inclusive_max,
            },
        )) => Some(format!("{inclusive_min}-{inclusive_max}")),
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(gender)) => {
            Some(gender.to_string())
        }
        _ => None,
    }
}

#[cfg(feature = "server")]
fn current_value_for_attribute(
    attribute: PanelRequirementAttribute,
    age: Option<u8>,
    gender: Option<Gender>,
    university: Option<String>,
) -> Option<String> {
    match attribute {
        PanelRequirementAttribute::Age => age.map(|value| value.to_string()),
        PanelRequirementAttribute::Gender => gender.map(|value| value.to_string()),
        PanelRequirementAttribute::University => university.filter(|value| !value.is_empty()),
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
