use std::{fmt::Display, str::FromStr};

use crate::common::*;

const BASE62_ALPHABET: &[u8; 62] =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub const SCORE_DIGITS: usize = 20;
pub const WINDOW_DIGITS: usize = 4;

const SECONDS_PER_SECOND: u64 = 1;
const SECONDS_PER_MINUTE: u64 = 60;
const SECONDS_PER_HOUR: u64 = 60 * 60;
const SECONDS_PER_DAY: u64 = 24 * SECONDS_PER_HOUR;
const SECONDS_PER_WEEK: u64 = 7 * SECONDS_PER_DAY;
const SECONDS_PER_MONTH: u64 = 30 * SECONDS_PER_DAY;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(JsonSchema, OperationIo))]
pub enum TimeBucket {
    /// Realtime resolution. With a 30-day window, max freshness =
    /// 30 * 24 * 3600 = 2,592,000 — well within `SCORE_DIGITS=20` base-62.
    Second,
    Minute,
    #[default]
    Hour,
    Day,
    Week,
    Month,
}

impl TimeBucket {
    pub const fn seconds(self) -> u64 {
        match self {
            TimeBucket::Second => SECONDS_PER_SECOND,
            TimeBucket::Minute => SECONDS_PER_MINUTE,
            TimeBucket::Hour => SECONDS_PER_HOUR,
            TimeBucket::Day => SECONDS_PER_DAY,
            TimeBucket::Week => SECONDS_PER_WEEK,
            TimeBucket::Month => SECONDS_PER_MONTH,
        }
    }

    /// How many of `self` buckets fit into `seconds`.
    /// Use this to express "1 unit of factor X is worth N hours of freshness":
    ///   `TimeBucket::Hour.units_for_seconds(2 * 3600)` => 2
    pub fn units_for_seconds(self, seconds: u64) -> u64 {
        seconds / self.seconds()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(JsonSchema, OperationIo))]
pub struct Factor {
    pub value: u64,
    pub multiplier: u64,
}

impl Factor {
    pub fn new(value: u64, multiplier: u64) -> Self {
        Self { value, multiplier }
    }

    pub fn weight(&self) -> u64 {
        self.value.saturating_mul(self.multiplier)
    }
}

/// Windowed sort key: `W{window:0WINDOW_DIGITS}{score:0SCORE_DIGITS}` in base-62.
///
/// - `window_idx` is derived from `last_active_at` (or `created_at` if never updated)
///   divided by `window_size`. Newer windows always lex-sort above older ones.
/// - Inside a window, the score combines:
///     freshness_units = (window_end - last_active_at) / resolution
///     factor_sum      = sum(value * multiplier)
/// - Sort is **descending by quality**: we encode `MAX_SCORE - score`, so an
///   ascending GSI scan returns best-first within each window.
/// - When `last_active_at` is updated past the original window, callers must
///   rebuild with the new timestamp; the new key falls into a newer window
///   and naturally outranks every older-window entry.
#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(JsonSchema, OperationIo))]
pub struct WindowedRankKey {
    pub window_idx: u64,
    pub score: u64,
}

impl Default for WindowedRankKey {
    fn default() -> Self {
        Self {
            window_idx: 0,
            score: 0,
        }
    }
}

impl WindowedRankKey {
    /// Maximum encodable score: 62^SCORE_DIGITS - 1.
    pub fn max_score() -> u128 {
        let mut m: u128 = 1;
        for _ in 0..SCORE_DIGITS {
            m *= 62;
        }
        m - 1
    }

    /// Build a key from raw factors plus freshness derived from
    /// `last_active_at` within a window of `window_size_secs` (anchored at unix epoch).
    pub fn build(
        resolution: TimeBucket,
        window_size_secs: u64,
        last_active_at_secs: u64,
        factors: &[Factor],
    ) -> Self {
        let window_idx = last_active_at_secs / window_size_secs;
        let window_end = window_idx
            .saturating_add(1)
            .saturating_mul(window_size_secs);

        let freshness_secs = window_end.saturating_sub(last_active_at_secs);
        let freshness_units = freshness_secs / resolution.seconds();

        let factor_sum: u64 = factors.iter().map(Factor::weight).fold(0u64, u64::saturating_add);
        let score = freshness_units.saturating_add(factor_sum);

        Self { window_idx, score }
    }

    /// True if `now_secs` has crossed the boundary of the window this key was built for.
    /// Callers should rebuild the key (with `last_active_at = now_secs`) when this returns true,
    /// so revived items jump up into the current window.
    pub fn is_expired(&self, window_size_secs: u64, now_secs: u64) -> bool {
        let window_end = self
            .window_idx
            .saturating_add(1)
            .saturating_mul(window_size_secs);
        now_secs >= window_end
    }
}

impl Display for WindowedRankKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max = Self::max_score();
        let score = (self.score as u128).min(max);
        let inverted = max - score;

        let window = base62_pad(self.window_idx as u128, WINDOW_DIGITS);
        let score_part = base62_pad(inverted, SCORE_DIGITS);
        write!(f, "W{}{}", window, score_part)
    }
}

impl FromStr for WindowedRankKey {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let expected_len = 1 + WINDOW_DIGITS + SCORE_DIGITS;
        if bytes.len() != expected_len || bytes[0] != b'W' {
            return Err(Error::InvalidPartitionKey(format!(
                "invalid WindowedRankKey: {s}"
            )));
        }

        let window_part = &s[1..1 + WINDOW_DIGITS];
        let score_part = &s[1 + WINDOW_DIGITS..];

        let window_idx = base62_decode(window_part)
            .map_err(|e| Error::InvalidPartitionKey(format!("window: {e}")))?;
        let inverted = base62_decode(score_part)
            .map_err(|e| Error::InvalidPartitionKey(format!("score: {e}")))?;

        let max = Self::max_score();
        let score = max.saturating_sub(inverted);

        Ok(Self {
            window_idx: window_idx as u64,
            score: score as u64,
        })
    }
}

fn base62_pad(mut n: u128, width: usize) -> String {
    let mut buf = vec![b'0'; width];
    let mut i = width;
    while n > 0 && i > 0 {
        i -= 1;
        let r = (n % 62) as usize;
        buf[i] = BASE62_ALPHABET[r];
        n /= 62;
    }
    // saturate if n didn't fit
    if n > 0 {
        for slot in buf.iter_mut() {
            *slot = BASE62_ALPHABET[61];
        }
    }
    String::from_utf8(buf).expect("base62 alphabet is ascii")
}

fn base62_decode(s: &str) -> std::result::Result<u128, String> {
    let mut acc: u128 = 0;
    for c in s.bytes() {
        let v = match c {
            b'0'..=b'9' => (c - b'0') as u128,
            b'A'..=b'Z' => (c - b'A') as u128 + 10,
            b'a'..=b'z' => (c - b'a') as u128 + 36,
            _ => return Err(format!("invalid base62 char: {}", c as char)),
        };
        acc = acc
            .checked_mul(62)
            .and_then(|x| x.checked_add(v))
            .ok_or_else(|| "base62 overflow".to_string())?;
    }
    Ok(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_is_fixed_width() {
        let k = WindowedRankKey {
            window_idx: 1,
            score: 0,
        };
        let s = k.to_string();
        assert_eq!(s.len(), 1 + WINDOW_DIGITS + SCORE_DIGITS);
        assert!(s.starts_with('W'));
    }

    #[test]
    fn higher_score_sorts_first_ascending() {
        let low = WindowedRankKey {
            window_idx: 5,
            score: 10,
        }
        .to_string();
        let high = WindowedRankKey {
            window_idx: 5,
            score: 1000,
        }
        .to_string();
        // descending-by-quality encoding => higher score has smaller lex value
        assert!(high < low, "high={high} should sort before low={low}");
    }

    #[test]
    fn newer_window_outranks_older_window() {
        let old = WindowedRankKey {
            window_idx: 1,
            score: u64::MAX / 2,
        }
        .to_string();
        let new = WindowedRankKey {
            window_idx: 2,
            score: 0,
        }
        .to_string();
        // newer windows lex-sort *after* older windows; consumers query newest-first
        // by scanning descending. What matters is that the window prefix never overlaps.
        assert!(new > old, "newer window prefix must exceed older");
    }

    #[test]
    fn round_trip() {
        let k = WindowedRankKey {
            window_idx: 42,
            score: 9_876_543,
        };
        let s = k.to_string();
        let parsed: WindowedRankKey = s.parse().unwrap();
        assert_eq!(parsed, k);
    }

    #[test]
    fn build_assigns_window_from_last_active() {
        let window = 30 * SECONDS_PER_DAY;
        // last_active at day 31 -> window_idx = 1
        let k = WindowedRankKey::build(TimeBucket::Hour, window, 31 * SECONDS_PER_DAY, &[]);
        assert_eq!(k.window_idx, 1);
    }

    #[test]
    fn rebuild_promotes_revived_item() {
        let window = 30 * SECONDS_PER_DAY;
        let original = WindowedRankKey::build(TimeBucket::Hour, window, 5 * SECONDS_PER_DAY, &[]);
        assert_eq!(original.window_idx, 0);
        assert!(original.is_expired(window, 35 * SECONDS_PER_DAY));

        let revived = WindowedRankKey::build(TimeBucket::Hour, window, 35 * SECONDS_PER_DAY, &[]);
        assert_eq!(revived.window_idx, 1);
        assert!(revived.to_string() > original.to_string());
    }

    #[test]
    fn factor_weighting_combines_with_freshness() {
        let window = 30 * SECONDS_PER_DAY;
        let now = 0; // freshness = full window in `Hour` units = 720
        let bare = WindowedRankKey::build(TimeBucket::Hour, window, now, &[]);
        let with_factors = WindowedRankKey::build(
            TimeBucket::Hour,
            window,
            now,
            &[Factor::new(12, 2), Factor::new(5, 1)],
        );
        assert_eq!(bare.score, 720);
        assert_eq!(with_factors.score, 720 + 12 * 2 + 5);
    }

    #[test]
    fn second_resolution_within_30_day_window() {
        let window = 30 * SECONDS_PER_DAY;
        // last_active right at window start -> full window of seconds remaining
        let k = WindowedRankKey::build(TimeBucket::Second, window, 0, &[]);
        assert_eq!(k.score, 30 * 24 * 3600);
        // round-trips even at this scale
        let s = k.to_string();
        let parsed: WindowedRankKey = s.parse().unwrap();
        assert_eq!(parsed, k);
    }

    #[test]
    fn base62_alphabet_is_ascii_sorted() {
        for w in BASE62_ALPHABET.windows(2) {
            assert!(w[0] < w[1], "alphabet not ascending at {:?}", w);
        }
    }
}
