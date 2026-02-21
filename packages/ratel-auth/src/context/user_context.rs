use crate::*;

#[derive(Store, Default, Clone, Debug)]
pub struct UserContext {
    pub user: Option<User>,
    pub refresh_token: Option<String>,
}

impl UserContext {
    pub fn is_logged_in(&self) -> bool {
        self.user.is_some()
    }
}
