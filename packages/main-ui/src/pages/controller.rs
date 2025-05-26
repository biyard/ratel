use bdk::prelude::*;
use dto::dioxus_popup::PopupService;
use dto::{
    ContentType, Feed, FeedQuery, FeedSummary, MyInfo, News, NewsQuery, NewsSummary, Promotion,
    User,
};
use dto::{Follower, LandingData};
use serde::{Deserialize, Serialize};

use crate::pages::components::EditProfilePopup;
use crate::{
    config, route::Route, services::user_service::UserService, utils::text::extract_title_from_html,
};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub nav: Navigator,
    pub popup: PopupService,
    pub is_write: Signal<bool>,

    pub landing_data: Resource<LandingData>,
    pub my_info: Signal<MyInfo>,
    pub hot_promotions: Resource<Promotion>,
    pub news: Resource<Vec<NewsSummary>>,

    pub feeds: Resource<Vec<FeedSummary>>,
    pub accounts: Resource<Vec<AccountList>>,

    pub size: Signal<usize>,
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
        let mut my_info = use_signal(|| MyInfo::default());
        let size = use_signal(|| 10);

        use_effect(move || {
            if !user_service.loggedin() {
                nav.replace(Route::LandingPage {});
            }

            my_info.set(user_service.my_info());
        });

        let landing_data = use_server_future(move || {
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

        let feeds = use_server_future(move || {
            let size = size();
            async move {
                match Feed::get_client(config::get().main_api_endpoint)
                    .query(FeedQuery {
                        size,
                        bookmark: None,
                    })
                    .await
                {
                    Ok(feed) => feed.items,
                    Err(e) => {
                        tracing::debug!("query spaces failed with error: {:?}", e);
                        Default::default()
                    }
                }
            }
        })?;

        let user = user_service.user_info();
        tracing::debug!("user info: {:?}", user);

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

        let accounts = use_server_future(move || async move { vec![] })?;

        let ctrl = Self {
            lang,
            nav: use_navigator(),
            is_write: use_signal(|| false),
            size,
            popup: use_context(),
            my_info,
            landing_data,
            hot_promotions,
            news,
            accounts,
            feeds,
        };

        use_context_provider(move || ctrl);

        Ok(ctrl)
    }

    pub fn edit_profile(&mut self) {
        let my_info = self.my_info();

        let profile = my_info.profile_url;
        let nickname = my_info.nickname;
        let description = my_info.html_contents;

        let mut ctrl = self.clone();

        self.popup
            .open(rsx! {
                EditProfilePopup {
                    lang: self.lang,
                    profile,
                    nickname,
                    description,
                    onedit: move |(profile, nickname, description): (String, String, String)| async move {
                        ctrl.edit(profile, nickname, description).await;
                        ctrl.popup.close();
                    },
                }
            })
            .with_title("Edit Profile");
    }

    pub async fn edit(&mut self, profile: String, nickname: String, description: String) {
        tracing::debug!(
            "profile: {:?} nickname: {:?} description: {:?}",
            profile,
            nickname,
            description
        );
    }

    pub fn add_size(&mut self) {
        self.size.set(self.size() + 5);
    }

    pub fn change_write(&mut self, is_write: bool) {
        self.is_write.set(is_write);
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

        match Feed::get_client(config::get().main_api_endpoint)
            .write_post(description, user_id, industry_id, Some(title), None)
            .await
        {
            Ok(_) => {
                btracing::info!("success to create space");
                self.landing_data.restart();
                self.feeds.restart();
                self.is_write.set(false);
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
