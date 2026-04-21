use crate::common::*;
use crate::features::notifications::components::notification_panel::notification_item::NotificationItem;
use crate::features::notifications::controllers::mark_all_read::mark_all_read_handler;
use crate::features::notifications::controllers::mark_read::mark_read_handler;
use crate::features::notifications::hooks::{use_inbox, use_unread_count};
use crate::features::notifications::i18n::NotificationsTranslate;
use crate::features::notifications::types::InboxNotificationResponse;
use crate::notifications::hooks::UseInbox;

#[component]
pub fn NotificationPanel(open: bool, on_close: EventHandler<()>) -> Element {
    let tr: NotificationsTranslate = use_translate();
    let UseInbox {
        mut inbox,
        mut handle_item_click,
        mut handle_mark_all,
        ..
    } = use_inbox()?;

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "notification-panel",
            "data-open": "{open}",
            "data-testid": "notification-panel",
            div { class: "notification-panel__header",
                h3 { class: "notification-panel__title", "{tr.panel_title}" }
                button {
                    class: "notification-panel__mark-all",
                    "data-testid": "mark-all-read",
                    onclick: move |_| {
                        handle_mark_all.call();
                    },
                    "{tr.mark_all_read}"
                }
                button {
                    class: "notification-panel__close",
                    "aria-label": "Close",
                    onclick: move |_| on_close.call(()),
                    lucide_dioxus::X { class: "w-5 h-5 [&>path]:stroke-icon-primary" }
                }
            }
            div {
                class: "notification-panel__list",
                "data-testid": "notification-list",
                if inbox.items().is_empty() && !inbox.is_loading() {
                    div { class: "notification-panel__empty", "{tr.empty}" }
                } else {
                    for item in inbox.items() {
                        NotificationItem {
                            key: "{item.id.0}",
                            item: item.clone(),
                            onclick: move |it| handle_item_click.call(it),
                        }
                    }
                    {inbox.more_element()}
                }
            }
        }
    }
}
