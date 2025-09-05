mod bedrock_runtime;
pub use bedrock_runtime::{BedrockClient, BedrockModel};

mod rekognition;
pub use rekognition::RekognitionClient;

mod textract;
pub use textract::TextractClient;

mod s3;
pub use s3::{S3Client, S3ContentType, S3Object};
