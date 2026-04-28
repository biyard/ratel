//! Shared shard-key derivation for the two cross-posting sweeper GSIs.
//!
//! All call sites (Stage 1 enqueue, Stage 2 success / failure paths, and
//! both 1D sweepers) MUST go through `shard_for(post_id)` so the shard
//! mapping is identical everywhere. Inline `hash(post_id) % N` at multiple
//! call sites is forbidden.
//!
//! The hash function is **SHA-256** (not because we need cryptographic
//! collision resistance — we don't — but because `sha2` is already a
//! workspace dependency and produces a deterministic, process-independent
//! value across every Lambda invocation). `std::collections::hash_map::
//! DefaultHasher` is FORBIDDEN here because it uses a per-process random
//! seed and would produce different shards on different invocations.

use crate::features::cross_posting::models::SHARD_COUNT;
use sha2::{Digest, Sha256};

/// Compute the shard key for a given post id.
///
/// Format: `"SDS#{n}"` where `n = first 4 SHA-256 bytes of post_id, mod
/// SHARD_COUNT`. Result is byte-identical across processes / hosts /
/// architectures.
pub fn shard_for(post_id: &str) -> String {
    let digest = Sha256::digest(post_id.as_bytes());
    let head = u32::from_be_bytes([digest[0], digest[1], digest[2], digest[3]]);
    format!("SDS#{}", head % SHARD_COUNT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shard_for_is_deterministic() {
        let a = shard_for("post-abc-123");
        let b = shard_for("post-abc-123");
        assert_eq!(a, b);
    }

    #[test]
    fn shard_for_returns_value_in_range() {
        for i in 0..50 {
            let s = shard_for(&format!("post-{i}"));
            let n: u32 = s.strip_prefix("SDS#").unwrap().parse().unwrap();
            assert!(n < SHARD_COUNT, "shard index {n} out of range for SHARD_COUNT={SHARD_COUNT}");
        }
    }

    #[test]
    fn shard_for_distributes_across_buckets() {
        // With SHARD_COUNT=4 and 200 random ids, every shard should see at
        // least 20 hits — well below the ~50/shard expected uniform mean.
        let mut counts = vec![0u32; SHARD_COUNT as usize];
        for i in 0..200 {
            let s = shard_for(&format!("post-{i}-xyz"));
            let n: u32 = s.strip_prefix("SDS#").unwrap().parse().unwrap();
            counts[n as usize] += 1;
        }
        for (i, c) in counts.iter().enumerate() {
            assert!(*c >= 20, "shard {i} got only {c} hits — distribution skew");
        }
    }

    #[test]
    fn shard_for_handles_empty_string() {
        // Edge case: empty post_id should still produce a valid shard.
        let s = shard_for("");
        assert!(s.starts_with("SDS#"));
    }
}
