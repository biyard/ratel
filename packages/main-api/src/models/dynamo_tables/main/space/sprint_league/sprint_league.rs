use crate::{
    models::{SprintLeaguePlayer, SprintLeagueVote},
    types::*,
};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct SprintLeague {
    pub pk: Partition,
    pub sk: EntityType,

    pub voters: i64,
    pub win_player: Option<EntityType>,
}

impl SprintLeague {
    pub fn new(space_pk: Partition) -> crate::Result<Self> {
        let post_id = match space_pk {
            Partition::Space(id) => id,
            _ => {
                return Err(crate::Error::InvalidPartitionKey(
                    "SprintLeague must be under Space partition".to_string(),
                ));
            }
        };

        Ok(Self {
            pk: Partition::SprintLeague(post_id),
            sk: EntityType::SprintLeague,
            voters: 0,
            win_player: None,
        })
    }
}

impl SprintLeague {
    pub fn increment_voters(&mut self) {
        self.voters += 1;
    }

    pub async fn is_voted(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
    ) -> crate::Result<bool> {
        let vote = SprintLeagueVote::find_one(cli, &self.pk, user_pk).await?;
        Ok(vote.is_some())
    }

    pub async fn vote(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
        player_sk: &EntityType,
        referral_code: Option<String>,
    ) -> crate::Result<()> {
        let vote = SprintLeagueVote::find_one(cli, &self.pk, user_pk).await?;
        if vote.is_some() {
            return Err(crate::Error::AlreadyVoted);
        }

        let sprint_league_tx = SprintLeague::updater(&self.pk, &self.sk)
            .increase_voters(1)
            .transact_write_item();

        let sprint_league_player_tx =
            SprintLeaguePlayer::updater(self.pk.clone(), player_sk.clone())
                .increase_voter(1)
                .transact_write_item();
        let sprint_league_vote_tx = SprintLeagueVote::new(
            self.pk.clone(),
            user_pk.clone(),
            player_sk.clone(),
            referral_code,
        )?
        .create_transact_write_item();

        cli.transact_write_items()
            .set_transact_items(Some(vec![
                sprint_league_tx,
                sprint_league_player_tx,
                sprint_league_vote_tx,
            ]))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to vote in sprint league: {}", e);
                crate::Error::SprintLeagueVoteError(e.to_string())
            })?;
        Ok(())
    }
}
