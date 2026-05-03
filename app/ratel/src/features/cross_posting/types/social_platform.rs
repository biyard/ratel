use crate::common::*;
use std::str::FromStr;

/// Phase 1 cross-posting target platforms (FR-1 #1).
///
/// Display impl produces the lowercase identifier used in URLs / DynamoDB
/// sort-key inners (e.g. `SocialConnection("bluesky")`); FromStr accepts
/// the same lowercase form. Use `display_name()` for user-facing strings.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default, strum::Display,
)]
#[cfg_attr(feature = "server", derive(JsonSchema))]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SocialPlatform {
    #[default]
    Bluesky,
    LinkedIn,
    Threads,
}

impl FromStr for SocialPlatform {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "bluesky" => Ok(SocialPlatform::Bluesky),
            "linkedin" => Ok(SocialPlatform::LinkedIn),
            "threads" => Ok(SocialPlatform::Threads),
            _ => Err(()),
        }
    }
}

impl SocialPlatform {
    /// Capitalized name for UI rendering and Bluesky banner copy parameterization.
    pub fn display_name(self) -> &'static str {
        match self {
            SocialPlatform::Bluesky => "Bluesky",
            SocialPlatform::LinkedIn => "LinkedIn",
            SocialPlatform::Threads => "Threads",
        }
    }

    /// Per-platform character limit (FR-4 #25). Used by both the compose-time
    /// warning and Stage 2 truncation.
    pub fn char_limit(self) -> usize {
        match self {
            SocialPlatform::Bluesky => 300,
            SocialPlatform::LinkedIn => 3_000,
            SocialPlatform::Threads => 500,
        }
    }

    /// Maximum number of images the platform accepts on a single post (FR-5 #31).
    pub fn max_images(self) -> usize {
        match self {
            SocialPlatform::Bluesky => 4,
            SocialPlatform::LinkedIn => 1,
            SocialPlatform::Threads => 1,
        }
    }
}
