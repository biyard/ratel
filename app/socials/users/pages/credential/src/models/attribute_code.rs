use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(DynamoEntity))]
pub struct AttributeCodeLocal {
    pub pk: Partition,
    pub sk: EntityType,
    pub created_at: i64,
    pub birth_date: Option<String>,
    pub gender: Option<String>,
    pub university: Option<String>,
}
