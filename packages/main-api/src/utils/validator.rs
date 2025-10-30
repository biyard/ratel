use validator::ValidationError;

pub fn validate_username(name: &str) -> Result<(), ValidationError> {
    let len = name.chars().count();
    if len < 3 || len > 20 {
        return Err(ValidationError::new(
            "Username must be between 3 and 20 characters",
        ));
    }
    let re = regex::Regex::new(r"^[a-z0-9_-]+$").unwrap();
    if !re.is_match(name) {
        return Err(ValidationError::new(
            "Username must only contain lowercase alphanumeric characters, underscores, and hyphens",
        ));
    }
    Ok(())
}

pub fn validate_nickname(nickname: &str) -> Result<(), ValidationError> {
    let trimmed = nickname.trim();
    let len = trimmed.chars().count();

    if len < 1 || len > 30 {
        return Err(ValidationError::new(
            "Display name must be between 1 and 30 characters",
        ));
    }

    if trimmed.is_empty() {
        return Err(ValidationError::new(
            "Display name cannot be empty or only whitespace",
        ));
    }

    // Count words (split by whitespace)
    let word_count = trimmed.split_whitespace().count();
    if word_count > 2 {
        return Err(ValidationError::new("Display name must be at most 2 words"));
    }

    Ok(())
}

pub fn validate_description(description: &str) -> Result<(), ValidationError> {
    let len = description.chars().count();
    if len < 10 {
        return Err(ValidationError::new(
            "Description must be at least 10 characters",
        ));
    }
    Ok(())
}

pub fn validate_image_url(_url: &str) -> Result<(), ValidationError> {
    // FIXME: Check if URL is valid and points to an image
    Ok(())
}

pub fn validate_title(title: &str) -> Result<(), ValidationError> {
    let len = title.chars().count();
    if len < 3 || len > 50 {
        return Err(ValidationError::new(
            "Title must be between 3 and 50 characters",
        ));
    }
    Ok(())
}

pub fn validate_content(content: &str) -> Result<(), ValidationError> {
    let plain_text = extract_plain_text(content);
    let len = plain_text.chars().count();
    if len < 10 || len > 5000 {
        return Err(ValidationError::new(
            "Content must be between 10 and 5000 characters",
        ));
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
