use crate::{
    models::{team::Team, user::User},
    types::Partition,
};

pub struct Author {
    pub pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
}

impl From<User> for Author {
    fn from(
        User {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        Self {
            pk,
            display_name,
            profile_url,
            username,
        }
    }
}
impl From<Team> for Author {
    fn from(
        Team {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: Team,
    ) -> Self {
        Self {
            pk,
            display_name,
            profile_url,
            username,
        }
    }
}
