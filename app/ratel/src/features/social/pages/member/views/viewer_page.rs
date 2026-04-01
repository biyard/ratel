use super::super::*;

#[component]
pub fn ViewerPage(username: String) -> Element {
    let tr: ViewerPageTranslate = use_translate();

    rsx! {
        div { class: "flex flex-col items-center justify-center w-full h-full gap-2",
            h1 { class: "text-2xl font-bold text-text-primary", "{tr.no_permission}" }
            p { class: "text-foreground-muted", "{tr.no_permission_desc}" }
            p { class: "text-foreground-muted", "{tr.team_prefix} {username}" }
        }
    }
}
