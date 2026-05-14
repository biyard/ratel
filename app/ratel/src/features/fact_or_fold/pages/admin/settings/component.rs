use crate::common::*;
use super::i18n::FactFoldAdminSettingsTranslate;

/// `/admin/fact-or-fold/settings` — admin-tunable RP / stage timing /
/// queue threshold knobs. Backend already shipped in PR1; UI lands
/// in a dedicated commit.
#[component]
pub fn FactFoldAdminSettingsPage() -> Element {
    let tr: FactFoldAdminSettingsTranslate = use_translate();
    rsx! {
        SeoMeta { title: "{tr.page_title}" }
        section { class: "ff-admin-page ff-admin-page--placeholder",
            h1 { class: "ff-admin-page__title", "{tr.page_title}" }
            p { class: "ff-admin-page__hint", "{tr.coming_soon}" }
        }
    }
}
