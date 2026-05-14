use crate::common::*;
use super::i18n::FactFoldAdminHeadlinesTranslate;

/// `/admin/fact-or-fold/headlines` — list of every headline ever
/// authored, filterable by status. Filled in by the next commit;
/// stub keeps the route compile-clean.
#[component]
pub fn FactFoldAdminHeadlinesPage() -> Element {
    let tr: FactFoldAdminHeadlinesTranslate = use_translate();
    rsx! {
        SeoMeta { title: "{tr.page_title}" }
        section { class: "ff-admin-page ff-admin-page--placeholder",
            h1 { class: "ff-admin-page__title", "{tr.page_title}" }
            p { class: "ff-admin-page__hint", "{tr.coming_soon}" }
        }
    }
}
