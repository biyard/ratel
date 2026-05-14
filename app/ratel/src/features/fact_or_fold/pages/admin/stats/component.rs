use crate::common::*;
use super::i18n::FactFoldAdminStatsTranslate;

/// `/admin/fact-or-fold/stats` — round accuracy / insider effect /
/// mind-flip count aggregation. Real data depends on Round entity
/// (PR4); placeholder until then.
#[component]
pub fn FactFoldAdminStatsPage() -> Element {
    let tr: FactFoldAdminStatsTranslate = use_translate();
    rsx! {
        SeoMeta { title: "{tr.page_title}" }
        section { class: "ff-admin-page ff-admin-page--placeholder",
            h1 { class: "ff-admin-page__title", "{tr.page_title}" }
            p { class: "ff-admin-page__hint", "{tr.coming_soon}" }
        }
    }
}
