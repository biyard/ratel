// Migrated from packages/main-api/src/utils/validator.rs
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

pub fn validate_image_url(_url: &str) -> Result<(), ValidationError> {
    // FIXME: Check if URL is valid and points to an image
    Ok(())
}
