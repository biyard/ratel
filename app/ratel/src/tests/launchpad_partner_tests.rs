use super::*;
use crate::common::types::{EntityType, Partition};
use crate::features::launchpad_partner::models::LaunchpadDeduction;

#[tokio::test]
async fn deduction_row_round_trips() {
    let ctx = TestContext::setup().await;
    let user = format!("u-{}", uuid::Uuid::new_v4());
    let key = format!("lp_{}", uuid::Uuid::new_v4());

    let row = LaunchpadDeduction::new(&user, &key, 500, "tx_demo", 740);
    row.create(&ctx.ddb).await.expect("create");

    let fetched = LaunchpadDeduction::get(
        &ctx.ddb,
        Partition::User(user.clone()),
        Some(EntityType::LaunchpadDeduction(key.clone())),
    )
    .await
    .expect("get")
    .expect("row present");

    assert_eq!(fetched.point_amount, 500);
    assert_eq!(fetched.brand_tx_id, "tx_demo");
    assert_eq!(fetched.remaining_points, 740);
}
