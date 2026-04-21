use crate::common::*;
use crate::features::notifications::components::notification_panel::notification_item::NotificationItem;
use crate::features::notifications::controllers::mark_all_read::mark_all_read_handler;
use crate::features::notifications::controllers::mark_read::mark_read_handler;
use crate::features::notifications::hooks::{use_inbox, use_unread_count};
use crate::features::notifications::i18n::NotificationsTranslate;
use crate::features::notifications::types::InboxNotificationResponse;

#[component]
pub fn NotificationPanel(open: bool, on_close: EventHandler<()>) -> Element {
    let tr: NotificationsTranslate = use_translate();
    let mut inbox = use_inbox(false)?;
    let mut unread_count = use_unread_count();
    let nav = use_navigator();

    let on_item_click = move |item: InboxNotificationResponse| {
        let inbox_id = item.id.clone();
        let cta = match &item.payload {
            InboxPayload::ReplyOnComment { cta_url, .. } => cta_url.clone(),
            InboxPayload::MentionInComment { cta_url, .. } => cta_url.clone(),
            InboxPayload::SpaceStatusChanged { cta_url, .. } => cta_url.clone(),
            InboxPayload::SpaceInvitation { cta_url, .. } => cta_url.clone(),
        };
        spawn(async move {
            let _ = mark_read_handler(inbox_id).await;
        });
        if !cta.is_empty() {
            nav.push(cta);
        }
    };

    let on_mark_all = move |_| {
        spawn(async move {
            if let Err(e) = mark_all_read_handler().await {
                error!("mark-all-read failed: {e}");
                return;
            }
            unread_count.set(0);
            inbox.refresh();
        });
    };

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
                    onclick: on_mark_all,
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
                            onclick: on_item_click,
                        }
                    }
                    {inbox.more_element()}
                }
            }
        }
    }
}
