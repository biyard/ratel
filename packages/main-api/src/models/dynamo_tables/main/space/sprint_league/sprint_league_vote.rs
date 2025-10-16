use crate::{types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct SprintLeagueSpaceVote {
    pub pk: Partition,
    pub sk: EntityType,
    pub created_at: i64,

    pub user_pk: Partition, // Voter Pk

    pub player_sk: EntityType, // Voted Player Sk(uuid)

    pub referral_code: Option<String>,
}

impl SprintLeagueSpaceVote {
    pub fn new(
        space_pk: Partition,
        user_pk: Partition,
        player_sk: EntityType,
        referral_code: Option<String>,
    ) -> crate::Result<Self> {
        let (pk, sk) = Self::keys(&space_pk, &user_pk)?;

        let now = get_now_timestamp_millis();

        Ok(Self {
            pk,
            sk: EntityType::SprintLeagueSpaceVote(user_id),
            created_at: now,
            player_sk: player_sk.clone(),
            user_pk,
            referral_code,
        })
    }

    pub fn keys(
        space_pk: &Partition,
        user_pk: &Partition,
    ) -> crate::Result<(Partition, EntityType)> {
        let pk = match space_pk {
            Partition::Space(s) if !s.is_empty() => Partition::SprintLeagueVote(s.clone()),
            _ => {
                return Err(Error2::InvalidPartitionKey(
                    "space_pk must be Partition::Space with non-empty inner value".to_string(),
                ));
            }
        };

        let sk = match user_pk {
            Partition::User(id) => EntityType::SprintLeagueSpaceVote(id.clone()),
            _ => {
                return Err(Error2::InvalidPartitionKey(
                    "user_pk must be Partition::User".to_string(),
                ));
            }
        };

        Ok((pk, sk))
    }

    pub async fn find_one(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: &Partition,
        user_pk: &Partition,
    ) -> crate::Result<Option<Self>> {
        let (pk, sk) = Self::keys(space_pk, user_pk)?;

        SprintLeagueSpaceVote::get(&cli, pk, Some(sk)).await
    }
}
