use crate::common::models::auth::AdminUser;
use crate::features::admin::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackfillSpaceScoreRankResponse {
    pub scanned: usize,
    pub updated: usize,
}

#[post("/api/admin/migrations/space-score-rank", _user: AdminUser)]
pub async fn backfill_space_score_rank() -> Result<BackfillSpaceScoreRankResponse> {
    use crate::features::activity::models::SpaceScore;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let mut bookmark = None;
    let mut scanned = 0usize;
    let mut updated = 0usize;

    loop {
        let (scores, next_bookmark) = SpaceScore::find_all(
            cli,
            EntityType::SpaceScore,
            SpaceScore::opt_with_bookmark(bookmark).limit(100),
        )
        .await?;
        scanned += scores.len();

        for score in scores {

            SpaceScore::updater(&score.pk, &score.sk)
                .with_space_pk(score.space_pk.clone())
                .with_rank_total_score(-score.total_score)
                .with_updated_at(score.updated_at)
                .with_user_pk(score.user_pk.clone())
                .execute(cli)
                .await?;

            updated += 1;
        }

        if next_bookmark.is_none() {
            break;
        }

        bookmark = next_bookmark;
    }

    Ok(BackfillSpaceScoreRankResponse { scanned, updated })
}
