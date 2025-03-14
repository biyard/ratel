use bdk::prelude::*;
use by_types::QueryResponse;
use dto::*;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    pub politicians: Resource<QueryResponse<AssemblyMemberSummary>>,
}

impl Controller {
    pub fn new(_lang: Language) -> std::result::Result<Self, RenderError> {
        let size = 20;
        let page = use_signal(|| 1);

        let politicians = use_server_future(move || {
            let cli = AssemblyMember::get_client(&crate::config::get().main_api_endpoint);
            let page = page();
            async move {
                cli.query(AssemblyMemberQuery::new(size).with_page(page))
                    .await
                    .unwrap_or_default()
            }
        })?;

        let ctrl = Self { politicians };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }
}
