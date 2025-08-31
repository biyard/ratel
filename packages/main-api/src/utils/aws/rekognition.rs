use std::time::Duration;

use crate::config;
use aws_config::{Region, retry::RetryConfig, timeout::TimeoutConfig};

use aws_sdk_rekognition::{
    Client, Config,
    config::Credentials,
    types::{Image, Label},
};
// use aws_sdk_textract::types::Document;

use dto::{Error, Result};
#[derive(Clone)]
pub struct RekognitionClient {
    client: Client,
}

impl RekognitionClient {
    pub fn new() -> Self {
        let conf = config::get();
        let timeout_config = TimeoutConfig::builder()
            .operation_attempt_timeout(Duration::from_secs(5))
            .build();

        let retry_config = RetryConfig::standard().with_max_attempts(3);
        let aws_config = Config::builder()
            .credentials_provider(
                Credentials::builder()
                    .access_key_id(conf.aws.access_key_id)
                    .secret_access_key(conf.aws.secret_access_key)
                    .provider_name("ratel")
                    .build(),
            )
            .region(Region::new(conf.aws.region))
            .timeout_config(timeout_config)
            .retry_config(retry_config)
            .behavior_version_latest()
            .build();

        let client = Client::from_conf(aws_config);

        Self { client }
    }

    pub async fn detect_labels_from_image(
        &self,
        image: Image,
        max_labels: Option<i32>,
        min_confidence: Option<f32>,
    ) -> Result<Vec<Label>> {
        let rek_output = self
            .client
            .detect_labels()
            .image(image)
            .max_labels(max_labels.unwrap_or(10))
            .min_confidence(min_confidence.unwrap_or(0.0))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to detect labels: {:?}", e);
                Error::ServerError("Failed to detect labels".to_string())
            })?;

        let labels = rek_output.labels.ok_or_else(|| {
            tracing::error!("No labels found");
            Error::AwsRekognitionError("No labels found".to_string())
        })?;

        Ok(labels)
    }

    pub fn get_image_from_s3_object(bucket: &str, key: &str) -> Image {
        tracing::debug!(
            "Getting image from S3 object: bucket={}, key={}",
            bucket,
            key
        );
        let s3_object = aws_sdk_rekognition::types::S3Object::builder()
            .bucket(bucket)
            .name(key)
            .build();
        Image::builder().s3_object(s3_object).build()
    }
}

/*

aws iam create-policy \
    --policy-name PrivateS3AccessPolicy \
    --policy-document file://policy.json

aws iam attach-group-policy \
    --group-name dev \
    --policy-arn arn:aws:iam::385474633683:policy/PrivateS3AccessPolicy


 */
