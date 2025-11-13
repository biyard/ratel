use crate::features::did::{VerifiableAttribute, VerifiedAttributes};
use crate::features::spaces::panels::{CollectiveAttribute, PanelAttribute};
use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema, Default)]
pub struct SpacePanels {
    pub pk: Partition,
    pub sk: EntityType,

    pub quotas: i64,
    pub remains: i64,
    pub attributes: Vec<PanelAttribute>,
}

impl SpacePanels {
    pub fn new(space_pk: Partition, quotas: i64, attributes: Vec<PanelAttribute>) -> Self {
        Self {
            pk: space_pk,
            sk: EntityType::SpacePanels,
            quotas,
            remains: quotas,
            attributes,
        }
    }
}

impl PartialEq<VerifiedAttributes> for SpacePanels {
    fn eq(&self, other: &VerifiedAttributes) -> bool {
        for p in self.attributes.iter() {
            match p {
                PanelAttribute::CollectiveAttribute(CollectiveAttribute::University) => {
                    if other.university.is_none() {
                        return false;
                    }
                }
                PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender) => {
                    if other.gender.is_none() {
                        return false;
                    }
                }
                PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age) => {
                    if other.birth_date.is_none() {
                        return false;
                    }
                }
                PanelAttribute::CollectiveAttribute(CollectiveAttribute::None) => {}

                PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(Age::Specific(
                    age,
                ))) => {
                    let age = age.clone() as u32;
                    if other.age().unwrap_or_default() != age {
                        return false;
                    }
                }
                PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(Age::Range {
                    inclusive_max,
                    inclusive_min,
                })) => {
                    let age = other.age().unwrap_or_default() as u8;
                    if age < *inclusive_min || age > *inclusive_max {
                        return false;
                    }
                }

                PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_gender)) => {
                    let Some(gender) = other.gender else {
                        return false;
                    };
                    if &gender != _gender {
                        return false;
                    }
                }
                PanelAttribute::VerifiableAttribute(VerifiableAttribute::Generation(
                    _generation,
                )) => {
                    todo!()
                }
                PanelAttribute::VerifiableAttribute(VerifiableAttribute::IsAdult(_is_adult)) => {
                    if other.age().unwrap_or_default() < 19 {
                        return false;
                    }
                }
                PanelAttribute::VerifiableAttribute(VerifiableAttribute::None) => {}
                PanelAttribute::None => {}
            }
        }

        true
    }
}
