use chrono::{TimeZone, Utc};

pub fn format_prev_time(timestamp: i64) -> String {
    let now = Utc::now();

    let target_time = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .unwrap_or(Utc::now());

    let duration = now.signed_duration_since(target_time);

    if duration.num_seconds() < 60 {
        return format!("{}s ago", check_date(duration.num_seconds()));
    } else if duration.num_minutes() < 60 {
        return format!("{}m ago", check_date(duration.num_minutes()));
    } else if duration.num_hours() < 24 {
        return format!("{}h ago", check_date(duration.num_hours()));
    } else if duration.num_days() < 30 {
        return format!("{}d ago", check_date(duration.num_days()));
    } else if duration.num_days() < 365 {
        let months = duration.num_days() / 30;
        return format!("{}th ago", check_date(months));
    } else {
        let years = duration.num_days() / 365;
        return format!("{}y ago", check_date(years));
    }
}

fn check_date(date: i64) -> i64 {
    if date >= 0 { date } else { 0 }
}
