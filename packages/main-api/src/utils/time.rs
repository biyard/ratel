use chrono::TimeZone;

pub fn format_timestamp<T>(timestamp: i64, timezone: T) -> Option<String>
where
    T: TimeZone,
    <T as TimeZone>::Offset: std::fmt::Display,
{
    timezone
        .timestamp_opt(timestamp, 0)
        .single()
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
}

pub fn get_now_timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}

pub fn get_now_timestamp_millis() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub fn now() -> i64 {
    get_now_timestamp_millis()
}
