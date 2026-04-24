use super::*;
use crate::spaces::InvitationStatus;

fn avatar_initials(name: &str, email: &str) -> String {
    let src = if !name.trim().is_empty() { name } else { email };
    src.split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

#[component]
pub fn InviteParticipant(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: GeneralTranslate = use_translate();
    let mut toast = use_toast();

    let UseSpaceGeneralSettings {
        mut invitations,
        mut send_invitations,
        mut delete_invitation,
        ..
    } = use_space_general_settings(space_id)?;

    let mut email_input = use_signal(String::new);
    let mut invited_emails = use_signal(Vec::<String>::new);

    let invitation_items = invitations.items();
    let more_element = invitations.more_element();
    let invite_pending = send_invitations.pending();

    rsx! {
        section { class: "sga-section", "data-testid": "section-invite",
            div { class: "sga-section__head",
                span { class: "sga-section__label", "{tr.invite_participant}" }
                span { class: "sga-section__hint", "Send email invitations to join this space" }
            }

            div { class: "sga-field",
                span { class: "sga-field__label", "{tr.email_address}" }
                div { class: "sga-input-group",
                    input {
                        class: "sga-input",
                        r#type: "email",
                        placeholder: "{tr.email_placeholder}",
                        value: "{email_input()}",
                        "data-testid": "invite-email-input",
                        oninput: move |e: FormEvent| email_input.set(e.value()),
                        onchange: move |e: FormEvent| email_input.set(e.value()),
                        onkeydown: move |e: KeyboardEvent| {
                            if e.key() == Key::Enter {
                                e.stop_propagation();
                                let Ok(parsed_emails) = normalize_email_inputs(&email_input()) else {
                                    toast.error(Error::InvalidEmail);
                                    return;
                                };
                                invited_emails
                                    .with_mut(|emails| {
                                        for email in parsed_emails {
                                            if !emails.iter().any(|v| v == &email) {
                                                emails.push(email);
                                            }
                                        }
                                    });
                                email_input.set(String::new());
                            }
                        },
                    }
                    button {
                        r#type: "button",
                        class: "sga-btn sga-btn--accent",
                        "data-testid": "invite-send-btn",
                        disabled: invite_pending,
                        onclick: move |_| {
                            let mut emails = invited_emails();
                            let current_input = email_input();
                            if !current_input.trim().is_empty() {
                                let Ok(parsed_emails) = normalize_email_inputs(&current_input) else {
                                    toast.error(Error::InvalidEmail);
                                    return;
                                };
                                for email in parsed_emails {
                                    if !emails.iter().any(|value| value == &email) {
                                        emails.push(email);
                                    }
                                }
                            }
                            if emails.is_empty() {
                                return;
                            }
                            // Clear local state optimistically; the hook's action
                            // refreshes the invitations list on success.
                            invited_emails.set(vec![]);
                            email_input.set(String::new());
                            send_invitations.call(emails);
                        },
                        if invite_pending {
                            {tr.inviting}
                        } else {
                            {tr.invite}
                        }
                    }
                }
            }

            // Queue preview — chips for emails the user has typed but
            // hasn't sent yet.
            if !invited_emails().is_empty() {
                div { class: "sga-creator-row", "data-testid": "invite-queue",
                    for (idx, value) in invited_emails().iter().enumerate() {
                        InviteEmailChip {
                            key: "{idx}-{value}",
                            value: value.clone(),
                            on_remove: move |_| {
                                invited_emails
                                    .with_mut(|emails| {
                                        if idx < emails.len() {
                                            emails.remove(idx);
                                        }
                                    });
                            },
                        }
                    }
                }
            }

            // Invited accounts — includes both Pending and Accepted.
            div { class: "sga-field",
                span { class: "sga-field__label", "{tr.invited_accounts}" }
                div { class: "sga-creator-row", "data-testid": "invite-list",
                    if invitation_items.is_empty() && invitations.is_loading() {
                        div { class: "sga-empty", "{tr.loading_invitations}" }
                    } else if invitation_items.is_empty() {
                        div { class: "sga-empty", "{tr.no_invited_accounts}" }
                    } else {
                        for item in invitation_items {
                            InvitedAccountChip {
                                key: "{item.user_id}",
                                item: item.clone(),
                                on_remove: move |user_id: UserPartition| { delete_invitation.call(user_id) },
                            }
                        }
                    }
                }
                div { {more_element} }
            }
        }
    }
}

#[component]
pub fn InviteEmailChip(value: String, on_remove: EventHandler<MouseEvent>) -> Element {
    let initials = avatar_initials(&value, &value);
    rsx! {
        div { class: "sga-creator-chip",
            span { class: "sga-creator-chip__avatar", "{initials}" }
            span { class: "sga-creator-chip__name", "{value}" }
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

#[component]
fn InvitedAccountChip(
    item: SpaceInvitationListItem,
    on_remove: EventHandler<UserPartition>,
) -> Element {
    let initials = avatar_initials(&item.display_name, &item.email);
    let name_display = if item.display_name.trim().is_empty() {
        item.email.clone()
    } else {
        item.display_name.clone()
    };
    let role_class = match item.status {
        InvitationStatus::Accepted => "sga-creator-chip__role sga-creator-chip__role--admin",
        InvitationStatus::Declined => "sga-creator-chip__role",
        InvitationStatus::Pending | InvitationStatus::Invited => {
            "sga-creator-chip__role sga-creator-chip__role--pending"
        }
    };
    let role_text = match item.status {
        InvitationStatus::Accepted => "Accepted",
        InvitationStatus::Declined => "Declined",
        InvitationStatus::Pending | InvitationStatus::Invited => "Pending",
    };
    let user_id = item.user_id.clone();

    rsx! {
        div { class: "sga-creator-chip",
            if item.profile_url.trim().is_empty() {
                span { class: "sga-creator-chip__avatar", "{initials}" }
            } else {
                span { class: "sga-creator-chip__avatar",
                    img { src: "{item.profile_url}", alt: "{name_display}" }
                }
            }
            span { class: "sga-creator-chip__name", "{name_display}" }
            span { class: "{role_class}", "{role_text}" }
            button {
                r#type: "button",
                class: "sga-creator-chip__x",
                onclick: move |_| on_remove.call(user_id.clone()),
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
