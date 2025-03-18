use bdk::prelude::*;
use dto::AssemblyMember;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    pub lang: Language,
    pub politician: Resource<AssemblyMember>,
}

impl Controller {
    pub fn new(lang: Language, id: ReadOnlySignal<i64>) -> std::result::Result<Self, RenderError> {
        let politician = use_server_future(move || {
            let id = id();

            async move {
                let endpoint = crate::config::get().main_api_endpoint;

                AssemblyMember::get_client(endpoint)
                    .get(id)
                    .await
                    .unwrap_or_default()
            }
        })?;

        let ctrl = Self { lang, politician };

        Ok(ctrl)
    }
}
