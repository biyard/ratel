use crate::features::spaces::analyzes::SpaceAnalyze;
use crate::utils::aws::BedrockClient;
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
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path((space_pk, _analyze_pk)): Path<(Partition, String)>,
    Json(payload): Json<AiChatRequest>,
) -> Result<Json<AiChatResponse>> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundDeliberationSpace);
    }

    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let user_membership = user.get_user_membership(&dynamo.client).await?;
    if !user_membership.is_active() {
        return Err(Error::ExpiredMembership);
    }
    let membership_name = user_membership.membership_pk.to_string();
    if membership_name.contains("Free") {
        return Err(Error::NoMembershipFound);
    }

    let analyze =
        SpaceAnalyze::get(&dynamo.client, &space_pk, Some(EntityType::SpaceAnalyze)).await?;
    let analyze = analyze.ok_or(Error::AnalyzeNotFound)?;

    let file_url = analyze.metadata_url.unwrap_or_default();

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

    let bedrock_client = BedrockClient::new();
    let session_id = payload
        .session_id
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let (ai_response, returned_session_id) =
        bedrock_client.invoke_agent(session_id, prompt).await?;

    Ok(Json(AiChatResponse {
        message: ai_response,
        session_id: returned_session_id,
    }))
}
