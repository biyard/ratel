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
use crate::common::models::auth::OptionalUser;
use crate::common::models::User;
use crate::common::models::space::{SpaceAdmin, SpaceAuthor, SpaceCommon, SpaceParticipant};
use crate::common::types::FeedPartition;
use crate::common::types::TeamPartition;
use crate::common::types::{
    CompositePartition, EntityType, Partition, SpaceActionFollowEntityType, SpacePartition,
    SpacePollEntityType, SpacePostEntityType, SpaceQuizEntityType, UserType,
};
use crate::common::SpaceUserRole;
use crate::features::auth::controllers::get_me_handler_mcp_impl;
use crate::features::auth::{UserTeam, UserTeamQueryOption};
use crate::features::posts::controllers::{
    create_post_handler_mcp_impl, create_space_handler_mcp_impl, delete_post_handler_mcp_impl,
    get_post_handler_mcp_impl, like_post_handler_mcp_impl, list_posts_handler_mcp_impl,
    update_post_handler_mcp_impl, CreateSpaceRequest, UpdatePostRequest,
};
use crate::features::posts::models::{Post, Team};
use crate::features::posts::types::PostStatus;
use crate::features::spaces::pages::actions::actions::discussion::{SpaceCategory, SpacePost};
use crate::features::spaces::pages::actions::actions::follow::SpaceFollowAction;
use crate::features::spaces::pages::actions::actions::poll::SpacePoll;
use crate::features::spaces::pages::actions::actions::quiz::{SpaceQuiz, SpaceQuizAnswer};
use crate::features::spaces::pages::actions::models::SpaceAction;
use crate::features::spaces::pages::actions::types::SpaceActionType;
// UpdateSpaceRequest is pub-re-exported from the general app controller module.
use crate::features::spaces::pages::apps::apps::general::UpdateSpaceRequest;

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
    user: User,
    tool_router: ToolRouter<Self>,
}

// ── MCP Tool Request Types ──────────────────────────────────────────

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

// ── Space MCP Request Types ─────────────────────────────────────────

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct GetSpaceMcpRequest {
    #[schemars(description = "Space partition key (e.g. 'SPACE#<uuid>' or '<uuid>')")]
    pub space_id: SpacePartition,
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct UpdateSpaceMcpRequest {
    #[schemars(description = "Space partition key")]
    pub space_id: SpacePartition,
    #[schemars(
        description = "Update data as JSON. Supported variants: {\"visibility\": \"Public\"}, {\"anonymous_participation\": true}, {\"join_anytime\": true}, {\"start\": true}, {\"finished\": true}, {\"quotas\": 100}, {\"title\": \"...\"}, {\"content\": \"...\"}, {\"publish\": true, \"visibility\": \"Public\"}, {\"logo\": \"url\"}"
    )]
    pub update: serde_json::Value,
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct DeleteSpaceMcpRequest {
    #[schemars(description = "Space partition key")]
    pub space_id: SpacePartition,
}

// Poll
#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct SpaceActionMcpRequest {
    #[schemars(description = "Space partition key")]
    pub space_id: SpacePartition,
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct UpdatePollMcpRequest {
    #[schemars(description = "Space partition key")]
    pub space_id: SpacePartition,
    #[schemars(description = "Poll sort key (e.g. 'SpacePoll#<uuid>')")]
    pub poll_id: String,
    #[schemars(
        description = "Poll update data as JSON. Supported variants: {\"title\": \"...\"}, {\"started_at\": <ms>, \"ended_at\": <ms>}, {\"questions\": [...]}, {\"response_editable\": true}"
    )]
    pub update: serde_json::Value,
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct DeletePollMcpRequest {
    #[schemars(description = "Space partition key")]
    pub space_id: SpacePartition,
    #[schemars(description = "Poll sort key (e.g. 'SpacePoll#<uuid>')")]
    pub poll_id: String,
}

// Quiz
#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct UpdateQuizMcpRequest {
    #[schemars(description = "Space partition key")]
    pub space_id: SpacePartition,
    #[schemars(description = "Quiz sort key (e.g. 'SpaceQuiz#<uuid>')")]
    pub quiz_id: String,
    #[schemars(
        description = "Quiz update data as JSON. Fields: title, description, started_at, ended_at, retry_count, pass_score, questions, answers, files (all optional)"
    )]
    pub update: serde_json::Value,
}

// Discussion
#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct UpdateDiscussionMcpRequest {
    #[schemars(description = "Space partition key")]
    pub space_id: SpacePartition,
    #[schemars(description = "Discussion sort key (e.g. 'SpacePost#<uuid>')")]
    pub discussion_id: String,
    #[schemars(
        description = "Discussion update data as JSON. Fields: title, html_contents, category_name, started_at, ended_at (all optional)"
    )]
    pub update: serde_json::Value,
}

#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct DeleteDiscussionMcpRequest {
    #[schemars(description = "Space partition key")]
    pub space_id: SpacePartition,
    #[schemars(description = "Discussion sort key (e.g. 'SpacePost#<uuid>')")]
    pub discussion_id: String,
}

// Apps
#[derive(Debug, serde::Deserialize, rmcp::schemars::JsonSchema)]
pub struct SpaceAppMcpRequest {
    #[schemars(description = "Space partition key")]
    pub space_id: SpacePartition,
    #[schemars(
        description = "App type: 'General', 'File', 'Analyzes', or 'Panels'"
    )]
    pub app_type: String,
}

// ── MCP Tool Implementations ────────────────────────────────────────

#[tool_router]
impl RatelMcpServer {
    pub fn new(user: User) -> Result<Self, std::io::Error> {
        Ok(Self {
            user,
            tool_router: Self::tool_router(),
        })
    }

    #[rmcp::tool(
        name = "create_post",
        description = "Create a new draft post in Ratel. Returns the post partition key."
    )]
    async fn create_post(&self, Parameters(req): Parameters<CreatePostMcpRequest>) -> McpResult {
        create_post_handler_mcp_impl(self.user.clone(), req.team_id)
            .await
            .into_mcp()
    }

    #[rmcp::tool(name = "get_me", description = "Get current user info and membership details.")]
    async fn get_me(&self) -> McpResult {
        let user = OptionalUser(Some(self.user.clone()));
        get_me_handler_mcp_impl(user).await.into_mcp()
    }

    #[rmcp::tool(name = "get_post", description = "Get post details by ID.")]
    async fn get_post(&self, Parameters(req): Parameters<GetPostMcpRequest>) -> McpResult {
        let user = OptionalUser(Some(self.user.clone()));
        get_post_handler_mcp_impl(user, req.post_id)
            .await
            .into_mcp()
    }

    #[rmcp::tool(name = "list_posts", description = "List posts from the feed.")]
    async fn list_posts(&self, Parameters(req): Parameters<ListPostsMcpRequest>) -> McpResult {
        let user = OptionalUser(Some(self.user.clone()));
        list_posts_handler_mcp_impl(user, req.bookmark)
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
                        format!("Invalid visibility value: '{}'. Expected 'Public' or 'Private'.", v),
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
        update_post_handler_mcp_impl(self.user.clone(), req.post_id, update_req)
            .await
            .into_mcp()
    }

    #[rmcp::tool(name = "delete_post", description = "Delete a post by ID.")]
    async fn delete_post(&self, Parameters(req): Parameters<DeletePostMcpRequest>) -> McpResult {
        delete_post_handler_mcp_impl(self.user.clone(), req.post_id, req.force)
            .await
            .into_mcp()
    }

    #[rmcp::tool(name = "like_post", description = "Like or unlike a post.")]
    async fn like_post(&self, Parameters(req): Parameters<LikePostMcpRequest>) -> McpResult {
        like_post_handler_mcp_impl(self.user.clone(), req.post_id, req.like)
            .await
            .into_mcp()
    }

    #[rmcp::tool(name = "create_space", description = "Create a space on an existing post.")]
    async fn create_space(&self, Parameters(req): Parameters<CreateSpaceMcpRequest>) -> McpResult {
        create_space_handler_mcp_impl(
            self.user.clone(),
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
    async fn get_space(&self, Parameters(req): Parameters<GetSpaceMcpRequest>) -> McpResult {
        get_space_impl(&self.user, req.space_id).await.into_mcp()
    }

    #[rmcp::tool(
        name = "update_space",
        description = "Update a space (publish, change visibility, content, title, start, finish, quota, etc.). Requires creator role."
    )]
    async fn update_space(
        &self,
        Parameters(req): Parameters<UpdateSpaceMcpRequest>,
    ) -> McpResult {
        update_space_impl(&self.user, req.space_id, req.update)
            .await
            .into_mcp()
    }

    #[rmcp::tool(
        name = "delete_space",
        description = "Delete a space and unlink its post. Requires creator role."
    )]
    async fn delete_space(
        &self,
        Parameters(req): Parameters<DeleteSpaceMcpRequest>,
    ) -> McpResult {
        delete_space_impl(&self.user, req.space_id).await.into_mcp()
    }

    // ── Poll tools ──────────────────────────────────────────────────

    #[rmcp::tool(
        name = "create_poll",
        description = "Create a new poll action in a space. Requires creator role."
    )]
    async fn create_poll(
        &self,
        Parameters(req): Parameters<SpaceActionMcpRequest>,
    ) -> McpResult {
        create_poll_impl(&self.user, req.space_id).await.into_mcp()
    }

    #[rmcp::tool(
        name = "update_poll",
        description = "Update a poll (title, time range, questions, response_editable). Requires creator role."
    )]
    async fn update_poll(
        &self,
        Parameters(req): Parameters<UpdatePollMcpRequest>,
    ) -> McpResult {
        update_poll_impl(&self.user, req.space_id, req.poll_id, req.update)
            .await
            .into_mcp()
    }

    #[rmcp::tool(
        name = "delete_poll",
        description = "Delete a poll from a space. Requires creator role."
    )]
    async fn delete_poll(
        &self,
        Parameters(req): Parameters<DeletePollMcpRequest>,
    ) -> McpResult {
        delete_poll_impl(&self.user, req.space_id, req.poll_id)
            .await
            .into_mcp()
    }

    // ── Quiz tools ──────────────────────────────────────────────────

    #[rmcp::tool(
        name = "create_quiz",
        description = "Create a new quiz action in a space. Requires creator role."
    )]
    async fn create_quiz(
        &self,
        Parameters(req): Parameters<SpaceActionMcpRequest>,
    ) -> McpResult {
        create_quiz_impl(&self.user, req.space_id).await.into_mcp()
    }

    #[rmcp::tool(
        name = "update_quiz",
        description = "Update a quiz (title, description, time, questions, answers, pass_score, retry_count, files). Requires creator role."
    )]
    async fn update_quiz(
        &self,
        Parameters(req): Parameters<UpdateQuizMcpRequest>,
    ) -> McpResult {
        update_quiz_impl(&self.user, req.space_id, req.quiz_id, req.update)
            .await
            .into_mcp()
    }

    // ── Discussion tools ────────────────────────────────────────────

    #[rmcp::tool(
        name = "create_discussion",
        description = "Create a new discussion action in a space. Requires creator role."
    )]
    async fn create_discussion(
        &self,
        Parameters(req): Parameters<SpaceActionMcpRequest>,
    ) -> McpResult {
        create_discussion_impl(&self.user, req.space_id)
            .await
            .into_mcp()
    }

    #[rmcp::tool(
        name = "update_discussion",
        description = "Update a discussion (title, html_contents, category_name, started_at, ended_at). Requires creator role."
    )]
    async fn update_discussion(
        &self,
        Parameters(req): Parameters<UpdateDiscussionMcpRequest>,
    ) -> McpResult {
        update_discussion_impl(&self.user, req.space_id, req.discussion_id, req.update)
            .await
            .into_mcp()
    }

    #[rmcp::tool(
        name = "delete_discussion",
        description = "Delete a discussion from a space. Requires creator role."
    )]
    async fn delete_discussion(
        &self,
        Parameters(req): Parameters<DeleteDiscussionMcpRequest>,
    ) -> McpResult {
        delete_discussion_impl(&self.user, req.space_id, req.discussion_id)
            .await
            .into_mcp()
    }

    // ── Follow tools ────────────────────────────────────────────────

    #[rmcp::tool(
        name = "create_follow",
        description = "Create a follow action in a space. Requires creator role."
    )]
    async fn create_follow(
        &self,
        Parameters(req): Parameters<SpaceActionMcpRequest>,
    ) -> McpResult {
        create_follow_impl(&self.user, req.space_id)
            .await
            .into_mcp()
    }

    // ── App tools ───────────────────────────────────────────────────

    #[rmcp::tool(
        name = "install_space_app",
        description = "Install an app in a space. Requires creator role. Types: General, File, Analyzes, Panels."
    )]
    async fn install_space_app(
        &self,
        Parameters(req): Parameters<SpaceAppMcpRequest>,
    ) -> McpResult {
        install_space_app_impl(&self.user, req.space_id, &req.app_type)
            .await
            .into_mcp()
    }

    #[rmcp::tool(
        name = "uninstall_space_app",
        description = "Uninstall an app from a space. Requires creator role."
    )]
    async fn uninstall_space_app(
        &self,
        Parameters(req): Parameters<SpaceAppMcpRequest>,
    ) -> McpResult {
        uninstall_space_app_impl(&self.user, req.space_id, &req.app_type)
            .await
            .into_mcp()
    }

    // ── Team tools ──────────────────────────────────────────────────

    #[rmcp::tool(
        name = "list_teams",
        description = "List all teams the user belongs to with their role and permissions. Permissions include: PostRead, PostWrite, PostEdit, PostDelete, SpaceRead, SpaceWrite, SpaceEdit, SpaceDelete, TeamAdmin, TeamEdit, GroupEdit. Use team_id from the result as the team_id parameter in create_post to post under a team."
    )]
    async fn list_teams(&self) -> McpResult {
        list_teams_impl(&self.user).await.into_mcp()
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

// ── Implementation helpers ──────────────────────────────────────────

/// Resolve the user's role in a space. Simplified version for MCP context.
/// Checks: creator (individual), team admin, space admin. Otherwise Viewer.
async fn resolve_space_user_role(
    cli: &aws_sdk_dynamodb::Client,
    user: &User,
    space: &SpaceCommon,
) -> crate::common::Result<SpaceUserRole> {
    // Direct creator check
    if user.pk == space.user_pk {
        return Ok(SpaceUserRole::Creator);
    }

    // Team admin check
    if matches!(&space.user_pk, Partition::Team(_)) {
        use crate::features::posts::types::TeamGroupPermission;
        if Team::has_permission(
            cli,
            &space.user_pk,
            &user.pk,
            TeamGroupPermission::TeamAdmin,
        )
        .await
        .unwrap_or(false)
        {
            return Ok(SpaceUserRole::Creator);
        }
    }

    // Space admin check
    let space_admin_sk = EntityType::SpaceAdmin(user.pk.to_string());
    if SpaceAdmin::get(cli, &space.pk, Some(&space_admin_sk))
        .await
        .ok()
        .flatten()
        .is_some()
    {
        return Ok(SpaceUserRole::Creator);
    }

    Ok(SpaceUserRole::Viewer)
}

/// Fetch space and verify the user has creator role.
async fn require_space_creator(
    cli: &aws_sdk_dynamodb::Client,
    user: &User,
    space_id: SpacePartition,
) -> crate::common::Result<SpaceCommon> {
    let space_pk: Partition = space_id.into();
    let space = SpaceCommon::get(cli, &space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or_else(|| crate::common::Error::NotFound("Space not found".to_string()))?;
    let role = resolve_space_user_role(cli, user, &space).await?;
    if role != SpaceUserRole::Creator {
        return Err(crate::common::Error::NoPermission);
    }
    Ok(space)
}

// ── get_space ───────────────────────────────────────────────────────

async fn get_space_impl(
    user: &User,
    space_id: SpacePartition,
) -> crate::common::Result<serde_json::Value> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    let space_pk: Partition = space_id.into();
    let space = SpaceCommon::get(cli, &space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or_else(|| crate::common::Error::NotFound("Space not found".to_string()))?;

    let post_pk = space.pk.clone().to_post_key()?;
    let post = Post::get(cli, &post_pk, Some(EntityType::Post))
        .await?
        .ok_or_else(|| crate::common::Error::NotFound("Post not found".to_string()))?;

    let permissions: i64 = post.get_permissions(cli, Some(user.clone())).await?.into();
    let liked = post.is_liked(cli, &user.pk).await?;

    let (participant_pk, participant_sk) =
        SpaceParticipant::keys(space.pk.clone(), user.pk.clone());
    let participant =
        SpaceParticipant::get(cli, &participant_pk, Some(&participant_sk)).await?;
    let is_participation_open = space.is_participation_open();
    let can_participate = participant.is_none() && is_participation_open;

    let (participated, participant_display_name, participant_profile_url, participant_username) =
        if let Some(p) = participant {
            (
                true,
                Some(p.display_name),
                Some(p.profile_url),
                Some(p.username),
            )
        } else {
            (false, None, None, None)
        };

    Ok(serde_json::json!({
        "id": space.pk.to_string(),
        "post_id": post_pk.to_string(),
        "sk": space.sk.to_string(),
        "title": post.title,
        "content": if space.content.is_empty() { &post.html_contents } else { &space.content },
        "created_at": space.created_at,
        "updated_at": space.updated_at,
        "status": format!("{:?}", space.status),
        "permissions": permissions,
        "author_display_name": post.author_display_name,
        "author_username": post.author_username,
        "author_profile_url": post.author_profile_url,
        "likes": post.likes,
        "comments": post.comments,
        "shares": post.shares,
        "liked": liked,
        "reports": space.reports,
        "rewards": space.rewards,
        "visibility": format!("{:?}", space.visibility),
        "publish_state": format!("{:?}", space.publish_state),
        "anonymous_participation": space.anonymous_participation,
        "join_anytime": space.join_anytime,
        "can_participate": can_participate,
        "participated": participated,
        "participant_display_name": participant_display_name,
        "participant_profile_url": participant_profile_url,
        "participant_username": participant_username,
        "remains": space.remains,
        "quota": space.quota,
        "logo": space.logo,
    }))
}

// ── update_space ────────────────────────────────────────────────────

async fn update_space_impl(
    user: &User,
    space_id: SpacePartition,
    update: serde_json::Value,
) -> crate::common::Result<serde_json::Value> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    let space = require_space_creator(cli, user, space_id.clone()).await?;
    let req: UpdateSpaceRequest = serde_json::from_value(update)
        .map_err(|_| crate::common::McpServerError::InvalidUpdateData)?;

    let space_pk: Partition = space_id.into();
    let now = chrono::Utc::now().timestamp_millis();
    let mut su = SpaceCommon::updater(&space.pk, &space.sk).with_updated_at(now);
    let mut pu: Option<_> = None;
    let mut updated_space = space.clone();

    match req {
        UpdateSpaceRequest::Publish {
            publish,
            visibility,
        } => {
            let post_pk = space_pk.clone().to_post_key()?;
            if !publish {
                return Err(crate::common::McpServerError::UnpublishNotSupported.into());
            }
            su = su
                .with_publish_state(crate::common::SpacePublishState::Published)
                .with_status(crate::common::SpaceStatus::InProgress)
                .with_visibility(visibility.clone());
            let post_updater = Post::updater(post_pk, EntityType::Post)
                .with_updated_at(now)
                .with_space_visibility(visibility.clone())
                .with_visibility(visibility.clone().into())
                .with_status(PostStatus::Published);
            pu = Some(post_updater);
            updated_space.publish_state = crate::common::SpacePublishState::Published;
            updated_space.visibility = visibility;
        }
        UpdateSpaceRequest::Visibility { visibility } => {
            su = su.with_visibility(visibility.clone());
            let post_pk = space_pk.clone().to_post_key()?;
            let post_updater = Post::updater(post_pk, EntityType::Post)
                .with_updated_at(now)
                .with_space_visibility(visibility.clone())
                .with_visibility(visibility.clone().into());
            pu = Some(post_updater);
            updated_space.visibility = visibility;
        }
        UpdateSpaceRequest::Content { content } => {
            su = su.with_content(content.clone());
            updated_space.content = content;
        }
        UpdateSpaceRequest::Title { title } => {
            let post_pk = space_pk.clone().to_post_key()?;
            let post_updater = Post::updater(post_pk, EntityType::Post)
                .with_updated_at(now)
                .with_title(title);
            pu = Some(post_updater);
        }
        UpdateSpaceRequest::Start { start } => {
            if updated_space.status != Some(crate::common::SpaceStatus::InProgress) {
                return Err(crate::common::McpServerError::StartNotAvailable.into());
            }
            if !start {
                return Err(crate::common::McpServerError::CannotUndoStart.into());
            }
            su = su.with_status(crate::common::SpaceStatus::Started);
            updated_space.status = Some(crate::common::SpaceStatus::Started);
        }
        UpdateSpaceRequest::Finish { finished } => {
            if updated_space.status != Some(crate::common::SpaceStatus::Started) {
                return Err(crate::common::McpServerError::FinishNotAvailable.into());
            }
            if !finished {
                return Err(crate::common::McpServerError::CannotUndoFinish.into());
            }
            su = su.with_status(crate::common::SpaceStatus::Finished);
            updated_space.status = Some(crate::common::SpaceStatus::Finished);
        }
        UpdateSpaceRequest::Anonymous {
            anonymous_participation,
        } => {
            su = su.with_anonymous_participation(anonymous_participation);
            updated_space.anonymous_participation = anonymous_participation;
        }
        UpdateSpaceRequest::JoinAnytime { join_anytime } => {
            su = su.with_join_anytime(join_anytime);
            updated_space.join_anytime = join_anytime;
        }
        UpdateSpaceRequest::ChangeVisibility { .. } => {
            return Err(crate::common::McpServerError::DeprecatedOperation.into());
        }
        UpdateSpaceRequest::Logo { logo } => {
            su = su.with_logo(logo.clone());
            updated_space.logo = logo;
        }
        UpdateSpaceRequest::Quota { quotas } => {
            let remains = updated_space.remains + (quotas - updated_space.quota);
            if remains < 0 {
                return Err(crate::common::McpServerError::InvalidPanelQuota.into());
            }
            su = su.with_quota(quotas).with_remains(remains);
            updated_space.quota = quotas;
            updated_space.remains = remains;
        }
    }

    if let Some(pu) = pu {
        crate::transact_write!(cli, su.transact_write_item(), pu.transact_write_item())?;
    } else {
        su.execute(cli).await?;
    }

    Ok(serde_json::json!({
        "pk": updated_space.pk.to_string(),
        "sk": updated_space.sk.to_string(),
        "status": format!("{:?}", updated_space.status),
        "publish_state": format!("{:?}", updated_space.publish_state),
        "visibility": format!("{:?}", updated_space.visibility),
        "content": updated_space.content,
        "anonymous_participation": updated_space.anonymous_participation,
        "join_anytime": updated_space.join_anytime,
        "quota": updated_space.quota,
        "remains": updated_space.remains,
        "logo": updated_space.logo,
    }))
}

// ── delete_space ────────────────────────────────────────────────────

async fn delete_space_impl(
    user: &User,
    space_id: SpacePartition,
) -> crate::common::Result<serde_json::Value> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    let space = require_space_creator(cli, user, space_id.clone()).await?;

    let space_pk: Partition = space.pk.clone();
    let post_pk = space_pk.clone().to_post_key()?;

    SpaceCommon::delete(cli, &space.pk, Some(space.sk)).await?;
    SpacePoll::delete_one(cli, &space_pk).await.ok();

    if Post::get(cli, &post_pk, Some(EntityType::Post))
        .await?
        .is_some()
    {
        Post::updater(post_pk, EntityType::Post)
            .remove_space_pk()
            .remove_space_type()
            .remove_space_visibility()
            .execute(cli)
            .await?;
    }

    Ok(serde_json::json!({
        "message": format!("Space '{}' deleted", space_id)
    }))
}

// ── create_poll ─────────────────────────────────────────────────────

async fn create_poll_impl(
    user: &User,
    space_id: SpacePartition,
) -> crate::common::Result<serde_json::Value> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    let _space = require_space_creator(cli, user, space_id.clone()).await?;

    let poll = SpacePoll::new(space_id.clone())?;
    let space_action = SpaceAction::new(
        space_id.clone(),
        SpacePollEntityType::from(poll.sk.clone()).to_string(),
        SpaceActionType::Poll,
    );

    let items = vec![
        poll.create_transact_write_item(),
        space_action.create_transact_write_item(),
    ];
    crate::transact_write_items!(cli, items)
        .map_err(|e| crate::common::Error::Unknown(format!("Failed to create poll: {e}")))?;

    Ok(serde_json::json!({
        "pk": poll.pk.to_string(),
        "sk": poll.sk.to_string(),
        "message": "Poll created"
    }))
}

// ── update_poll ─────────────────────────────────────────────────────

async fn update_poll_impl(
    user: &User,
    space_id: SpacePartition,
    poll_id: String,
    update: serde_json::Value,
) -> crate::common::Result<String> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    let _space = require_space_creator(cli, user, space_id.clone()).await?;

    use crate::features::spaces::pages::actions::actions::poll::controllers::UpdatePollRequest;
    let req: UpdatePollRequest = serde_json::from_value(update)
        .map_err(|_| crate::common::McpServerError::InvalidUpdateData)?;

    let space_pk: Partition = space_id.clone().into();
    let poll_sk_type: SpacePollEntityType = poll_id.into();
    let poll_sk: EntityType = poll_sk_type.clone().into();

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut poll_updater = SpacePoll::updater(&space_pk, &poll_sk).with_updated_at(now);

    let action_pk =
        CompositePartition::<SpacePartition, String>(space_id, poll_sk_type.to_string());
    let mut action_updater =
        SpaceAction::updater(&action_pk, &EntityType::SpaceAction).with_updated_at(now);
    let mut update_action = false;

    match req {
        UpdatePollRequest::Title { title } => {
            poll_updater = poll_updater.with_title(title.clone());
            action_updater = action_updater.with_title(title);
            update_action = true;
        }
        UpdatePollRequest::Time {
            started_at,
            ended_at,
        } => {
            if started_at >= ended_at {
                return Err(crate::common::McpServerError::InvalidTimeRange.into());
            }
            poll_updater = poll_updater
                .with_started_at(started_at)
                .with_ended_at(ended_at);
        }
        UpdatePollRequest::Question { questions } => {
            if questions.is_empty() {
                return Err(crate::common::McpServerError::EmptyQuestions.into());
            }
            let description = questions
                .first()
                .map(|q| q.title().to_string())
                .unwrap_or_default();
            poll_updater = poll_updater
                .with_questions(questions)
                .with_description(description.clone());
            action_updater = action_updater.with_description(description);
            update_action = true;
        }
        UpdatePollRequest::ResponseEditable { response_editable } => {
            poll_updater = poll_updater.with_response_editable(response_editable);
        }
        UpdatePollRequest::CanisterUploadEnabled {
            canister_upload_enabled,
        } => {
            poll_updater = poll_updater.with_canister_upload_enabled(canister_upload_enabled);
            if canister_upload_enabled {
                poll_updater = poll_updater.with_response_editable(false);
            }
        }
    }

    poll_updater.execute(cli).await?;
    if update_action {
        action_updater.execute(cli).await?;
    }

    Ok("success".to_string())
}

// ── delete_poll ─────────────────────────────────────────────────────

async fn delete_poll_impl(
    user: &User,
    space_id: SpacePartition,
    poll_id: String,
) -> crate::common::Result<String> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    let _space = require_space_creator(cli, user, space_id.clone()).await?;

    let space_pk: Partition = space_id.into();
    let poll_sk_type: SpacePollEntityType = poll_id.into();
    let poll_sk: EntityType = poll_sk_type.into();

    let _poll = SpacePoll::get(cli, &space_pk, Some(poll_sk.clone()))
        .await?
        .ok_or(crate::common::Error::NotFound("Poll not found".into()))?;

    SpacePoll::delete(cli, &space_pk, Some(poll_sk)).await?;

    Ok("success".to_string())
}

// ── create_quiz ─────────────────────────────────────────────────────

async fn create_quiz_impl(
    user: &User,
    space_id: SpacePartition,
) -> crate::common::Result<serde_json::Value> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    let _space = require_space_creator(cli, user, space_id.clone()).await?;

    let quiz = SpaceQuiz::new(space_id.clone())?;
    let space_action = SpaceAction::new(
        space_id.clone(),
        SpaceQuizEntityType::from(quiz.sk.clone()).to_string(),
        SpaceActionType::Quiz,
    );
    let items = vec![
        quiz.create_transact_write_item(),
        space_action.create_transact_write_item(),
    ];
    crate::transact_write_items!(cli, items)
        .map_err(|e| crate::common::Error::Unknown(format!("Failed to create quiz: {e}")))?;

    let quiz_id: SpaceQuizEntityType = match &quiz.sk {
        EntityType::SpaceQuiz(id) => id.clone().into(),
        _ => SpaceQuizEntityType::default(),
    };
    let answers = quiz
        .questions
        .iter()
        .map(
            crate::features::spaces::pages::actions::actions::quiz::QuizCorrectAnswer::for_question,
        )
        .collect::<Vec<_>>();
    let answer = SpaceQuizAnswer::new(space_id, quiz_id, answers);
    answer.create(cli).await?;

    Ok(serde_json::json!({
        "pk": quiz.pk.to_string(),
        "sk": quiz.sk.to_string(),
        "message": "Quiz created"
    }))
}

// ── update_quiz ─────────────────────────────────────────────────────

async fn update_quiz_impl(
    user: &User,
    space_id: SpacePartition,
    quiz_id: String,
    update: serde_json::Value,
) -> crate::common::Result<String> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    let _space = require_space_creator(cli, user, space_id.clone()).await?;

    use crate::features::spaces::pages::actions::actions::quiz::controllers::update_quiz::UpdateQuizRequest;
    let req: UpdateQuizRequest = serde_json::from_value(update)
        .map_err(|_| crate::common::McpServerError::InvalidUpdateData)?;

    let space_pk: Partition = space_id.clone().into();
    let quiz_sk_type: SpaceQuizEntityType = quiz_id.clone().into();
    let quiz_sk: EntityType = quiz_sk_type.clone().into();

    let _existing = SpaceQuiz::get(cli, &space_pk, Some(quiz_sk.clone()))
        .await?
        .ok_or(crate::common::Error::NotFound("Quiz not found".into()))?;

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut updater = SpaceQuiz::updater(&space_pk, &quiz_sk).with_updated_at(now);
    let action_pk = CompositePartition(space_id, quiz_sk_type.to_string());
    let action_sk = EntityType::SpaceAction;
    let mut action_updater =
        SpaceAction::updater(&action_pk, &action_sk).with_updated_at(now);
    let mut should_update_action = false;

    if let Some(title) = req.title {
        action_updater = action_updater.with_title(title);
        should_update_action = true;
    }
    if let Some(description) = req.description {
        action_updater = action_updater.with_description(description);
        should_update_action = true;
    }
    if req.started_at.is_some() || req.ended_at.is_some() {
        let started_at = req
            .started_at
            .ok_or(crate::common::McpServerError::RequiredFieldMissing)?;
        let ended_at = req
            .ended_at
            .ok_or(crate::common::McpServerError::RequiredFieldMissing)?;
        if started_at >= ended_at {
            return Err(crate::common::Error::BadRequest(
                "Invalid time range".into(),
            ));
        }
        action_updater = action_updater
            .with_started_at(started_at)
            .with_ended_at(ended_at);
        should_update_action = true;
    }
    if let Some(retry_count) = req.retry_count {
        updater = updater.with_retry_count(retry_count);
    }
    if let Some(questions) = req.questions {
        updater = updater.with_questions(questions);
    }
    if let Some(pass_score) = req.pass_score {
        updater = updater.with_pass_score(pass_score);
    }
    if let Some(mut files) = req.files {
        for file in &mut files {
            if file.id.is_empty() {
                file.id = crate::common::uuid::Uuid::now_v7().to_string();
            }
        }
        updater = updater.with_files(files);
    }

    updater.execute(cli).await?;
    if should_update_action {
        action_updater.execute(cli).await?;
    }

    if let Some(answers) = req.answers {
        let answer_sk = EntityType::SpaceQuizAnswer(quiz_id);
        let answer_updater = SpaceQuizAnswer::updater(&space_pk, &answer_sk)
            .with_created_at(now)
            .with_updated_at(now)
            .with_space_pk(space_pk.clone())
            .with_answers(answers);
        answer_updater.execute(cli).await?;
    }

    Ok("success".to_string())
}

// ── create_discussion ───────────────────────────────────────────────

async fn create_discussion_impl(
    user: &User,
    space_id: SpacePartition,
) -> crate::common::Result<serde_json::Value> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    let _space = require_space_creator(cli, user, space_id.clone()).await?;

    let author = SpaceAuthor::from(user.clone());
    let post = SpacePost::new(
        space_id.clone(),
        String::new(),
        String::new(),
        String::new(),
        &author,
        None,
        None,
    );

    let space_action = SpaceAction::new(
        space_id.clone(),
        SpacePostEntityType::from(post.sk.clone()).to_string(),
        SpaceActionType::TopicDiscussion,
    );

    let items = vec![
        post.create_transact_write_item(),
        space_action.create_transact_write_item(),
    ];
    crate::transact_write_items!(cli, items).map_err(|e| {
        crate::common::Error::Unknown(format!("Failed to create discussion: {e}"))
    })?;

    Ok(serde_json::json!({
        "pk": post.pk.to_string(),
        "sk": post.sk.to_string(),
        "message": "Discussion created"
    }))
}

// ── update_discussion ───────────────────────────────────────────────

async fn update_discussion_impl(
    user: &User,
    space_id: SpacePartition,
    discussion_id: String,
    update: serde_json::Value,
) -> crate::common::Result<serde_json::Value> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    let _space = require_space_creator(cli, user, space_id.clone()).await?;

    use crate::features::spaces::pages::actions::actions::discussion::controllers::UpdateDiscussionRequest;
    let req: UpdateDiscussionRequest = serde_json::from_value(update)
        .map_err(|_| crate::common::McpServerError::InvalidUpdateData)?;

    let space_pk: Partition = space_id.clone().into();
    let discussion_sk_type: SpacePostEntityType = discussion_id.into();
    let discussion_sk: EntityType = discussion_sk_type.clone().into();

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut updater = SpacePost::updater(&space_pk, &discussion_sk).with_updated_at(now);

    let action_pk = CompositePartition::<SpacePartition, String>(
        space_id.clone(),
        discussion_sk_type.to_string(),
    );
    let mut action_updater =
        SpaceAction::updater(&action_pk, &EntityType::SpaceAction).with_updated_at(now);
    let mut update_action = false;

    if let Some(title) = req.title {
        updater = updater.with_title(title.clone());
        action_updater = action_updater.with_title(title);
        update_action = true;
    }
    if let Some(html_contents) = req.html_contents {
        updater = updater.with_html_contents(html_contents.clone());
        action_updater = action_updater.with_description(html_contents);
        update_action = true;
    }
    if let Some(ref category_name) = req.category_name {
        updater = updater.with_category_name(category_name.clone());
    }
    if let Some(started_at) = req.started_at {
        updater = updater.with_started_at(started_at);
    }
    if let Some(ended_at) = req.ended_at {
        updater = updater.with_ended_at(ended_at);
    }

    let updated_post = updater.execute(cli).await?;

    if update_action {
        action_updater.execute(cli).await?;
    }

    if let Some(category_name) = req.category_name {
        if !category_name.is_empty() {
            let cat = SpaceCategory::new(space_id, category_name);
            let _ = cat.upsert(cli).await;
        }
    }

    Ok(serde_json::json!({
        "pk": updated_post.pk.to_string(),
        "sk": updated_post.sk.to_string(),
        "message": "Discussion updated"
    }))
}

// ── delete_discussion ───────────────────────────────────────────────

async fn delete_discussion_impl(
    user: &User,
    space_id: SpacePartition,
    discussion_id: String,
) -> crate::common::Result<String> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    let _space = require_space_creator(cli, user, space_id.clone()).await?;

    let space_pk: Partition = space_id.into();
    let discussion_sk_type: SpacePostEntityType = discussion_id.into();
    let discussion_sk: EntityType = discussion_sk_type.into();

    SpacePost::delete(cli, &space_pk, Some(discussion_sk)).await?;

    Ok("success".to_string())
}

// ── create_follow ───────────────────────────────────────────────────

async fn create_follow_impl(
    user: &User,
    space_id: SpacePartition,
) -> crate::common::Result<serde_json::Value> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    let space = require_space_creator(cli, user, space_id.clone()).await?;

    let pk: Partition = space_id.clone().into();
    let sk = EntityType::SpaceSubscription;

    if let Some(existing) = SpaceFollowAction::get(cli, &pk, Some(sk.clone())).await? {
        return Ok(serde_json::json!({
            "pk": existing.pk.to_string(),
            "sk": existing.sk.to_string(),
            "message": "Follow action already exists"
        }));
    }

    let follow = SpaceFollowAction::new(space_id.clone());
    let mut space_action = SpaceAction::new(
        space_id.clone(),
        SpaceActionFollowEntityType::from(follow.sk.clone()).to_string(),
        SpaceActionType::Follow,
    );
    space_action.title = if space.author_display_name.is_empty() {
        space.author_username
    } else {
        space.author_display_name
    };
    let items = vec![
        follow.create_transact_write_item(),
        space_action.create_transact_write_item(),
    ];
    crate::transact_write_items!(cli, items)
        .map_err(|e| crate::common::Error::Unknown(format!("Failed to create follow: {e}")))?;

    Ok(serde_json::json!({
        "pk": follow.pk.to_string(),
        "sk": follow.sk.to_string(),
        "message": "Follow action created"
    }))
}

// ── install_space_app ───────────────────────────────────────────────

async fn install_space_app_impl(
    user: &User,
    space_id: SpacePartition,
    app_type_str: &str,
) -> crate::common::Result<serde_json::Value> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    let _space = require_space_creator(cli, user, space_id.clone()).await?;

    let space_pk_partition: Partition = space_id.into();
    let sk = EntityType::SpaceApp(app_type_str.to_string());
    let now = crate::common::utils::time::get_now_timestamp_millis();

    // Build the item attributes directly since SpaceApp/SpaceAppType are in private modules.
    use aws_sdk_dynamodb::types::AttributeValue;
    let table = crate::common::models::space::SpaceCommon::table_name();
    cli.put_item()
        .table_name(table)
        .item("pk", AttributeValue::S(space_pk_partition.to_string()))
        .item("sk", AttributeValue::S(sk.to_string()))
        .item("app_type", AttributeValue::S(app_type_str.to_string()))
        .item("created_at", AttributeValue::N(now.to_string()))
        .item("updated_at", AttributeValue::N(now.to_string()))
        .send()
        .await
        .map_err(|e| crate::common::Error::Unknown(format!("Failed to install app: {e}")))?;

    Ok(serde_json::json!({
        "pk": space_pk_partition.to_string(),
        "sk": sk.to_string(),
        "app_type": app_type_str,
        "message": "App installed"
    }))
}

// ── uninstall_space_app ─────────────────────────────────────────────

async fn uninstall_space_app_impl(
    user: &User,
    space_id: SpacePartition,
    app_type_str: &str,
) -> crate::common::Result<serde_json::Value> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();
    let _space = require_space_creator(cli, user, space_id.clone()).await?;

    let space_pk_partition: Partition = space_id.into();
    let sk = EntityType::SpaceApp(app_type_str.to_string());
    let table = crate::common::models::space::SpaceCommon::table_name();

    cli.delete_item()
        .table_name(table)
        .key("pk", aws_sdk_dynamodb::types::AttributeValue::S(space_pk_partition.to_string()))
        .key("sk", aws_sdk_dynamodb::types::AttributeValue::S(sk.to_string()))
        .send()
        .await
        .map_err(|e| crate::common::Error::Unknown(format!("Failed to uninstall app: {e}")))?;

    Ok(serde_json::json!({
        "app_type": app_type_str,
        "message": "App uninstalled"
    }))
}

// ── list_teams ──────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
struct McpTeamItem {
    team_id: String,
    nickname: String,
    username: String,
    profile_url: String,
    description: String,
    permissions: Vec<String>,
}

fn permission_name(p: crate::features::posts::types::TeamGroupPermission) -> &'static str {
    use crate::features::posts::types::TeamGroupPermission;
    match p {
        TeamGroupPermission::PostRead => "PostRead",
        TeamGroupPermission::PostWrite => "PostWrite",
        TeamGroupPermission::PostEdit => "PostEdit",
        TeamGroupPermission::PostDelete => "PostDelete",
        TeamGroupPermission::SpaceRead => "SpaceRead",
        TeamGroupPermission::SpaceWrite => "SpaceWrite",
        TeamGroupPermission::SpaceEdit => "SpaceEdit",
        TeamGroupPermission::SpaceDelete => "SpaceDelete",
        TeamGroupPermission::TeamAdmin => "TeamAdmin",
        TeamGroupPermission::TeamEdit => "TeamEdit",
        TeamGroupPermission::GroupEdit => "GroupEdit",
        TeamGroupPermission::ManagePromotions => "ManagePromotions",
        TeamGroupPermission::ManageNews => "ManageNews",
    }
}

async fn list_teams_impl(user: &User) -> crate::common::Result<Vec<McpTeamItem>> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();

    let sk_prefix = "UserTeam".to_string();
    let opt = UserTeamQueryOption::builder().sk(sk_prefix);
    let (user_teams, _): (Vec<UserTeam>, _) = UserTeam::query(cli, &user.pk, opt).await?;

    let mut items = Vec::new();
    for ut in user_teams {
        let team_pk = match ut.sk.clone() {
            EntityType::UserTeam(team_pk) => team_pk,
            _ => continue,
        };

        let team_pk_partition: Partition = team_pk.parse().unwrap_or_default();
        let perms = Team::get_permissions_by_team_pk(cli, &team_pk_partition, &user.pk)
            .await
            .unwrap_or_else(|_| crate::features::posts::types::TeamGroupPermissions::empty());
        let description = Team::get(cli, &team_pk_partition, Some(EntityType::Team))
            .await
            .ok()
            .flatten()
            .map(|team| team.description)
            .unwrap_or_default();

        items.push(McpTeamItem {
            team_id: team_pk,
            nickname: ut.display_name,
            username: ut.username,
            profile_url: ut.profile_url,
            description,
            permissions: perms.0.into_iter().map(|p| permission_name(p).to_string()).collect(),
        });
    }

    Ok(items)
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
    user_pk: crate::common::types::Partition,
}

/// Global cache of MCP services keyed by hashed client_secret.
/// Each secret gets a persistent service so MCP sessions survive across requests.
/// Entries are evicted after `MCP_CACHE_TTL_SECS` or when the cache exceeds
/// `MAX_MCP_CACHE_SIZE`.
static MCP_SERVICES: std::sync::LazyLock<RwLock<HashMap<String, CachedMcpService>>> =
    std::sync::LazyLock::new(|| RwLock::new(HashMap::new()));

/// Resolve a user from the client secret stored in DynamoDB.
/// The incoming `client_secret` is the raw token; we hash it before lookup.
async fn resolve_user(client_secret: &str) -> Option<User> {
    let conf = ServerConfig::default();
    let cli = conf.dynamodb();

    let hashed = McpClientSecret::hash_secret(client_secret);
    let opt = McpClientSecret::opt().limit(1);
    let (secrets, _) = McpClientSecret::find_by_secret(cli, &hashed, opt)
        .await
        .ok()?;

    let secret = secrets.first()?;
    User::get(cli, &secret.pk, Some(EntityType::User))
        .await
        .ok()?
}

/// Get or create a cached MCP service for the given raw client_secret.
async fn get_or_create_service(client_secret: &str) -> Option<McpService> {
    let hashed_key = McpClientSecret::hash_secret(client_secret);
    let now = chrono::Utc::now().timestamp();

    // Fast path: check if a non-expired service already exists
    {
        let cache = MCP_SERVICES.read().await;
        if let Some(entry) = cache.get(&hashed_key) {
            if now - entry.created_at < MCP_CACHE_TTL_SECS {
                return Some(entry.service.clone());
            }
        }
    }

    // Slow path: resolve user and create service
    let user = resolve_user(client_secret).await?;

    let user_pk = user.pk.clone();
    let service = StreamableHttpService::new(
        move || RatelMcpServer::new(user.clone()),
        Arc::new(LocalSessionManager::default()),
        Default::default(),
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
            user_pk,
        },
    );
    Some(service)
}

/// Invalidate all cached MCP services for a given user partition key.
/// Called when a user regenerates their secret.
pub async fn invalidate_user_services(user_pk: &crate::common::types::Partition) {
    let mut cache = MCP_SERVICES.write().await;
    cache.retain(|_, entry| &entry.user_pk != user_pk);
}

/// Build the axum router for the MCP endpoint.
/// The client secret must be sent via the `Authorization: Bearer <token>` header.
pub fn mcp_router() -> Router {
    Router::new().route("/mcp", axum::routing::any(mcp_handler))
}

async fn mcp_handler(
    request: axum::http::Request<axum::body::Body>,
) -> axum::response::Response {
    let client_secret = request
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string());

    let Some(client_secret) = client_secret else {
        return axum::response::Response::builder()
            .status(401)
            .body(axum::body::Body::from(
                "Missing or invalid Authorization header. Expected: Bearer <token>",
            ))
            .unwrap();
    };

    let Some(service) = get_or_create_service(&client_secret).await else {
        return axum::response::Response::builder()
            .status(401)
            .body(axum::body::Body::from("Invalid client secret"))
            .unwrap();
    };

    let resp = service.handle(request).await;
    let (parts, body) = resp.into_parts();
    axum::response::Response::from_parts(parts, axum::body::Body::new(body))
}
