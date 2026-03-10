use crate::features::spaces::pages::actions::actions::subscription::controllers::{
    AddSubscriptionUsersRequest, CheckSubscriptionUsersRequest, add_subscription_users,
    check_subscription_users,
};
use crate::features::spaces::pages::actions::actions::subscription::*;
mod i18n;
use i18n::SubscriptionUserInviteTranslate;

fn normalize_identifier_input(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|value| value.trim().trim_start_matches('@').to_ascii_lowercase())
        .filter(|value| !value.is_empty())
        .collect()
}

#[component]
pub fn SubscriptionUserInvite(space_id: SpacePartition, on_refresh: EventHandler<()>) -> Element {
    let tr: SubscriptionUserInviteTranslate = use_translate();
    let mut identifier_input = use_signal(String::new);
    let mut pending_identifiers = use_signal(Vec::<String>::new);
    let mut loading = use_signal(|| false);

    let on_add = {
        let space_id = space_id.clone();
        let mut loading = loading.clone();
        move |_| {
            if loading() {
                return;
            }
            let mut identifiers = pending_identifiers();
            let input_identifiers = normalize_identifier_input(&identifier_input());
            identifiers.extend(input_identifiers);
            if !identifiers.is_empty() {
                let mut seen = std::collections::HashSet::<String>::new();
                identifiers.retain(|value| seen.insert(value.clone()));
            }
            if identifiers.is_empty() {
                return;
            }

            loading.set(true);
            let on_refresh = on_refresh.clone();
            spawn({
                let space_id = space_id.clone();
                let mut identifier_input = identifier_input.clone();
                async move {
                    let res = add_subscription_users(
                        space_id.clone(),
                        AddSubscriptionUsersRequest { identifiers },
                    )
                    .await;
                    loading.set(false);
                    if res.is_ok() {
                        pending_identifiers.set(vec![]);
                        identifier_input.set(String::new());
                        on_refresh.call(());
                    }
                }
            });
        }
    };

    rsx! {
        SpaceCard { class: "overflow-visible w-full shrink-0 rounded-[12px] !p-0".to_string(),
            div { class: "flex justify-between items-center self-stretch px-5 py-4 border-b border-separator",
                p { class: "font-semibold text-center sp-dash-font-raleway text-[17px]/[20px] tracking-[-0.18px] text-font-primary",
                    {tr.title}
                }
            }
            div { class: "flex flex-col gap-5 items-start self-stretch p-5 bg-card max-mobile:p-4",
                div { class: "flex items-start w-full gap-[10px] max-tablet:flex-col",
                    div { class: "flex flex-col flex-1 gap-2 justify-center items-start",
                        p { class: "font-semibold sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-font-primary",
                            {tr.identifier_label}
                        }
                        Input {
                            class: "flex flex-col justify-center items-start px-3 py-2.5 w-full font-medium leading-6 border-gray-600 rounded-[8px] border-[0.5px] bg-web-input sp-dash-font-raleway text-[15px] tracking-[0.5px] text-font-primary placeholder:text-card-more-muted",
                            placeholder: tr.identifier_placeholder.to_string(),
                            value: identifier_input(),
                            oninput: move |evt: Event<FormData>| {
                                identifier_input.set(evt.value().to_string());
                            },
                            onkeydown: move |evt: Event<KeyboardData>| {
                                if evt.key() != Key::Enter {
                                    return;
                                }
                                let identifiers = normalize_identifier_input(&identifier_input());
                                if identifiers.is_empty() {
                                    return;
                                };
                                let space_id = space_id.clone();
                                let mut pending_identifiers = pending_identifiers.clone();
                                let mut identifier_input = identifier_input.clone();
                                spawn(async move {
                                    if let Ok(res) = check_subscription_users(
                                        space_id,
                                        CheckSubscriptionUsersRequest { identifiers },
                                    )
                                    .await
                                    {
                                        let existing = res.existing_identifiers;
                                        if !existing.is_empty() {
                                            pending_identifiers.with_mut(|list| {
                                                for value in existing {
                                                    if !list.iter().any(|v| v == &value) {
                                                        list.push(value);
                                                    }
                                                }
                                            });
                                        }
                                    }
                                    identifier_input.set(String::new());
                                });
                            },
                        }

                        div { class: "flex flex-wrap gap-2 items-center w-full",
                            for (idx , value) in pending_identifiers().iter().enumerate() {
                                {
                                    let value = value.clone();
                                    rsx! {
                                        InviteEmailChip {
                                            key: "{value}",
                                            value: value.clone(),
                                            on_remove: move |_| {
                                                pending_identifiers
                                                    .with_mut(|identifiers| {
                                                        if idx < identifiers.len() {
                                                            identifiers.remove(idx);
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
            Button {
                class: "size-5 rounded-full !p-0 hover:!bg-transparent disabled:opacity-50".to_string(),
                size: ButtonSize::Icon,
                style: ButtonStyle::Text,
                onclick: move |evt| on_remove.call(evt),
                icons::ratel::XMarkIcon {
                    width: "12",
                    height: "12",
                    class: "w-3 h-3 [&>path]:stroke-btn-primary-text/90",
                }
            }
        }
    }
}
