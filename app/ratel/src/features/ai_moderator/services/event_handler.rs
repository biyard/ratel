use crate::common::Result;
use crate::features::ai_moderator::types::AiModeratorError;
use crate::common::types::*;
use crate::features::ai_moderator::models::*;
use crate::features::ai_moderator::services::moderation_handler;
// SpacePost and SpacePostComment are publicly re-exported from the discussion module
use crate::features::spaces::pages::actions::actions::discussion::SpacePost;
use crate::features::spaces::pages::actions::actions::discussion::SpacePostComment;

const AI_MODERATOR_DISPLAY_NAME: &str = "AI Moderator";
const AI_MODERATOR_USERNAME: &str = "ai-moderator";
const MAX_RECENT_REPLIES: usize = 20;

/// Handle a SpacePost stream event: check if AI moderation should trigger.
///
/// Called from EventBridge when a SpacePost's `comments` field changes.
/// The `post` is deserialized from the DynamoDB NewImage.
pub async fn handle_ai_moderator_event(post: SpacePost) -> Result<()> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let space_pk_str = post.pk.to_string();
    let discussion_sk_str = post.sk.to_string();

    // Extract SpacePartition from the space pk string (e.g., "SPACE#abc" → "abc")
    let space_id = {
        let inner = space_pk_str
            .strip_prefix("SPACE#")
            .unwrap_or(&space_pk_str);
        SpacePartition(inner.to_string())
    };

    // Check if AI moderation should trigger for this reply count
    let config = match moderation_handler::should_moderate(
        cli,
        &space_id,
        &discussion_sk_str,
        post.comments,
    )
    .await?
    {
        Some(config) => config,
        None => return Ok(()), // No moderation needed
    };

    tracing::info!(
        space_pk = %space_pk_str,
        discussion_sk = %discussion_sk_str,
        comments = post.comments,
        reply_interval = config.reply_interval,
        "AI Moderator triggered"
    );

    // Fetch recent replies for context
    // SpacePostPartition is derived from the discussion sk string (e.g., "SPACE_POST#uuid" → "uuid")
    let discussion_id = discussion_sk_str
        .strip_prefix("SPACE_POST#")
        .unwrap_or(&discussion_sk_str);
    let space_post_pk = SpacePostPartition(discussion_id.to_string());
    let post_pk: Partition = space_post_pk.clone().into();

    let opt = SpacePostComment::opt()
        .sk(EntityType::SpacePostComment(String::default()).to_string())
        .limit(MAX_RECENT_REPLIES as i32);
    let (comments, _): (Vec<SpacePostComment>, Option<String>) =
        SpacePostComment::query(cli, &post_pk, opt).await?;

    let recent_replies: Vec<String> = comments
        .iter()
        .map(|c| format!("[{}]: {}", c.author_display_name, c.content))
        .collect();

    // Fetch material context from Qdrant (best-effort)
    let material_context = fetch_material_context(
        &common_config,
        &space_pk_str,
        &discussion_sk_str,
        &recent_replies,
    )
    .await;

    // Generate AI moderation reply
    let reply_text =
        moderation_handler::generate_moderation_reply(&config, recent_replies, material_context)
            .await?;

    // Save the AI moderator reply as a SpacePostComment
    let ai_author = crate::common::models::space::SpaceUser {
        pk: Partition::default(),
        display_name: AI_MODERATOR_DISPLAY_NAME.to_string(),
        username: AI_MODERATOR_USERNAME.to_string(),
        profile_url: String::new(),
    };

    let comment: SpacePostComment = SpacePost::comment(
        cli,
        space_id,
        space_post_pk,
        reply_text,
        vec![],
        &ai_author,
    )
    .await
    .map_err(|e| {
        crate::error!("Failed to save AI moderator reply: {e}");
        AiModeratorError::ReplySaveFailed
    })?;

    tracing::info!(
        comment_sk = %comment.sk,
        "AI Moderator reply saved"
    );

    Ok(())
}

/// Fetch relevant material context from Qdrant (best-effort, errors are logged and ignored).
async fn fetch_material_context(
    config: &crate::common::CommonConfig,
    _space_id: &str,
    _discussion_sk: &str,
    recent_replies: &[String],
) -> Vec<String> {
    let query_text = recent_replies
        .iter()
        .take(5)
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");

    if query_text.is_empty() {
        return Vec::new();
    }

    let bedrock = config.bedrock_embeddings();
    let _embedding = match bedrock.embed(&query_text).await {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!(error = %e, "Failed to embed query for material search");
            return Vec::new();
        }
    };

    let _qdrant = config.qdrant();
    // TODO: Implement Qdrant search for material retrieval using the official client.
    // Use MaterialPayload::collection_name() and filter by type=material, space_id, discussion_id.
    // Use _qdrant.search_points(...) with _embedding vector.

    Vec::new()
}
