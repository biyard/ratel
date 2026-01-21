use super::*;
use crate::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// This is a mock implementation of the Biyard service
// It uses in-memory storage for user balances and transactions
// All Request will Succeed
#[derive(Debug, Clone, Default)]
struct BiyardStorage {
    user_balances: HashMap<String, HashMap<String, UserBalance>>,
    transactions: Vec<Transaction>,
}
#[derive(Debug, Clone)]
struct UserBalance {
    balance: i64,
    total_earned: i64,
    total_spent: i64,
}

#[derive(Debug, Clone)]
struct Transaction {
    user_id: String,
    month: String,
    transaction_type: String,
    amount: i64,
    description: String,
    created_at: i64,
}

fn current_month() -> String {
    chrono::Utc::now().format("%Y-%m").to_string()
}

fn get_timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}

#[derive(Debug, Clone)]
pub struct Biyard {
    storage: Arc<RwLock<BiyardStorage>>,
}
impl Biyard {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(BiyardStorage::default())),
        }
    }
    fn convert_to_meta_user_id(user_pk: &Partition) -> String {
        match user_pk {
            Partition::User(id) => id.clone(),
            Partition::Team(id) => id.clone(),
            _ => panic!("Biyard user_pk must be of Partition::User or Partition::Team type"),
        }
    }

    /// Get token info (mock - always returns same data)
    pub async fn get_project_info(&self) -> Result<TokenResponse> {
        Ok(TokenResponse {
            pk: "RATEL".to_string(),
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
        user_pk: Partition,
        points: i64,
        description: String,
        month: Option<String>,
    ) -> Result<AwardPointResponse> {
        let user_id = Self::convert_to_meta_user_id(&user_pk);
        let month = month.unwrap_or_else(current_month);
        let now = get_timestamp();

        let mut storage = self.storage.write().await;

        let user_months = storage.user_balances.entry(user_id.clone()).or_default();
        let balance = user_months.entry(month.clone()).or_insert(UserBalance {
            balance: 0,
            total_earned: 0,
            total_spent: 0,
        });

        if points >= 0 {
            balance.balance += points;
            balance.total_earned += points;
        } else {
            balance.balance += points;
            balance.total_spent += points.abs();
        }

        storage.transactions.push(Transaction {
            user_id: user_id.clone(),
            month: month.clone(),
            transaction_type: if points >= 0 {
                "Award".into()
            } else {
                "Deduct".into()
            },
            amount: points,
            description,
            created_at: now,
        });

        Ok(TransactPointResponse {
            transaction_id: format!("tx-{}-{}", user_id, now),
            month,
            meta_user_id: user_id,
            transaction_type: "Award".into(),
            amount: points,
        })
    }

    pub async fn get_user_balance(
        &self,
        user_pk: Partition,
        month: String,
    ) -> Result<UserPointBalanceResponse> {
        let user_id = Self::convert_to_meta_user_id(&user_pk);

        // 3. 읽기 잠금 획득 (여러 명이 동시에 읽기 가능)
        let storage = self.storage.read().await;

        let (balance, total_earned, total_spent) = storage
            .user_balances
            .get(&user_id)
            .and_then(|m| m.get(&month))
            .map(|b| (b.balance, b.total_earned, b.total_spent))
            .unwrap_or((0, 0, 0));

        let project_total: i64 = storage
            .user_balances
            .values()
            .flat_map(|m| m.get(&month))
            .map(|b| b.total_earned)
            .sum();

        Ok(UserPointBalanceResponse {
            month,
            balance,
            total_earned,
            total_spent,
            updated_at: get_timestamp(),
            project_total_points: project_total,
            monthly_token_supply: 10000,
        })
    }

    pub async fn list_user_transactions(
        &self,
        user_pk: Partition,
        month: String,
        _bookmark: Option<String>,
        limit: Option<i32>,
    ) -> Result<ListItemsResponse<UserPointTransactionResponse>> {
        let user_id = Self::convert_to_meta_user_id(&user_pk);
        let limit = limit.unwrap_or(50) as usize;

        let storage = self.storage.read().await;
        let items: Vec<UserPointTransactionResponse> = storage
            .transactions
            .iter()
            .filter(|tx| tx.user_id == user_id && tx.month == month)
            .take(limit)
            .map(|tx| UserPointTransactionResponse {
                month: tx.month.clone(),
                transaction_type: tx.transaction_type.clone(),
                amount: tx.amount,
                target_user_id: Some(tx.user_id.clone()),
                description: Some(tx.description.clone()),
                created_at: tx.created_at,
            })
            .collect();

        Ok(ListItemsResponse {
            items,
            bookmark: None,
        })
    }

    pub async fn exchange_points(
        &self,
        user_pk: Partition,
        amount: i64,
        month: String,
    ) -> Result<TransactPointResponse> {
        let user_id = Self::convert_to_meta_user_id(&user_pk);

        // Deduct points
        {
            let mut storage = self.storage.write().await;
            if let Some(user_months) = storage.user_balances.get_mut(&user_id) {
                if let Some(balance) = user_months.get_mut(&month) {
                    if balance.balance < amount {
                        return Err(Error::InternalServerError(
                            "Insufficient balance".to_string(),
                        ));
                    }
                    balance.balance -= amount;
                    balance.total_spent += amount;
                }
            }
        }

        Ok(TransactPointResponse {
            transaction_id: format!("exchange-{}-{}", user_id, get_timestamp()),
            month,
            meta_user_id: user_id,
            transaction_type: "Exchange".to_string(),
            amount,
        })
    }

    pub async fn get_token_balance(&self, user_pk: Partition) -> Result<TokenBalanceResponse> {
        let user_id = Self::convert_to_meta_user_id(&user_pk);

        Ok(TokenBalanceResponse {
            project_id: "PROJECT#ratel".to_string(),
            meta_user_id: user_id,
            balance: 1000,
            created_at: 1750000000,
            updated_at: get_timestamp(),
        })
    }

    pub async fn mint_tokens(
        &self,
        user_pk: Partition,
        amount: i64,
    ) -> Result<TokenBalanceResponse> {
        let user_id = Self::convert_to_meta_user_id(&user_pk);

        Ok(TokenBalanceResponse {
            project_id: "PROJECT#ratel".to_string(),
            meta_user_id: user_id,
            balance: amount,
            created_at: 1750000000,
            updated_at: get_timestamp(),
        })
    }

    pub async fn list_transactions(
        &self,
        _date: Option<String>,
        _bookmark: Option<String>,
        limit: Option<i32>,
    ) -> Result<ListItemsResponse<ProjectPointTransactionResponse>> {
        let limit = limit.unwrap_or(50) as usize;

        let storage = self.storage.read().await;
        let items: Vec<ProjectPointTransactionResponse> = storage
            .transactions
            .iter()
            .take(limit)
            .map(|tx| ProjectPointTransactionResponse {
                meta_user_id: tx.user_id.clone(),
                month: tx.month.clone(),
                transaction_type: tx.transaction_type.clone(),
                amount: tx.amount,
                target_user_id: None,
                description: Some(tx.description.clone()),
                created_at: tx.created_at,
            })
            .collect();

        Ok(ListItemsResponse {
            items,
            bookmark: None,
        })
    }
}
