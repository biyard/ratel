use crate::common::*;
use crate::features::notifications::hooks::use_unread_count;

#[component]
pub fn NotificationBell(onclick: EventHandler<()>, #[props(default)] class: String) -> Element {
    let count = use_unread_count()();
    let label = if count >= 100 {
        "99+".to_string()
    } else {
        count.to_string()
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        button {
            class: "notification-bell {class}",
            "aria-label": "Notifications",
            "data-testid": "notification-bell",
            onclick: move |_| onclick.call(()),
            lucide_dioxus::Bell { class: "w-6 h-6 [&>path]:stroke-icon-primary" }
            if count > 0 {
                span {
                    class: "notification-bell__badge",
                    "data-testid": "notification-bell-badge",
                    "{label}"
                }
            }
        }
    }
}
