use crate::features::posts::*;

pub fn validate_title(title: &str) -> Result<()> {
    let len = title.chars().count();
    if len < 3 || len > 50 {
        return Err(Error::BadRequest(
            "Title must be between 3 and 50 characters".into(),
        ));
    }
    Ok(())
}

pub fn validate_content(content: &str) -> Result<()> {
    let plain_text = extract_plain_text(content);
    let len = plain_text.chars().count();
    if len < 10 {
        return Err(Error::ValidationTooShortContents);
    }

    Ok(())
}

pub fn extract_plain_text(html: &str) -> String {
    let re_img = regex::Regex::new(r"<img[^>]*>").unwrap();
    let without_images = re_img.replace_all(html, "");

    let re_tags = regex::Regex::new(r"<[^>]+>").unwrap();
    let without_tags = re_tags.replace_all(&without_images, "");

    let re_urls = regex::Regex::new(r"https?://[^\s]+").unwrap();
    let without_urls = re_urls.replace_all(&without_tags, "");

    let re_whitespace = regex::Regex::new(r"\s+").unwrap();
    let normalized = re_whitespace.replace_all(&without_urls, " ");

    normalized.trim().to_string()
}
