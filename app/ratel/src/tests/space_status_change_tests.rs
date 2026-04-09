use super::*;
use crate::common::models::space::SpaceStatusChangeEvent;
use crate::common::types::SpaceStatus;
use crate::common::types::Partition;
use crate::features::spaces::space_common::services::handle_space_status_change;

/// Smoke test: handler accepts an event for an unknown transition and returns Ok.
#[tokio::test]
async fn test_handle_unknown_transition_is_noop() {
    let ctx = TestContext::setup().await;
    let _ = ctx; // force setup so DynamoDB schema exists

    let event = SpaceStatusChangeEvent::new(
        Partition::Space("nonexistent".to_string()),
        Some(SpaceStatus::Finished),
        SpaceStatus::Open,
    );

    // Unknown/illegal transition → handler short-circuits before loading the space.
    let result = handle_space_status_change(event).await;
    assert!(result.is_ok(), "expected Ok, got {:?}", result);
}
