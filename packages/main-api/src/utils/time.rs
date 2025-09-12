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
