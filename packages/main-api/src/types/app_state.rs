use crate::config;
use crate::services::portone::PortOne;
use crate::utils::aws::{DynamoClient, S3Client, SesClient, SnsClient, get_aws_config};

#[derive(Clone)]
pub struct AppState {
    pub dynamo: DynamoClient,
    pub ses: SesClient,
    pub sns: SnsClient,
    pub portone: PortOne,
    pub s3: S3Client,
}

impl AppState {
    pub fn new(dynamo: DynamoClient, ses: SesClient, sns: SnsClient, s3: S3Client) -> Self {
        let conf = config::get();

        let portone = PortOne::new(&conf.portone.api_secret);

        Self {
            dynamo,
            ses,
            sns,
            portone,
            s3,
        }
    }

    pub fn from_conf() -> Self {
        let conf = config::get();
        let is_local = conf.env == "local" || conf.env == "test";
        let aws_sdk_config = get_aws_config();
        let dynamo = DynamoClient::new(Some(aws_sdk_config.clone()));
        let ses = SesClient::new(aws_sdk_config.clone(), is_local);
        let sns = SnsClient::new(aws_sdk_config, is_local);
        let s3 = S3Client::new(conf.bucket.name);

        let portone = PortOne::new(&conf.portone.api_secret);

        Self {
            dynamo,
            ses,
            sns,
            portone,
            s3,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::from_conf()
    }
}
