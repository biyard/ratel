use super::*;
use crate::spaces::space_common::providers::use_space_context;

fn avatar_initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

#[component]
pub fn Administrators(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: GeneralTranslate = use_translate();
    let space_ctx = use_space_context();

    let UseSpaceGeneralSettings {
        admins,
        mut add_admins,
        mut remove_admin,
        ..
    } = use_space_general_settings(space_id)?;

    let mut identifier_input = use_signal(String::new);
    let mut queued_identifiers = use_signal(Vec::<String>::new);

    let admin_list = admins();
    let is_admin_role = space_ctx.role().is_admin();
    let add_pending = add_admins.pending();

    rsx! {
        // Single section card — matches the Invite Participant layout:
        // header → "Add" field → queued chip preview → "Current" field
        // with the chip list. Viewer role sees only the list.
        section { class: "sga-section", "data-testid": "section-administrators",
            div { class: "sga-section__head",
                span { class: "sga-section__label", "{tr.administrator}" }
                span { class: "sga-section__hint", "Users with admin access to this space" }
            }

            if is_admin_role {
                div { class: "sga-field",
                    span { class: "sga-field__label", "{tr.add_admin}" }
                    div { class: "sga-input-group",
                        input {
                            class: "sga-input",
                            r#type: "text",
                            placeholder: "{tr.enter_username}",
                            value: "{identifier_input()}",
                            "data-testid": "admin-identifier-input",
                            oninput: move |e: FormEvent| identifier_input.set(e.value()),
                            onchange: move |e: FormEvent| identifier_input.set(e.value()),
                            onkeydown: move |e: KeyboardEvent| {
                                if e.key() == Key::Enter {
                                    e.stop_propagation();
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
                                }
                            },
                        }
                        button {
                            r#type: "button",
                            class: "sga-btn sga-btn--accent",
                            "data-testid": "admin-add-btn",
                            disabled: add_pending
                                || (queued_identifiers().is_empty() && identifier_input().trim().is_empty()),
                            onclick: move |_| {
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
                                queued_identifiers.set(vec![]);
                                identifier_input.set(String::new());
                                add_admins.call(targets);
                            },
                            {tr.add_admin}
                        }
                    }
                }

                // Queued preview — chips typed but not yet submitted.
                if !queued_identifiers().is_empty() {
                    div {
                        class: "sga-creator-row",
                        "data-testid": "admin-queue",
                        for (idx, value) in queued_identifiers().iter().enumerate() {
                            InviteEmailChip {
                                key: "q-{idx}-{value}",
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
            }

            // Current admins list — always visible (viewers can see too).
            div { class: "sga-field",
                span { class: "sga-field__label", "Current admins" }
                if admin_list.is_empty() {
                    div { class: "sga-empty", "{tr.administrator_not_found}" }
                } else {
                    div { class: "sga-creator-row", "data-testid": "admin-list",
                        for admin in admin_list.iter() {
                            AdminChip {
                                key: "{admin.user_id}",
                                name: admin.display_name.clone(),
                                username: admin.username.clone(),
                                profile_url: admin.profile_url.clone(),
                                is_owner: admin.is_owner,
                                on_remove: {
                                    let user_id = admin.user_id.clone();
                                    move |_| remove_admin.call(UserPartition(user_id.clone()))
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AdminChip(
    name: String,
    username: String,
    profile_url: String,
    is_owner: bool,
    on_remove: EventHandler<MouseEvent>,
) -> Element {
    let tr: GeneralTranslate = use_translate();
    let initials = avatar_initials(&name);
    let role_class = if is_owner {
        "sga-creator-chip__role"
    } else {
        "sga-creator-chip__role sga-creator-chip__role--admin"
    };
    let role_text = if is_owner {
        tr.owner.to_string()
    } else {
        tr.admin.to_string()
    };

    rsx! {
        div { class: "sga-creator-chip",
            if profile_url.trim().is_empty() {
                span { class: "sga-creator-chip__avatar", "{initials}" }
            } else {
                span { class: "sga-creator-chip__avatar",
                    img { src: "{profile_url}", alt: "{name}" }
                }
            }
            span { class: "sga-creator-chip__name",
                if name.trim().is_empty() {
                    "@{username}"
                } else {
                    "{name}"
                }
            }
            span { class: "{role_class}", "{role_text}" }
            if !is_owner {
                button {
                    r#type: "button",
                    class: "sga-creator-chip__x",
                    onclick: move |e| on_remove.call(e),
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        line {
                            x1: "18",
                            y1: "6",
                            x2: "6",
                            y2: "18",
                        }
                        line {
                            x1: "6",
                            y1: "6",
                            x2: "18",
                            y2: "18",
                        }
                    }
                }
            }
        }
    }
}
