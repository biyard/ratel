use crate::{Error, types::*};
use bdk::prelude::*;
use uuid::Uuid;

use super::super::PlayerImage;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, schemars::JsonSchema,
)]

pub struct SprintLeaguePlayer {
    // #[dynamo(
    //     prefix = "SPRINT_LEAGUE_PK",
    //     name = "order_by_votes",
    //     index = "gsi1",
    //     pk
    // )]
    pub pk: Partition,
    pub sk: EntityType,

    pub name: String,
    pub description: String,

    pub player_image: PlayerImage,
    // #[dynamo(index = "gsi1", sk)]
    pub votes: i64,
}

impl SprintLeaguePlayer {
    pub fn new(
        pk: Partition,
        name: String,
        description: String,
        player_image: PlayerImage,
    ) -> crate::Result<Self> {
        let uuid = Uuid::new_v4().to_string();
        if !matches!(pk, Partition::Space(_)) {
            return Err(crate::Error::InvalidPartitionKey(
                "SprintLeaguePlayer must be under SprintLeague partition".to_string(),
            ));
        }

        Ok(Self {
            pk,
            sk: EntityType::SprintLeaguePlayer(uuid),
            player_image,
            name,
            description,
            votes: 0,
        })
    }
    pub async fn get_all(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
    ) -> crate::Result<Vec<SprintLeaguePlayer>> {
        if !space_pk.is_space_key() {
            return Err(crate::Error::InvalidPartitionKey(
                "SprintLeaguePlayer must be under Space partition".to_string(),
            ));
        }

        let mut players = Vec::new();
        let mut bookmark = None::<String>;
        loop {
            let mut options = SprintLeaguePlayerQueryOption::builder()
                .sk(EntityType::SprintLeaguePlayer(String::default()).to_string());
            if let Some(b) = &bookmark {
                options = options.bookmark(b.clone());
            }
            let (mut queried_players, next_bookmark) =
                SprintLeaguePlayer::query(cli, space_pk, options).await?;

            players.append(&mut queried_players);

            match next_bookmark {
                Some(b) => bookmark = Some(b),
                None => break,
            }
        }

        Ok(players)
    }
    pub async fn delete_all(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
    ) -> crate::Result<()> {
        if !space_pk.is_space_key() {
            return Err(crate::Error::InvalidPartitionKey(
                "SprintLeaguePlayer must be under Space partition".to_string(),
            ));
        }

        let mut bookmark = None::<String>;
        loop {
            let mut options = SprintLeaguePlayerQueryOption::builder().limit(100);
            if let Some(b) = &bookmark {
                options = options.bookmark(b.clone());
            }
            let (players, next_bookmark) =
                SprintLeaguePlayer::query(cli, space_pk, options).await?;

            if players.is_empty() {
                break;
            }

            let tx_items = players
                .into_iter()
                .map(|player| SprintLeaguePlayer::delete_transact_write_item(player.pk, player.sk))
                .collect::<Vec<_>>();

            cli.transact_write_items()
                .set_transact_items(Some(tx_items))
                .send()
                .await
                .map_err(|e| Error::InternalServerError(e.to_string()))?;

            match next_bookmark {
                Some(b) => bookmark = Some(b),
                None => break,
            }
        }

        Ok(())
    }
}
