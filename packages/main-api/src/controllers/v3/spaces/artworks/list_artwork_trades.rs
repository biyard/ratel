use axum::extract::{Json, Path, Query, State};
use bdk::prelude::*;

use crate::{
    AppState, Error,
    controllers::v3::spaces::{SpacePath, SpacePathParam},
    features::spaces::artworks::{
        ListSpaceArtworkTradeResponse, SpaceArtworkTrade, SpaceArtworkTradeItem,
        SpaceArtworkTradeQueryOption,
    },
    types::{
        EntityType, Pagination, Partition, PostStatus, list_items_query::ListItemsQuery,
        list_items_response::ListItemsResponse,
    },
};

pub async fn list_space_artwork_trades_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(Pagination { bookmark }): ListItemsQuery,
) -> Result<Json<ListSpaceArtworkTradeResponse>, Error> {
    let space_pk = match space_pk {
        Partition::Space(_) => space_pk,
        _ => return Err(Error::InvalidSpacePartitionKey),
    };

    // Build query options with begins_with filter for SpaceArtworkTrade entities
    let mut query_options = SpaceArtworkTradeQueryOption::builder()
        .sk(EntityType::SpaceArtworkTrade(String::default()).to_string())
        .limit(20);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    // Query all SpaceArtworkTrade records for this space
    let (trades, new_bookmark) =
        SpaceArtworkTrade::query(&dynamo.client, space_pk.clone(), query_options).await?;

    let items: Vec<SpaceArtworkTradeItem> = trades.into_iter().map(|trade| trade.into()).collect();

    Ok(Json(ListSpaceArtworkTradeResponse {
        items,
        bookmark: new_bookmark,
    }))
}
