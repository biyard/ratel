#![allow(dead_code)]
use crate::*;

#[get("/api/spaces/:space_id/incentive/ranking?bookmark", _space: common::models::space::SpaceCommon)]
pub async fn list_ranking_handler(
    space_id: SpacePartition,
    bookmark: Option<String>,
) -> common::Result<space_common::types::dashboard::RankingTableData> {
    use space_common::types::dashboard::*;

    // TODO: Implement actual ranking query
    Ok(RankingTableData {
        page_size: 10,
        entries: vec![],
    })
}
