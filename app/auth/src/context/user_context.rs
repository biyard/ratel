use crate::*;

#[derive(Store, Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UserContext {
    pub user: Option<User>,
    pub refresh_token: Option<String>,
}

impl UserContext {
    pub fn is_logged_in(&self) -> bool {
        self.user.is_some()
    }

    pub fn user_id(&self) -> Option<String> {
        self.user.as_ref().map(|u| u.id())
    }

    pub fn did(&self) -> String {
        self.user
            .as_ref()
            .map(|u| u.did())
            .unwrap_or("-".to_string())
    }
}
