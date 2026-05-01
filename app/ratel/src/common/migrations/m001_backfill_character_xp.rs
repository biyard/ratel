//! Migration 001 — backfill CharacterXp + CharacterXpSource from the
//! existing SpaceScore rows. Idempotent: re-running computes the same end
//! state (we use `put` semantics, not `+=`, so a re-run converges).

use crate::common::*;
use crate::features::character::leveling;
use crate::features::character::models::{CharacterXp, CharacterXpSource};
use std::collections::HashMap;

/// Required version this migration advances `LastBackfillVersion` to.
pub const REQUIRED_VERSION: i64 = 1;

pub async fn run(cli: &aws_sdk_dynamodb::Client) -> crate::common::Result<()> {
    use aws_sdk_dynamodb::types::AttributeValue;
    use crate::features::activity::models::SpaceScore;

    tracing::info!("m001: scanning SpaceScore rows");

    // Scan SpaceScore rows. The volume here is bounded by total
    // (user, space) participation pairs across the platform; we paginate
    // via ExclusiveStartKey. SpaceScore.sk starts with "SPACE_SCORE#" so we
    // filter via begins_with to skip other entity types co-located in the
    // table.
    // Keyed by user_pk's String form to avoid requiring Hash on Partition.
    let mut totals: HashMap<String, i64> = HashMap::new();
    let mut sources: Vec<(Partition, String, i64)> = Vec::new();

    let mut last_evaluated_key: Option<HashMap<String, AttributeValue>> = None;
    let mut pages = 0;
    loop {
        pages += 1;
        if pages > 10_000 {
            tracing::error!("m001 exceeded 10000 scan pages; aborting");
            return Err(Error::Internal);
        }

        let mut req = cli
            .scan()
            .table_name(SpaceScore::table_name())
            .filter_expression("begins_with(sk, :prefix)")
            .expression_attribute_values(":prefix", AttributeValue::S("SPACE_SCORE#".into()))
            .limit(500);
        if let Some(esk) = last_evaluated_key.clone() {
            req = req.set_exclusive_start_key(Some(esk));
        }

        let resp = req
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

        for item in resp.items.unwrap_or_default() {
            let row: SpaceScore = serde_dynamo::from_item(item)?;
            // Convert AuthorPartition -> Partition. Skip Unknown/Team for
            // CharacterXp purposes; only User accounts have a character.
            let user_pk: Partition = match row.user_pk {
                crate::features::activity::types::AuthorPartition::User(id) => Partition::User(id),
                _ => continue,
            };
            let space_id = match row.space_pk {
                Partition::Space(s) => s,
                _ => continue,
            };
            let key = match &user_pk {
                Partition::User(id) => format!("USER#{id}"),
                _ => continue,
            };
            *totals.entry(key).or_insert(0) += row.total_score;
            sources.push((user_pk, space_id, row.total_score));
        }

        match resp.last_evaluated_key {
            Some(k) if !k.is_empty() => last_evaluated_key = Some(k),
            _ => break,
        }
    }

    tracing::info!(
        users = totals.len(),
        sources = sources.len(),
        "m001: aggregation done; writing"
    );

    let now = crate::common::utils::time::get_now_timestamp_millis();

    // Upsert CharacterXp per user with bounded concurrency.
    use futures::stream::{self, StreamExt};
    let xp_writes = stream::iter(totals.into_iter().map(|(user_key, total_xp)| {
        let user_pk = match user_key.strip_prefix("USER#") {
            Some(id) => Partition::User(id.to_string()),
            None => Partition::None,
        };
        let level = leveling::level_from_xp(total_xp);
        let row = CharacterXp {
            pk: user_pk,
            sk: EntityType::CharacterXp,
            created_at: now,
            updated_at: now,
            total_xp,
            level,
            total_sp_granted: leveling::total_sp_granted(level),
            total_sp_spent: 0,
        };
        async move { row.upsert(cli).await }
    }))
    .buffer_unordered(16);
    xp_writes
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<crate::common::Result<Vec<_>>>()?;

    let src_writes = stream::iter(sources.into_iter().map(|(user_pk, space_id, last_seen)| {
        let row = CharacterXpSource {
            pk: user_pk,
            sk: EntityType::CharacterXpSource(space_id),
            last_seen_score: last_seen,
            updated_at: now,
        };
        async move { row.upsert(cli).await }
    }))
    .buffer_unordered(16);
    src_writes
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<crate::common::Result<Vec<_>>>()?;

    tracing::info!("m001: complete");
    Ok(())
}
