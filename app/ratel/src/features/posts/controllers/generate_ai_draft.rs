//! POST /api/posts/:post_id/ai-draft — generate an AI-assisted opinion
//! gathering draft and apply it to the post.
//!
//! Enforces:
//!   - Caller is the post author / has PostEdit permission
//!   - Caller's membership tier is paid (Pro or higher)
//!   - The post hasn't already consumed its one-shot AI draft allowance
//!     (DynamoDB conditional update on `ai_draft_used` is the final guard)

use crate::features::auth::User;
use crate::features::membership::controllers::get_membership_handler;
use crate::features::posts::models::Post;
use crate::features::posts::services::ai_draft;
use crate::features::posts::types::{
    AiDraftLanguage, AiDraftTemplate, AiPostDraftError, TeamGroupPermission,
};
use crate::features::posts::*;

#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct GenerateAiDraftRequest {
    pub template: AiDraftTemplate,
    pub topic: String,
    pub background: String,
    pub feedback_request: String,
    #[serde(default)]
    pub participation_notes: Option<String>,
    pub language: AiDraftLanguage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct GenerateAiDraftResponse {
    pub title: String,
    pub body_html: String,
}

#[post("/api/posts/:post_id/ai-draft", user: User)]
pub async fn generate_ai_draft_handler(
    post_id: FeedPartition,
    req: GenerateAiDraftRequest,
) -> Result<GenerateAiDraftResponse> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.into();

    // 1. Permission: post must exist and the caller must be allowed to edit it.
    let (post, has_permission) =
        Post::has_permission(cli, &post_pk, Some(&user.pk), TeamGroupPermission::PostEdit).await?;
    if !has_permission {
        return Err(crate::features::posts::types::PostError::NotAccessible.into());
    }

    // 2. Cheap pre-check: don't burn an AI call if we already know this
    //    post used its allowance. The conditional update at step 6 is the
    //    final, race-free guard.
    if post.ai_draft_used {
        return Err(AiPostDraftError::AlreadyUsed.into());
    }

    // 3. Membership: only Pro and above can use AI drafting.
    let membership = get_membership_handler().await?;
    if !membership.is_paid() {
        return Err(AiPostDraftError::PaidOnly.into());
    }

    // 4. Input validation.
    if req.topic.trim().is_empty()
        || req.background.trim().is_empty()
        || req.feedback_request.trim().is_empty()
    {
        return Err(AiPostDraftError::InvalidInput.into());
    }

    // 5. Generate via the configured WriterAi backend.
    let AiDraftTemplate::OpinionGathering = req.template;
    let draft = ai_draft::generate_opinion_draft(ai_draft::OpinionDraftInput {
        topic: req.topic,
        background: req.background,
        feedback_request: req.feedback_request,
        participation_notes: req
            .participation_notes
            .filter(|s| !s.trim().is_empty()),
        language: req.language,
    })
    .await?;

    // 6. Conditional update — race-free one-shot guard.
    let now = chrono::Utc::now().timestamp_millis();
    let body_value = ContentBody::html(draft.body_html.clone());
    let body_attr: aws_sdk_dynamodb::types::AttributeValue =
        serde_dynamo::to_attribute_value(&body_value)
            .map_err(|_| AiPostDraftError::GenerationFailed)?;

    let mut key = std::collections::HashMap::new();
    key.insert(
        "pk".to_string(),
        aws_sdk_dynamodb::types::AttributeValue::S(post.pk.to_string()),
    );
    key.insert(
        "sk".to_string(),
        aws_sdk_dynamodb::types::AttributeValue::S(post.sk.to_string()),
    );

    let mut values = std::collections::HashMap::new();
    values.insert(
        ":t".to_string(),
        aws_sdk_dynamodb::types::AttributeValue::S(draft.title.clone()),
    );
    values.insert(":b".to_string(), body_attr);
    values.insert(
        ":u".to_string(),
        aws_sdk_dynamodb::types::AttributeValue::Bool(true),
    );
    values.insert(
        ":updated_at".to_string(),
        aws_sdk_dynamodb::types::AttributeValue::N(now.to_string()),
    );
    values.insert(
        ":false".to_string(),
        aws_sdk_dynamodb::types::AttributeValue::Bool(false),
    );

    let mut names = std::collections::HashMap::new();
    names.insert("#title".to_string(), "title".to_string());
    names.insert("#body".to_string(), "body".to_string());
    names.insert("#ai_draft_used".to_string(), "ai_draft_used".to_string());
    names.insert("#updated_at".to_string(), "updated_at".to_string());

    let result = cli
        .update_item()
        .table_name(Post::table_name())
        .set_key(Some(key))
        .update_expression(
            "SET #title = :t, #body = :b, #ai_draft_used = :u, #updated_at = :updated_at",
        )
        .condition_expression(
            "attribute_not_exists(#ai_draft_used) OR #ai_draft_used = :false",
        )
        .set_expression_attribute_values(Some(values))
        .set_expression_attribute_names(Some(names))
        .send()
        .await;

    if let Err(e) = result {
        use aws_sdk_dynamodb::error::ProvideErrorMetadata;
        let svc_err: aws_sdk_dynamodb::Error = e.into();
        if svc_err
            .code()
            .map(|c| c == "ConditionalCheckFailedException")
            .unwrap_or(false)
        {
            return Err(AiPostDraftError::AlreadyUsed.into());
        }
        tracing::error!(error = ?svc_err, post_pk = %post.pk, "ai draft conditional update failed");
        return Err(AiPostDraftError::GenerationFailed.into());
    }

    tracing::info!(
        user_pk = %user.pk,
        post_pk = %post.pk,
        backend = ?conf.common.ai_writer().kind,
        model = %conf.common.ai_writer().model,
        title_len = draft.title.len(),
        body_len = draft.body_html.len(),
        "ai draft generated and applied",
    );

    Ok(GenerateAiDraftResponse {
        title: draft.title,
        body_html: draft.body_html,
    })
}
