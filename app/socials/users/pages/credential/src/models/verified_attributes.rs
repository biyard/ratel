use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(DynamoEntity))]
pub struct VerifiedAttributesLocal {
    pub pk: CompositePartition,
    pub sk: EntityType,
    pub birth_date: Option<String>,
    pub gender: Option<String>,
    pub university: Option<String>,
}

impl VerifiedAttributesLocal {
    pub fn keys(user_pk: &Partition) -> (CompositePartition, EntityType) {
        (
            CompositePartition(user_pk.clone(), Partition::Attributes),
            EntityType::VerifiedAttributes,
        )
    }
}

#[cfg(feature = "server")]
impl VerifiedAttributesLocal {
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
}
