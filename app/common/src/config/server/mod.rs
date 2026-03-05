#![allow(static_mut_refs)]
pub mod aws_config;
pub mod aws_s3;
pub mod aws_ses;
pub mod aws_sns;
pub mod bedrock_embeddings_config;
pub mod biyard;
pub mod dynamodb;
pub mod qdrant_config;

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct ServerConfig {
    pub env: Environment,
    pub log_level: LogLevel,

    pub aws: aws_config::AwsConfig,
}

impl ServerConfig {
    pub fn dynamodb(&self) -> &aws_sdk_dynamodb::Client {
        &dynamodb::DB
    }
    pub fn s3(&self) -> &crate::utils::aws::S3Client {
        &aws_s3::S3_CLIENT
    }
    pub fn ses(&self) -> &crate::utils::aws::SesClient {
        &aws_ses::SES_CLIENT
    }
    pub fn sns(&self) -> &crate::utils::aws::SnsClient {
        &aws_sns::SNS_CLIENT
    }
    pub fn qdrant(&self) -> &crate::utils::aws::QdrantClient {
        &qdrant_config::QDRANT_CLIENT
    }
    pub fn bedrock_embeddings(&self) -> &crate::utils::aws::BedrockEmbeddingsClient {
        &bedrock_embeddings_config::BEDROCK_EMBEDDINGS
    }
    pub fn biyard(&self) -> &crate::services::BiyardService {
        &biyard::BIYARD_SERVICE
    }
}

static mut CONFIG: Option<ServerConfig> = None;

impl Default for ServerConfig {
    fn default() -> Self {
        unsafe {
            if CONFIG.is_none() {
                let obj = Self {
                    env: Default::default(),
                    log_level: Default::default(),
                    aws: Default::default(),
                };

                CONFIG = Some(obj);
            }
            CONFIG.clone().unwrap()
        }
    }
}
