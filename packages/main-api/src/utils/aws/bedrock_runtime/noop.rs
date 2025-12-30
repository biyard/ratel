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
}
