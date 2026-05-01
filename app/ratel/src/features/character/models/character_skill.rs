use crate::common::*;
use crate::features::character::types::SkillId;

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
pub struct CharacterSkill {
    pub pk: Partition,  // Partition::User(user_id)
    pub sk: EntityType, // EntityType::CharacterSkill(skill_id)

    pub level: i32, // 0..MAX_SKILL_LEVEL
    pub created_at: i64,
    pub updated_at: i64,
}

impl CharacterSkill {
    pub fn keys(user_pk: &Partition, skill_id: SkillId) -> (Partition, EntityType) {
        (
            user_pk.clone(),
            EntityType::CharacterSkill(skill_id.as_str().to_string()),
        )
    }
}

#[cfg(feature = "server")]
impl CharacterSkill {
    pub fn new(user_pk: Partition, skill_id: SkillId) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: user_pk,
            sk: EntityType::CharacterSkill(skill_id.as_str().to_string()),
            level: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Read a single skill row's level, treating "row absent" as level 0.
    /// Use only when you need exactly one skill (e.g. inside the level-up
    /// handler before mutating). For "show me every skill the user has",
    /// use `list_for_user` instead — it's a single Query.
    pub async fn level_or_zero(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
        skill_id: SkillId,
    ) -> crate::common::Result<i32> {
        let (pk, sk) = Self::keys(user_pk, skill_id);
        let row = Self::get(cli, &pk, Some(&sk)).await?;
        Ok(row.map(|r| r.level).unwrap_or(0))
    }

    /// Read every skill row for a user in a single DynamoDB Query
    /// (`pk = user_pk AND begins_with(sk, "CHARACTER_SKILL#")`). Returns
    /// the raw rows; the caller maps them to `(SkillId, level)` pairs and
    /// fills in `level = 0` for missing entries.
    pub async fn list_for_user(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
    ) -> crate::common::Result<Vec<Self>> {
        use aws_sdk_dynamodb::types::AttributeValue;

        let resp = cli
            .query()
            .table_name(Self::table_name())
            .key_condition_expression("pk = :pk AND begins_with(sk, :prefix)")
            .expression_attribute_values(":pk", AttributeValue::S(user_pk.to_string()))
            .expression_attribute_values(":prefix", AttributeValue::S("CHARACTER_SKILL#".to_string()))
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

        let items = resp.items.unwrap_or_default();
        let mut rows = Vec::with_capacity(items.len());
        for item in items {
            let row: Self = serde_dynamo::from_item(item)?;
            rows.push(row);
        }
        Ok(rows)
    }

    /// Convenience: turn a `list_for_user` result into a complete
    /// `(SkillId, level)` map for every known SkillId, defaulting absent
    /// rows to level 0.
    pub fn levels_by_id(rows: &[Self]) -> Vec<(SkillId, i32)> {
        let level_for = |id: SkillId| -> i32 {
            rows.iter()
                .find(|r| matches!(&r.sk, EntityType::CharacterSkill(s) if s == id.as_str()))
                .map(|r| r.level)
                .unwrap_or(0)
        };
        [
            SkillId::MoneyTree,
            SkillId::Ranker,
            SkillId::Influencer,
            SkillId::Sweeper,
        ]
        .into_iter()
        .map(|id| (id, level_for(id)))
        .collect()
    }
}
