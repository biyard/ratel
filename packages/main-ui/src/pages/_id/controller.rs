use bdk::prelude::*;

use crate::pages::controller::{
    AccountList, CommunityList, ContentType, FeedList, National, Profile, SpaceList,
};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,

    #[allow(dead_code)]
    pub my_feeds: Resource<Vec<FeedList>>,
    pub profile: Resource<Profile>,
    pub accounts: Resource<Vec<AccountList>>,
    pub spaces: Resource<Vec<SpaceList>>,
    pub communities: Resource<Vec<CommunityList>>,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
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

        let communities = use_server_future(move || async move {
            vec![
                CommunityList {
                    id: 0,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello</div>".to_string(),
                    title: Some("test1".to_string()),
                },
                CommunityList {
                    id: 0,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello</div>".to_string(),
                    title: Some("test12".to_string()),
                },
                CommunityList {
                    id: 0,
                    created_at: 1747726155,
                    updated_at: 1747726155,
                    html_contents: "<div>hello</div>".to_string(),
                    title: Some("test123".to_string()),
                },
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

        let ctrl = Self {
            lang,
            my_feeds,
            profile,
            accounts,
            spaces,
            communities,
        };

        Ok(ctrl)
    }

    pub async fn add_account(&mut self) {
        tracing::debug!("add account");
    }

    pub fn signout(&mut self) {
        tracing::debug!("signout");
    }
}
