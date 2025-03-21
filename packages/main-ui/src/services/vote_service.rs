#![allow(non_snake_case)]
use crate::config;
use bdk::prelude::*;
use dto::*;

#[derive(Debug, Clone, Copy)]
pub struct VoteService {
    pub cli: Signal<VoteClient>,
    pub user_cli: Signal<UserClient>,
}

impl VoteService {
    pub fn init() {
        let conf: &config::Config = config::get();
        let cli = Vote::get_client(&conf.main_api_endpoint);
        let user = User::get_client(&conf.main_api_endpoint);
        use_context_provider(|| Self {
            cli: Signal::new(cli),
            user_cli: Signal::new(user),
        });
    }

    pub async fn vote(&self, bill_id: i64, vote: VoteOption) -> Result<Vote> {
        let cli = (self.cli)();
        let user_cli = (self.user_cli)();
        let user_id = user_cli.user_info().await?.id;
        tracing::debug!(
            "Voting for bill_id: {}, vote: {:?}, user_id: {}",
            bill_id,
            vote,
            user_id
        );
        cli.voting(bill_id, vote, user_id).await
    }
}
