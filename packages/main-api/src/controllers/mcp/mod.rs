use dto::{
    Feed, FeedCommentRequest, FeedCreateDraftRequest, FeedQuery, FeedType,
    by_axum::{self, auth::Authorization, axum},
    sqlx::PgPool,
};
use rmcp::{
    ErrorData, RoleServer, ServerHandler,
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    schemars,
    service::RequestContext,
    tool, tool_handler, tool_router,
    transport::streamable_http_server::{
        StreamableHttpService, session::local::LocalSessionManager,
    },
};

use crate::{config, controllers::v1::feeds::FeedController, utils::users::extract_user};

#[allow(dead_code)]
pub(self) type McpResult = Result<CallToolResult, ErrorData>;

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
    tool_router: ToolRouter<Self>,
}

async fn get_authorization(ctx: &RequestContext<RoleServer>) -> Option<Authorization> {
    if let Some(parts) = ctx.extensions.get::<axum::http::request::Parts>() {
        if let Some(auth) = parts.extensions.get::<by_axum::auth::Authorization>() {
            return Some(auth.clone());
        }
    }
    None
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct GetPostRequest {
    #[schemars(description = "the number of page starting from 1")]
    pub size: usize,
    #[schemars(description = "size of a page")]
    pub page: usize,
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct CreatePostRequest {
    #[schemars(description = "Post title. It should be short and descriptive.")]
    pub title: String,
    #[schemars(description = "Post content It allows HTML formatting.")]
    pub content: String,
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct CreateCommentRequest {
    #[schemars(description = "Target post Id to comment on. i.e., 12")]
    pub post_id: i64,
    #[schemars(description = "Comment content. It allows HTML formatting.")]
    pub content: String,
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct PostRequest {
    #[schemars(description = "Target Post Id")]
    pub id: i64,
}

#[derive(Debug, serde::Serialize)]
pub struct PostResponse {
    pub id: i64,
    pub title: Option<String>,
    pub html_contents: String,
    pub created_at: i64,
}

#[tool_router]
impl RatelMcpServer {
    pub fn new(pool: PgPool) -> Result<Self, std::io::Error> {
        Ok(Self {
            pool,
            tool_router: Self::tool_router(),
        })
    }

    #[tool(
        name = "check_if_logged_in",
        description = "It checks if you have been logged in or not in Ratel. Some APIs require login."
    )]
    async fn check_if_logged_in(&self, ctx: RequestContext<RoleServer>) -> McpResult {
        let auth = get_authorization(&ctx).await;
        if auth.is_none() {
            return Err(ErrorData::invalid_request(
                "You are not logged in. Please log in to use this API.",
                None,
            ));
        }

        let user = extract_user(&self.pool, auth).await.map_err(|e| {
            ErrorData::invalid_request(format!("Failed to extract user: {}", e), None)
        })?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "You are logged in as {}.",
            user.nickname
        ))]))
    }

    #[tool(
        name = "create_post",
        description = "Create a new post in Ratel. You must be logged in to use this API."
    )]
    async fn create_post(
        &self,
        Parameters(CreatePostRequest { title, content }): Parameters<CreatePostRequest>,
        ctx: RequestContext<RoleServer>,
    ) -> McpResult {
        let auth = get_authorization(&ctx).await;
        let user = extract_user(&self.pool, auth.clone()).await.map_err(|e| {
            ErrorData::invalid_request(format!("Failed to extract user: {}", e), None)
        })?;

        let ctrl = FeedController::new(self.pool.clone());
        let post = ctrl
            .create_draft(
                auth.clone(),
                FeedCreateDraftRequest {
                    feed_type: FeedType::Post,
                    user_id: user.id,
                },
            )
            .await
            .map_err(|e| {
                ErrorData::internal_error(format!("Failed to create draft: {}", e), None)
            })?;

        ctrl.update(
            post.id,
            auth.clone(),
            dto::FeedUpdateRequest {
                title: Some(title),
                html_contents: content,
                ..Default::default()
            },
        )
        .await
        .map_err(|e| ErrorData::internal_error(format!("Failed to update post: {}", e), None))?;

        ctrl.publish_draft(post.id, auth.clone())
            .await
            .map_err(|e| {
                ErrorData::internal_error(format!("Failed to publish draft: {}", e), None)
            })?;

        let domain = crate::config::get().signing_domain;
        let new_post = format!("https://{}/threads/{}", domain, post.id);
        Ok(CallToolResult::success(vec![
            Content::text(format!("New Post successfully created")),
            Content::text(format!("[Link]({})", new_post)),
        ]))
    }

    #[tool(
        name = "get_feed",
        description = "Get the latest posts from your Ratel feed. You must be logged in to use this API."
    )]
    async fn get_feed(
        &self,
        Parameters(GetPostRequest { size, page }): Parameters<GetPostRequest>,
        ctx: RequestContext<RoleServer>,
    ) -> McpResult {
        let auth = get_authorization(&ctx).await;

        let ctrl = FeedController::new(self.pool.clone());
        let feed = ctrl
            .query(
                auth,
                FeedQuery::new(size)
                    .with_page(page)
                    .with_status(dto::FeedStatus::Published)
                    .with_feed_type(FeedType::Post), // },
            )
            .await
            .map_err(|e| ErrorData::internal_error(format!("Failed to get feed: {}", e), None))?;

        let mut contents: Vec<Content> = vec![];

        feed.items.iter().for_each(|post| {
            let res = serde_json::to_string(&PostResponse {
                id: post.id,
                title: post.title.clone(),
                html_contents: post.html_contents.clone(),
                created_at: post.created_at,
            })
            .map_err(|e| {
                ErrorData::internal_error(format!("Failed to serialize post: {}", e), None)
            });
            contents.push(Content::text(res.unwrap()));
        });

        Ok(CallToolResult::success(contents))
    }

    #[tool(
        name = "get_post",
        description = "Get a post by ID in Ratel. Get More Detail About Post. You must be logged in to use this API."
    )]
    async fn get_post(
        &self,
        Parameters(PostRequest { id }): Parameters<PostRequest>,
        ctx: RequestContext<RoleServer>,
    ) -> McpResult {
        let auth = get_authorization(&ctx).await;
        if auth.is_none() {
            return Err(ErrorData::invalid_request(
                "You are not logged in. Please log in to use this API.",
                None,
            ));
        }
        let ctrl = FeedController::new(self.pool.clone());

        let post = ctrl
            .get(id, auth)
            .await
            .map_err(|e| ErrorData::internal_error(format!("Failed to get post: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&post).map_err(|e| {
                ErrorData::internal_error(format!("Failed to serialize post: {}", e), None)
            })?,
        )]))
    }

    #[tool(
        name = "create_comment",
        description = "Create a new comment on a post in Ratel. You must be logged in to use this API."
    )]
    async fn create_comment(
        &self,
        Parameters(CreateCommentRequest { post_id, content }): Parameters<CreateCommentRequest>,
        ctx: RequestContext<RoleServer>,
    ) -> McpResult {
        let auth = get_authorization(&ctx).await;
        if auth.is_none() {
            return Err(ErrorData::invalid_request(
                "You are not logged in. Please log in to use this API.",
                None,
            ));
        }
        let ctrl = FeedController::new(self.pool.clone());
        ctrl.comment(
            auth,
            FeedCommentRequest {
                parent_id: Some(post_id),
                html_contents: content,
            },
        )
        .await
        .map_err(|e| ErrorData::internal_error(format!("Failed to create comment: {}", e), None))?;

        let post = format!(
            "https://{}/threads/{}",
            config::get().signing_domain,
            post_id
        );
        Ok(CallToolResult::success(vec![
            Content::text(format!("Comment successfully created on post {}", post_id)),
            Content::text(format!("[Link]({})", post)),
        ]))
    }

    #[tool(
        name = "like_post",
        description = "Like a post by ID in Ratel. You must be logged in to use this API."
    )]
    async fn like_post(
        &self,
        Parameters(PostRequest { id }): Parameters<PostRequest>,
        ctx: RequestContext<RoleServer>,
    ) -> McpResult {
        let auth = get_authorization(&ctx).await;
        if auth.is_none() {
            return Err(ErrorData::invalid_request(
                "You are not logged in. Please log in to use this API.",
                None,
            ));
        }
        let ctrl = FeedController::new(self.pool.clone());
        ctrl.like(id, auth, true)
            .await
            .map_err(|e| ErrorData::internal_error(format!("Failed to like post: {}", e), None))?;

        let post = format!("https://{}/threads/{}", config::get().signing_domain, id);
        Ok(CallToolResult::success(vec![
            Content::text(format!("Post {} successfully liked", id)),
            Content::text(format!("[Link]({})", post)),
        ]))
    }

    #[tool(
        name = "list_my_posts",
        description = "List my posts in Ratel. You must be logged in to use this API."
    )]
    async fn list_my_post(
        &self,
        Parameters(GetPostRequest { size, page }): Parameters<GetPostRequest>,
        ctx: RequestContext<RoleServer>,
    ) -> McpResult {
        let auth = get_authorization(&ctx).await;
        let user = extract_user(&self.pool, auth.clone()).await.map_err(|e| {
            ErrorData::invalid_request(format!("Failed to extract user: {}", e), None)
        })?;

        let posts = Feed::query_builder(user.id)
            .user_id_equals(user.id)
            .status_equals(dto::FeedStatus::Published)
            .page(page as i32)
            .limit(size as i32)
            .query()
            .map(Feed::from)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                ErrorData::internal_error(format!("Failed to list my posts: {}", e), None)
            })?;

        let domain = crate::config::get().signing_domain;
        let mut contents = vec![Content::text(format!("You have {} posts.", posts.len()))];
        for post in posts {
            let post_url = format!("https://{}/threads/{}", domain, post.id);
            let title = if post.title.is_none() {
                format!("Post #{}", post.id)
            } else {
                post.title.unwrap()
            };
            contents.push(Content::text(format!("([{}]({}))", title, post_url)));
        }

        Ok(CallToolResult::success(contents))
    }
}

#[tool_handler]
impl ServerHandler for RatelMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_tool_list_changed()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides Ratel functionalities that discuss on a topic and post/read posts from feed.".to_string()),
        }
    }
}
