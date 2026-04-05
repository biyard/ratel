use crate::common::SpacePartition;
use dioxus::prelude::*;

#[get("/api/spaces/:space_id/incentive/ranking?bookmark", _space: crate::common::models::space::SpaceCommon)]
pub async fn list_ranking_handler(
    space_id: SpacePartition,
    bookmark: Option<String>,
) -> crate::common::Result<crate::features::spaces::space_common::types::dashboard::RankingTableData>
{
    use crate::features::spaces::space_common::types::dashboard::*;

    #[cfg(feature = "activity")]
    {
        use crate::features::activity::models::SpaceScore;

        let cfg = crate::common::CommonConfig::default();
        let cli = cfg.dynamodb();
        let space_pk: crate::common::Partition = space_id.into();

        let mut opts = SpaceScore::opt().limit(50);
        if let Some(bm) = bookmark {
            opts = opts.bookmark(bm);
        }

        let (scores, _) = SpaceScore::find_by_space(cli, &space_pk, opts).await?;

        let entries: Vec<RankingEntry> = scores
            .iter()
            .enumerate()
            .map(|(i, score)| RankingEntry {
                rank: (i as u32) + 1,
                name: score.user_name.clone(),
                avatar: score.user_avatar.clone(),
                score: score.total_score as f64,
                change: 0,
            })
            .collect();

        return Ok(RankingTableData {
            entries,
            page_size: 10,
        });
    }

    #[cfg(not(feature = "activity"))]
    Ok(RankingTableData {
        entries: vec![],
        page_size: 10,
    })
}
