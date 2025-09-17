use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct Space {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    #[dynamo(prefix = "TS", index = "gsi2", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "NAME", name = "find_by_name", index = "gsi1", pk)]
    pub name: String,
    #[dynamo(prefix = "OWNER", name = "find_by_owner", index = "gsi2", pk)]
    pub owner_id: String,

    pub display_name: String,
    pub description: String,
    pub image_url: String,
    pub banner_url: String,

    pub member_count: i64,
    pub post_count: i64,

    pub tags: Vec<String>,
    pub rules: String,
    pub settings: SpaceSettings,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SpaceSettings {
    pub allow_member_posts: bool,
    pub require_approval: bool,
    pub allow_discussions: bool,
    pub allow_polls: bool,
    pub auto_archive_days: Option<i32>,
}

impl Space {
    pub fn new(
        name: String,
        display_name: String,
        description: String,
        owner_id: String,
        is_public: bool,
    ) -> Self {
        let space_id = uuid::Uuid::new_v4().to_string();
        let pk = Partition::Space(space_id);
        let sk = EntityType::Space;

        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            name,
            display_name,
            description,
            owner_id,
            is_public,
            ..Default::default()
        }
    }

    pub fn space_id(&self) -> Option<String> {
        match &self.pk {
            Partition::Space(id) => Some(id.clone()),
            _ => None,
        }
    }
}
