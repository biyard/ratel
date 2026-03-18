use std::borrow::Cow;
use std::cell::RefCell;

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};

use crate::error::SamplingError;
use crate::types::ModelParams;

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_KEY_SIZE: u32 = 256;
const MAX_VALUE_SIZE: u32 = 512 * 1024;
const MAX_POLL_VALUE_SIZE: u32 = 1024 * 1024; // 1MB for ciphertext blobs

// Memory IDs for each StableBTreeMap
const MEMORY_ID_MODELS: MemoryId = MemoryId::new(0);
const MEMORY_ID_POLL_VOTES: MemoryId = MemoryId::new(1);
const MEMORY_ID_POLL_COUNTS: MemoryId = MemoryId::new(2);
const MEMORY_ID_POLL_VOTERS: MemoryId = MemoryId::new(3);

// ─── Storable Key/Value wrappers ───────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct StringKey(String);

impl Storable for StringKey {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(self.0.as_bytes().to_vec())
    }

    fn into_bytes(self) -> Vec<u8> {
        self.0.into_bytes()
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self(String::from_utf8(bytes.into_owned()).expect("invalid utf8 key"))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_KEY_SIZE,
        is_fixed_size: false,
    };
}

#[derive(Clone, Debug)]
struct StorableModelParams(ModelParams);

impl StorableModelParams {
    fn try_to_bytes(&self) -> Result<Vec<u8>, SamplingError> {
        candid::encode_one(&self.0).map_err(|e| SamplingError::EncodeFailed(e.to_string()))
    }

    fn try_from_bytes(bytes: &[u8]) -> Result<Self, SamplingError> {
        let params: ModelParams =
            candid::decode_one(bytes).map_err(|e| SamplingError::DecodeFailed(e.to_string()))?;
        Ok(Self(params))
    }
}

impl Storable for StorableModelParams {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(self.try_to_bytes().expect("encode ModelParams"))
    }

    fn into_bytes(self) -> Vec<u8> {
        self.try_to_bytes().expect("encode ModelParams")
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self::try_from_bytes(&bytes).expect("decode ModelParams")
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

// ─── Poll vote entry (Candid-encoded blob) ─────────────────────────

#[derive(Clone, Debug)]
struct StorablePollVote(Vec<u8>); // raw Candid-encoded QuestionVote

impl Storable for StorablePollVote {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(&self.0)
    }

    fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self(bytes.into_owned())
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_POLL_VALUE_SIZE,
        is_fixed_size: false,
    };
}

// ─── U64 storable for counts ───────────────────────────────────────

#[derive(Clone, Debug)]
struct StorableU64(u64);

impl Storable for StorableU64 {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(self.0.to_le_bytes().to_vec())
    }

    fn into_bytes(self) -> Vec<u8> {
        self.0.to_le_bytes().to_vec()
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        let arr: [u8; 8] = bytes.as_ref().try_into().expect("invalid u64 bytes");
        Self(u64::from_le_bytes(arr))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 8,
        is_fixed_size: true,
    };
}

// ─── Bool storable for voter dedup ─────────────────────────────────

#[derive(Clone, Debug)]
struct StorableBool(bool);

impl Storable for StorableBool {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(vec![self.0 as u8])
    }

    fn into_bytes(self) -> Vec<u8> {
        vec![self.0 as u8]
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self(bytes.as_ref().first().copied().unwrap_or(0) != 0)
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 1,
        is_fixed_size: true,
    };
}

// ─── Thread-local storage ──────────────────────────────────────────

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static MODELS: RefCell<StableBTreeMap<StringKey, StorableModelParams, Memory>> =
        RefCell::new(MEMORY_MANAGER.with(|mm| {
            StableBTreeMap::init(mm.borrow().get(MEMORY_ID_MODELS))
        }));

    // Poll votes: key = "{poll_sk}#{q_idx}#{opt_idx}#{voter_tag}"
    static POLL_VOTES: RefCell<StableBTreeMap<StringKey, StorablePollVote, Memory>> =
        RefCell::new(MEMORY_MANAGER.with(|mm| {
            StableBTreeMap::init(mm.borrow().get(MEMORY_ID_POLL_VOTES))
        }));

    // Poll counts: key = "{poll_sk}#{q_idx}#{opt_idx}"
    static POLL_COUNTS: RefCell<StableBTreeMap<StringKey, StorableU64, Memory>> =
        RefCell::new(MEMORY_MANAGER.with(|mm| {
            StableBTreeMap::init(mm.borrow().get(MEMORY_ID_POLL_COUNTS))
        }));

    // Voter dedup: key = "{poll_sk}#{voter_tag}"
    static POLL_VOTERS: RefCell<StableBTreeMap<StringKey, StorableBool, Memory>> =
        RefCell::new(MEMORY_MANAGER.with(|mm| {
            StableBTreeMap::init(mm.borrow().get(MEMORY_ID_POLL_VOTERS))
        }));
}

// ─── Model CRUD (existing) ─────────────────────────────────────────

pub fn save(id: &str, params: ModelParams) {
    MODELS.with(|m| {
        m.borrow_mut()
            .insert(StringKey(id.to_string()), StorableModelParams(params));
    });
}

pub fn load(id: &str) -> Option<ModelParams> {
    MODELS.with(|m| m.borrow().get(&StringKey(id.to_string())).map(|v| v.0))
}

// ─── Poll vote storage ─────────────────────────────────────────────

/// Check if a voter has already submitted for this poll.
pub fn poll_voter_exists(poll_sk: &str, voter_tag: &str) -> bool {
    let key = format!("{poll_sk}#{voter_tag}");
    POLL_VOTERS.with(|m| m.borrow().contains_key(&StringKey(key)))
}

/// Mark a voter as having submitted for this poll.
pub fn poll_voter_mark(poll_sk: &str, voter_tag: &str) {
    let key = format!("{poll_sk}#{voter_tag}");
    POLL_VOTERS.with(|m| {
        m.borrow_mut().insert(StringKey(key), StorableBool(true));
    });
}

/// Store an encrypted vote entry.
pub fn poll_vote_insert(
    poll_sk: &str,
    question_index: u32,
    option_index: u32,
    voter_tag: &str,
    encoded_vote: Vec<u8>,
) {
    let key = format!("{poll_sk}#{question_index}#{option_index}#{voter_tag}");
    POLL_VOTES.with(|m| {
        m.borrow_mut()
            .insert(StringKey(key), StorablePollVote(encoded_vote));
    });
}

/// Increment the count for a question option.
pub fn poll_count_increment(poll_sk: &str, question_index: u32, option_index: u32) {
    let key = format!("{poll_sk}#{question_index}#{option_index}");
    POLL_COUNTS.with(|m| {
        let mut map = m.borrow_mut();
        let current = map
            .get(&StringKey(key.clone()))
            .map(|v| v.0)
            .unwrap_or(0);
        map.insert(StringKey(key), StorableU64(current + 1));
    });
}

/// Get all vote counts for a poll. Returns (question_index, option_index, count) tuples.
pub fn poll_counts_by_poll(poll_sk: &str) -> Vec<(u32, u32, u64)> {
    let prefix = format!("{poll_sk}#");
    POLL_COUNTS.with(|m| {
        let map = m.borrow();
        let mut results = Vec::new();
        for entry in map.iter() {
            let key_str = &entry.key().0;
            if key_str.starts_with(&prefix) {
                let rest = &key_str[prefix.len()..];
                if let Some((qi_str, oi_str)) = rest.split_once('#') {
                    if let (Ok(qi), Ok(oi)) = (qi_str.parse::<u32>(), oi_str.parse::<u32>()) {
                        results.push((qi, oi, entry.value().0));
                    }
                }
            }
        }
        results
    })
}

/// Get all vote entries for a specific voter in a poll.
pub fn poll_votes_by_voter(poll_sk: &str, voter_tag: &str) -> Vec<(u32, u32, Vec<u8>)> {
    let suffix = format!("#{voter_tag}");
    let prefix = format!("{poll_sk}#");
    POLL_VOTES.with(|m| {
        let map = m.borrow();
        let mut results = Vec::new();
        for entry in map.iter() {
            let key_str = &entry.key().0;
            if key_str.starts_with(&prefix) && key_str.ends_with(&suffix) {
                let middle = &key_str[prefix.len()..key_str.len() - suffix.len()];
                if let Some((qi_str, oi_str)) = middle.split_once('#') {
                    if let (Ok(qi), Ok(oi)) = (qi_str.parse::<u32>(), oi_str.parse::<u32>()) {
                        results.push((qi, oi, entry.value().0.clone()));
                    }
                }
            }
        }
        results
    })
}
