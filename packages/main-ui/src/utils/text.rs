use html2text::from_read;
use regex::Regex;

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

pub fn extract_title_from_html(html: &str) -> String {
    let text = from_read(html.as_bytes(), 200);
    let first_line = text.lines().next().unwrap_or("").trim();

    let bold = Regex::new(r"\*\*(.*?)\*\*").unwrap();
    let italic = Regex::new(r"\*(.*?)\*").unwrap();
    let header = Regex::new(r"^#+\s*").unwrap();

    let mut result = first_line.to_string();
    result = bold.replace_all(&result, "$1").to_string();
    result = italic.replace_all(&result, "$1").to_string();
    result = header.replace_all(&result, "").to_string();

    result.trim().to_string()
}
