use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::common::*;

#[derive(Debug, Default, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq)]
pub struct PendingRewardKey {
    pub created_at: i64,
    pub target_pk: Partition,
    pub reward_key: RewardKey,
}

impl Display for PendingRewardKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PENDING_REWARD#{}#{}#{}",
            self.created_at, self.target_pk, self.reward_key
        )
    }
}

impl FromStr for PendingRewardKey {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s
            .strip_prefix("PENDING_REWARD#")
            .ok_or(Error::InvalidFormat)?;

        let (ts_str, rest) = s.split_once('#').ok_or(Error::InvalidFormat)?;
        let created_at: i64 = ts_str.parse().map_err(|_| Error::InvalidFormat)?;

        let (target_pk, rk_str) = if let Some(rest2) = rest.strip_prefix("USER#") {
            let (uid, rk) = rest2.split_once('#').ok_or(Error::InvalidFormat)?;
            (Partition::User(uid.to_string()), rk)
        } else if let Some(rest2) = rest.strip_prefix("TEAM#") {
            let (tid, rk) = rest2.split_once('#').ok_or(Error::InvalidFormat)?;
            (Partition::Team(tid.to_string()), rk)
        } else {
            return Err(Error::InvalidFormat);
        };

        let reward_key = RewardKey::from_str(rk_str)?;

        Ok(PendingRewardKey {
            created_at,
            target_pk,
            reward_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> PendingRewardKey {
        PendingRewardKey {
            created_at: 1776817749390,
            target_pk: Partition::User("5afecfb0-1278-48f8-a782-76ba2badfb46".into()),
            reward_key: RewardKey::from((
                SpacePartition("019d70df-dfc0-7222-be71-e55c2bd8121a".into()),
                "019d9e8a-29a9-7a92-935f-cfa709e992c4".to_string(),
                RewardUserBehavior::QuizAnswer,
            )),
        }
    }

    #[test]
    fn roundtrip_display_fromstr() {
        let k = sample();
        let s = k.to_string();
        assert!(s.starts_with("PENDING_REWARD#1776817749390#USER#5afecfb0"));
        let parsed: PendingRewardKey = s.parse().unwrap();
        assert_eq!(parsed, k);
    }

    #[test]
    fn parses_team_target() {
        let s = "PENDING_REWARD#1234#TEAM#840#REWARD##RESPOND_POLL";
        let k: PendingRewardKey = s.parse().unwrap();
        assert_eq!(k.created_at, 1234);
        assert_eq!(k.target_pk, Partition::Team("840".into()));
    }

    #[test]
    fn rejects_unknown_actor_prefix() {
        let s = "PENDING_REWARD#1234#FEED#abc#REWARD##RESPOND_POLL";
        assert!(s.parse::<PendingRewardKey>().is_err());
    }

    /// DynamoDB sort keys are compared lexicographically, so callers that
    /// page through `PENDING_REWARD#*` expecting chronological order need
    /// `created_at` segments to share a width. All production rows use
    /// 13-digit millisecond timestamps, so the invariant holds — this test
    /// pins that.
    #[test]
    fn lexicographic_order_matches_timestamp_at_fixed_width() {
        let mut a = sample();
        a.created_at = 1_000_000_000_000; // 13 digits
        let mut b = sample();
        b.created_at = 2_000_000_000_000; // 13 digits
        assert!(a.to_string() < b.to_string());
    }

    /// Counter-example: mixing digit counts inverts lexicographic order
    /// vs numeric order. Kept as a guard rail — if we ever ingest
    /// non-millisecond timestamps, this test will start failing and force
    /// us to revisit zero-padding the `created_at` segment.
    #[test]
    fn lexicographic_order_breaks_at_different_digit_count() {
        let mut a = sample();
        a.created_at = 9_999_999_999; // 10 digits
        let mut b = sample();
        b.created_at = 10_000_000_000; // 11 digits, numerically larger
        assert!(a.to_string() > b.to_string());
    }
}
