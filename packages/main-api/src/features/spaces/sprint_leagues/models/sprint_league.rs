use crate::types::{EntityType, Partition};

use super::{SprintLeaguePlayer, SprintLeagueVote};

use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct SprintLeague {
    pub pk: Partition,
    pub sk: EntityType,

    pub votes: i64,
    pub win_player: Option<EntityType>,
}

impl SprintLeague {
    pub fn new(space_pk: Partition) -> crate::Result<Self> {
        Ok(Self {
            pk: space_pk,
            sk: EntityType::SprintLeague,
            votes: 0,
            win_player: None,
        })
    }
}

impl SprintLeague {
    pub async fn is_voted(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
    ) -> crate::Result<bool> {
        let vote = SprintLeagueVote::find_one(cli, &self.pk, user_pk).await?;
        Ok(vote.is_some())
    }

    pub async fn vote(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        user_pk: &Partition,
        player_sk: &EntityType,
        referral_code: Option<String>,
    ) -> crate::Result<()> {
        let sprint_league_tx = SprintLeague::updater(space_pk, EntityType::SprintLeague)
            .increase_votes(1)
            .transact_write_item();

        let sprint_league_player_tx = SprintLeaguePlayer::updater(space_pk, player_sk)
            .increase_voter(1)
            .transact_write_item();

        let sprint_league_vote_tx = SprintLeagueVote::new(
            space_pk.clone(),
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
                tracing::error!("Failed to vote in sprint league: {:?}", e);
                println!("Failed to vote in sprint league: {:?}", e);
                crate::Error::SprintLeagueVoteError(e.to_string())
            })?;
        Ok(())
    }
}
