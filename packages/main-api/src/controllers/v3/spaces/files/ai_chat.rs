use crate::controllers::v3::spaces::dto::*;
use crate::features::spaces::files::SpaceFile;
use crate::models::space::SpaceCommon;
use crate::models::user::User;
use crate::utils::aws::{BedrockClient, BedrockModel, S3Client};
use crate::utils::pdf::extract_pdf_text;
use crate::*;

#[derive(Debug, Clone, serde::Deserialize, JsonSchema)]
pub struct AiChatRequest {
    pub message: String,
    pub context: PdfContext,
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
}

pub async fn ai_chat_handler(
    State(AppState { dynamo, s3, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path((space_pk, file_id)): Path<(Partition, String)>,
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

    // Extract S3 key from URL (format: https://{bucket_name}/{key})
    let s3_key = file_url
        .split('/')
        .skip(3)
        .collect::<Vec<_>>()
        .join("/");

    // Fetch PDF from S3
    let s3_object = s3.get_object_bytes(&s3_key).await?;

    // Extract text from PDF
    let pdf_text = extract_pdf_text(
        &s3_object.data,
        Some(payload.context.current_page as u32),
        2, // Include 2 pages before and after for context
    )
    .map_err(|e| Error::BadRequest(format!("Failed to extract PDF text: {}", e)))?;

    // Build AI prompt with PDF context
    let prompt = build_ai_prompt(&payload, &pdf_text);

    // Call AWS Bedrock
    let bedrock_client = BedrockClient::new();
    let ai_response = bedrock_client
        .send_message(BedrockModel::NovaLite, prompt, None)
        .await?;

    Ok(Json(AiChatResponse {
        message: ai_response,
    }))
}

fn build_ai_prompt(request: &AiChatRequest, pdf_text: &str) -> String {
    let mut prompt = format!(
        "You are an AI assistant helping a user analyze a PDF document.\n\n\
        Document: {}\n\
        Current Page: {} of {}\n\n",
        request.context.file_name, request.context.current_page, request.context.total_pages
    );

    if let Some(selected_text) = &request.context.selected_text {
        if !selected_text.is_empty() {
            prompt.push_str(&format!(
                "The user has selected the following text from the PDF:\n\
                \"{}\"\n\n",
                selected_text
            ));
        }
    }

    // Add extracted PDF content
    prompt.push_str("=== PDF Content ===\n");
    prompt.push_str(pdf_text);
    prompt.push_str("\n=== End of PDF Content ===\n\n");

    prompt.push_str(&format!("User question: {}\n\n", request.message));
    prompt.push_str(
        "Please provide a helpful and accurate response based on the PDF content provided above. \
        If the answer is not in the PDF content, let the user know.",
    );

    prompt
}
