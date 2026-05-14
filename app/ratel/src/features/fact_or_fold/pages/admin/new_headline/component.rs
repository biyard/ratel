use crate::common::*;
use super::i18n::FactFoldAdminNewHeadlineTranslate;

/// `/admin/fact-or-fold/headlines/new` — Draft creation + Schedule /
/// Publish controls. Largest admin form; full implementation in a
/// dedicated commit.
#[component]
pub fn FactFoldAdminNewHeadlinePage() -> Element {
    let tr: FactFoldAdminNewHeadlineTranslate = use_translate();
    rsx! {
        SeoMeta { title: "{tr.page_title}" }
        section { class: "ff-admin-page ff-admin-page--placeholder",
            h1 { class: "ff-admin-page__title", "{tr.page_title}" }
            p { class: "ff-admin-page__hint", "{tr.coming_soon}" }
        }
    }
}
