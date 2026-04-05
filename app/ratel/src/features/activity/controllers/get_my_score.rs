use crate::features::activity::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MyScoreResponse {
    pub total_score: i64,
    pub poll_score: i64,
    pub quiz_score: i64,
    pub follow_score: i64,
    pub discussion_score: i64,
    pub rank: u32,
}

#[get("/api/spaces/:space_id/my-score", _space: crate::common::models::space::SpaceCommon, user: crate::features::auth::User)]
pub async fn get_my_score_handler(
    space_id: SpacePartition,
) -> Result<MyScoreResponse> {
    use crate::features::activity::models::SpaceScore;

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let author = AuthorPartition::from(user.pk.clone());
    let (pk, sk) = SpaceScore::keys(&space_id, &author);

    let score = SpaceScore::get(cli, &pk, Some(sk))
        .await?
        .unwrap_or_default();

    // Calculate rank by counting entries with higher score
    let space_pk: Partition = space_id.into();
    let opts = SpaceScore::opt().limit(1000);

    let (all_scores, _) = SpaceScore::find_by_space(cli, &space_pk, opts).await?;
    let rank = all_scores
        .iter()
        .position(|s| s.total_score <= score.total_score)
        .map(|pos| (pos as u32) + 1)
        .unwrap_or(0);

    Ok(MyScoreResponse {
        total_score: score.total_score,
        poll_score: score.poll_score,
        quiz_score: score.quiz_score,
        follow_score: score.follow_score,
        discussion_score: score.discussion_score,
        rank,
    })
}
