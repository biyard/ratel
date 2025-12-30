use crate::controllers::v3::spaces::dto::*;
use crate::features::spaces::files::SpaceFile;
use crate::models::space::SpaceCommon;
use crate::models::user::User;
use crate::utils::aws::{BedrockClient, S3Client};
use crate::config::Config;
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
    Path((space_pk, file_id)): Path<(Partition, String)>,
    Json(payload): Json<AiChatRequest>,
) -> Result<Json<AiChatResponse>> {
    let config = Config::default();
    
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

    let files = files.ok_or_else(|| Error::NotFound("Space files not found".to_string()))?;

    // Decode file_id to match against file names
    let decoded_file_id = urlencoding::decode(&file_id)
        .map_err(|_| Error::BadRequest("Invalid file ID encoding".to_string()))?;

    // Find the file and get its URL
    let file = files
        .files
        .iter()
        .find(|f| f.name == decoded_file_id.as_ref())
        .ok_or_else(|| Error::NotFound("File not found in space".to_string()))?;

    let file_url = file
        .url
        .as_ref()
        .ok_or_else(|| Error::BadRequest("File URL not available".to_string()))?;

    // Extract S3 key from any URL format (CloudFront, Route53, or direct S3)
    // The S3 key is the path component after the domain
    let s3_key = file_url
        .split('/')
        .skip(3) // Skip scheme and domain parts
        .collect::<Vec<_>>()
        .join("/");
    
    // Build s3:// URI for Knowledge Base filtering
    let s3_uri = format!("s3://{}/{}", config.s3.name, s3_key);
    
    tracing::debug!("File URL: {}", file_url);
    tracing::debug!("Extracted S3 key: {}", s3_key);
    tracing::debug!("KB filter URI: {}", s3_uri);

    // Build context-aware prompt
    let mut prompt = format!(
        "Document: {}\nCurrent page: {} of {}\n\n",
        payload.context.file_name, payload.context.current_page, payload.context.total_pages
    );

    if let Some(selected_text) = &payload.context.selected_text {
        if !selected_text.is_empty() {
            prompt.push_str(&format!("Selected text: \"{}\"\n\n", selected_text));
        }
    }

    prompt.push_str(&format!("Question: {}", payload.message));

    // Use RetrieveAndGenerate with Knowledge Base
    let bedrock_client = BedrockClient::new();
    
    #[cfg(not(feature = "no-secret"))]
    let knowledge_base_id = config.bedrock.knowledge_base_id.to_string();
    #[cfg(feature = "no-secret")]
    let knowledge_base_id = "mock-kb-id".to_string();
    
    let (ai_response, returned_session_id) = bedrock_client
        .retrieve_and_generate(
            knowledge_base_id,
            payload.session_id.clone(),
            prompt,
            Some(s3_uri),
        )
        .await?;

    Ok(Json(AiChatResponse {
        message: ai_response,
        session_id: returned_session_id,
    }))
}
