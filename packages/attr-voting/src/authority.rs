use rabe::schemes::bsw::{self, CpAbeMasterKey, CpAbePublicKey, CpAbeSecretKey};
use serde::{Deserialize, Serialize};

use crate::error::AttrVotingError;
use crate::types::UserAttributes;

#[derive(Debug, Serialize, Deserialize)]
pub struct VotingAuthority {
    pub public_key: CpAbePublicKey,
    pub master_key: CpAbeMasterKey,
}

impl VotingAuthority {
    pub fn setup() -> Self {
        let (public_key, master_key) = bsw::setup();
        Self {
            public_key,
            master_key,
        }
    }

    pub fn generate_user_key(
        &self,
        attrs: &UserAttributes,
    ) -> Result<CpAbeSecretKey, AttrVotingError> {
        let attr_refs: Vec<&str> = attrs.as_slice().iter().map(|s| s.as_str()).collect();
        bsw::keygen(&self.public_key, &self.master_key, &attr_refs)
            .ok_or_else(|| AttrVotingError::KeygenFailed("keygen returned None".to_string()))
    }

    pub fn to_json(&self) -> Result<String, AttrVotingError> {
        serde_json::to_string(self).map_err(AttrVotingError::SerializationError)
    }

    pub fn from_json(json: &str) -> Result<Self, AttrVotingError> {
        serde_json::from_str(json).map_err(AttrVotingError::SerializationError)
    }
}
