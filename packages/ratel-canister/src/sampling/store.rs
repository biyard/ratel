use super::types::ModelParams;
use crate::canister::storage::{StorableModelParams, StringKey, SAMPLING_MODELS};

pub(crate) struct ModelStore;

impl ModelStore {
    pub fn save(id: &str, params: ModelParams) {
        SAMPLING_MODELS.with(|m| {
            m.borrow_mut()
                .insert(StringKey(id.to_string()), StorableModelParams(params));
        });
    }

    pub fn load(id: &str) -> Option<ModelParams> {
        SAMPLING_MODELS.with(|m| m.borrow().get(&StringKey(id.to_string())).map(|v| v.0))
    }
}
