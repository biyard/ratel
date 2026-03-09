use super::ServerConfig;
use crate::common::utils::aws::BedrockEmbeddingsClient;
use dioxus::fullstack::Lazy;

pub static BEDROCK_EMBEDDINGS: Lazy<BedrockEmbeddingsClient> = Lazy::new(|| async move {
    let config = ServerConfig::default();
    let aws_sdk_config = config.aws.get_sdk_config();

    dioxus::Ok(BedrockEmbeddingsClient::new(&aws_sdk_config))
});
