use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Timestamp;

pub fn sorted_uuid() -> String {
    use uuid::Uuid;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let ctx = uuid::ContextV7::new().with_adjust_by_millis(now.subsec_millis());
    let ts = Timestamp::from_unix(ctx, now.as_secs(), now.subsec_nanos());
    let uuid = Uuid::new_v7(ts);

    uuid.to_string()
}
