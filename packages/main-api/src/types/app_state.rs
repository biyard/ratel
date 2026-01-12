use crate::config;
use crate::services::biyard::Biyard;
use crate::services::portone::PortOne;
use crate::utils::aws::{
    DynamoClient, S3Client, SesClient, SnsClient, get_aws_config, get_aws_config_for_sns,
};

#[derive(Clone)]
pub struct AppState {
    pub dynamo: DynamoClient,
    pub ses: SesClient,
    pub sns: SnsClient,
    pub portone: PortOne,
    pub s3: S3Client,
    pub biyard: Biyard,
}

impl AppState {
    pub fn new(dynamo: DynamoClient, ses: SesClient, sns: SnsClient, s3: S3Client) -> Self {
        let conf = config::get();

        let portone = PortOne::new(&conf.portone.api_secret);
        let biyard = Biyard::new();
        Self {
            dynamo,
            ses,
            sns,
            portone,
            biyard,
            s3,
        }
    }

    pub fn from_conf() -> Self {
        let conf = config::get();
        let is_local = conf.env == "local" || conf.env == "test";
        let aws_sdk_config = get_aws_config();
        let aws_sns_config = get_aws_config_for_sns();
        let dynamo = DynamoClient::new(Some(aws_sdk_config.clone()), true);
        let ses = SesClient::new(aws_sdk_config.clone(), is_local, true);
        let sns = SnsClient::new(aws_sns_config);
        let s3 = S3Client::new(conf.s3.name);

        let portone = PortOne::new(&conf.portone.api_secret);
        let biyard = Biyard::new();

        Self {
            dynamo,
            ses,
            sns,
            portone,
            s3,
            biyard,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::from_conf()
    }
}
