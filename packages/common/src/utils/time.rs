pub fn get_now_timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}

pub fn get_now_timestamp_millis() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub fn get_now_timestamp_micros() -> i64 {
    chrono::Utc::now().timestamp_micros()
}

pub fn now() -> i64 {
    get_now_timestamp_millis()
}

pub fn current_month() -> String {
    chrono::Utc::now().format("%Y-%m").to_string()
}
