use crate::types::*;
use crate::utils::aws::DynamoClient;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct VerifiedAttributes {
    pub pk: CompositePartition,

    #[dynamo(prefix = "TS", name = "find_by_info_prefix", index = "gsi1", pk)]
    pub sk: EntityType,

    #[dynamo(prefix = "INFO", index = "gsi1", sk)]
    pub birth_date: Option<String>, // YYYYMMDD
    pub gender: Option<Gender>,
    pub university: Option<String>,
}

impl VerifiedAttributes {
    pub fn new(user_pk: Partition) -> Self {
        if !matches!(user_pk, Partition::User(_)) {
            panic!("pk for VerifiedAttributes must be Partition::User");
        };

        Self {
            pk: CompositePartition(user_pk, Partition::Attributes),
            sk: EntityType::VerifiedAttributes,
            ..Default::default()
        }
    }

    pub fn keys(user_pk: &Partition) -> (CompositePartition, EntityType) {
        (
            CompositePartition(user_pk.clone(), Partition::Attributes),
            EntityType::VerifiedAttributes,
        )
    }

    pub async fn get_attributes(
        dynamo: &DynamoClient,
        user_pk: Partition,
    ) -> Result<Option<RespondentAttr>> {
        let res = VerifiedAttributes::get(
            &dynamo.client,
            CompositePartition(user_pk, Partition::Attributes),
            None::<String>,
        )
        .await
        .unwrap_or_default()
        .unwrap_or(VerifiedAttributes::default());

        let age = if res.age().is_none() {
            None
        } else {
            match res.age().unwrap_or_default() {
                0..=17 => Some(Age::Range {
                    inclusive_max: 17,
                    inclusive_min: 0,
                }),
                18..=29 => Some(Age::Range {
                    inclusive_max: 29,
                    inclusive_min: 18,
                }),
                30..=39 => Some(Age::Range {
                    inclusive_max: 39,
                    inclusive_min: 30,
                }),
                40..=49 => Some(Age::Range {
                    inclusive_max: 49,
                    inclusive_min: 40,
                }),
                50..=59 => Some(Age::Range {
                    inclusive_max: 59,
                    inclusive_min: 50,
                }),
                60..=69 => Some(Age::Range {
                    inclusive_max: 69,
                    inclusive_min: 60,
                }),
                _ => Some(Age::Range {
                    inclusive_max: 100,
                    inclusive_min: 70,
                }),
            }
        };

        let gender = res.gender;
        let school = res.university;

        Ok(Some(RespondentAttr {
            age,
            gender,
            school,
        }))
    }

    pub fn age(&self) -> Option<u32> {
        use chrono::{Datelike, Utc};

        if self.birth_date.is_none() {
            return None;
        }

        let birth_date = self.birth_date.clone().unwrap();

        if birth_date.len() != 8 {
            return None;
        }

        let year = birth_date[0..4].parse::<i32>().ok()?;
        let month = birth_date[4..6].parse::<u32>().ok()?;
        let day = birth_date[6..8].parse::<u32>().ok()?;

        let now = Utc::now();
        let current_year = now.year();
        let current_month = now.month();
        let current_day = now.day();

        let mut age = (current_year - year) as u32;

        // Adjust if birthday hasn't occurred yet this year
        if current_month < month || (current_month == month && current_day < day) {
            age -= 1;
        }

        Some(age)
    }
}
