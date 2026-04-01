use super::super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct McpSecretResponse {
    pub secret: Option<String>,
}

/// Get the current MCP client secret for the logged-in user, if one exists.
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
        secret: existing.map(|s| s.secret),
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

    // Create a new secret
    let new_secret = crate::common::models::McpClientSecret::new(user.pk.clone());
    new_secret.create(cli).await?;

    Ok(McpSecretResponse {
        secret: Some(new_secret.secret),
    })
}
