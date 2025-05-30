use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn TeamsByIdPage(
    teamname: ReadOnlySignal<String>,
    #[props(default = Language::En)] lang: Language,
) -> Element {
    tracing::debug!(
        "TeamsByIdPage called with lang: {:?}, teamname: {:?}",
        lang,
        teamname
    );
    let mut _ctrl: Controller = use_context();
    let tr: TeamsByIdTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div { id: "teams-by-id", "{tr.title} PAGE" } // end of this page
    }
}
