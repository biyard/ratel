pub fn to_date(date: String) -> i32 {
    if let Ok(res) = date.replace("-", "").parse() {
        res
    } else {
        tracing::error!("Failed to parse date: {}", date);
        0
    }
}
