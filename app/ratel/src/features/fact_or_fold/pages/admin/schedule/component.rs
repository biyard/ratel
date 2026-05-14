use crate::common::*;
use super::i18n::FactFoldAdminScheduleTranslate;

/// `/admin/fact-or-fold/schedule` — calendar view of upcoming
/// scheduled headlines + queue alarm banner.
#[component]
pub fn FactFoldAdminSchedulePage() -> Element {
    let tr: FactFoldAdminScheduleTranslate = use_translate();
    rsx! {
        SeoMeta { title: "{tr.page_title}" }
        section { class: "ff-admin-page ff-admin-page--placeholder",
            h1 { class: "ff-admin-page__title", "{tr.page_title}" }
            p { class: "ff-admin-page__hint", "{tr.coming_soon}" }
        }
    }
}
