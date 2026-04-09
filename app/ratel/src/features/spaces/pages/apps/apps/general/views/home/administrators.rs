use crate::spaces::space_common::providers::use_space_context;

use super::*;

/// Splits a comma-separated list of identifiers (usernames or emails)
/// into trimmed, non-empty entries. Used by the Administrators panel to
/// queue multiple targets at once via the chip input.
fn normalize_identifier_inputs(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect()
}

#[component]
pub fn Administrators() -> Element {
    let tr: GeneralTranslate = use_translate();
    let ctx = use_space_context();
    let space = use_space();
    let mut toast = use_toast();

    let mut admins = use_loader(move || async move {
        let space_id = space().id;
        if ctx.role().is_admin() {
            list_space_admins(space_id).await
        } else {
            Ok(vec![])
        }
    })?;

    let mut identifier_input = use_signal(String::new);
    let mut queued_identifiers = use_signal(Vec::<String>::new);
    let mut adding = use_signal(|| false);

    rsx! {
        Card {
            div { class: "flex justify-between items-center self-stretch py-4 px-5 border-b border-separator",
                p { class: "font-bold font-raleway text-[24px]/[28px] tracking-[-0.24px] text-web-font-primary",
                    {tr.administrator}
                }
            }

            div { class: "flex flex-col items-start self-stretch p-5 gap-[10px] bg-card max-mobile:p-4",
                {
                    let admin_list = admins();
                    if admin_list.is_empty() {
                        rsx! {
                            p { class: "font-medium leading-6 font-raleway text-[15px] tracking-[0.5px] text-card-meta",
                                {tr.administrator_not_found}
                            }
                        }
                    } else {
                        rsx! {
                            for admin in admin_list.iter() {
                                AdministratorRow {
                                    key: "{admin.user_id}",
                                    name: admin.display_name.clone(),
                                    username: admin.username.clone(),
                                    profile_url: admin.profile_url.clone(),
                                    is_owner: admin.is_owner,
                                    on_remove: {
                                        let user_id = admin.user_id.clone();
                                        let space_id = space().id;
                                        move |_| {
                                            let user_id = user_id.clone();
                                            let space_id = space_id.clone();
                                            spawn(async move {
                                                match remove_space_admin(space_id, UserPartition(user_id)).await {
                                                    Ok(_) => {
                                                        admins.restart();
                                                    }
                                                    Err(err) => {
                                                        toast.error(err);
                                                    }
                                                }
                                            });
                                        }
                                    },
                                }
                            }
                        }
                    }
                }

                // Add admin form — uses the same chip-based Input layout
                // as `InviteParticipant`, but accepts either a username
                // or an email address (the backend resolves both).
                if ctx.role().is_admin() {
                    div { class: "flex flex-col gap-3 items-start pt-2 w-full",
                        Input {
                            id: "admin-identifier-input",
                            r#type: InputType::Text,
                            placeholder: tr.enter_username,
                            value: identifier_input(),
                            oninput: move |evt: FormEvent| {
                                identifier_input.set(evt.value());
                            },
                            onchange: move |evt: FormEvent| {
                                identifier_input.set(evt.value());
                            },
                            onconfirm: move |evt: KeyboardEvent| {
                                evt.stop_propagation();
                                let parsed = normalize_identifier_inputs(&identifier_input());
                                if parsed.is_empty() {
                                    return;
                                }
                                queued_identifiers
                                    .with_mut(|values| {
                                        for value in parsed {
                                            if !values.iter().any(|v| v == &value) {
                                                values.push(value);
                                            }
                                        }
                                    });
                                identifier_input.set(String::new());
                            },
                        }

                        if !queued_identifiers().is_empty() {
                            div { class: "flex flex-wrap gap-2 items-center w-full",
                                for (idx , value) in queued_identifiers().iter().enumerate() {
                                    InviteEmailChip {
                                        key: "{idx}-{value}",
                                        value: value.clone(),
                                        on_remove: move |_| {
                                            queued_identifiers
                                                .with_mut(|values| {
                                                    if idx < values.len() {
                                                        values.remove(idx);
                                                    }
                                                });
                                        },
                                    }
                                }
                            }
                        }

                        div { class: "flex justify-end w-full",
                            Button {
                                size: ButtonSize::Small,
                                loading: adding(),
                                disabled: adding()
                                    || (queued_identifiers().is_empty() && identifier_input().trim().is_empty()),
                                onclick: {
                                    let space_id = space().id;
                                    move |_| {
                                        let space_id = space_id.clone();
                                        async move {
                                            if adding() {
                                                return;
                                            }
                                            // Flush whatever is in the input box too,
                                            // matching the InviteParticipant behavior.
                                            let mut targets = queued_identifiers();
                                            let current = identifier_input();
                                            for value in normalize_identifier_inputs(&current) {
                                                if !targets.iter().any(|v| v == &value) {
                                                    targets.push(value);
                                                }
                                            }
                                            if targets.is_empty() {
                                                return;
                                            }
                                            adding.set(true);
                                            let mut last_err = None;
                                            for target in targets {
                                                if let Err(err) = add_space_admin(
                                                        space_id.clone(),
                                                        AddSpaceAdminRequest { target },
                                                    )
                                                    .await
                                                {
                                                    last_err = Some(err);
                                                    break;
                                                }
                                            }
                                            adding.set(false);
                                            match last_err {
                                                Some(err) => {
                                                    toast.error(err);
                                                }
                                                None => {
                                                    queued_identifiers.set(vec![]);
                                                    identifier_input.set(String::new());
                                                    admins.restart();
                                                }
                                            }
                                        }
                                    }
                                },
                                {tr.add_admin}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AdministratorRow(
    name: String,
    username: String,
    profile_url: String,
    is_owner: bool,
    on_remove: EventHandler<MouseEvent>,
) -> Element {
    let tr: GeneralTranslate = use_translate();
    let profile = if profile_url.trim().is_empty() {
        DEFAULT_PROFILE_IMAGE.to_string()
    } else {
        profile_url
    };

    rsx! {
        div { class: "flex justify-between items-center py-3 px-4 w-full border rounded-[12px] border-separator bg-card max-tablet:flex-col max-tablet:items-start max-tablet:gap-3",
            div { class: "flex items-center gap-[10px]",
                img {
                    src: "{profile}",
                    alt: "{name}",
                    class: "object-cover w-12 h-12 rounded-full shrink-0",
                }

                div { class: "flex flex-col gap-1 items-start",
                    div { class: "flex gap-2 items-center",
                        p { class: "font-bold leading-5 font-raleway text-[17px] tracking-[-0.18px] text-web-font-primary",
                            "{name}"
                        }
                        if is_owner {
                            Badge {
                                color: BadgeColor::Blue,
                                size: BadgeSize::Normal,
                                {tr.owner}
                            }
                        } else {
                            Badge {
                                color: BadgeColor::Green,
                                size: BadgeSize::Normal,
                                {tr.admin}
                            }
                        }
                    }
                    p { class: "font-semibold leading-4 font-raleway text-[13px] tracking-[-0.14px] text-web-font-neutral",
                        "@{username}"
                    }
                }
            }

            if !is_owner {
                Button {
                    size: ButtonSize::Small,
                    style: ButtonStyle::Outline,
                    onclick: move |e| on_remove.call(e),
                    {tr.remove_admin}
                }
            }
        }
    }
}
