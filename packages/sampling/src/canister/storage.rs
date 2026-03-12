use std::borrow::Cow;
use std::cell::RefCell;

use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};

use crate::error::SamplingError;
use crate::types::ModelParams;

type Memory = DefaultMemoryImpl;

const MAX_KEY_SIZE: u32 = 256;
const MAX_VALUE_SIZE: u32 = 512 * 1024;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ModelKey(String);

impl Storable for ModelKey {
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

thread_local! {
    static MODELS: RefCell<StableBTreeMap<ModelKey, StorableModelParams, Memory>> =
        RefCell::new(StableBTreeMap::init(DefaultMemoryImpl::default()));
}

pub fn save(id: &str, params: ModelParams) {
    MODELS.with(|m| {
        m.borrow_mut()
            .insert(ModelKey(id.to_string()), StorableModelParams(params));
    });
}

pub fn load(id: &str) -> Option<ModelParams> {
    MODELS.with(|m| m.borrow().get(&ModelKey(id.to_string())).map(|v| v.0))
}
