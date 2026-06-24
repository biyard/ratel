use crate::common::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

/// A reviewer/QA test account. App-store and Play-store reviewers can't receive
/// real verification emails, so listed emails skip email delivery and accept the
/// fixed verification code "000000" at login — at runtime, without the
/// compile-time `bypass` feature (which is never shipped to production).
///
/// Singleton partition (`pk = TestAccount`) keyed by email in the sort key, so
/// the full list is a single-partition query and existence is a point `get`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, DynamoEntity, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct TestAccount {
    pub pk: Partition,
    pub sk: EntityType,

    pub email: String,
    /// Free-form admin note, e.g. "App Store reviewer".
    pub note: String,
    pub created_at: i64,
}

#[cfg(feature = "server")]
impl TestAccount {
    pub fn new(email: String, note: String) -> Self {
        Self {
            pk: Partition::TestAccount,
            sk: EntityType::TestAccount(email.clone()),
            email,
            note,
            created_at: crate::common::utils::time::now(),
        }
    }

    /// True when `email` is registered as a reviewer test account. Email match
    /// is case-insensitive (emails are stored lowercased by the controller).
    pub async fn is_test_account(cli: &aws_sdk_dynamodb::Client, email: &str) -> bool {
        let key = EntityType::TestAccount(email.trim().to_lowercase());
        matches!(
            Self::get(cli, Partition::TestAccount, Some(key)).await,
            Ok(Some(_))
        )
    }
}
