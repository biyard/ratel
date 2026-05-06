use super::super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct McpSecretResponse {
    /// The raw token (only present immediately after generate/regenerate).
    pub secret: Option<String>,
    /// Whether a secret has been generated for this user.
    pub has_secret: bool,
}

/// Check whether the logged-in user has an MCP client secret.
/// The raw token is NOT returned here (it's only available at generation time).
#[get("/api/me/mcp-secret", user: crate::features::auth::User)]
pub async fn get_mcp_secret_handler() -> Result<McpSecretResponse> {
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    let existing = crate::common::models::McpClientSecret::get(
        cli,
        &user.pk,
        Some(EntityType::McpClientSecret),
    )
    .await?;

    Ok(McpSecretResponse {
        secret: None,
        has_secret: existing.is_some(),
    })
}

/// Generate or regenerate the MCP client secret for the logged-in user.
/// If one already exists, it is replaced.
#[post("/api/me/mcp-secret/regenerate", user: crate::features::auth::User)]
pub async fn regenerate_mcp_secret_handler() -> Result<McpSecretResponse> {
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    // Delete existing secret if present
    let existing = crate::common::models::McpClientSecret::get(
        cli,
        &user.pk,
        Some(EntityType::McpClientSecret),
    )
    .await?;

    if existing.is_some() {
        crate::common::models::McpClientSecret::delete(
            cli,
            &user.pk,
            Some(EntityType::McpClientSecret),
        )
        .await?;
    }

    // Create a new secret (returns entity with hash + raw token for user)
    let (new_secret, raw_token) = crate::common::models::McpClientSecret::new(user.pk.clone());
    new_secret.create(cli).await?;

    // Invalidate any cached MCP service for the old secret
    #[cfg(feature = "server")]
    crate::common::mcp::invalidate_user_services(&user.pk).await;

    Ok(McpSecretResponse {
        secret: Some(raw_token),
        has_secret: true,
    })
}
