use aws_config::BehaviorVersion;
use aws_sdk_bedrockruntime::types::{ContentBlock, ConversationRole, Message};
use aws_sdk_bedrockruntime::Client as BedrockClient;
use dioxus::prelude::ServerFnError;

const MAX_SNAPSHOT_CHARS: usize = 60_000;

fn truncate_snapshot(snapshot_json: &str) -> String {
    if snapshot_json.chars().count() <= MAX_SNAPSHOT_CHARS {
        return snapshot_json.to_string();
    }
    let mut truncated = snapshot_json
        .chars()
        .take(MAX_SNAPSHOT_CHARS)
        .collect::<String>();
    truncated.push_str("\n\n[TRUNCATED]");
    truncated
}

fn strip_code_fences(raw: &str) -> String {
    let mut s = raw.trim().to_string();
    if s.starts_with("```") {
        // Remove opening fence line (``` or ```html).
        if let Some(pos) = s.find('\n') {
            s = s[(pos + 1)..].to_string();
        } else {
            return String::new();
        }
    }
    if s.ends_with("```") {
        s.truncate(s.len().saturating_sub(3));
    }
    s.trim().to_string()
}

pub async fn build_bedrock_client() -> Result<BedrockClient, ServerFnError> {
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    Ok(BedrockClient::new(&aws_config))
}

pub async fn generate_html_contents(
    space_pk: &str,
    snapshot_json: &str,
) -> Result<String, ServerFnError> {
    let config = crate::config::server_config::get();
    let model_id = config.bedrock_model_id;

    let snapshot_payload = truncate_snapshot(snapshot_json);
    let prompt = format!(
        "You are generating HTML contents for a report.\n\
Return ONLY valid HTML fragments (no markdown, no code fences/backticks).\n\
Space PK: {space_pk}\n\
Snapshot JSON:\n\
{snapshot_payload}"
    );

    let message = Message::builder()
        .role(ConversationRole::User)
        .content(ContentBlock::Text(prompt))
        .build()
        .map_err(|e| ServerFnError::new(format!("bedrock message build failed: {e:?}")))?;

    let client = build_bedrock_client().await?;
    let response = client
        .converse()
        .model_id(model_id)
        .messages(message)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("bedrock converse failed: {e:?}")))?;

    let mut html = String::new();
    if let Some(output) = response.output() {
        if let Ok(message) = output.as_message() {
            for block in message.content() {
                if let Ok(text) = block.as_text() {
                    html.push_str(text);
                } else if let ContentBlock::Text(text) = block {
                    html.push_str(&text);
                }
            }
        }
    }

    let html = strip_code_fences(&html);

    if html.trim().is_empty() {
        return Err(ServerFnError::new("bedrock returned empty html"));
    }

    Ok(html)
}
