use crate::*;
use bdk::prelude::*;

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct SignedAttribute {
    pub key: String,
    pub value: serde_json::Value,
    pub signature: String,
    pub verification_method: String,
    pub signed_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct AttributeIssuanceResponse {
    pub signed_attributes: Vec<SignedAttribute>,
    pub issuer_did: String,
    pub issuer_did_document_url: String,
    pub credential_schema: Option<String>,
}
