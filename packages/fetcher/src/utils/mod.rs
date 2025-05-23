pub fn to_date(date: String) -> i32 {
    if let Ok(res) = date.replace("-", "").parse() {
        res
    } else {
        tracing::error!("Failed to parse date: {}", date);
        0
    }
}

// Converts a date string in the format "YYYY-MM-DDTHH:MM:SS" to an i32.
pub fn iso_to_date(date: String) -> i32 {
    let date_part = date.split('T').next().unwrap_or(&date);

    to_date(date_part.to_string())
}
