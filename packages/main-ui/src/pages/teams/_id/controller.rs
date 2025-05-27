use bdk::prelude::*;
use dto::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    #[allow(dead_code)]
    pub team: Resource<Team>,
}

impl Controller {
    pub fn new(
        lang: Language,
        id: ReadOnlySignal<String>,
    ) -> std::result::Result<Self, RenderError> {
        let team = use_server_future(move || {
            let username = id();
            async move {
                Team::get_client(crate::config::get().main_api_endpoint)
                    .get_by_username(username)
                    .await
                    .unwrap_or_default()
            }
        })?;
        let ctrl = Self { lang, team };

        use_context_provider(move || ctrl);

        Ok(ctrl)
    }
}
