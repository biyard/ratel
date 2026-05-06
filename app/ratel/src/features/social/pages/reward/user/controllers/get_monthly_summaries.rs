use super::super::*;
use crate::common::services::MonthlySummariesResponse;

#[get("/api/users/points/monthly-summaries?username")]
pub async fn get_monthly_summaries_handler(
    username: String,
) -> Result<MonthlySummariesResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let (users, _) = crate::features::auth::User::find_by_username(
        cli,
        &username,
        crate::features::auth::User::opt()
            .sk("TS#".to_string())
            .limit(1),
    )
    .await?;
    let user = users
        .into_iter()
        .find(|u| u.username == username)
        .ok_or(Error::NotFound(format!("User not found: {}", username)))?;

    let biyard = cfg.biyard();
    biyard.get_monthly_summaries(user.pk.clone()).await
}
