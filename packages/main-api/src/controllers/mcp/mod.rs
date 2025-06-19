use bdk::prelude::*;

use dto::{FeedQuery, by_axum::auth::Authorization};
use rmcp::{
    ServerHandler,
    model::{
        CallToolResult, Content, ErrorCode, Implementation, ProtocolVersion, ServerCapabilities,
        ServerInfo,
    },
    tool,
    transport::streamable_http_server::{
        StreamableHttpService, session::local::LocalSessionManager,
    },
};

use crate::config;

use super::v1::feeds::FeedController;

pub(self) type McpResult = Result<CallToolResult, rmcp::Error>;

pub async fn route() -> dto::Result<StreamableHttpService<RatelMcpServer>> {
    Ok(StreamableHttpService::new(
        RatelMcpServer::new,
        LocalSessionManager::default().into(),
        Default::default(),
    ))
}

#[derive(Clone)]
pub struct RatelMcpServer {
    feed: FeedController,
}

#[tool(tool_box)]
impl RatelMcpServer {
    pub fn new() -> Result<Self, std::io::Error> {
        let conf = config::get();
        let pool = if let by_types::DatabaseConfig::Postgres { url, pool_size } = conf.database {
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(pool_size)
                .connect_lazy(url)
                .expect("Failed to connect to the database")
        } else {
            panic!("Database is not initialized. Call init() first.");
        };

        Ok(Self {
            feed: FeedController::new(pool),
        })
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
    async fn read_feeds(
        &self,
        #[tool(param)]
        #[schemars(description = "The number of page starting from 1")]
        page: usize,

        #[tool(param)]
        #[schemars(description = "size of a page")]
        size: usize,
    ) -> McpResult {
        tracing::debug!("Reading feeds");
        let auth = self.session_to_authorization().await;
        let feeds = self
            .feed
            .query(
                auth,
                FeedQuery::new(size)
                    .with_page(page)
                    .with_status(dto::FeedStatus::Published),
            )
            .await
            .map_err(|e| {
                tracing::error!("Failed to read feeds: {}", e);
                rmcp::Error::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)
            })?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&feeds).map_err(|e| {
                rmcp::Error::new(
                    ErrorCode::INTERNAL_ERROR,
                    format!("Failed to serialize feeds: {}", e),
                    None,
                )
            })?,
        )]))
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

    async fn session_to_authorization(&self) -> Option<Authorization> {
        None
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
