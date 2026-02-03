use crate::controllers::v3::spaces::dto::*;
use crate::features::spaces::analyzes::SpaceAnalyze;
use crate::features::spaces::files::SpaceFile;
use crate::types::File;
use crate::*;

#[derive(Debug, Clone, serde::Deserialize, JsonSchema)]
pub struct AiChatRequest {
    pub message: String,
    pub context: PdfContext,
    pub session_id: Option<String>,
    #[serde(flatten)]
    pub target: AiChatTarget,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, JsonSchema)]
pub struct PdfContext {
    pub file_name: String,
    pub current_page: i32,
    pub total_pages: i32,
    pub selected_text: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, JsonSchema)]
pub struct AiChatResponse {
    pub message: String,
    pub session_id: String,
}

#[derive(Debug, Clone, serde::Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum AiChatTarget {
    File { file_id: String },
    Analyze { analyze_pk: String },
}

pub async fn ai_chat_handler(
    State(AppState {
        dynamo, bedrock, ..
    }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(payload): Json<AiChatRequest>,
) -> Result<Json<AiChatResponse>> {
    // Verify space access
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundDeliberationSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    let file_url = match payload.target {
        AiChatTarget::File { file_id } => {
            let (pk, sk) = SpaceFile::keys(&space_pk);
            let files = SpaceFile::get(&dynamo.client, &pk, Some(sk)).await?;

            let files: SpaceFile =
                files.ok_or_else(|| Error::NotFound("Space files not found".to_string()))?;

            let file: &File = files
                .files
                .iter()
                .find(|f| f.id == file_id)
                .ok_or_else(|| Error::NotFound("File not found in space".to_string()))?;

            file.url.as_deref().unwrap_or("").to_string()
        }
        AiChatTarget::Analyze { analyze_pk: _ } => {
            let user_membership = user.get_user_membership(&dynamo.client).await?;
            if !user_membership.is_active() {
                return Err(Error::ExpiredMembership);
            }
            if !user_membership.is_paid_membership() {
                return Err(Error::InvalidMembership);
            }

            let analyze =
                SpaceAnalyze::get(&dynamo.client, &space_pk, Some(EntityType::SpaceAnalyze))
                    .await?;
            let analyze = analyze.ok_or(Error::AnalyzeNotFound)?;
            analyze.metadata_url.unwrap_or_default()
        }
    };

    // Build context-aware prompt with file information
    let mut prompt = format!(
        "Document: {}\nFile URL: {}\nCurrent page: {} of {}\n\n",
        payload.context.file_name,
        file_url,
        payload.context.current_page,
        payload.context.total_pages
    );

    if let Some(selected_text) = &payload.context.selected_text {
        if !selected_text.is_empty() {
            prompt.push_str(&format!("Selected text: \"{}\"\n\n", selected_text));
        }
    }

    prompt.push_str(&format!("Question: {}", payload.message));

    // Use invoke_agent to leverage the agent's conversational instructions
    // Generate session ID if not provided
    let (ai_response, returned_session_id) =
        bedrock.invoke_agent(payload.session_id, prompt).await?;

    Ok(Json(AiChatResponse {
        message: ai_response,
        session_id: returned_session_id,
    }))
}
