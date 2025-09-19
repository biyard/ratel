use validator::ValidationError;

pub fn validate_nickname(name: &str) -> Result<(), ValidationError> {
    let len = name.chars().count();
    if len < 3 || len > 30 {
        return Err(ValidationError::new(
            "Nickname must be between 3 and 30 characters",
        ));
    }
    let re = regex::Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    if !re.is_match(name) {
        return Err(ValidationError::new(
            "Nickname must only contain alphanumeric characters and underscores",
        ));
    }
    Ok(())
}
