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

pub fn add_one_month(timestamp_millis: i64) -> Option<i64> {
    let dt = chrono::DateTime::from_timestamp_millis(timestamp_millis)?;
    let next = dt.checked_add_months(chrono::Months::new(1))?;
    Some(next.timestamp_millis())
}

pub fn kst_date_time_to_utc_millis(date: time::Date, hour: u8, minute: u8) -> i64 {
    let datetime = date.with_hms(hour, minute, 0).expect("valid time");
    let offset_datetime = datetime.assume_utc() - time::Duration::hours(9);
    offset_datetime.unix_timestamp() * 1000
}

pub async fn sleep(_duration: std::time::Duration) {
    #[cfg(feature = "web")]
    gloo_timers::future::sleep(_duration).await;

    #[cfg(feature = "server")]
    tokio::time::sleep(_duration).await;
}

/// Convert a UTC epoch (milliseconds) into the string format expected by
/// HTML `<input type="datetime-local">`, interpreted in the **browser's
/// local timezone**. The server-side fallback uses UTC.
pub fn epoch_ms_to_datetime_local(ms: i64) -> String {
    if ms <= 0 {
        return String::new();
    }

    #[cfg(all(feature = "web", not(feature = "server")))]
    {
        use chrono::{FixedOffset, Utc};
        let dt = chrono::DateTime::<Utc>::from_timestamp_millis(ms).unwrap_or_default();
        // `Date.prototype.getTimezoneOffset()` returns minutes WEST of UTC.
        let offset_min = js_sys::Date::new_0().get_timezone_offset() as i32;
        let offset = FixedOffset::west_opt(offset_min * 60)
            .unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());
        let local = dt.with_timezone(&offset);
        return local.format("%Y-%m-%dT%H:%M").to_string();
    }

    #[cfg(not(all(feature = "web", not(feature = "server"))))]
    {
        let dt = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(ms).unwrap_or_default();
        dt.format("%Y-%m-%dT%H:%M").to_string()
    }
}

/// Parse a `<input type="datetime-local">` value (interpreted in the
/// browser's local timezone) into a UTC epoch in milliseconds.
pub fn datetime_local_to_epoch_ms(s: &str) -> Option<i64> {
    let naive = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M").ok()?;

    #[cfg(all(feature = "web", not(feature = "server")))]
    {
        use chrono::{FixedOffset, TimeZone};
        let offset_min = js_sys::Date::new_0().get_timezone_offset() as i32;
        let offset = FixedOffset::west_opt(offset_min * 60)?;
        let local = offset.from_local_datetime(&naive).single()?;
        return Some(local.timestamp_millis());
    }

    #[cfg(not(all(feature = "web", not(feature = "server"))))]
    {
        Some(naive.and_utc().timestamp_millis())
    }
}

pub fn time_ago(timestamp_millis: i64) -> String {
    let now = chrono::Utc::now().timestamp_millis();
    let diff = now - timestamp_millis;

    if diff < 60 * 1000 {
        format!("{}s ago", diff / 1000)
    } else if diff < 3600 * 1000 {
        format!("{}m ago", diff / 1000 / 60)
    } else if diff < 86400 * 1000 {
        format!("{}h ago", diff / 1000 / 3600)
    } else if diff < 604800 * 1000 {
        format!("{}d ago", diff / 1000 / 86400)
    } else if diff < 31536000 * 1000 {
        format!("{}w ago", diff / 1000 / 604800)
    } else {
        format!("{}y ago", diff / 1000 / 31536000)
    }
}
