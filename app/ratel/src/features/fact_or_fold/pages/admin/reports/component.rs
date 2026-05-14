use crate::common::*;
use super::i18n::FactFoldAdminReportsTranslate;

/// `/admin/fact-or-fold/reports` — moderation queue for in-round user
/// reports. Real data depends on Report entity (PR4+).
#[component]
pub fn FactFoldAdminReportsPage() -> Element {
    let tr: FactFoldAdminReportsTranslate = use_translate();
    rsx! {
        SeoMeta { title: "{tr.page_title}" }
        section { class: "ff-admin-page ff-admin-page--placeholder",
            h1 { class: "ff-admin-page__title", "{tr.page_title}" }
            p { class: "ff-admin-page__hint", "{tr.coming_soon}" }
        }
    }
}
