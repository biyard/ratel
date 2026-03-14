use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotePayload {
    pub choice: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct UserAttributes(pub Vec<String>);

impl UserAttributes {
    pub fn authority() -> Self {
        Self(vec!["ratel-authority".to_string()])
    }

    pub fn voter(id: &str) -> Self {
        Self(vec![format!("voter-{id}")])
    }

    pub fn authority_and_voter(id: &str) -> Self {
        Self(vec!["ratel-authority".to_string(), format!("voter-{id}")])
    }

    pub fn as_slice(&self) -> &[String] {
        &self.0
    }
}
