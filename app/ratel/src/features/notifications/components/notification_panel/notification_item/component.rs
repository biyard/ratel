use crate::common::*;
use crate::features::notifications::i18n::NotificationsTranslate;
use crate::features::notifications::types::InboxNotificationResponse;
use dioxus_translate::use_language;

fn relative_time(now_ms: i64, then_ms: i64, tr: &NotificationsTranslate) -> String {
    let diff = (now_ms - then_ms).max(0);
    let secs = diff / 1000;
    let mins = secs / 60;
    let hours = mins / 60;
    let days = hours / 24;
    if secs < 60 {
        tr.relative_now.clone()
    } else if mins < 60 {
        tr.relative_minute.replace("{n}", &mins.to_string())
    } else if hours < 24 {
        tr.relative_hour.replace("{n}", &hours.to_string())
    } else {
        tr.relative_day.replace("{n}", &days.to_string())
    }
}

#[component]
pub fn NotificationItem(
    item: ReadSignal<InboxNotificationResponse>,
    onclick: EventHandler<InboxNotificationResponse>,
) -> Element {
    let tr: NotificationsTranslate = use_translate();
    let lang = use_language();
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let data = item();
    let (title, body, avatar_url): (String, String, Option<String>) = match &data.payload {
        InboxPayload::ReplyOnComment {
            replier_name,
            comment_preview,
            replier_profile_url,
            ..
        } => (
            tr.reply_title.replace("{name}", replier_name),
            comment_preview.clone(),
            Some(replier_profile_url.clone()),
        ),
        InboxPayload::MentionInComment {
            mentioned_by_name,
            comment_preview,
            ..
        } => (
            tr.mention_title.replace("{name}", mentioned_by_name),
            comment_preview.clone(),
            None,
        ),
        InboxPayload::SpaceStatusChanged {
            space_title,
            new_status,
            ..
        } => (
            tr.space_status_title
                .replace("{space}", space_title)
                .replace("{status}", &new_status.translate(&lang())),
            String::new(),
            None,
        ),
        InboxPayload::SpaceInvitation {
            space_title,
            inviter_name,
            ..
        } => (
            tr.space_invite_title
                .replace("{name}", inviter_name)
                .replace("{space}", space_title),
            String::new(),
            None,
        ),
    };
    let is_unread = !data.is_read;
    let rel = relative_time(now, data.created_at, &tr);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        button {
            class: "notification-item",
            "aria-relevant": "{is_unread}",
            "data-testid": "notification-item",
            onclick: {
                let d = data.clone();
                move |_| onclick.call(d.clone())
            },
            div { class: "notification-item__avatar",
                if let Some(url) = avatar_url.filter(|u| !u.is_empty()) {
                    img { src: "{url}", alt: "" }
                } else {
                    lucide_dioxus::Bell { class: "w-5 h-5 [&>path]:stroke-icon-primary" }
                }
            }
            div { class: "notification-item__body",
                div { class: "notification-item__title", "{title}" }
                if !body.is_empty() {
                    div { class: "notification-item__preview", "{body}" }
                }
                div { class: "notification-item__time", "{rel}" }
            }
            if is_unread {
                span {
                    class: "notification-item__dot",
                    "data-testid": "unread-dot",
                }
            }
        }
    }
}
