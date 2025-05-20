use bdk::prelude::*;
use dto::FeedType;
use serde::{Deserialize, Serialize};

use crate::{route::Route, services::user_service::UserService};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    #[allow(dead_code)]
    pub my_feeds: Resource<Vec<FeedList>>,
    #[allow(dead_code)]
    pub following_feeds: Resource<Vec<FeedList>>,

    pub profile: Signal<String>,
    pub nickname: Signal<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Translate, Default)]
pub enum ContentType {
    #[translate(ko = "Crypto", en = "Crypto")]
    #[default]
    Crypto,
    #[translate(ko = "Social", en = "Social")]
    Social,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct FeedList {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub html_contents: String,
    pub feed_type: FeedType,

    pub user_id: i64,
    pub industry_id: i64,
    pub parent_id: Option<i64>,
    pub title: Option<String>,
    pub part_id: Option<i64>,
    pub quote_feed_id: Option<i64>,

    //additional info
    pub profile: String,
    pub nickname: String,

    pub content_type: ContentType,

    pub number_of_likes: i64,
    pub number_of_comments: i64,
    pub number_of_rewards: i64,
    pub number_of_shared: i64,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user_service: UserService = use_context();
        let nav = use_navigator();

        let info = user_service.user_info();

        use_effect(move || {
            if !crate::config::get().experiment || !user_service.loggedin() {
                nav.replace(Route::LandingPage {});
            }
        });

        let user = user_service.user_info();
        tracing::debug!("user info: {:?}", user);

        let my_feeds = use_server_future(move || async move {
            vec![
                FeedList {
                    id: 0,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello</div>".to_string(),
                    feed_type: dto::FeedType::Post,
                    user_id: 1,
                    industry_id: 1,
                    parent_id: None,
                    title: Some("test".to_string()),
                    part_id: None,
                    quote_feed_id: None,
                    content_type: ContentType::Crypto,
                    number_of_likes: 20,
                    number_of_comments: 30,
                    number_of_rewards: 30,
                    number_of_shared: 40,

                    profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                    nickname: "victor".to_string()
                },
                FeedList {
                    id: 1,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello2</div>".to_string(),
                    feed_type: dto::FeedType::Post,
                    user_id: 1,
                    industry_id: 1,
                    parent_id: None,
                    title: Some("test".to_string()),
                    part_id: None,
                    quote_feed_id: None,
                    content_type: ContentType::Crypto,
                    number_of_likes: 30,
                    number_of_comments: 40,
                    number_of_rewards: 50,
                    number_of_shared: 60,

                     profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                    nickname: "victor".to_string()
                },
                FeedList {
                    id: 2,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello3</div>".to_string(),
                    feed_type: dto::FeedType::Post,
                    user_id: 1,
                    industry_id: 1,
                    parent_id: None,
                    title: Some("test".to_string()),
                    part_id: None,
                    quote_feed_id: None,
                    content_type: ContentType::Crypto,
                    number_of_likes: 40,
                    number_of_comments: 50,
                    number_of_rewards: 50,
                    number_of_shared: 60,

                    profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                    nickname: "victor".to_string()
                },
            ]
        })?;

        let following_feeds = use_server_future(move || async move {
            vec![
                FeedList {
                    id: 0,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello</div>".to_string(),
                    feed_type: dto::FeedType::Post,
                    user_id: 1,
                    industry_id: 1,
                    parent_id: None,
                    title: Some("test3".to_string()),
                    part_id: None,
                    quote_feed_id: None,
                    content_type: ContentType::Crypto,
                    number_of_likes: 20,
                    number_of_comments: 30,
                    number_of_rewards: 30,
                    number_of_shared: 40,

                    profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                    nickname: "victor".to_string()
                },
                FeedList {
                    id: 1,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello2</div>".to_string(),
                    feed_type: dto::FeedType::Post,
                    user_id: 1,
                    industry_id: 1,
                    parent_id: None,
                    title: Some("test4".to_string()),
                    part_id: None,
                    quote_feed_id: None,
                    content_type: ContentType::Crypto,
                    number_of_likes: 30,
                    number_of_comments: 40,
                    number_of_rewards: 50,
                    number_of_shared: 60,

                    profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                    nickname: "victor".to_string()
                },
                FeedList {
                    id: 2,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello3</div>".to_string(),
                    feed_type: dto::FeedType::Post,
                    user_id: 1,
                    industry_id: 1,
                    parent_id: None,
                    title: Some("test5".to_string()),
                    part_id: None,
                    quote_feed_id: None,
                    content_type: ContentType::Crypto,
                    number_of_likes: 40,
                    number_of_comments: 50,
                    number_of_rewards: 50,
                    number_of_shared: 60,

                    profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                    nickname: "victor".to_string()
                },
            ]
        })?;

        let mut ctrl = Self {
            lang,
            my_feeds,
            following_feeds,

            nickname: use_signal(|| info.nickname.unwrap_or_default()),
            profile: use_signal(|| info.profile_url.unwrap_or_default()),
        };

        use_effect(move || {
            let info = user_service.user_info();

            ctrl.nickname.set(info.nickname.unwrap_or_default());
            ctrl.profile.set(info.profile_url.unwrap_or_default());
        });

        Ok(ctrl)
    }

    pub async fn create_feed(&mut self, content_type: ContentType, description: String) {
        tracing::debug!("create feed info: {:?} {:?}", content_type, description);
    }
}
