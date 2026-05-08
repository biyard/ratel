use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::common::*;

#[derive(
    Debug, Clone, Copy, SerializeDisplay, DeserializeFromStr, Eq, PartialEq, Default,
)]
pub enum PendingRewardStatus {
    #[default]
    Pending,
    Completed,
}

impl Display for PendingRewardStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Pending => "PENDING",
            Self::Completed => "COMPLETED",
        };
        f.write_str(s)
    }
}

impl FromStr for PendingRewardStatus {
    type Err = Error;

    // Case-insensitive so legacy rows that wrote "pending" still decode.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "PENDING" => Ok(Self::Pending),
            "COMPLETED" => Ok(Self::Completed),
            _ => Err(Error::InvalidFormat),
        }
    }
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
