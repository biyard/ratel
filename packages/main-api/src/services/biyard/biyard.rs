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
        let path = format!("{}/v1/projects/{}/points", self.base_url, self.project_id);
        let body = vec![TransactPointRequest {
            tx_type: "Award".to_string(),
            to: Some(Self::convert_to_meta_user_id(&user_pk)),
            from: None,
            amount: points,
            description: Some(description),
            month,
        }];

        let res = self.cli.post(&path).json(&body).send().await?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(Error::Unknown(format!(
                "Biyard API error: {} - {}",
                status, error_text
            )));
        }

        let responses: Vec<TransactPointResponse> = res.json().await?;
        responses.into_iter().next().ok_or_else(|| {
            Error::Unknown("Biyard API returned empty response".to_string())
        })
    }

    pub async fn get_user_balance(
        &self,
        user_pk: Partition,
        month: String,
    ) -> Result<UserPointBalanceResponse> {
        let path = format!(
            "{}/v1/projects/{}/points/{}?month={}",
            self.base_url,
            self.project_id,
            Self::convert_to_meta_user_id(&user_pk),
            month
        );

        let res = self.cli.get(&path).send().await?;

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(Error::Unknown(format!(
                "Biyard API error: {} - {}",
                status, error_text
            )));
        }

        let list_response: ListItemsResponse<UserPointBalanceResponse> = res.json().await?;
        list_response.items.into_iter().next().ok_or_else(|| {
            Error::Unknown("No balance found for the specified month".to_string())
        })
    }

    pub async fn get_user_transactions(
        &self,
        user_pk: Partition,
        month: String,
        bookmark: Option<String>,
        limit: Option<i32>,
    ) -> Result<ListItemsResponse<UserPointTransactionResponse>> {
        let mut path = format!(
            "{}/v1/projects/{}/points/{}/transactions?month={}",
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

        if !res.status().is_success() {
            let status = res.status();
            let error_text = res.text().await.unwrap_or_default();
            return Err(Error::Unknown(format!(
                "Biyard API error: {} - {}",
                status, error_text
            )));
        }

        let transactions = res.json().await?;
        Ok(transactions)
    }
}
