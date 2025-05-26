use bdk::prelude::*;
use dto::{Feed, MyInfo};

use crate::{config, pages::controller::AccountList, services::user_service::UserService};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub my_info: Signal<MyInfo>,
    pub accounts: Resource<Vec<AccountList>>,
    pub feed: Resource<Feed>,
}

impl Controller {
    pub fn new(lang: Language, id: i64) -> std::result::Result<Self, RenderError> {
        let user_service: UserService = use_context();
        let accounts = use_server_future(move || async move { vec![] })?;

        let feed = use_server_future(move || async move {
            match Feed::get_client(config::get().main_api_endpoint)
                .get(id)
                .await
            {
                Ok(feed) => feed,
                Err(e) => {
                    tracing::debug!("query feed failed with error: {:?}", e);
                    Default::default()
                }
            }
        })?;

        let my_info = user_service.my_info();

        let mut ctrl = Self {
            lang,
            accounts,
            my_info: use_signal(|| my_info),
            feed,
        };

        use_effect(move || {
            ctrl.my_info.set(user_service.my_info());
        });

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
