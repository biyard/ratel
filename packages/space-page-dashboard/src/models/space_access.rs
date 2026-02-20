use common::{DynamoEntity, EntityType, Partition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, DynamoEntity)]
pub struct SpaceCommonRef {
    pub pk: Partition,
    pub sk: EntityType,
    pub user_pk: Partition,
}

pub async fn is_team_admin(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
    user_pk: &Partition,
) -> common::Result<bool> {
    use ratel_auth::models::UserTeamGroup;

    let (groups, _) = UserTeamGroup::find_by_team_pk(
        cli,
        team_pk.clone(),
        UserTeamGroup::opt_one().sk(user_pk.to_string()),
    )
    .await?;

    Ok(groups
        .first()
        .map(|g| (g.team_group_permissions & (1 << 20)) != 0)
        .unwrap_or(false))
}
