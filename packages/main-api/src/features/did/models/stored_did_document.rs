use crate::features::did::types::*;
use crate::types::*;
use crate::utils::time::get_now_timestamp;
use bdk::prelude::*;

/// Stored DID Document in DynamoDB
/// PK: DID#{did} (e.g., "DID#did:web:example.com")
/// SK: "DidDocument"
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema)]
#[dynamo(table = "main", pk = "pk", sk = "sk")]
pub struct StoredDidDocument {
    /// Partition key: DID#{did}
    pub pk: Partition,

    /// Sort key: DidDocument
    pub sk: EntityType,

    /// The full DID string
    pub did: String,

    /// The DID method (web, key, plc, etc.)
    pub method: DidMethod,

    /// The DID document content
    pub document: DidDocument,

    /// User who owns/controls this DID
    pub owner_pk: Partition,

    /// Creation timestamp
    pub created_at: i64,

    /// Last update timestamp
    pub updated_at: i64,

    /// Whether this DID is active or deactivated
    pub is_active: bool,
}

impl StoredDidDocument {
    /// Create a new stored DID document
    pub fn new(did: String, method: DidMethod, document: DidDocument, owner_pk: Partition) -> Self {
        let now = get_now_timestamp();
        let pk = Partition::Did(did.clone());
        let sk = EntityType::DidDocument;

        Self {
            pk,
            sk,
            did,
            method,
            document,
            owner_pk,
            created_at: now,
            updated_at: now,
            is_active: true,
        }
    }

    /// Get a DID document by DID string
    pub async fn get_by_did(
        cli: &aws_sdk_dynamodb::Client,
        did: &str,
    ) -> Result<Option<Self>, crate::Error> {
        let pk = Partition::Did(did.to_string());
        Self::get(cli, &pk, Some(&EntityType::DidDocument)).await
    }

    /// Update the DID document
    pub async fn update_document(
        &mut self,
        cli: &aws_sdk_dynamodb::Client,
        document: DidDocument,
    ) -> Result<(), crate::Error> {
        self.document = document;
        self.updated_at = get_now_timestamp();

        let updater = Self::updater(&self.pk, &self.sk)
            .with_document(self.document.clone())
            .with_updated_at(self.updated_at);

        updater.execute(cli).await?;
        Ok(())
    }

    /// Deactivate the DID
    pub async fn deactivate(&mut self, cli: &aws_sdk_dynamodb::Client) -> Result<(), crate::Error> {
        self.is_active = false;
        self.updated_at = get_now_timestamp();

        let updater = Self::updater(&self.pk, &self.sk)
            .with_is_active(false)
            .with_updated_at(self.updated_at);

        updater.execute(cli).await?;
        Ok(())
    }

    /// Check if user owns this DID
    pub fn is_owned_by(&self, user_pk: &Partition) -> bool {
        &self.owner_pk == user_pk
    }
}
