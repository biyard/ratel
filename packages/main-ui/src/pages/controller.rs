use crate::pages::components::{CreateSpacePopup, CreateTeamPopup};
use bdk::prelude::*;
use dto::dioxus_popup::PopupService;
use dto::{
    ContentType, Feed, FeedQuery, FeedSummary, File, MyInfo, News, NewsQuery, NewsSummary,
    Promotion, Space, SpaceType, TotalInfoQuery, TotalInfoSummary, User,
};
use dto::{Follower, LandingData};
use dto::{Team, TotalInfo};
use regex::Regex;
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
    pub user_service: UserService,
    pub is_write: Signal<bool>,

    pub total_users: Signal<Resource<Vec<TotalInfoSummary>>>,
    pub landing_data: Resource<LandingData>,
    pub my_info: Signal<MyInfo>,
    pub hot_promotions: Resource<Promotion>,
    pub news: Resource<Vec<NewsSummary>>,

    pub feeds: Resource<Vec<FeedSummary>>,

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

        let total_users = use_server_future(move || async move {
            match TotalInfo::get_client(config::get().main_api_endpoint)
                .query(TotalInfoQuery {
                    size: 100,
                    bookmark: None,
                })
                .await
            {
                Ok(info) => info.items,
                Err(e) => {
                    tracing::debug!("query feed failed with error: {:?}", e);
                    Default::default()
                }
            }
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

        let ctrl = Self {
            lang,
            nav: use_navigator(),
            is_write: use_signal(|| false),
            size,
            popup: use_context(),
            user_service,
            my_info,
            landing_data,
            total_users: use_signal(|| total_users),
            hot_promotions,
            news,
            feeds,
        };

        use_context_provider(move || ctrl);

        Ok(ctrl)
    }

    pub fn create_space(&mut self, feed_id: i64) {
        let users = match self.total_users()() {
            Some(v) => v,
            None => vec![],
        };
        let mut ctrl = self.clone();

        self.popup
            .open(rsx! {
                CreateSpacePopup {
                    lang: self.lang,
                    users,
                    onsend: move |ids: Vec<i64>| async move {
                        tracing::debug!("selected user ids: {:?}", ids);
                        ctrl.create_space_request(feed_id, ids).await;
                        ctrl.popup.close();
                    },
                }
            })
            .with_title("Invite to Committee");
    }

    pub async fn create_space_request(&mut self, feed_id: i64, user_ids: Vec<i64>) {
        match Space::get_client(config::get().main_api_endpoint)
            .create_space(SpaceType::Post, feed_id, user_ids)
            .await
        {
            Ok(_) => {
                tracing::info!("success to create space");
                self.feeds.restart();
            }
            Err(e) => {
                btracing::error!(
                    "failed to create space with error: {}",
                    e.translate(&self.lang)
                );
            }
        };
    }

    pub fn create_team(&mut self) {
        tracing::debug!("create_team");
        let mut ctrl = self.clone();

        self.popup
            .open(rsx! {
                CreateTeamPopup {
                    lang: self.lang,
                    oncreate: move |(profile, username): (String, String)| async move {
                        if !ctrl.create_team_validation_check(profile.clone(), username.clone()) {
                            return;
                        }
                        ctrl.create_team_request(profile, username).await;
                        ctrl.popup.close();
                    },
                }
            })
            .with_title("Edit Profile");
    }

    pub async fn create_team_request(&mut self, profile: String, username: String) {
        tracing::debug!("profile: {:?} username: {:?}", profile, username);

        match Team::get_client(config::get().main_api_endpoint)
            .create(profile, username)
            .await
        {
            Ok(_) => {
                tracing::debug!("success to create team");
                self.user_service.update_my_info().await;
            }
            Err(e) => {
                btracing::error!("failed to create team with error: {:?}", e);
            }
        };
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
                        if !ctrl
                            .edit_profile_validation_check(
                                profile.clone(),
                                nickname.clone(),
                                description.clone(),
                            )
                        {
                            return;
                        }
                        ctrl.edit_profile_request(profile, nickname, description).await;
                        ctrl.popup.close();
                    },
                }
            })
            .with_title("Edit Profile");
    }

    pub async fn edit_profile_request(
        &mut self,
        profile: String,
        nickname: String,
        description: String,
    ) {
        let info = self.my_info();

        tracing::debug!(
            "profile: {:?} nickname: {:?} description: {:?}",
            profile,
            nickname,
            description
        );

        match User::get_client(config::get().main_api_endpoint)
            .edit_profile(info.id, nickname, profile, description)
            .await
        {
            Ok(_) => {
                tracing::debug!("success to edit profile");
                self.user_service.update_my_info().await;
            }
            Err(e) => {
                btracing::error!("failed to edit profile with error: {:?}", e);
            }
        };
    }

    pub fn add_size(&mut self) {
        self.size.set(self.size() + 5);
    }

    pub fn change_write(&mut self, is_write: bool) {
        self.is_write.set(is_write);
    }

    pub async fn create_feed(
        &mut self,
        files: Vec<File>,
        content_type: ContentType,
        description: String,
    ) {
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
            .write_post(description, user_id, industry_id, Some(title), None, files)
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

    pub async fn signout(&mut self) {
        tracing::debug!("signout");
        let mut user: UserService = use_context();
        user.logout().await;
    }

    pub fn move_to_threads(&self, id: i64) {
        self.nav.push(Route::ThreadPage { id });
    }

    pub fn edit_profile_validation_check(
        &self,
        profile: String,
        nickname: String,
        description: String,
    ) -> bool {
        if profile.is_empty() {
            btracing::e!(self.lang, ValidationError::ProfileRequired);
            return false;
        }
        if nickname.is_empty() {
            btracing::e!(self.lang, ValidationError::NicknameRequired);
            return false;
        }
        if description.is_empty() {
            btracing::e!(self.lang, ValidationError::DescriptionRequired);
            return false;
        }
        true
    }

    pub fn create_team_validation_check(&self, profile: String, username: String) -> bool {
        let valid_pattern = Regex::new("^[a-z0-9_-]*$").unwrap();

        if profile.is_empty() {
            btracing::e!(self.lang, ValidationError::TeamProfileRequired);
            return false;
        }
        if username.is_empty() {
            btracing::e!(self.lang, ValidationError::UsernameRequired);
            return false;
        }
        if !valid_pattern.is_match(&username) {
            btracing::e!(self.lang, ValidationError::UsernameFormatFailed);
            return false;
        }
        true
    }
}

#[derive(Debug, PartialEq, Eq, Translate)]
pub enum ValidationError {
    #[translate(en = "Please select the team profile.")]
    TeamProfileRequired,
    #[translate(en = "Please enter the username.")]
    UsernameRequired,
    #[translate(en = "Please enter a username that matches the format.")]
    UsernameFormatFailed,

    #[translate(en = "Please select the profile.")]
    ProfileRequired,
    #[translate(en = "Please enter the nickname.")]
    NicknameRequired,
    #[translate(en = "Please enter the description.")]
    DescriptionRequired,
}
