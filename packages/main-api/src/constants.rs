pub const SESSION_KEY_USER_ID: &str = "user_id";

// Verification code
pub const EXPIRATION_TIME: u64 = 1800; // 30 minutes
pub const MAX_ATTEMPT_COUNT: i32 = 5;
pub const ATTEMPT_BLOCK_TIME: i64 = 300; // 5 minutes
