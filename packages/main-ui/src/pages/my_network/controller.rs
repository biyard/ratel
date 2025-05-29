use bdk::prelude::*;
use super::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self { lang };

        Ok(ctrl)
    }
}



#[derive(Clone)]
pub struct FollowingController {
    pub following_users: Vec<User>,
    pub suggested_accounts: Vec<User>,
}

impl FollowingController {
    pub fn new() -> Self {
        Self {
            following_users: vec![],
            suggested_accounts: vec![],
        }
    }

    pub fn get_following_users(&self) -> &[User] {
        &self.following_users
    }

    pub fn get_suggested_accounts(&self) -> &[User] {
        &self.suggested_accounts
    }

    pub async fn load_following_data(&mut self) {
        let (following, suggested) = fetch_dummy_data().await;
        self.following_users = following;
        self.suggested_accounts = suggested;
    }
}

async fn fetch_dummy_data() -> (Vec<User>, Vec<User>) {
    use std::time::Duration;
    use tokio::time::sleep;

    sleep(Duration::from_millis(300)).await;

    let following = vec![
        User::new("Jane Doe", "Candidate for State Senate, NY, Reform Alliance"),
        User::new("Rami Yusuf", "Candidate for State Senate, NY, Reform Alliance"),
    ];

    let suggested = vec![
        User::new("John Smith", "Candidate for State Senate, NY, Reform Alliance"),
        User::new("Thelma Bae", "Candidate for State Senate, NY, Reform Alliance"),
    ];

    (following, suggested)
}