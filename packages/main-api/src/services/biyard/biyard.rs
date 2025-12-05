use super::*;
use crate::*;

#[derive(Debug, Clone)]
pub struct Biyard {
    project_id: String,
    base_url: String,
    cli: reqwest::Client,
}

impl Biyard {
    pub fn new() -> Self {
        let biyard_conf = config::get().biyard;
        let cli = reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "Authorization",
                    reqwest::header::HeaderValue::from_str(&format!(
                        "Bearer {}",
                        biyard_conf.api_secret
                    ))
                    .unwrap(),
                );
                headers
            })
            .build()
            .unwrap();
        Self {
            cli,
            base_url: biyard_conf.base_url.to_string(),
            project_id: biyard_conf.project_id.to_string(),
        }
    }

    fn convert_to_meta_user_id(user_pk: &Partition) -> String {
        match user_pk {
            Partition::User(id) => id.clone(),
            _ => panic!("Biyard user_pk must be of Partition::User type"),
        }
    }
    pub async fn award_points(
        &self,
        user_pk: Partition,
        points: i64,
        description: String,
        month: Option<String>,
    ) -> Result<AwardPointResponse> {
        //TODO: Validate Month format: "YYYY-MM"

        let path = format!("{}/projects/{}/points", self.base_url, self.project_id);
        let body = AwardPointRequest {
            tx_type: "Award".to_string(),
            to: Self::convert_to_meta_user_id(&user_pk),
            amount: points,
            description,
            month,
        };

        let res = self.cli.post(&path).json(&body).send().await?;
        Ok(res.json().await?)
    }

    pub async fn get_user_balance(
        &self,
        user_pk: Partition,
        month: String,
    ) -> Result<UserPointBalanceResponse> {
        let path = format!(
            "{}/projects/{}/points/{}??month={}",
            self.base_url,
            self.project_id,
            Self::convert_to_meta_user_id(&user_pk),
            month
        );

        let res = self.cli.get(&path).send().await?;
        Ok(res.json().await?)
    }

    pub async fn get_user_transactions(
        &self,
        user_pk: Partition,
        month: String,
        bookmark: Option<String>,
        limit: Option<i32>,
    ) -> Result<ListItemsResponse<UserPointTransactionResponse>> {
        let mut path = format!(
            "{}/projects/{}/points/{}/transactions?month={}",
            self.base_url,
            self.project_id,
            Self::convert_to_meta_user_id(&user_pk),
            month
        );
        if let Some(bm) = bookmark {
            path = format!("{}&bookmark={}", path, bm);
        }
        if let Some(lim) = limit {
            path = format!("{}&limit={}", path, lim);
        }

        let res = self.cli.get(&path).send().await?;
        let transactions = res.json().await?;
        Ok(transactions)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]

pub struct AwardPointRequest {
    pub tx_type: String,
    pub to: String,
    pub amount: i64,
    pub description: String,
    pub month: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]

pub struct AwardPointResponse {
    pub month: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]

pub struct UserPointTransactionResponse {
    pub month: String,
    pub transaction_type: String,
    pub amount: i64,
    pub target_user_id: Option<String>,
    pub description: Option<String>,
    pub created_at: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]

pub struct UserPointBalanceResponse {
    pub month: String,
    pub balance: i64,
    pub total_earned: i64,
    pub total_spent: i64,
    pub updated_at: i64,
}
