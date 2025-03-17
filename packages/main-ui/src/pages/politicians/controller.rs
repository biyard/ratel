use bdk::prelude::*;
use by_types::QueryResponse;
use dto::*;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    pub politicians: Resource<QueryResponse<AssemblyMemberSummary>>,
    pub sort: Signal<Option<AssemblyMemberSorter>>,
    pub stance: Signal<Option<CryptoStance>>,
    #[allow(dead_code)]
    pub page: Signal<usize>,
}

impl Controller {
    pub fn new(_lang: Language) -> std::result::Result<Self, RenderError> {
        let size = 20;
        let page = use_signal(|| 1);
        let sort = use_signal(|| None);
        let stance = use_signal(|| None);

        let politicians = use_server_future(move || {
            let cli = AssemblyMember::get_client(&crate::config::get().main_api_endpoint);
            let page = page();
            let sort = sort();
            let stance = stance();

            async move {
                let mut q = AssemblyMemberQuery::new(size).with_page(page);
                if let Some(sort) = sort {
                    q = q.with_sort(sort);
                }

                if let Some(stance) = stance {
                    q = q.with_stance(stance);
                }

                cli.query(q).await.unwrap_or_default()
            }
        })?;

        let ctrl = Self {
            politicians,
            sort,
            page,
            stance,
        };
        use_context_provider(|| ctrl);

        Ok(ctrl)
    }
}
