use bdk::prelude::*;
use dto::{MyInfo, Space};

use crate::{config, route::Route, services::user_service::UserService};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    #[allow(dead_code)]
    pub feed_id: i64,
    #[allow(dead_code)]
    pub id: i64,

    #[allow(dead_code)]
    pub space: Resource<Space>,
    #[allow(dead_code)]
    pub my_info: Signal<MyInfo>,
}

impl Controller {
    pub fn new(lang: Language, feed_id: i64, id: i64) -> std::result::Result<Self, RenderError> {
        let user_service: UserService = use_context();
        let nav = use_navigator();
        let mut my_info = use_signal(|| MyInfo::default());

        use_effect(move || {
            if !user_service.loggedin() {
                nav.replace(Route::LandingPage {});
            }

            my_info.set(user_service.my_info());
        });

        let space = use_server_future(move || async move {
            match Space::get_client(config::get().main_api_endpoint)
                .find_by_id(id)
                .await
            {
                Ok(space) => space,
                Err(e) => {
                    tracing::debug!("query space failed with error: {:?}", e);
                    Default::default()
                }
            }
        })?;

        let ctrl = Self {
            lang,
            feed_id,
            id,
            space,

            my_info,
        };

        ctrl.check_permission();

        Ok(ctrl)
    }

    pub fn check_permission(&self) {
        let nav = use_navigator();

        let (user_id, members) = match self.space() {
            Ok(v) => (v.user_id, v.members),
            Err(_) => (0, vec![]),
        };

        let members: Vec<i64> = members.iter().map(|v| v.user_id).collect();

        let ctrl = self.clone();

        use_effect(move || {
            let id = ctrl.clone().my_info().id;
            if id != 0 {
                if user_id != id && !members.contains(&id) {
                    nav.replace(Route::LandingPage {});
                }
            }
        });
    }
}
