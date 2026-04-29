use crate::common::*;
use crate::features::auth::User;
use crate::features::cross_posting::models::SocialConnection;
use crate::features::cross_posting::types::{ConnectionResponse, CrossPostingError};

/// Lists every `SocialConnection` row for the authenticated user.
/// Includes Connected, AuthExpired, and Revoked rows alike — the UI
/// renders Revoked rows as "Not connected" (the row is kept so we can
/// resolve historical handles for past `SyndicationJob` references).
#[get("/api/cross-posting/connections", user: User)]
pub async fn list_connections_handler() -> Result<Vec<ConnectionResponse>> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    // Filter the partition by sk-prefix so only SocialConnection rows are
    // returned (the user's pk also contains User, McpClientSecret,
    // UserOnboardingFlags, etc.). At most 3 rows per user (Bluesky / LinkedIn
    // / Threads), so a small limit is sufficient and pagination is not used.
    let sk_prefix = EntityType::SocialConnection(String::new()).to_string();
    let opt = SocialConnection::opt_with_bookmark(None).sk(sk_prefix).limit(10);
    let (rows, _next): (Vec<SocialConnection>, _) =
        SocialConnection::query(cli, &user.pk, opt).await.map_err(|e| {
            crate::error!("list_connections query failed: {e}");
            CrossPostingError::ListFailed
        })?;

    Ok(rows.into_iter().map(Into::into).collect())
}
