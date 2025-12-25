use super::*;
use crate::*;

#[derive(Debug, Clone)]
pub struct Biyard;

impl Biyard {
    pub fn new() -> Self {
        Self
    }

    #[allow(dead_code)]
    fn convert_to_meta_user_id(user_pk: &Partition) -> String {
        match user_pk {
            Partition::User(id) => id.clone(),
            _ => panic!("Biyard user_pk must be of Partition::User type"),
        }
    }

    /// Get token info (mock - always returns same data)
    pub async fn get_token(&self) -> Result<TokenResponse> {
        Ok(TokenResponse {
            project_id: "PROJECT#ratel".to_string(),
            name: "Ratel Token".to_string(),
            symbol: "RATEL".to_string(),
            decimals: 18,
            total_supply: 1000000,
            circulating_supply: 500000,
            description: Some("Ratel governance token".to_string()),
            created_at: 1750000000,
            updated_at: 1750000000,
        })
    }

    pub async fn award_points(
        &self,
        _user_pk: Partition,
        _points: i64,
        _description: String,
        _month: Option<String>,
    ) -> Result<AwardPointResponse> {
        Ok(TransactPointResponse {
            transaction_id: "test-transaction-id".to_string(),
            month: "2025-12".to_string(),
            meta_user_id: "test-user-id".to_string(),
            transaction_type: "Award".to_string(),
            amount: 100,
        })
    }

    pub async fn get_user_balance(
        &self,
        _user_pk: Partition,
        _month: String,
    ) -> Result<UserPointBalanceResponse> {
        Ok(UserPointBalanceResponse {
            month: "2025-12".to_string(),
            balance: 100,
            total_earned: 100,
            total_spent: 0,
            updated_at: 1750000000,
            project_total_points: 100000,
            monthly_token_supply: 10000,
        })
    }

    pub async fn get_user_transactions(
        &self,
        _user_pk: Partition,
        _month: String,
        _bookmark: Option<String>,
        _limit: Option<i32>,
    ) -> Result<ListItemsResponse<UserPointTransactionResponse>> {
        Ok(ListItemsResponse {
            items: vec![UserPointTransactionResponse {
                month: "2025-12".to_string(),
                transaction_type: "Award".to_string(),
                amount: 100,
                target_user_id: Some("test-user-id".to_string()),
                description: Some("test-description".to_string()),
                created_at: 1750000000,
            }],
            bookmark: None,
        })
    }

    pub async fn exchange_points(
        &self,
        _user_pk: Partition,
        amount: i64,
        month: String,
    ) -> Result<TransactPointResponse> {
        Ok(TransactPointResponse {
            transaction_id: "test-exchange-transaction-id".to_string(),
            month,
            meta_user_id: "test-user-id".to_string(),
            transaction_type: "Exchange".to_string(),
            amount,
        })
    }

    pub async fn get_token_balance(&self, _user_pk: Partition) -> Result<TokenBalanceResponse> {
        Ok(TokenBalanceResponse {
            project_id: "PROJECT#ratel".to_string(),
            meta_user_id: "test-user-id".to_string(),
            balance: 1000,
            created_at: 1750000000,
            updated_at: 1750000000,
        })
    }

    pub async fn mint_tokens(
        &self,
        _user_pk: Partition,
        amount: i64,
    ) -> Result<TokenBalanceResponse> {
        Ok(TokenBalanceResponse {
            project_id: "PROJECT#ratel".to_string(),
            meta_user_id: "test-user-id".to_string(),
            balance: amount,
            created_at: 1750000000,
            updated_at: 1750000000,
        })
    }
}
