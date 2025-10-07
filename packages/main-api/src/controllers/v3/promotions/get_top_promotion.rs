use axum::*;
use bdk::prelude::*;

use crate::types::{Partition, SpaceType};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct GetTopPromotionResponse {
    id: Partition,
    created_at: i64,
    updated_at: i64,

    name: String,
    image_url: String,

    feed_id: Partition,
    space_id: Option<Partition>,
    space_type: Option<SpaceType>,
}

pub async fn get_top_promotion_handler() -> Result<Json<GetTopPromotionResponse>, crate::Error2> {
    // TODO: implement the logic to fetch the top promotion

    Ok(Json(GetTopPromotionResponse {
        id: Partition::Promotion("1".to_string()),
        created_at: 1749801927,
        updated_at: 1749801927,
        name: "디지털자산기본법안 발의".to_string(),
        image_url: "https://metadata.ratel.foundation/metadata/digital-act.png".to_string(),
        feed_id: Partition::Feed("7".to_string()),
        space_id: Some(Partition::Space("2".to_string())),
        space_type: Some(SpaceType::Commitee),
    }))
}
