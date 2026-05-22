//! Register FOF's `RoomChannel` impls with the global arcade hub at
//! process start. Idempotent — last registration wins so tests that
//! swap in a mock channel still work.

use crate::features::arcade::games::fact_or_fold::realtime::chat::FactFoldChatChannel;
use crate::features::arcade::realtime::hub::global_hub;

pub async fn register_channels() {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb().clone();
    let hub = global_hub();
    hub.register(FactFoldChatChannel::new(cli)).await;
}
