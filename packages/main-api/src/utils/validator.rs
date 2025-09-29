use validator::ValidationError;

pub fn validate_username(name: &str) -> Result<(), ValidationError> {
    let len = name.chars().count();
    if len < 3 || len > 20 {
        return Err(ValidationError::new(
            "Nickname must be between 3 and 20 characters",
        ));
    }
    let re = regex::Regex::new(r"^[a-z0-9_-]+$").unwrap();
    if !re.is_match(name) {
        return Err(ValidationError::new(
            "Nickname must only contain lowercase alphanumeric characters, underscores, and hyphens",
        ));
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
    let len = content.chars().count();
    if len < 10 || len > 5000 {
        return Err(ValidationError::new(
            "Content must be between 10 and 5000 characters",
        ));
    }
    Ok(())
}
