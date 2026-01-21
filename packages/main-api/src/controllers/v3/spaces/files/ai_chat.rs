use crate::types::File;
use crate::features::spaces::files::SpaceFile;
use crate::utils::aws::BedrockClient;
use crate::controllers::v3::spaces::dto::*;
use crate::*;

#[derive(Debug, Clone, serde::Deserialize, JsonSchema)]
pub struct AiChatRequest {
    pub message: String,
    pub context: PdfContext,
    pub session_id: Option<String>,
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

pub async fn ai_chat_handler(
    State(AppState { dynamo, s3: _, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path((space_pk, file_id)): Path<(Partition, String)>, // file_id is the unique file ID (UUID)
    Json(payload): Json<AiChatRequest>,
) -> Result<Json<AiChatResponse>> {
    // Verify space access
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundDeliberationSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    // Verify file exists in space
    let (pk, sk) = SpaceFile::keys(&space_pk);
    let files = SpaceFile::get(&dynamo.client, &pk, Some(sk)).await?;

    let files: SpaceFile = files.ok_or_else(|| Error::NotFound("Space files not found".to_string()))?;

    // Find the file by its unique ID
    let file: &File = files
        .files
        .iter()
        .find(|f| f.id == file_id)
        .ok_or_else(|| Error::NotFound("File not found in space".to_string()))?;

    let file_url = file.url.as_deref().unwrap_or("");

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
    let bedrock_client = BedrockClient::new();
    
    // Generate session ID if not provided
    let session_id = payload.session_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    
    let (ai_response, returned_session_id) = bedrock_client
        .invoke_agent(
            session_id,
            prompt,
        )
        .await?;

    Ok(Json(AiChatResponse {
        message: ai_response,
        session_id: returned_session_id,
    }))
}
