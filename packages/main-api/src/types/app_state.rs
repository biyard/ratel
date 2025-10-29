use crate::config;
use crate::services::portone::PortOne;
use crate::utils::aws::{DynamoClient, SesClient};

#[derive(Clone)]
pub struct AppState {
    pub dynamo: DynamoClient,
    pub ses: SesClient,
    pub pool: bdk::prelude::sqlx::PgPool,
    pub portone: PortOne,
}

impl AppState {
    pub fn new(dynamo: DynamoClient, ses: SesClient, pool: bdk::prelude::sqlx::PgPool) -> Self {
        let conf = config::get();

        let portone = PortOne::new(&conf.portone.api_secret);

        Self {
            dynamo,
            ses,
            pool,
            portone,
        }
    }
}
