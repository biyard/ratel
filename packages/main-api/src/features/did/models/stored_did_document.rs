use crate::features::did::{generate_did_by_username, types::*};
use crate::models::User;
use crate::types::*;
use crate::utils::time::{get_now_timestamp, get_now_timestamp_millis};

use crate::*;
use ssi::dids::Document;

/// Stored DID Document in DynamoDB
/// PK: DID#{did} (e.g., "DID#did:web:example.com")
/// SK: "DidDocument"
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema)]
#[dynamo(table = "main", pk = "pk", sk = "sk")]
pub struct StoredDidDocument {
    /// Partition key: DID#{did}
    pub pk: CompositePartition,

    /// Sort key: DidDocument
    pub sk: EntityType,

    /// The full DID string
    #[dynamo(index = "gsi1", name = "find_by_did", pk)]
    pub did: String,

    /// The DID document content
    pub document: Option<Document>,

    /// User who owns/controls this DID
    pub owner_pk: Partition,

    /// Creation timestamp
    #[dynamo(index = "gsi1", name = "find_by_did", sk)]
    pub created_at: i64,

    /// Last update timestamp
    pub updated_at: i64,

    /// Whether this DID is active or deactivated
    pub is_active: bool,
}

impl StoredDidDocument {
    /// Create a new stored DID document
    pub fn new(user_pk: Partition, username: String) -> Result<Self> {
        if !matches!(user_pk, Partition::User(_)) {
            panic!("user_pk must be of type Partition::User");
        }

        let now = get_now_timestamp_millis();
        let sk = EntityType::DidDocument;
        let document = generate_did_by_username(&username)?;
        let did = format!("{}", document.id);

        Ok(Self {
            pk: CompositePartition(user_pk.clone(), Partition::Did),
            sk,
            did,
            document: Some(document),
            owner_pk: user_pk,
            created_at: now,
            updated_at: now,
            is_active: true,
        })
    }
}
