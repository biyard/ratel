//! "Pending applications / 신청 대기" tab — consumes `UseSubTeamQueue`.
//!
//! Shows each pending application with approve/return/reject buttons.
//! Approve requires no extra input; return prompts for a comment;
//! reject prompts for a reason. Both prompts are inline (small popups).

use crate::features::sub_team::{
    use_sub_team_queue, SubTeamAnnouncementStatusLabel as _, SubTeamApplicationResponse,
    SubTeamTranslate, UseSubTeamQueue,
};
use crate::*;

#[component]
pub fn QueueTab() -> Element {
    let tr: SubTeamTranslate = use_translate();
    let UseSubTeamQueue {
        mut queue,
        mut handle_approve,
        mut handle_reject,
        mut handle_return,
        ..
    } = use_sub_team_queue()?;

    let items = queue.items();
    let item_count = items.len();

    rsx! {
        section { class: "card",
            div { class: "card__head",
                h2 { class: "card__title", "{tr.tab_queue}" }
                span { class: "card__dash" }
                span { class: "card__meta", "{item_count}" }
            }

            if item_count == 0 && !queue.is_loading() {
                div { class: "inline-note", "{tr.empty_list}" }
            } else {
                div { class: "u-col u-gap-10", id: "queue-list",
                    for app in items.iter() {
                        QueueRow {
                            key: "{app.id}",
                            app: app.clone(),
                            on_approve: move |id| handle_approve.call(id),
                            on_reject: move |(id, reason)| handle_reject.call(id, reason),
                            on_return: move |(id, comment)| handle_return.call(id, comment),
                        }
                    }
                }
                {queue.more_element()}
            }
        }
    }
}

#[component]
fn QueueRow(
    app: SubTeamApplicationResponse,
    on_approve: EventHandler<String>,
    on_reject: EventHandler<(String, String)>,
    on_return: EventHandler<(String, String)>,
) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let app_id = app.id.clone();
    let app_id_for_approve = app_id.clone();
    let app_id_for_return = app_id.clone();
    let app_id_for_reject = app_id.clone();

    let mut mode: Signal<QueueRowMode> = use_signal(|| QueueRowMode::Idle);
    let mut text: Signal<String> = use_signal(String::new);

    let submitter = app.submitter_user_id.clone();

    rsx! {
        div { class: "queue-row",
            div { class: "avatar avatar--teal", "•" }
            div { class: "queue-row__body",
                span { class: "queue-row__name", "Application {app.id}" }
                div { class: "queue-row__meta",
                    span { "by {submitter}" }
                    span { "submitted {app.submitted_at.unwrap_or(app.created_at)}" }
                }
            }
            div { class: "queue-row__actions",
                button {
                    class: "queue-row__action queue-row__action--approve",
                    onclick: move |_| on_approve.call(app_id_for_approve.clone()),
                    "{tr.approve}"
                }
                button {
                    class: "queue-row__action queue-row__action--return",
                    onclick: move |_| {
                        mode.set(QueueRowMode::Return);
                        text.set(String::new());
                    },
                    "{tr.r#return}"
                }
                button {
                    class: "queue-row__action queue-row__action--reject",
                    onclick: move |_| {
                        mode.set(QueueRowMode::Reject);
                        text.set(String::new());
                    },
                    "{tr.reject}"
                }
            }
            if mode() != QueueRowMode::Idle {
                div { class: "field",
                    textarea {
                        class: "field__textarea",
                        placeholder: if mode() == QueueRowMode::Return { "{tr.return_comment}" } else { "{tr.reject_reason}" },
                        value: "{text()}",
                        oninput: move |e| text.set(e.value()),
                    }
                    div { class: "u-flex u-gap-10",
                        button {
                            class: "btn btn--ghost btn--small",
                            onclick: move |_| mode.set(QueueRowMode::Idle),
                            "{tr.cancel}"
                        }
                        button {
                            class: "btn btn--primary btn--small",
                            onclick: move |_| {
                                let content = text().clone();
                                match mode() {
                                    QueueRowMode::Return => on_return.call((app_id_for_return.clone(), content)),
                                    QueueRowMode::Reject => on_reject.call((app_id_for_reject.clone(), content)),
                                    QueueRowMode::Idle => {}
                                }
                                mode.set(QueueRowMode::Idle);
                                text.set(String::new());
                            },
                            if mode() == QueueRowMode::Return {
                                "{tr.r#return}"
                            } else {
                                "{tr.reject}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum QueueRowMode {
    Idle,
    Return,
    Reject,
}
