use by_types::QueryResponse;
use dioxus_aws::prelude::*;
use dioxus_translate::Language;
use dto::*;

#[derive(Debug, Clone, Copy)]
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

    pub fn politicians(&self) -> Vec<AssemblyMemberSummary> {
        self.politicians.with(|f| {
            // tracing::debug!("politicians: {:?}", f);
            if let Some(value) = f {
                value.items.clone()
            } else {
                Vec::new()
            }
        })
    }
}
