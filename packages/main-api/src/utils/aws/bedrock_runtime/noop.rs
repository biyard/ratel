use aws_sdk_bedrockruntime::types::ContentBlock;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum BedrockModel {
    NovaLite,
    NovaMicro,
}
#[derive(Clone)]
pub struct BedrockClient {}

impl BedrockClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn send_message(
        &self,
        _model: BedrockModel,
        prompt: String,
        _content: Option<Vec<ContentBlock>>,
    ) -> crate::Result<String> {
        Ok(prompt)
    }

    pub async fn invoke_agent(
        &self,
        session_id: String,
        prompt: String,
    ) -> crate::Result<(String, String)> {
        Ok((format!("Mock response: {}", prompt), session_id))
    }

    pub async fn retrieve_and_generate(
        &self,
        _knowledge_base_id: String,
        session_id: Option<String>,
        prompt: String,
        _s3_uri: Option<String>,
    ) -> crate::Result<(String, String)> {
        let session = session_id.unwrap_or_else(|| "mock-session-id".to_string());
        Ok((format!("Mock KB response: {}", prompt), session))
    }
}
