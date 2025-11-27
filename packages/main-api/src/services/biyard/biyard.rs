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

    pub async fn award_points(
        &self,
        user_id: &str,
        points: i64,
        month: Option<String>,
    ) -> Result<()> {
        //TODO: Validate Month format: "YYYY-MM"

        let path = format!("{}/projects/{}/points", self.base_url, self.project_id);
        let body = AwardPointRequest {
            tx_type: "Award".to_string(),
            to: user_id.to_string(),
            amount: points,
            description: "Reward points for user activity".to_string(),
            month,
        };

        let res = self.cli.post(&path).json(&body).send().await?;
        Ok(res.json().await?)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AwardPointRequest {
    pub tx_type: String,
    pub to: String,
    pub amount: i64,
    pub description: String,
    pub month: Option<String>,
}
