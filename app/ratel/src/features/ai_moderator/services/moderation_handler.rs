use aws_config::BehaviorVersion;
use aws_sdk_bedrockruntime::types::{
    ContentBlock, ConversationRole, InferenceConfiguration, Message,
};
use aws_sdk_bedrockruntime::Client as BedrockClient;

use crate::common::Result;
use crate::features::ai_moderator::types::AiModeratorError;
use crate::features::ai_moderator::models::*;
use crate::common::types::*;

const DEFAULT_MODEL_ID: &str = "anthropic.claude-sonnet-4-20250514";
const MAX_OUTPUT_TOKENS: i32 = 1024;
const MAX_PROMPT_CHARS: usize = 50_000;
const MAX_MATERIALS: usize = 5;
const MAX_REPLIES: usize = 50;

fn model_id() -> String {
    std::env::var("AI_MODERATOR_MODEL_ID").unwrap_or_else(|_| DEFAULT_MODEL_ID.to_string())
}

pub async fn should_moderate(
    cli: &aws_sdk_dynamodb::Client,
    space_id: &SpacePartition,
    discussion_sk: &str,
    reply_count: i64,
) -> Result<Option<AiModeratorConfig>> {
    let pk = CompositePartition(space_id.clone(), discussion_sk.to_string());
    let config =
        AiModeratorConfig::get(cli, &pk, Some(EntityType::AiModeratorConfig)).await?;

    match config {
        Some(config) if config.enabled && config.reply_interval > 0 => {
            if reply_count > 0 && reply_count % config.reply_interval == 0 {
                Ok(Some(config))
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

pub async fn generate_moderation_reply(
    config: &AiModeratorConfig,
    recent_replies: Vec<String>,
    material_context: Vec<String>,
) -> Result<String> {
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = BedrockClient::new(&aws_config);

    let mut prompt_parts = vec![
        "You are an AI discussion moderator. Your job is to moderate and summarize the discussion."
            .to_string(),
    ];

    if !config.guidelines.is_empty() {
        let guidelines = truncate_str(&config.guidelines, 5_000);
        prompt_parts.push(format!("\n## Moderation Guidelines\n{}", guidelines));
    }

    let limited_materials: Vec<_> = material_context.into_iter().take(MAX_MATERIALS).collect();
    if !limited_materials.is_empty() {
        prompt_parts.push("\n## Reference Materials".to_string());
        for (i, material) in limited_materials.iter().enumerate() {
            let truncated = truncate_str(material, 5_000);
            prompt_parts.push(format!("### Material {}\n{}", i + 1, truncated));
        }
    }

    let limited_replies: Vec<_> = recent_replies.into_iter().take(MAX_REPLIES).collect();
    prompt_parts.push("\n## Recent Discussion Replies".to_string());
    for (i, reply) in limited_replies.iter().enumerate() {
        let truncated = truncate_str(reply, 1_000);
        prompt_parts.push(format!("{}. {}", i + 1, truncated));
    }

    prompt_parts.push(
        "\n## Your Task\nBased on the above guidelines and discussion context, write a moderation response. Write directly as the moderator."
            .to_string(),
    );

    let mut full_prompt = prompt_parts.join("\n");
    if full_prompt.len() > MAX_PROMPT_CHARS {
        full_prompt = full_prompt[..MAX_PROMPT_CHARS].to_string();
    }

    let message = Message::builder()
        .role(ConversationRole::User)
        .content(ContentBlock::Text(full_prompt))
        .build()
        .map_err(|e| {
            crate::error!("Failed to build message: {e:?}");
            AiModeratorError::MessageBuildFailed
        })?;

    let response = client
        .converse()
        .model_id(model_id())
        .inference_config(
            InferenceConfiguration::builder()
                .max_tokens(MAX_OUTPUT_TOKENS)
                .build(),
        )
        .messages(message)
        .send()
        .await
        .map_err(|e| {
            crate::error!("Bedrock converse failed: {e:?}");
            AiModeratorError::BedrockConverseFailed
        })?;

    let mut text = String::new();
    if let Some(output) = response.output() {
        if let Ok(msg) = output.as_message() {
            for block in msg.content() {
                if let Ok(t) = block.as_text() {
                    text.push_str(t);
                }
            }
        }
    }

    if text.is_empty() {
        text = "Unable to generate moderation response.".to_string();
    }

    Ok(text)
}

fn truncate_str(s: &str, max_chars: usize) -> &str {
    if s.len() <= max_chars {
        s
    } else {
        let mut end = max_chars;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        &s[..end]
    }
}
