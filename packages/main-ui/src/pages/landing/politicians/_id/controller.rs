use bdk::prelude::*;
use dto::AssemblyMember;

use crate::services::user_service::UserService;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub politician: Resource<AssemblyMember>,
}

impl Controller {
    pub fn new(lang: Language, id: ReadOnlySignal<i64>) -> std::result::Result<Self, RenderError> {
        let user: UserService = use_context();

        let politician = use_server_future(move || {
            let id = id();
            let _ = user.loggedin();

            async move {
                let endpoint = crate::config::get().main_api_endpoint;

                AssemblyMember::get_client(endpoint)
                    .get(id)
                    .await
                    .unwrap_or_default()
            }
        })?;

        let ctrl = Self { lang, politician };
        use_context_provider(move || ctrl);

        Ok(ctrl)
    }
}
