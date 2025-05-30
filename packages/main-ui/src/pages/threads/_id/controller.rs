use bdk::prelude::*;
use dto::{
    Feed, MyInfo, Space, SpaceForm, SpaceType, TotalInfo, TotalInfoQuery, TotalInfoSummary,
    dioxus_popup::PopupService,
};

use crate::{
    config,
    pages::components::{CreateSpacePopup, SelectSpaceFormPopup},
    route::Route,
    services::user_service::UserService,
};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub nav: Navigator,
    pub total_users: Signal<Resource<Vec<TotalInfoSummary>>>,
    pub my_info: Signal<MyInfo>,
    pub feed: Resource<Feed>,
    #[allow(dead_code)]
    pub id: i64,
    #[allow(dead_code)]
    pub popup: PopupService,
}

impl Controller {
    pub fn new(lang: Language, id: i64) -> std::result::Result<Self, RenderError> {
        let user_service: UserService = use_context();

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

        let total_users = use_server_future(move || async move {
            match TotalInfo::get_client(config::get().main_api_endpoint)
                .query(TotalInfoQuery {
                    size: 100,
                    bookmark: None,
                })
                .await
            {
                Ok(feed) => feed.items,
                Err(e) => {
                    tracing::debug!("query feed failed with error: {:?}", e);
                    Default::default()
                }
            }
        })?;

        let my_info = user_service.my_info();

        let mut ctrl = Self {
            lang,
            nav: use_navigator(),
            my_info: use_signal(|| my_info),
            total_users: use_signal(|| total_users),
            feed,
            popup: use_context(),
            id,
        };

        use_effect(move || {
            ctrl.my_info.set(user_service.my_info());
        });

        Ok(ctrl)
    }

    pub async fn create_team(&mut self) {
        tracing::debug!("create team");
    }

    pub async fn signout(&mut self) {
        tracing::debug!("signout");
        let mut user: UserService = use_context();
        user.logout().await;
    }

    pub async fn create(&mut self, user_ids: Vec<i64>, space_form: SpaceForm) {
        match Space::get_client(config::get().main_api_endpoint)
            .create_space(SpaceType::Post, space_form, self.id, user_ids)
            .await
        {
            Ok(_) => {
                tracing::info!("success to create space");
            }
            Err(e) => {
                btracing::error!(
                    "failed to create space with error: {}",
                    e.translate(&self.lang)
                );
            }
        };
    }

    pub fn enter_space(&mut self) {
        self.nav.replace(Route::IndexPage {});
    }

    pub fn create_space(&mut self) {
        let users = match self.total_users()() {
            Some(v) => v,
            None => vec![],
        };
        let mut ctrl = *self;

        self.popup
            .open(rsx! {
                CreateSpacePopup {
                    lang: self.lang,
                    users,
                    onsend: move |ids: Vec<i64>| {
                        tracing::debug!("selected user ids: {:?}", ids);
                        ctrl.open_select_space_form_popup_modal(ids);
                    },
                }
            })
            .with_title("Invite to Committee");
    }

    pub fn open_select_space_form_popup_modal(&mut self, ids: Vec<i64>) {
        let mut ctrl = *self;

        self.popup
            .open(rsx! {
                SelectSpaceFormPopup {
                    lang: self.lang,
                    onsend: {
                        move |form: SpaceForm| {
                            let ids = ids.clone();
                            async move {
                                tracing::debug!("space form: {:?}", form);
                                ctrl.create(ids.clone(), form).await;
                                ctrl.popup.close();
                            }
                        }
                    },
                }
            })
            .with_title("Select a Space Form");
    }

    pub fn prev_page(&self) {
        self.nav.replace(Route::IndexPage {});
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
