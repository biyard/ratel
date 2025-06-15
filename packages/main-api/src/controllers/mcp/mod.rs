use bdk::prelude::*;

use rmcp::{
    ServerHandler,
    model::{CallToolResult, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    tool,
    transport::streamable_http_server::{
        StreamableHttpService, session::local::LocalSessionManager,
    },
};

pub(self) type McpResult = Result<CallToolResult, rmcp::Error>;

pub async fn route(
    _pool: sqlx::Pool<sqlx::Postgres>,
) -> dto::Result<StreamableHttpService<RatelMcpServer>> {
    Ok(StreamableHttpService::new(
        RatelMcpServer::new,
        LocalSessionManager::default().into(),
        Default::default(),
    ))
}

#[derive(Clone)]
pub struct RatelMcpServer {}

#[tool(tool_box)]
impl RatelMcpServer {
    pub fn new() -> Self {
        Self {}
    }

    #[tool(description = "Login with email into Ratel")]
    async fn login_with_email(
        &self,
        #[tool(param)]
        #[schemars(description = "User email address")]
        email: String,
    ) -> McpResult {
        tracing::debug!("Login with email: {}", email);

        todo!()
    }

    #[tool(
        description = "It checks if you have been logged in or not in Ratel. Some APIs require login."
    )]
    async fn check_if_logged_in(&self) -> McpResult {
        tracing::debug!("Checking if user is logged in");

        todo!()
    }

    #[tool(description = "Get my information. This should be called after login.")]
    async fn get_my_info(&self) -> McpResult {
        tracing::debug!("Getting user info");

        todo!()
    }

    #[tool(description = "Post a feed to Ratel. This should be called after login.")]
    async fn post_feed(
        &self,
        #[tool(param)]
        #[schemars(description = "Feed content. It allows HTML formatting.")]
        content: String,
    ) -> McpResult {
        tracing::debug!("Posting feed: {}", content);

        todo!()
    }

    #[tool(description = "Read feeds from Ratel")]
    async fn read_feeds(&self) -> McpResult {
        tracing::debug!("Reading feeds");

        todo!()
    }

    #[tool(description = "Like a feed in Ratel.This should be called after login")]
    async fn like_feed(
        &self,
        #[tool(param)]
        #[schemars(description = "Feed ID to like")]
        feed_id: i64,
    ) -> McpResult {
        tracing::debug!("Liking feed with ID: {}", feed_id);

        todo!()
    }
}

#[tool(tool_box)]
impl ServerHandler for RatelMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides Ratel functionalities that discuss on a topic and post/read feeds.".to_string()),
        }
    }
}
