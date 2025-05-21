pub fn insert_word_breaks(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_digit() || c == '/' || c == '_' || c == '.' {
                format!("{}\u{200B}", c)
            } else {
                c.to_string()
            }
        })
        .collect()
}
