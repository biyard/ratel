use crate::types::*;
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
