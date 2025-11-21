use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Timestamp;

pub fn sorted_uuid() -> String {
    use uuid::Uuid;
    let uid = Uuid::now_v7();

    uid.to_string()
}
