use bdk::prelude::*;

use dto::{by_axum::axum, sqlx::PgPool};
use rmcp::{
    RoleServer, ServerHandler,
    model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    service::RequestContext,
    tool,
    transport::streamable_http_server::{
        StreamableHttpService, session::local::LocalSessionManager,
    },
};

use crate::utils::users::extract_user;

#[allow(dead_code)]
pub(self) type McpResult = Result<CallToolResult, rmcp::Error>;

pub async fn route(pool: PgPool) -> dto::Result<StreamableHttpService<RatelMcpServer>> {
    Ok(StreamableHttpService::new(
        move || RatelMcpServer::new(pool.clone()),
        LocalSessionManager::default().into(),
        Default::default(),
    ))
}
#[derive(Clone)]
pub struct RatelMcpServer {
    #[allow(dead_code)]
    pool: PgPool,
}

#[tool(tool_box)]
impl RatelMcpServer {
    pub fn new(pool: PgPool) -> Result<Self, std::io::Error> {
        Ok(Self { pool })
    }

    #[tool(
        description = "It checks if you have been logged in or not in Ratel. Some APIs require login."
    )]
    async fn check_if_logged_in(&self, ctx: RequestContext<RoleServer>) -> McpResult {
        let token_info = if let Some(parts) = ctx.extensions.get::<axum::http::request::Parts>() {
            let auth = parts.extensions.get::<by_axum::auth::Authorization>();
            let user = extract_user(&self.pool, auth.cloned()).await;
            if user.is_err() {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "You are not logged in. Error: {}",
                    user.err().unwrap()
                ))]));
            }
            let user = user.unwrap();
            format!("User ID: {}, Nickname: {}", user.id, user.nickname)
        } else {
            " (No HTTP context)".to_string()
        };

        tracing::debug!("Token info extracted: {:?}", token_info);
        Ok(CallToolResult::success(vec![Content::text(format!(
            "You are logged in. Token info: {}",
            token_info
        ))]))
    }
}

#[tool(tool_box)]
impl ServerHandler for RatelMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides Ratel functionalities that discuss on a topic and post/read feeds.".to_string()),
        }
    }
}
