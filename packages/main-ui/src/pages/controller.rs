use bdk::prelude::*;
use dto::{ContentType, News, NewsQuery, NewsSummary, Promotion, Space, User};
use dto::{Follower, LandingData};
use serde::{Deserialize, Serialize};

use crate::{
    config, route::Route, services::user_service::UserService, utils::text::extract_title_from_html,
};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub nav: Navigator,

    pub landing_data: Resource<LandingData>,
    pub hot_promotions: Resource<Promotion>,
    pub news: Resource<Vec<NewsSummary>>,
    pub profile: Resource<Profile>,

    pub communities: Resource<Vec<CommunityList>>,
    pub accounts: Resource<Vec<AccountList>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Translate, Default)]
pub enum National {
    #[translate(ko = "United State", en = "United State")]
    #[default]
    US,
    #[translate(ko = "Korea", en = "Korea")]
    Korea,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct AccountList {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub profile: String,
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct CommunityList {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub html_contents: String,
    pub title: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct Profile {
    pub profile: String,
    pub nickname: String,
    pub email: String,
    pub description: Option<String>,

    pub national: National,
    pub tier: i64,

    pub exp: i64,
    pub total_exp: i64,

    pub followers: i64,
    pub replies: i64,
    pub posts: i64,
    pub spaces: i64,
    pub votes: i64,
    pub surveys: i64,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user_service: UserService = use_context();
        let nav = use_navigator();

        let mut landing_data = use_server_future(move || {
            let _user_service: UserService = user_service.clone();
            async move {
                match LandingData::get_client(config::get().main_api_endpoint)
                    .find_one()
                    .await
                {
                    Ok(space) => space,
                    Err(e) => {
                        tracing::debug!("query spaces failed with error: {:?}", e);
                        Default::default()
                    }
                }
            }
        })?;

        use_effect(move || {
            if !user_service.loggedin() {
                nav.replace(Route::LandingPage {});
            }

            //FIXME: remove this line when cookie issue is resolved.
            landing_data.restart();
        });

        let user = user_service.user_info();
        tracing::debug!("user info: {:?}", user);

        // let my_feeds = use_server_future(move || async move {
        //     match Space::get_client(config::get().main_api_endpoint)
        //         .query_my_spaces()
        //         .await
        //     {
        //         Ok(promotion) => promotion.items,
        //         Err(e) => {
        //             tracing::debug!("query hot promotion failed with error: {:?}", e);
        //             Default::default()
        //         }
        //     }
        // });

        let communities = use_server_future(move || async move {
            vec![
                // CommunityList {
                //     id: 0,
                //     created_at: 1747726155,
                //     updated_at: 1747726155,
                //     html_contents: "<div>hello</div>".to_string(),
                //     title: Some("test1".to_string()),
                // },
                // CommunityList {
                //     id: 0,
                //     created_at: 1747726155,
                //     updated_at: 1747726155,
                //     html_contents: "<div>hello</div>".to_string(),
                //     title: Some("test12".to_string()),
                // },
                // CommunityList {
                //     id: 0,
                //     created_at: 1747726155,
                //     updated_at: 1747726155,
                //     html_contents: "<div>hello</div>".to_string(),
                //     title: Some("test123".to_string()),
                // },
            ]
        })?;

        let hot_promotions = use_server_future(move || async move {
            match Promotion::get_client(config::get().main_api_endpoint)
                .hot_promotion()
                .await
            {
                Ok(promotion) => promotion,
                Err(e) => {
                    tracing::debug!("query hot promotion failed with error: {:?}", e);
                    Default::default()
                }
            }
        })?;

        let news = use_server_future(move || async move {
            match News::get_client(config::get().main_api_endpoint)
                .query(NewsQuery {
                    size: 3,
                    bookmark: None,
                })
                .await
            {
                Ok(promotion) => promotion.items,
                Err(e) => {
                    tracing::debug!("query news failed with error: {:?}", e);
                    Default::default()
                }
            }
        })?;

        let profile = use_server_future(move || async move {
            Profile {
                profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                nickname: "Jongseok Park".to_string(),
                email: "victor@biyard.co".to_string(),
                description: Some("Office of Rep.".to_string()),

                national: National::US,
                tier: 1,

                exp: 4,
                total_exp: 6,

                followers: 12501,
                replies: 503101,
                posts: 420201,
                spaces: 3153,
                votes: 125,
                surveys: 3153
            }
        })?;

        let accounts = use_server_future(move || async move {
            vec! [
                // AccountList {
                //     id: 0,
                //     created_at: 1747726155,
                //     updated_at: 1747726155,
                //     profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                //     email: "victor@biyard.co".to_string(),
                // },
                // AccountList {
                //     id: 1,
                //     created_at: 1747726155,
                //     updated_at: 1747726155,
                //     profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                //     email: "victor1@biyard.co".to_string(),
                // }
            ]
        })?;

        let ctrl = Self {
            lang,
            nav: use_navigator(),
            landing_data,
            hot_promotions,
            news,
            profile,
            communities,
            accounts,
        };

        use_context_provider(move || ctrl);

        Ok(ctrl)
    }

    pub async fn create_feed(&mut self, content_type: ContentType, description: String) {
        //FIXME: fix to real industry_id
        let industry_id = 1;
        let title = extract_title_from_html(&description);
        tracing::debug!(
            "create feed info: {:?} {:?} {:?} {:?}",
            content_type,
            industry_id,
            title,
            description
        );

        let user_id = match User::get_client(config::get().main_api_endpoint)
            .user_info()
            .await
        {
            Ok(v) => v.id,
            Err(e) => {
                btracing::error!("failed to get user id with error: {:?}", e);
                0
            }
        };

        if user_id == 0 {
            return;
        }

        match Space::get_client(config::get().main_api_endpoint)
            .create_space(description, dto::SpaceType::Post, Some(title), content_type)
            .await
        {
            Ok(_) => {
                btracing::info!("success to create space");
                self.landing_data.restart();
            }
            Err(e) => {
                btracing::error!("failed to create space with error: {:?}", e);
            }
        };
    }

    pub async fn follow(&mut self, id: i64) {
        tracing::debug!("follow user id: {:?}", id);
        match Follower::get_client(config::get().main_api_endpoint)
            .follow(id)
            .await
        {
            Ok(_) => {
                btracing::info!("success to follow user");
                self.landing_data.restart();
            }
            Err(e) => {
                btracing::error!("failed to follow user with error: {:?}", e);
            }
        };
    }

    pub async fn add_account(&mut self) {
        tracing::debug!("add account");
    }

    pub async fn signout(&mut self) {
        tracing::debug!("signout");
        let mut user: UserService = use_context();
        user.logout().await;
    }

    pub fn move_to_threads(&self, id: i64) {
        self.nav.push(Route::ThreadPage { id });
    }
}
