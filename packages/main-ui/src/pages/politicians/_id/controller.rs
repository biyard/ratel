use bdk::prelude::*;
use dto::{AssemblyMember, BillSummary};

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

                let mut res = AssemblyMember::get_client(endpoint)
                    .get(id)
                    .await
                    .unwrap_or_default();

                if res.bills.is_empty() {
                    res.bills = vec![BillSummary {
                        id: 1,
                        created_at: 100000000,
                        title: "DAO Treasury Transparency Act & Crypto Investor Protection Act"
                            .to_string(),
                        bill_no: "1".to_string(),
                        en_title: Some(
                            "DAO Treasury Transparency Act & Crypto Investor Protection Act"
                                .to_string(),
                        ),
                        book_id: "1".to_string(),
                        site_url: "https://example.com".to_string(),
                        summary: Some(
                            "This bill aims to provide transparency to DAO treasuries and protect crypto investors.".to_string(),
                        ),
                        en_summary: Some(
                            "This bill aims to provide transparency to DAO treasuries and protect crypto investors.".to_string(),
                        ),
                        votes: vec![],
                    }];
                }

                res
            }
        })?;

        let ctrl = Self { lang, politician };

        Ok(ctrl)
    }
}
