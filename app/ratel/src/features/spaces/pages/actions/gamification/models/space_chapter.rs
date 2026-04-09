use crate::common::macros::DynamoEntity;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::features::spaces::pages::actions::gamification::*;

/// A chapter in the Quest Map of a single space.
///
/// Chapters are the backbone of the Quest Map: a space has N ordered
/// chapters, each gated by the previous one. Each chapter declares which
/// `SpaceUserRole` may submit its actions (`actor_role`) and what happens
/// when every action inside it is cleared (`completion_benefit`).
///
/// DynamoDB layout: `pk = SPACE#{space_id}`, `sk = SPACE_CHAPTER#{chapter_id}`.
/// All chapters for a space live under the space partition; listing them
/// for the Quest Map is a single `find_by_pk` query.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceChapter {
    pub pk: Partition,  // Partition::Space
    pub sk: EntityType, // EntityType::SpaceChapter

    pub created_at: i64,
    pub updated_at: i64,

    /// 0-based serial order of the chapter within the space.
    pub order: u32,

    pub name: String,

    #[serde(default)]
    pub description: Option<String>,

    /// Which `SpaceUserRole` is allowed to submit actions in this chapter.
    pub actor_role: SpaceUserRole,

    /// What the user receives upon finishing every action in this chapter.
    pub completion_benefit: ChapterBenefit,
}

#[cfg(feature = "server")]
impl SpaceChapter {
    pub fn new(
        space_id: SpacePartition,
        chapter_id: String,
        order: u32,
        name: String,
        actor_role: SpaceUserRole,
        completion_benefit: ChapterBenefit,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let pk: Partition = space_id.into();
        let sk = EntityType::SpaceChapter(chapter_id);

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            order,
            name,
            description: None,
            actor_role,
            completion_benefit,
        }
    }

    pub fn keys(space_pk: &Partition, chapter_id: &str) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::SpaceChapter(chapter_id.to_string()),
        )
    }
}
