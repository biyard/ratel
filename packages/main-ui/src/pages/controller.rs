use bdk::prelude::*;
use dto::{Feed, FeedType, News, NewsQuery, NewsSummary, Promotion, User};
use serde::{Deserialize, Serialize};

use crate::{config, route::Route, services::user_service::UserService, utils::text::extract_title_from_html};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    #[allow(dead_code)]
    pub my_feeds: Resource<Vec<FeedList>>,
    #[allow(dead_code)]
    pub following_feeds: Resource<Vec<FeedList>>,
    pub hot_promotions: Resource<Promotion>,
    pub news: Resource<Vec<NewsSummary>>,
    pub followers: Resource<Vec<Follower>>,
    pub profile: Resource<Profile>,

    pub spaces: Resource<Vec<SpaceList>>,
    pub communities: Resource<Vec<CommunityList>>,
    pub accounts: Resource<Vec<AccountList>>
}

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq, Translate, Default)]
pub enum ContentType {
    #[translate(ko = "Crypto", en = "Crypto")]
    #[default]
    Crypto,
    #[translate(ko = "Social", en = "Social")]
    Social,
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
pub struct SpaceList {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub html_contents: String,
    pub space_type: FeedType,

    pub user_id: i64,
    pub parent_id: Option<i64>,
    pub title: Option<String>,
    pub part_id: Option<i64>,
    pub quote_feed_id: Option<i64>,

    //additional info
    pub profile: String,
    pub nickname: String,
    pub saved: bool,

    pub content_type: ContentType,

    pub number_of_accepters: i64,
    pub number_of_rejecters: i64,
    pub number_of_comments: i64,
    pub number_of_rewards: i64,
    pub number_of_shared: i64,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct Profile {
    pub profile: String,
    pub nickname: String,
    pub email: String,
    pub description: Option<String>,

    pub national: National,
    pub tier: i64,

    pub exp: i64, //나의 현재 경험치
    pub total_exp: i64, //레벨업 하기 위한 필요한 총 경험치 

    pub followers: i64,
    pub replies: i64,
    pub posts: i64,
    pub spaces: i64,
    pub votes: i64,
    pub surveys: i64,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Follower {
    pub id: i64,

    pub image: String,
    pub title: String,
    pub description: String,

    pub followed: bool,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct FeedList {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub html_contents: String,
    pub feed_type: FeedType,

    pub user_id: i64,
    pub parent_id: Option<i64>,
    pub title: Option<String>,
    pub part_id: Option<i64>,
    pub quote_feed_id: Option<i64>,

    //additional info
    pub profile: String,
    pub nickname: String,
    pub saved: bool,

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
                    nickname: "victor".to_string(),
                    saved: false,
                };100
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
                    nickname: "victor".to_string(),
                    saved: false,
                };
                100
            ]
        })?;

        let communities = use_server_future(move || async move {
            vec![
                CommunityList {
                    id: 0,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello</div>".to_string(),
                    title: Some("test1".to_string())
                },
                CommunityList {
                    id: 0,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello</div>".to_string(),
                    title: Some("test12".to_string())
                },
                CommunityList {
                    id: 0,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello</div>".to_string(),
                    title: Some("test123".to_string())
                }
            ]
        })?;

        let spaces = use_server_future(move || async move {
            vec![
                 SpaceList {
                    id: 0,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello</div>".to_string(),
                    space_type: dto::FeedType::Post,
                    user_id: 1,
                    parent_id: None,
                    title: Some("test3".to_string()),
                    part_id: None,
                    quote_feed_id: None,
                    content_type: ContentType::Crypto,
                    number_of_accepters: 705,
                    number_of_rejecters: 212,
                    number_of_comments: 30,
                    number_of_rewards: 30,
                    number_of_shared: 40,

                    profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                    nickname: "victor".to_string(),
                    saved: false,
                },
                SpaceList {
                    id: 0,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello</div>".to_string(),
                    space_type: dto::FeedType::Post,
                    user_id: 1,
                    parent_id: None,
                    title: Some("test4".to_string()),
                    part_id: None,
                    quote_feed_id: None,
                    content_type: ContentType::Crypto,
                    number_of_accepters: 705,
                    number_of_rejecters: 212,
                    number_of_comments: 30,
                    number_of_rewards: 30,
                    number_of_shared: 40,

                    profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                    nickname: "victor".to_string(),
                    saved: false,
                },
                SpaceList {
                    id: 0,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello</div>".to_string(),
                    space_type: dto::FeedType::Post,
                    user_id: 1,
                    parent_id: None,
                    title: Some("test5".to_string()),
                    part_id: None,
                    quote_feed_id: None,
                    content_type: ContentType::Crypto,
                    number_of_accepters: 705,
                    number_of_rejecters: 212,
                    number_of_comments: 30,
                    number_of_rewards: 30,
                    number_of_shared: 40,

                    profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                    nickname: "victor".to_string(),
                    saved: false,
                },
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

        let followers = use_server_future(move || async move {
            vec![
                Follower {
                    id: 1,
                    image: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                    title: "Donald Trump".to_string(),
                    description: "President of the US".to_string(),
                    followed: false,
                }, 
                Follower {
                    id: 2,
                    image: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                    title: "Elon Musk".to_string(),
                    description: "CEO of Tesla and SpaceX".to_string(),
                    followed: false,
                }, 
                Follower {
                    id: 3,
                    image: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                    title: "Jongseok Park".to_string(),
                    description: "National Assembly of blah blah".to_string(),
                    followed: false,
                }
            ]
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
                AccountList {
                    id: 0,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                    email: "victor@biyard.co".to_string(),
                },
                AccountList {
                    id: 1,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                    email: "victor1@biyard.co".to_string(),
                }
            ]
        })?;

        let ctrl = Self {
            lang,
            my_feeds,
            following_feeds,
            hot_promotions,
            news,
            followers,
            profile,
            spaces,
            communities,
            accounts
        };

        Ok(ctrl)
    }

    pub async fn create_feed(&mut self, content_type: ContentType, description: String) {
        //FIXME: fix to real industry_id
        let industry_id = 1;
        let title = extract_title_from_html(&description);
        tracing::debug!("create feed info: {:?} {:?} {:?} {:?}", content_type, industry_id, title, description);

        let user_id = match User::get_client(config::get().main_api_endpoint).user_info().await {
            Ok(v) => v.id,
            Err(e) => {
                btracing::error!("failed to get user id with error: {:?}", e);
                0
            },
        };

        if user_id == 0 {
            return;
        }
        
        match Feed::get_client(config::get().main_api_endpoint).write_post(description, user_id, 1, Some(title), None).await {
            Ok(_) => {
                btracing::info!("success to create feed");
                self.my_feeds.restart();
                self.following_feeds.restart();
            },
            Err(e) => {
                btracing::error!("failed to create feed with error: {:?}", e);
            },
        };
    }

    pub async fn follow(&mut self, id: i64) {
        tracing::debug!("follow user id: {:?}", id);
    }

    pub async fn add_account(&mut self) {
        tracing::debug!("add account");
    }

    pub async fn signout(&mut self) {
        tracing::debug!("signout");
        let mut user: UserService = use_context();
        user.logout().await;
    }
}
