use crate::*;

pub fn convert_number_to_string(n: i64) -> String {
    let suffixes = ["K", "M", "B"];
    let mut value = n as f64;
    let mut i = 0;

    while value >= 1000.0 && i < suffixes.len() {
        value /= 1000.0;
        i += 1;
    }

    if i == 0 {
        format!("{}", n)
    } else {
        format!("{} {}", value as i64, suffixes[i - 1])
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
