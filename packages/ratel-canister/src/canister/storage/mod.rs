use std::borrow::Cow;
use std::cell::RefCell;

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

    // 투표 1건당 키 1개: (vote_key + voter_tag) → ballot 전체(암호문 포함). O(1) 저장.
    pub(crate) static BALLOTS: RefCell<StableBTreeMap<StringKey, StorableBallot, Memory>> =
        RefCell::new(MEMORY_MANAGER.with(|mm| {
            StableBTreeMap::init(mm.borrow().get(MEMORY_ID_VOTES))
        }));

    // 집계 카운터: (vote_key + question + option) → 득표 수. get_vote_counts 결과 보존.
    pub(crate) static VOTE_COUNTS: RefCell<StableBTreeMap<StringKey, u64, Memory>> =
        RefCell::new(MEMORY_MANAGER.with(|mm| {
            StableBTreeMap::init(mm.borrow().get(MEMORY_ID_VOTE_COUNTS))
        }));
}
