use chrono::{NaiveTime, TimeZone};

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

pub fn get_now_timestamp_micros() -> i64 {
    chrono::Utc::now().timestamp_micros()
}

pub fn now() -> i64 {
    get_now_timestamp_millis()
}

pub fn current_month() -> String {
    chrono::Utc::now().format("%Y-%m").to_string()
}

pub fn after_days_from_now_rfc_3339(days: i64) -> String {
    let conf = crate::config::get();

    if conf.is_local() {
        crate::warn!("Using local time duration for after_days_from_now_rfc_3339");
        // NOTE: For local development, we use minutes instead of days
        // to make testing faster.
        return (chrono::Utc::now() + chrono::Duration::minutes(days))
            .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .unwrap()
            .to_rfc3339();
    }

    (chrono::Utc::now() + chrono::Duration::days(days))
        .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .unwrap()
        .to_rfc3339()
}
