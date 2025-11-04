use crate::config;
use crate::services::portone::PortOne;
use crate::utils::aws::{DynamoClient, S3Client, SesClient};

#[derive(Clone)]
pub struct AppState {
    pub dynamo: DynamoClient,
    pub ses: SesClient,
    pub portone: PortOne,
    pub s3: S3Client,
}

impl AppState {
    pub fn new(dynamo: DynamoClient, ses: SesClient, s3: S3Client) -> Self {
        let conf = config::get();

        let portone = PortOne::new(&conf.portone.api_secret);

        Self {
            dynamo,
            ses,
            portone,
            s3,
        }
    }
}
