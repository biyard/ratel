pub fn get_now_timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}

pub fn now() -> i64 {
    chrono::Utc::now().timestamp_millis()
}
