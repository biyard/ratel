use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};

use crate::sampling::error::SamplingError;
use crate::sampling::types::ModelParams;
use crate::voting::error::VotingError;
use crate::voting::store::VoterBallotData;

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_KEY_SIZE: u32 = 256;
const MAX_MODEL_VALUE_SIZE: u32 = 512 * 1024;
const MAX_VOTE_VALUE_SIZE: u32 = 2 * 1024 * 1024;

const MEMORY_ID_MODELS: MemoryId = MemoryId::new(0);
const MEMORY_ID_VOTES: MemoryId = MemoryId::new(1);
const MEMORY_ID_VOTE_COUNTS: MemoryId = MemoryId::new(2);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct StringKey(pub(crate) String);

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
pub(crate) struct StorableModelParams(pub(crate) ModelParams);

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
        max_size: MAX_MODEL_VALUE_SIZE,
        is_fixed_size: false,
    };
}

#[derive(Clone, Debug)]
pub(crate) struct StorableBallot(pub(crate) VoterBallotData);

impl StorableBallot {
    fn try_to_bytes(&self) -> Result<Vec<u8>, VotingError> {
        serde_cbor::to_vec(&self.0).map_err(|e| VotingError::EncodeFailed(e.to_string()))
    }

    fn try_from_bytes(bytes: &[u8]) -> Result<Self, VotingError> {
        let data: VoterBallotData =
            serde_cbor::from_slice(bytes).map_err(|e| VotingError::DecodeFailed(e.to_string()))?;
        Ok(Self(data))
    }
}

impl Storable for StorableBallot {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(self.try_to_bytes().expect("encode VoterBallotData"))
    }

    fn into_bytes(self) -> Vec<u8> {
        self.try_to_bytes().expect("encode VoterBallotData")
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self::try_from_bytes(&bytes).expect("decode VoterBallotData")
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VOTE_VALUE_SIZE,
        is_fixed_size: false,
    };
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub(crate) static SAMPLING_MODELS: RefCell<StableBTreeMap<StringKey, StorableModelParams, Memory>> =
        RefCell::new(MEMORY_MANAGER.with(|mm| {
            StableBTreeMap::init(mm.borrow().get(MEMORY_ID_MODELS))
        }));

    // ── 핫패스(heap, RAM) ─────────────────────────────────────────────
    // 투표 처리는 stable memory(디스크) 대신 heap 에서 수행 → 직렬화/페이지 I/O 없이 빠름.
    // 콘텐츠(암호문/집계)는 stable 버전과 동일, 저장 위치만 RAM. 업그레이드 보존은 아래 pre/post_upgrade.
    // 투표 1건당 키 1개: (vote_key + voter_tag) → ballot 전체(암호문 포함).
    pub(crate) static BALLOTS: RefCell<HashMap<String, VoterBallotData>> =
        RefCell::new(HashMap::new());

    // 집계 카운터: (vote_key + question + option) → 득표 수.
    pub(crate) static VOTE_COUNTS: RefCell<HashMap<String, u64>> =
        RefCell::new(HashMap::new());

    // ── 업그레이드 보존용 stable 백업 ─────────────────────────────────
    // 평소엔 비어 있고, pre_upgrade 때 heap 을 여기로 flush → 업그레이드 후 post_upgrade 가 heap 으로 복원.
    pub(crate) static BALLOTS_STABLE: RefCell<StableBTreeMap<StringKey, StorableBallot, Memory>> =
        RefCell::new(MEMORY_MANAGER.with(|mm| {
            StableBTreeMap::init(mm.borrow().get(MEMORY_ID_VOTES))
        }));

    pub(crate) static VOTE_COUNTS_STABLE: RefCell<StableBTreeMap<StringKey, u64, Memory>> =
        RefCell::new(MEMORY_MANAGER.with(|mm| {
            StableBTreeMap::init(mm.borrow().get(MEMORY_ID_VOTE_COUNTS))
        }));
}

/// 업그레이드 직전: heap(BALLOTS/VOTE_COUNTS) → stable 백업으로 통째 flush.
/// (벤치마크는 install 만 하고 업그레이드하지 않으므로 평소엔 호출되지 않는다.)
#[cfg(feature = "canister")]
#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    BALLOTS.with(|h| {
        BALLOTS_STABLE.with(|s| {
            let mut s = s.borrow_mut();
            for (k, v) in h.borrow().iter() {
                s.insert(StringKey(k.clone()), StorableBallot(v.clone()));
            }
        });
    });
    VOTE_COUNTS.with(|h| {
        VOTE_COUNTS_STABLE.with(|s| {
            let mut s = s.borrow_mut();
            for (k, v) in h.borrow().iter() {
                s.insert(StringKey(k.clone()), *v);
            }
        });
    });
}

/// 업그레이드 직후: stable 백업 → heap 으로 복원 (콘텐츠 보존).
#[cfg(feature = "canister")]
#[ic_cdk::post_upgrade]
fn post_upgrade() {
    BALLOTS_STABLE.with(|s| {
        BALLOTS.with(|h| {
            let mut h = h.borrow_mut();
            for entry in s.borrow().iter() {
                h.insert(entry.key().0.clone(), entry.value().0);
            }
        });
    });
    VOTE_COUNTS_STABLE.with(|s| {
        VOTE_COUNTS.with(|h| {
            let mut h = h.borrow_mut();
            for entry in s.borrow().iter() {
                h.insert(entry.key().0.clone(), entry.value());
            }
        });
    });
}
