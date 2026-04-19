use crate::features::activity::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct RankingEntryResponse {
    pub rank: u32,
    pub user_pk: String,
    pub name: String,
    pub avatar: String,
    pub total_score: i64,
}

#[get("/api/spaces/:space_id/ranking?bookmark", _space: crate::common::models::space::SpaceCommon)]
pub async fn get_ranking_handler(
    space_id: SpacePartition,
    bookmark: Option<String>,
) -> Result<ListResponse<RankingEntryResponse>> {
    use crate::features::activity::models::SpaceScore;

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let space_pk: Partition = space_id.into();

    let opts = SpaceScore::opt_with_bookmark(bookmark)
        .limit(50)
        .scan_index_forward(false);

    let (scores, next_bookmark) = SpaceScore::find_by_space_rank(cli, &space_pk, opts).await?;

    let items: Vec<RankingEntryResponse> = scores
        .iter()
        .enumerate()
        .map(|(i, score)| RankingEntryResponse {
            rank: (i as u32) + 1,
            user_pk: score.user_pk.to_string(),
            name: score.user_name.clone(),
            avatar: score.user_avatar.clone(),
            total_score: score.total_score,
        })
        .collect();

    Ok((items, next_bookmark).into())
}
