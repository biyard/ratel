use bdk::prelude::*;
use dto::{LandingData, Space};

use crate::{
    config,
    pages::controller::{AccountList, CommunityList, National, Profile},
    services::user_service::UserService,
};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,

    #[allow(dead_code)]
    pub landing_data: Resource<LandingData>,
    pub profile: Resource<Profile>,

    pub communities: Resource<Vec<CommunityList>>,
    pub accounts: Resource<Vec<AccountList>>,

    pub space: Resource<Space>,
}

impl Controller {
    pub fn new(lang: Language, id: i64) -> std::result::Result<Self, RenderError> {
        let landing_data = use_server_future(move || async move {
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

        let space = use_server_future(move || async move {
            match Space::get_client(config::get().main_api_endpoint)
                .find_by_id(id)
                .await
            {
                Ok(space) => space,
                Err(e) => {
                    tracing::debug!("query spaces failed with error: {:?}", e);
                    Default::default()
                }
            }
        })?;

        let ctrl = Self {
            lang,
            profile,
            accounts,
            communities,
            landing_data,

            space,
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
