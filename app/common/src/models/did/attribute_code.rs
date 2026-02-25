use crate::{attribute::Gender, *};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(DynamoEntity))]
pub struct AttributeCode {
    pub pk: Partition,
    pub sk: EntityType,
    pub created_at: i64,
    pub birth_date: Option<String>,
    pub gender: Option<Gender>,
    pub university: Option<String>,
}
