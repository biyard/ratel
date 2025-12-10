use super::*;
use crate::*;

#[derive(Debug, Clone)]
pub struct Biyard {}

impl Biyard {
    pub fn new() -> Self {
        Self {}
    }

    fn convert_to_meta_user_id(user_pk: &Partition) -> String {
        match user_pk {
            Partition::User(id) => id.clone(),
            _ => panic!("Biyard user_pk must be of Partition::User type"),
        }
    }
    pub async fn award_points(
        &self,
        _user_pk: Partition,
        _points: i64,
        _description: String,
        _month: Option<String>,
    ) -> Result<AwardPointResponse> {
        Ok(AwardPointResponse {
            month: "2025-12".to_string(),
            transaction_id: "test-transaction-id".to_string(),
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
            bookmark: Some("test-bookmark".to_string()),
        })
    }
}
