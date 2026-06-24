use crate::common::models::auth::{AdminUser, TestAccount};
use crate::features::admin::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTestAccountRequest {
    pub email: String,
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TestAccountResponse {
    pub email: String,
    pub note: String,
    pub created_at: i64,
}

#[cfg(feature = "server")]
impl From<TestAccount> for TestAccountResponse {
    fn from(t: TestAccount) -> Self {
        Self {
            email: t.email,
            note: t.note,
            created_at: t.created_at,
        }
    }
}

/// Register a reviewer test account. Listed emails log in with code "000000"
/// and never receive a real verification email. Admin only.
#[post("/api/admin/test-accounts", _user: AdminUser)]
pub async fn create_test_account(req: CreateTestAccountRequest) -> Result<TestAccountResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let email = req.email.trim().to_lowercase();
    if email.is_empty() || !email.contains('@') {
        return Err(Error::InvalidEmail);
    }

    let account = TestAccount::new(email, req.note.trim().to_string());
    account.create(cli).await?;
    Ok(account.into())
}

/// List all reviewer test accounts. Admin only.
#[get("/api/admin/test-accounts", _user: AdminUser)]
pub async fn list_test_accounts() -> Result<Vec<TestAccountResponse>> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let (items, _) =
        TestAccount::query(cli, Partition::TestAccount, TestAccount::opt().limit(200)).await?;
    Ok(items.into_iter().map(TestAccountResponse::from).collect())
}

/// Remove a reviewer test account by email. Admin only.
#[delete("/api/admin/test-accounts/:email", _user: AdminUser)]
pub async fn delete_test_account(email: String) -> Result<TestAccountResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let email = email.trim().to_lowercase();
    TestAccount::delete(cli, Partition::TestAccount, Some(EntityType::TestAccount(email)))
        .await
        .map(TestAccountResponse::from)
}
