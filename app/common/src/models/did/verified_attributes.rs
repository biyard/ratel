use crate::{attribute::Gender, *};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(DynamoEntity))]
pub struct VerifiedAttributes {
    pub pk: CompositePartition,
    #[cfg_attr(
        feature = "server",
        dynamo(prefix = "TS", name = "find_by_info_prefix", index = "gsi1", pk)
    )]
    pub sk: EntityType,
    #[cfg_attr(feature = "server", dynamo(prefix = "INFO", index = "gsi1", sk))]
    pub birth_date: Option<String>, // YYYYMMDD
    pub gender: Option<Gender>,
    pub university: Option<String>,
}

impl VerifiedAttributes {
    pub fn new(user_pk: Partition) -> Self {
        if !matches!(user_pk, Partition::User(_)) {
            panic!("pk for VerifiedAttributes must be Partition::User");
        }

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

    #[cfg(feature = "server")]
    pub fn age(&self) -> Option<u32> {
        use chrono::{Datelike, Utc};

        let birth_date = self.birth_date.as_ref()?;
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
        if current_month < month || (current_month == month && current_day < day) {
            age = age.saturating_sub(1);
        }

        Some(age)
    }

    #[cfg(not(feature = "server"))]
    pub fn age(&self) -> Option<u32> {
        None
    }
}
