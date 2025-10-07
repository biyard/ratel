use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct TeamMember {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    pub display_name: String,
    pub profile_url: String,
    pub username: String,

    #[dynamo(prefix = "USER", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
}

impl TeamMember {
    pub fn new(
        team_pk: Partition,
        User {
            pk: user_pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        let team_id = match &team_pk {
            Partition::Team(team_id) => team_id.clone(),
            _ => panic!("TeamMember::new requires a team Partition"),
        };
        let user_id = match &user_pk {
            Partition::User(user_id) => user_id.clone(),
            _ => panic!("TeamMember::new requires a user Partition"),
        };
        Self {
            pk: team_pk,
            sk: EntityType::TeamMember(team_id, user_id),
            display_name,
            profile_url,
            username,
            user_pk: user_pk,
        }
    }
}
