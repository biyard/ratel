use crate::utils::aws::{dynamo::DynamoClient, s3::S3Client};
use dioxus::fullstack::{FullstackContext, extract::FromRef};

#[derive(Clone)]
pub struct ClientState {
    pub dynamo: DynamoClient,
    // pub s3: S3Client,
}

impl FromRef<FullstackContext> for ClientState {
    fn from_ref(state: &FullstackContext) -> Self {
        state.extension::<ClientState>().unwrap()
    }
}
