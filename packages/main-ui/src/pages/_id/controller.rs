use bdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    dto::{
        comment::Comment,
        content_type::ContentType,
        file::{File, FileExtension},
    },
    pages::controller::{AccountList, CommunityList, FeedList, National, Profile, SpaceList},
    services::user_service::UserService,
};

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct Thread {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub number_of_comments: i64,
    pub number_of_rewards: i64,
    pub number_of_shared: i64,

    pub content_type: ContentType,

    pub profile: String,
    pub proposer: String,
    pub title: String,
    pub description: String, //html format

    pub files: Vec<File>,
    pub comments: Vec<Comment>,
}

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

    pub threads: Resource<Thread>,
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

        let threads = use_server_future(move || async move {
            Thread {
                id: 1,
                created_at: 1747726155,
                updated_at: 1747726155,
                number_of_comments: 201,
                number_of_rewards: 221,
                number_of_shared: 403,
                content_type: ContentType::Crypto,
                profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(),
                proposer: "victor".to_string(),
                title: "Crypto/Temporary Increase of Staking Rewards to 8% for 90 Days".to_string(),
                description: "<h1>Summary</h1> <div>This proposal suggests a temporary increase of the staking rewards from 6% to 8% APR for a trial period of 90 days.  The goal is to boost staking participation, enhance long-term commitment from token holders, and assess how a slightly more generous reward structure impacts token velocity, treasury health, and user behavior.</div>".to_string(),
                files: vec![
                    File { name: "의안 원문".to_string(), size: "5.3MB".to_string(), ext: FileExtension::PDF, url: Some("https://metadata.voice-korea.dev.biyard.co/metadata/fe54f3d0-9b67-4360-be96-bad1c6365fcc.pdf".to_string()) },
                    File { name: "의안 원문 2".to_string(), size: "5.3MB".to_string(), ext: FileExtension::PDF, url: Some("https://metadata.voice-korea.dev.biyard.co/metadata/fe54f3d0-9b67-4360-be96-bad1c6365fcc.pdf".to_string()) }
                ],
                comments: vec![
                    Comment { id: 1, created_at: 1747726155, updated_at: 1747726155, profile_url: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(), profile_name: "victor".to_string(), comment: "<div>Thank you for this proposal. It will undoubtedly be an extensive effort over time. I wonder if it would be beneficial to define some indicative categories for these goals. This could help prevent certain areas from being overlooked while also enabling proposers to efficiently identify similar proposals and foster collaboration.
What I mean is that within this proposal, we could predefine a few categories such as governance, DeFi, and grants. This would allow proposers to focus on reviewing proposals within each area, reducing duplication. Of course, this would not restrict proposers from suggesting goals in new categories—they could simply be classified under</div>".to_string(), replies: vec![
                        Comment { id: 4, created_at: 1747726155, updated_at: 1747726155, profile_url: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(), profile_name: "victor".to_string(), comment: "<div>Thank you for this proposal. It will undoubtedly be an extensive effort over time. I wonder if it would be beneficial to define some indicative categories for these goals. This could help prevent certain areas from being overlooked while also enabling proposers to efficiently identify similar proposals and foster collaboration.
                        What I mean is that within this proposal, we could predefine a few categories such as governance, DeFi, and grants. This would allow proposers to focus on reviewing proposals within each area, reducing duplication. Of course, this would not restrict proposers from suggesting goals in new categories—they could simply be classified under</div>".to_string(), replies: vec![], number_of_comments: 201, number_of_likes: 481 }
                    ], number_of_comments: 201, number_of_likes: 481 },
                    Comment { id: 2, created_at: 1747726155, updated_at: 1747726155, profile_url: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(), profile_name: "victor".to_string(), comment: "<div>Thank you for this proposal. It will undoubtedly be an extensive effort over time. I wonder if it would be beneficial to define some indicative categories for these goals. This could help prevent certain areas from being overlooked while also enabling proposers to efficiently identify similar proposals and foster collaboration.
                    What I mean is that within this proposal, we could predefine a few categories such as governance, DeFi, and grants. This would allow proposers to focus on reviewing proposals within each area, reducing duplication. Of course, this would not restrict proposers from suggesting goals in new categories—they could simply be classified under</div>".to_string(), replies: vec![], number_of_comments: 201, number_of_likes: 481 },
                    Comment { id: 3, created_at: 1747726155, updated_at: 1747726155, profile_url: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c".to_string(), profile_name: "victor".to_string(), comment: "<div>Thank you for this proposal. It will undoubtedly be an extensive effort over time. I wonder if it would be beneficial to define some indicative categories for these goals. This could help prevent certain areas from being overlooked while also enabling proposers to efficiently identify similar proposals and foster collaboration.
                    What I mean is that within this proposal, we could predefine a few categories such as governance, DeFi, and grants. This would allow proposers to focus on reviewing proposals within each area, reducing duplication. Of course, this would not restrict proposers from suggesting goals in new categories—they could simply be classified under</div>".to_string(), replies: vec![], number_of_comments: 201, number_of_likes: 481 }
                ],
            }
        })?;

        let ctrl = Self {
            lang,
            my_feeds,
            profile,
            accounts,
            spaces,
            communities,

            threads,
        };

        Ok(ctrl)
    }

    pub async fn add_account(&mut self) {
        tracing::debug!("add account");
    }

    pub async fn signout(&mut self) {
        tracing::debug!("signout");
        let mut user: UserService = use_context();
        user.logout().await;
    }

    #[allow(unused)]
    pub async fn download_file(&self, name: String, url: Option<String>) {
        if url.is_none() {
            return;
        }

        let url = url.unwrap_or_default();

        #[cfg(feature = "web")]
        {
            use wasm_bindgen::JsCast;

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", &name).unwrap();

            document.body().unwrap().append_child(&a).unwrap();
            let a: web_sys::HtmlElement = a.unchecked_into();
            a.click();
            a.remove();
        }
    }
}
