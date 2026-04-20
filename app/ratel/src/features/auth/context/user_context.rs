use crate::features::auth::*;

use crate::features::membership::models::UserMembershipResponse;

#[derive(Store, Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserContext {
    pub user: Option<User>,
    pub refresh_token: Option<String>,
    pub membership: Option<UserMembershipResponse>,
}

impl UserContext {
    pub fn is_logged_in(&self) -> bool {
        self.user.is_some()
    }

    pub fn user_id(&self) -> Option<String> {
        self.user.as_ref().map(|u| u.id())
    }

    pub fn user_pk(&self) -> Option<String> {
        self.user.as_ref().map(|u| u.pk.to_string())
    }

    pub fn did(&self) -> String {
        self.user
            .as_ref()
            .map(|u| u.did())
            .unwrap_or("-".to_string())
    }
}
