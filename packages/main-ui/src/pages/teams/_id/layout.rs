use super::*;
use crate::Route;
use bdk::prelude::*;
use controller::*;

// id: teamname(username)
#[component]
pub fn TeamsByIdLayout(
    #[props(default = Language::En)] lang: Language,
    teamname: ReadOnlySignal<String>,
) -> Element {
    tracing::debug!(
        "TeamsByIdLayout called with lang: {:?}, id: {:?}",
        lang,
        teamname
    );

    let mut _ctrl = Controller::new(lang, teamname)?;

    rsx! {
        SuspenseBoundary {
            fallback: |_| rsx! {
                div { class: "loader w-200" }
            },
            Outlet::<Route> {}
        }
    }
}
