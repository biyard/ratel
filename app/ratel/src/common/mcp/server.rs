use std::collections::HashMap;
use std::sync::Arc;

use dioxus::server::axum::{self, Router};
use rmcp::{
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    tool_handler, tool_router,
    transport::streamable_http_server::{
        session::local::LocalSessionManager, StreamableHttpService,
    },
    ErrorData, RoleServer, ServerHandler,
};
use tokio::sync::RwLock;

use crate::common::config::ServerConfig;
use crate::common::models::McpClientSecret;
use crate::common::types::{EntityType, FeedPartition, TeamPartition};
use crate::features::auth::controllers::get_me_handler_mcp_impl;
use crate::features::posts::controllers::{
    create_post_handler_mcp_impl, create_space_handler_mcp_impl, delete_post_handler_mcp_impl,
    get_post_handler_mcp_impl, like_post_handler_mcp_impl, list_posts_handler_mcp_impl,
    update_post_handler_mcp_impl, CreateSpaceRequest, UpdatePostRequest,
};

pub type McpResult = Result<CallToolResult, ErrorData>;

/// Extension trait to convert `common::Result<T: Serialize>` into `McpResult`.
pub trait IntoMcpResult {
    fn into_mcp(self) -> McpResult;
}

impl<T: serde::Serialize> IntoMcpResult for crate::common::Result<T> {
    fn into_mcp(self) -> McpResult {
        match self {
            Ok(data) => match serde_json::to_string(&data) {
                Ok(json) => Ok(CallToolResult::success(vec![Content::text(json)])),
                Err(err) => Err(ErrorData::internal_error(
                    format!("failed to serialize MCP response payload: {err}"),
                    None,
                )),
            },
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Clone)]
pub struct RatelMcpServer {
    mcp_secret: String,
    tool_router: ToolRouter<Self>,
}

// ── MCP Tool Request Types (post-related, kept because they are hand-written) ──

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct CreatePostMcpRequest {
    #[schemars(
        description = "Optional team ID (e.g. 'TEAM#<uuid>' or '<uuid>') to create the post under a team. Omit to post as the user."
    )]
    pub team_id: Option<TeamPartition>,
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct GetPostMcpRequest {
    #[schemars(description = "Post partition key (e.g. 'POST#<uuid>' or '<uuid>')")]
    pub post_id: FeedPartition,
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct UpdatePostMcpRequest {
    #[schemars(description = "Post partition key")]
    pub post_id: FeedPartition,
    #[schemars(description = "Post title")]
    pub title: String,
    #[schemars(description = "Post content in HTML")]
    pub content: String,
    #[schemars(description = "Whether to publish the post")]
    pub publish: bool,
    #[schemars(description = "Visibility: 'Public' or 'Private'")]
    pub visibility: Option<String>,
    #[schemars(description = "Post categories")]
    pub categories: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct DeletePostMcpRequest {
    #[schemars(description = "Post partition key")]
    pub post_id: FeedPartition,
    #[schemars(description = "Force delete even if post has a space")]
    pub force: Option<bool>,
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct LikePostMcpRequest {
    #[schemars(description = "Post partition key")]
    pub post_id: FeedPartition,
    #[schemars(description = "true to like, false to unlike")]
    pub like: bool,
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct ListPostsMcpRequest {
    #[schemars(description = "Pagination bookmark. Omit for first page.")]
    pub bookmark: Option<String>,
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct CreateSpaceMcpRequest {
    #[schemars(description = "Post partition key to create a space on")]
    pub post_id: FeedPartition,
}

// ── MCP Tool Implementations ────────────────────────────────────────

#[tool_router]
impl RatelMcpServer {
    pub fn new(mcp_secret: String) -> Result<Self, std::io::Error> {
        Ok(Self {
            mcp_secret,
            tool_router: Self::tool_router(),
        })
    }

    #[rmcp::tool(
        name = "create_post",
        description = "Create a new draft post in Ratel. Returns the post partition key."
    )]
    async fn create_post(&self, Parameters(req): Parameters<CreatePostMcpRequest>) -> McpResult {
        create_post_handler_mcp_impl(self.mcp_secret.clone(), req.team_id)
            .await
            .into_mcp()
    }

    #[rmcp::tool(
        name = "get_me",
        description = "Get current user info and membership details."
    )]
    async fn get_me(&self) -> McpResult {
        get_me_handler_mcp_impl(self.mcp_secret.clone()).await.into_mcp()
    }

    #[rmcp::tool(name = "get_post", description = "Get post details by ID.")]
    async fn get_post(&self, Parameters(req): Parameters<GetPostMcpRequest>) -> McpResult {
        get_post_handler_mcp_impl(self.mcp_secret.clone(), req.post_id)
            .await
            .into_mcp()
    }

    #[rmcp::tool(name = "list_posts", description = "List posts from the feed.")]
    async fn list_posts(&self, Parameters(req): Parameters<ListPostsMcpRequest>) -> McpResult {
        list_posts_handler_mcp_impl(self.mcp_secret.clone(), req.bookmark)
            .await
            .into_mcp()
    }

    #[rmcp::tool(
        name = "update_post",
        description = "Update a post (publish, edit, change visibility)."
    )]
    async fn update_post(&self, Parameters(req): Parameters<UpdatePostMcpRequest>) -> McpResult {
        let visibility = req
            .visibility
            .map(|v| {
                v.parse().map_err(|_| {
                    ErrorData::invalid_params(
                        format!(
                            "Invalid visibility value: '{}'. Expected 'Public' or 'Private'.",
                            v
                        ),
                        None,
                    )
                })
            })
            .transpose()?;
        let update_req = UpdatePostRequest::Publish {
            title: req.title,
            content: req.content,
            image_urls: None,
            publish: req.publish,
            visibility,
            categories: req.categories,
        };
        update_post_handler_mcp_impl(self.mcp_secret.clone(), req.post_id, update_req)
            .await
            .into_mcp()
    }

    #[rmcp::tool(name = "delete_post", description = "Delete a post by ID.")]
    async fn delete_post(&self, Parameters(req): Parameters<DeletePostMcpRequest>) -> McpResult {
        delete_post_handler_mcp_impl(self.mcp_secret.clone(), req.post_id, req.force)
            .await
            .into_mcp()
    }

    #[rmcp::tool(name = "like_post", description = "Like or unlike a post.")]
    async fn like_post(&self, Parameters(req): Parameters<LikePostMcpRequest>) -> McpResult {
        like_post_handler_mcp_impl(self.mcp_secret.clone(), req.post_id, req.like)
            .await
            .into_mcp()
    }

    #[rmcp::tool(
        name = "create_space",
        description = "Create a space on an existing post."
    )]
    async fn create_space(&self, Parameters(req): Parameters<CreateSpaceMcpRequest>) -> McpResult {
        create_space_handler_mcp_impl(
            self.mcp_secret.clone(),
            CreateSpaceRequest {
                post_id: req.post_id,
            },
        )
        .await
        .into_mcp()
    }

    // ── Space tools ─────────────────────────────────────────────────

    #[rmcp::tool(
        name = "get_space",
        description = "Get space details by ID, including status, visibility, participation info."
    )]
    async fn get_space(
        &self,
        Parameters(req): Parameters<crate::features::spaces::space_common::controllers::GetSpaceMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::space_common::controllers::get_space_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "update_space",
        description = "Update a space (publish, change visibility, content, title, start, finish, quota, etc.). Requires creator role."
    )]
    async fn update_space(
        &self,
        Parameters(req): Parameters<crate::features::spaces::space_common::controllers::UpdateSpaceMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::space_common::controllers::update_space_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "delete_space",
        description = "Delete a space and unlink its post. Requires creator role."
    )]
    async fn delete_space(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::apps::apps::general::controllers::DeleteSpaceMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::apps::apps::general::controllers::delete_space_mcp_handler(&self.mcp_secret, req).await
    }

    // ── Poll tools ──────────────────────────────────────────────────

    #[rmcp::tool(
        name = "create_poll",
        description = "Create a new poll action in a space. Requires creator role."
    )]
    async fn create_poll(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::poll::controllers::CreatePollMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::poll::controllers::create_poll_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "update_poll",
        description = "Update a poll (title, time range, questions, response_editable). Requires creator role."
    )]
    async fn update_poll(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::poll::controllers::UpdatePollMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::poll::controllers::update_poll_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "delete_poll",
        description = "Delete a poll from a space. Requires creator role."
    )]
    async fn delete_poll(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::poll::controllers::DeletePollMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::poll::controllers::delete_poll_mcp_handler(&self.mcp_secret, req).await
    }

    // ── Quiz tools ──────────────────────────────────────────────────

    #[rmcp::tool(
        name = "create_quiz",
        description = "Create a new quiz action in a space. Requires creator role."
    )]
    async fn create_quiz(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::quiz::controllers::CreateQuizMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::quiz::controllers::create_quiz_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "update_quiz",
        description = "Update a quiz (title, description, time, questions, answers, pass_score, retry_count, files). Requires creator role."
    )]
    async fn update_quiz(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::quiz::controllers::UpdateQuizMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::quiz::controllers::update_quiz_mcp_handler(&self.mcp_secret, req).await
    }

    // ── Discussion tools ────────────────────────────────────────────

    #[rmcp::tool(
        name = "create_discussion",
        description = "Create a new discussion action in a space. Requires creator role."
    )]
    async fn create_discussion(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::discussion::controllers::CreateDiscussionMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::discussion::controllers::create_discussion_mcp_handler(
            &self.mcp_secret, req,
        ).await
    }

    #[rmcp::tool(
        name = "update_discussion",
        description = "Update a discussion (title, html_contents, category_name, started_at, ended_at). Requires creator role."
    )]
    async fn update_discussion(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::discussion::controllers::UpdateDiscussionMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::discussion::controllers::update_discussion_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "delete_discussion",
        description = "Delete a discussion from a space. Requires creator role."
    )]
    async fn delete_discussion(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::discussion::controllers::DeleteDiscussionMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::discussion::controllers::delete_discussion_mcp_handler(&self.mcp_secret, req).await
    }

    // ── Follow tools ────────────────────────────────────────────────

    #[rmcp::tool(
        name = "create_follow",
        description = "Create a follow action in a space. Requires creator role."
    )]
    async fn create_follow(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::follow::controllers::CreateFollowMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::follow::controllers::create_follow_mcp_handler(&self.mcp_secret, req).await
    }

    // ── Meet tools ──────────────────────────────────────────────────

    #[rmcp::tool(
        name = "create_meet",
        description = "Create a new meet action in a space. Requires creator role."
    )]
    async fn create_meet(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::meet::controllers::CreateMeetMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::meet::controllers::create_meet_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "get_meet",
        description = "Fetch a meet action with its companion SpaceAction row."
    )]
    async fn get_meet(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::meet::controllers::GetMeetMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::meet::controllers::get_meet_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "update_meet",
        description = "Update meet-specific fields (mode, start_time, duration_min). Requires creator role."
    )]
    async fn update_meet(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::meet::controllers::UpdateMeetMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::meet::controllers::update_meet_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "delete_meet",
        description = "Delete a meet action from a space. Requires creator role."
    )]
    async fn delete_meet(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::meet::controllers::DeleteMeetMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::meet::controllers::delete_meet_mcp_handler(&self.mcp_secret, req).await
    }

    // ── App tools ───────────────────────────────────────────────────

    #[rmcp::tool(
        name = "install_space_app",
        description = "Install an app in a space. Requires creator role. Types: General, File, Analyzes, Panels."
    )]
    async fn install_space_app(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::apps::controllers::InstallSpaceAppMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::apps::controllers::install_space_app_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "uninstall_space_app",
        description = "Uninstall an app from a space. Requires creator role."
    )]
    async fn uninstall_space_app(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::apps::controllers::UninstallSpaceAppMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::apps::controllers::uninstall_space_app_mcp_handler(&self.mcp_secret, req).await
    }

    // ── Poll participant tools ───────────────────────────────────────

    #[rmcp::tool(
        name = "get_poll",
        description = "Get poll details including questions and the current user's response."
    )]
    async fn get_poll(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::poll::controllers::GetPollMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::poll::controllers::get_poll_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "respond_poll",
        description = "Submit answers to a poll. Requires participant role and space in Ongoing status."
    )]
    async fn respond_poll(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::poll::controllers::RespondPollMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::poll::controllers::respond_poll_mcp_handler(&self.mcp_secret, req).await
    }

    // ── Quiz participant tools ──────────────────────────────────────

    #[rmcp::tool(
        name = "get_quiz",
        description = "Get quiz details including questions, attempt count, and the current user's score."
    )]
    async fn get_quiz(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::quiz::controllers::GetQuizMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::quiz::controllers::get_quiz_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "respond_quiz",
        description = "Submit answers to a quiz. Requires participant role. Returns score."
    )]
    async fn respond_quiz(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::quiz::controllers::RespondQuizMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::quiz::controllers::respond_quiz_mcp_handler(&self.mcp_secret, req).await
    }

    // ── Discussion participant tools ────────────────────────────────

    #[rmcp::tool(
        name = "get_discussion",
        description = "Get discussion details by ID."
    )]
    async fn get_discussion(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::discussion::controllers::GetDiscussionMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::discussion::controllers::get_discussion_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "add_comment",
        description = "Add a comment to a discussion. Requires participant role and discussion in progress."
    )]
    async fn add_comment(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::discussion::controllers::AddCommentMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::discussion::controllers::add_comment_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "list_comments",
        description = "List comments on a discussion, sorted by likes. Supports pagination. Pass `since` (unix seconds) to fetch only comments created after that time, ordered newest-first with no pagination."
    )]
    async fn list_comments(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::discussion::controllers::ListCommentsMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::discussion::controllers::list_comments_mcp_handler(&self.mcp_secret, req).await
    }

    // ── Follow participant tools ────────────────────────────────────

    #[rmcp::tool(
        name = "get_follow",
        description = "Get follow action details including the target user to follow."
    )]
    async fn get_follow(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::follow::controllers::GetFollowMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::follow::controllers::get_follow_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "follow_user",
        description = "Follow a user as part of a space follow action. Requires participant role."
    )]
    async fn follow_user(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::follow::controllers::FollowUserMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::follow::controllers::follow_user_mcp_handler(&self.mcp_secret, req).await
    }

    // ── Action listing tools ────────────────────────────────────────

    #[rmcp::tool(
        name = "list_actions",
        description = "List all actions in a space (polls, quizzes, discussions, follow). Shows action type, title, and status."
    )]
    async fn list_actions(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::controllers::ListActionsMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::controllers::list_actions_mcp_handler(&self.mcp_secret, req).await
    }

    // ── AI Moderator tools ───────────────────────────────────────────

    #[rmcp::tool(
        name = "update_ai_moderator",
        description = "Configure AI moderator for a discussion. Sets enabled state, reply interval, and guidelines. Requires creator role and premium membership."
    )]
    async fn update_ai_moderator(
        &self,
        Parameters(req): Parameters<crate::features::ai_moderator::controllers::UpdateAiModeratorConfigMcpRequest>,
    ) -> McpResult {
        crate::features::ai_moderator::controllers::update_ai_moderator_config_mcp_handler(&self.mcp_secret, req).await
    }

    // ── Team tools ──────────────────────────────────────────────────

    #[rmcp::tool(
        name = "list_teams",
        description = "List all teams the user belongs to with their role and permissions. Use team_id from the result as the team_id parameter in create_post to post under a team."
    )]
    async fn list_teams(&self) -> McpResult {
        crate::features::social::controllers::get_user_teams_handler_mcp_handler(&self.mcp_secret).await
    }

    // ── Notification inbox tools ────────────────────────────────────

    #[rmcp::tool(
        name = "list_inbox",
        description = "List the current user's notification inbox, newest first. Supports optional `unread_only` filter and `bookmark` pagination."
    )]
    async fn list_inbox(
        &self,
        Parameters(req): Parameters<crate::features::notifications::controllers::list_inbox::ListInboxHandlerMcpRequest>,
    ) -> McpResult {
        crate::features::notifications::controllers::list_inbox::list_inbox_handler_mcp_handler(&self.mcp_secret, req).await
    }

    #[rmcp::tool(
        name = "get_unread_count",
        description = "Get the count of unread notifications for the current user (capped at 100)."
    )]
    async fn get_unread_count(&self) -> McpResult {
        crate::features::notifications::controllers::get_unread_count::get_unread_count_handler_mcp_handler(&self.mcp_secret).await
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
            server_info: Implementation {
                name: "ratel-mcp".to_string(),
                title: None,
                version: env!("CARGO_PKG_VERSION").to_string(),
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "Ratel MCP Server. Create posts, manage spaces and teams on the Ratel platform."
                    .to_string(),
            ),
        }
    }
}

// ── Route Setup ─────────────────────────────────────────────────────

type McpService = StreamableHttpService<RatelMcpServer, LocalSessionManager>;

/// Maximum number of cached MCP services to prevent unbounded memory growth.
const MAX_MCP_CACHE_SIZE: usize = 1000;

/// TTL for cached MCP services (1 hour in seconds).
const MCP_CACHE_TTL_SECS: i64 = 3600;

/// Cached MCP service entry with creation timestamp for TTL eviction.
#[derive(Clone)]
struct CachedMcpService {
    service: McpService,
    created_at: i64,
}

/// Global cache of MCP services keyed by hashed client_secret.
/// Each secret gets a persistent service so MCP sessions survive across requests.
/// Entries are evicted after `MCP_CACHE_TTL_SECS` or when the cache exceeds
/// `MAX_MCP_CACHE_SIZE`.
static MCP_SERVICES: std::sync::LazyLock<RwLock<HashMap<String, CachedMcpService>>> =
    std::sync::LazyLock::new(|| RwLock::new(HashMap::new()));

/// Get or create a cached MCP service for the given raw client_secret.
/// Auth validation happens later during oneshot via the User extractor's
/// McpSecret support (see `extract_user_from_mcp_secret` in user.rs).
async fn get_or_create_service(client_secret: &str) -> McpService {
    let hashed_key = McpClientSecret::hash_secret(client_secret);
    let now = chrono::Utc::now().timestamp();

    // Fast path: check if a non-expired service already exists
    {
        let cache = MCP_SERVICES.read().await;
        if let Some(entry) = cache.get(&hashed_key) {
            if now - entry.created_at < MCP_CACHE_TTL_SECS {
                return entry.service.clone();
            }
        }
    }

    let secret = client_secret.to_string();
    let service = StreamableHttpService::new(
        move || RatelMcpServer::new(secret.clone()),
        Arc::new(LocalSessionManager::default()),
        rmcp::transport::streamable_http_server::StreamableHttpServerConfig {
            sse_keep_alive: Some(std::time::Duration::from_secs(15)),
            stateful_mode: false,
        },
    );

    let mut cache = MCP_SERVICES.write().await;

    // Evict expired entries
    cache.retain(|_, entry| now - entry.created_at < MCP_CACHE_TTL_SECS);

    // Evict oldest entries if cache is at capacity
    while cache.len() >= MAX_MCP_CACHE_SIZE {
        if let Some(oldest_key) = cache
            .iter()
            .min_by_key(|(_, entry)| entry.created_at)
            .map(|(k, _)| k.clone())
        {
            cache.remove(&oldest_key);
        } else {
            break;
        }
    }

    cache.insert(
        hashed_key,
        CachedMcpService {
            service: service.clone(),
            created_at: now,
        },
    );
    service
}

/// Invalidate cached MCP service when a user regenerates their secret.
pub async fn invalidate_user_services(user_pk: &crate::common::types::Partition) {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    if let Ok(Some(secret)) =
        McpClientSecret::get(cli, user_pk, Some(EntityType::McpClientSecret)).await
    {
        let mut cache = MCP_SERVICES.write().await;
        cache.remove(&secret.secret);
    }
}

/// Build the axum router for the MCP endpoint.
pub fn mcp_router() -> Router {
    Router::new().route("/mcp/{client_secret}", axum::routing::any(mcp_handler))
}

async fn mcp_handler(
    axum::extract::Path(client_secret): axum::extract::Path<String>,
    request: axum::http::Request<axum::body::Body>,
) -> axum::response::Response {
    let service = get_or_create_service(&client_secret).await;

    let resp = service.handle(request).await;
    let (parts, body) = resp.into_parts();
    axum::response::Response::from_parts(parts, axum::body::Body::new(body))
}
