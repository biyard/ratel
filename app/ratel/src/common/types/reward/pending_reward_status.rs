use serde_with::{DeserializeFromStr, SerializeDisplay};

// Case-insensitive parse keeps legacy rows that wrote "pending" decoding.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, SerializeDisplay, DeserializeFromStr, strum::Display, strum::EnumString)]
#[strum(serialize_all = "UPPERCASE", ascii_case_insensitive)]
pub enum PendingRewardStatus {
    #[default]
    Pending,
    Completed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_uses_screaming_snake() {
        assert_eq!(PendingRewardStatus::Pending.to_string(), "PENDING");
        assert_eq!(PendingRewardStatus::Completed.to_string(), "COMPLETED");
    }

    #[test]
    fn from_str_is_case_insensitive() {
        assert_eq!(
            "pending".parse::<PendingRewardStatus>().unwrap(),
            PendingRewardStatus::Pending
        );
    }

    #[test]
    fn unknown_variant_errors() {
        assert!("bogus".parse::<PendingRewardStatus>().is_err());
    }
}
