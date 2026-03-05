use crate::controllers::{AddSubscriptionUsersRequest, add_subscription_users};
use crate::*;
mod i18n;
use i18n::SubscriptionUserInviteTranslate;

fn normalize_email_input(raw: &str) -> Option<String> {
    let email = raw.trim().to_ascii_lowercase();
    if email.is_empty() || !email.contains('@') {
        return None;
    }
    Some(email)
}

#[component]
pub fn SubscriptionUserInvite(space_id: SpacePartition, on_refresh: EventHandler<()>) -> Element {
    let tr: SubscriptionUserInviteTranslate = use_translate();
    let mut email_input = use_signal(String::new);
    let mut pending_emails = use_signal(Vec::<String>::new);
    let mut loading = use_signal(|| false);

    let on_add = {
        let space_id = space_id.clone();
        let mut pending_emails = pending_emails.clone();
        let mut loading = loading.clone();
        move |_| {
            if loading() {
                return;
            }
            let emails = pending_emails();
            if emails.is_empty() {
                return;
            }

            loading.set(true);
            let on_refresh = on_refresh.clone();
            spawn({
                let space_id = space_id.clone();
                async move {
                    let res = add_subscription_users(
                        space_id.clone(),
                        AddSubscriptionUsersRequest { emails },
                    )
                    .await;
                    loading.set(false);
                    if res.is_ok() {
                        pending_emails.set(vec![]);
                        on_refresh.call(());
                    }
                }
            });
        }
    };

    rsx! {
        div { class: "overflow-visible w-full shrink-0 rounded-[12px] bg-card",
            div { class: "flex justify-between items-center self-stretch px-5 py-4 border-b border-separator",
                p { class: "font-semibold text-center sp-dash-font-raleway text-[17px]/[20px] tracking-[-0.18px] text-font-primary",
                    {tr.title}
                }
            }
            div { class: "flex flex-col gap-5 items-start self-stretch p-5 bg-card max-mobile:p-4",
                div { class: "flex items-start w-full gap-[10px] max-tablet:flex-col",
                    div { class: "flex flex-col flex-1 gap-2 justify-center items-start",
                        p { class: "font-semibold sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-font-primary",
                            {tr.email_label}
                        }
                        Input {
                            class: "flex flex-col justify-center items-start px-3 py-2.5 w-full font-medium leading-6 border-gray-600 rounded-[8px] border-[0.5px] bg-web-input sp-dash-font-raleway text-[15px] tracking-[0.5px] text-font-primary placeholder:text-card-more-muted",
                            placeholder: tr.email_placeholder.to_string(),
                            value: email_input(),
                            oninput: move |evt: Event<FormData>| {
                                email_input.set(evt.value().to_string());
                            },
                            onkeydown: move |evt: Event<KeyboardData>| {
                                if evt.key() != Key::Enter {
                                    return;
                                }
                                let Some(email) = normalize_email_input(&email_input()) else {
                                    return;
                                };
                                pending_emails
                                    .with_mut(|emails| {
                                        if !emails.iter().any(|v| v == &email) {
                                            emails.push(email);
                                        }
                                    });
                                email_input.set(String::new());
                            },
                        }

                        div { class: "flex flex-wrap gap-2 items-center w-full",
                            for (idx , value) in pending_emails().iter().enumerate() {
                                {
                                    let value = value.clone();
                                    rsx! {
                                        InviteEmailChip {
                                            key: "{value}",
                                            value: value.clone(),
                                            on_remove: move |_| {
                                                pending_emails
                                                    .with_mut(|emails| {
                                                        if idx < emails.len() {
                                                            emails.remove(idx);
                                                        }
                                                    })
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "flex justify-end w-full max-tablet:justify-stretch",
                    Button {
                        class: "w-fit max-tablet:w-full",
                        style: ButtonStyle::Primary,
                        disabled: loading(),
                        onclick: on_add,
                        if loading() {
                            {tr.adding}
                        } else {
                            {tr.add}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InviteEmailChip(value: String, on_remove: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex gap-1 items-center pr-1 pl-3 h-7 rounded-[100px] bg-btn-primary-bg",
            span { class: "font-medium leading-6 sp-dash-font-raleway text-[15px] tracking-[0.5px] text-btn-primary-text",
                "{value}"
            }
            button {
                class: "flex justify-center items-center rounded-full size-5 text-btn-primary-text/90",
                onclick: move |evt| on_remove.call(evt),
                icons::ratel::XMarkIcon {
                    width: "12",
                    height: "12",
                    class: "w-3 h-3 text-btn-primary-text/90",
                }
            }
        }
    }
}
