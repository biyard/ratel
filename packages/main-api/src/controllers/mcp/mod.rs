use bdk::prelude::*;

use dto::{
    FeedCommentRequest, FeedCreateDraftRequest, FeedQuery, FeedUpdateRequest, User,
    by_axum::auth::{Authorization, UserSession},
};
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

use crate::utils::users::{extract_user, extract_user_id};

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
    pool: sqlx::Pool<sqlx::Postgres>,
    feed: FeedController,
}

#[tool(tool_box)]
impl RatelMcpServer {
    pub fn new() -> Result<Self, std::io::Error> {
        let conf = crate::config::get();
        let pool = if let by_types::DatabaseConfig::Postgres { url, pool_size } = conf.database {
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(pool_size)
                .connect_lazy(url)
                .expect("Failed to connect to the database")
        } else {
            panic!("Database is not initialized. Call init() first.");
        };

        Ok(Self {
            feed: FeedController::new(pool.clone()),
            pool,
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
        let auth = self.session_to_authorization().await;
        let user = extract_user(&self.pool, auth).await;
        tracing::debug!("User ID extracted: {:?}", user);
        Ok(match user {
            Ok(user) => CallToolResult::success(vec![Content::text(format!(
                "You are logged in as user ID: {}",
                user.nickname
            ))]),

            Err(e) => CallToolResult::error(vec![Content::text(format!(
                "You are not logged in. Error: {}",
                e
            ))]),
        })
    }

    #[tool(description = "Get my information. This should be called after login.")]
    async fn get_my_info(&self) -> McpResult {
        tracing::debug!("Getting user info");
        let auth = self.session_to_authorization().await;
        let user_id = extract_user_id(&self.pool, auth).await.map_err(|e| {
            tracing::error!("Failed to extract user ID: {}", e);
            rmcp::Error::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)
        })?;

        let user = User::query_builder()
            .id_equals(user_id)
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch user: {}", e);
                rmcp::Error::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)
            })?;
        let result = format!("My Name: {:?}", user.nickname);
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Post a feed to Ratel. This should be called after login.")]
    async fn post_feed(
        &self,
        #[tool(param)]
        #[schemars(description = "Feed title. It should be short and descriptive.")]
        title: String,
        #[tool(param)]
        #[schemars(description = "Feed content. It allows HTML formatting.")]
        content: String,
    ) -> McpResult {
        tracing::debug!("Posting feed: {}", content);

        let auth = self.session_to_authorization().await;
        let user_id = extract_user_id(&self.pool, auth.clone())
            .await
            .map_err(|e| {
                tracing::error!("Failed to extract user ID: {}", e);
                rmcp::Error::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)
            })?;

        let feed = self
            .feed
            .create_draft(
                auth.clone(),
                FeedCreateDraftRequest {
                    feed_type: dto::FeedType::Post,
                    user_id,
                },
            )
            .await
            .map_err(|e| {
                tracing::error!("Failed to create feed draft: {}", e);
                rmcp::Error::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)
            })?;

        self.feed
            .update(
                feed.id,
                auth.clone(),
                FeedUpdateRequest {
                    title: Some(title),
                    html_contents: content,
                    industry_id: 1,
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| {
                tracing::error!("Failed to update feed: {}", e);
                rmcp::Error::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)
            })?;

        self.feed.publish_draft(feed.id, auth).await.map_err(|e| {
            tracing::error!("Failed to publish feed: {}", e);
            rmcp::Error::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)
        })?;
        let domain = crate::config::get().domain;
        Ok(CallToolResult::success(vec![Content::text(format!(
            "Feed posted successfully with ID: https://{domain}/threads/{}",
            feed.id
        ))]))
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
        let auth = self.session_to_authorization().await;

        self.feed.like(feed_id, auth, true).await.map_err(|e| {
            tracing::error!("Failed to like feed: {}", e);
            rmcp::Error::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)
        })?;
        Ok(CallToolResult::success(vec![Content::text(format!(
            "Feed with ID {} liked successfully",
            feed_id
        ))]))
    }

    #[tool(description = "Comment a feed in Ratel.This should be called after login")]
    async fn comment_feed(
        &self,
        #[tool(param)]
        #[schemars(description = "Feed ID to comment")]
        feed_id: i64,
        #[tool(param)]
        #[schemars(description = "Comment content. It allows HTML formatting.")]
        content: String,
    ) -> McpResult {
        tracing::debug!("Liking feed with ID: {}", feed_id);
        let auth = self.session_to_authorization().await;
        self.feed
            .comment(
                auth,
                FeedCommentRequest {
                    parent_id: Some(feed_id),
                    html_contents: content,
                },
            )
            .await
            .map_err(|e| {
                tracing::error!("Failed to comment feed: {}", e);
                rmcp::Error::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)
            })?;
        Ok(CallToolResult::success(vec![Content::text(format!(
            "Comment on Feed ID {} commented successfully",
            feed_id
        ))]))
    }

    #[tool(description = "Join a discussion in Ratel. This should be called after login")]
    pub async fn join_discussion(
        &self,
        #[tool(param)]
        #[schemars(description = "Space ID to join")]
        space_id: i64,
    ) -> McpResult {
        tracing::debug!("Joining discussion with ID: {}", space_id);
        // let auth = self.session_to_authorization().await;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Joined discussion with ID: {}",
            space_id
        ))]))
    }

    async fn session_to_authorization(&self) -> Option<Authorization> {
        //FIXME: Implement Session Management
        let user = User::query_builder()
            .email_equals("ryan@biyard.co".to_string())
            .query()
            .map(User::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch user: {}", e);
                rmcp::Error::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None)
            })
            .ok()?;
        Some(Authorization::Session(UserSession {
            user_id: user.id,
            email: user.email,
            principal: user.principal,
        }))
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
