use super::*;
use crate::common::hooks::use_infinite_query;
use crate::spaces::InvitationStatus;

const INVITATION_PAGE_SIZE: i32 = 20;

#[component]
pub fn InviteParticipant() -> Element {
    let space = use_space();
    let tr: GeneralTranslate = use_translate();

    let mut toast = use_toast();

    let mut email_input = use_signal(String::new);
    let mut invited_emails = use_signal(Vec::<String>::new);
    let mut invite_loading = use_signal(|| false);
    let mut invitations_query = use_infinite_query(move |bookmark| {
        list_space_invitations(space().id, bookmark, Some(INVITATION_PAGE_SIZE))
    })?;
    let invitation_items = invitations_query.items();
    let more_element = invitations_query.more_element();

    rsx! {
        Card {
            div { class: "flex justify-between items-center self-stretch py-4 px-5 border-b border-separator",
                p { class: "font-semibold text-center font-raleway text-[17px]/[20px] tracking-[-0.18px] text-web-font-primary",
                    {tr.invite_participant}
                }
            }

            div { class: "flex flex-col gap-5 items-start self-stretch p-5 bg-card max-mobile:p-4",
                div {
                    class: "flex items-start w-full gap-[10px] max-tablet:flex-col",
                    div { class: "flex flex-col flex-1 gap-2 justify-center items-start",
                        p { class: "font-semibold font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-primary",
                            {tr.email_address}
                        }
                        Input {
                            id: "email-input",
                            r#type: InputType::Email,
                            placeholder: tr.email_placeholder,
                            value: email_input(),
                            oninput: move |evt: FormEvent| {
                                email_input.set(evt.value());
                            },
                            onchange: move |evt: FormEvent| {
                                email_input.set(evt.value());
                            },
                            onconfirm: move |evt: KeyboardEvent| {
                                evt.stop_propagation();
                                let Ok(email) = normalize_email_input(&email_input()) else {
                                    toast.error(Error::InvalidEmail);
                                    return;
                                };
                                invited_emails
                                    .with_mut(|emails| {
                                        if !emails.iter().any(|v| v == &email) {
                                            emails.push(email);
                                        }
                                    });
                                email_input.set(String::new());
                            },
                        }

                        div { class: "flex flex-wrap gap-2 items-center w-full",
                            for (idx , value) in invited_emails().iter().enumerate() {
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
                
                // div { class: "flex flex-col flex-1 gap-2 justify-center items-start w-full",
                //     p { class: "font-semibold font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-primary",
                //         {tr.default_reward}
                //     }
                //     RewardRoleCard {
                //         title: tr.participant.to_string(),
                //         description: tr.can_participate_in_this_space.to_string(),
                //     }
                // }
                }

                div { class: "flex justify-end w-full max-tablet:justify-stretch",
                    Button {
                        class: "self-stretch w-fit max-tablet:w-full",
                        style: ButtonStyle::Primary,
                        disabled: invite_loading(),
                        onclick: move |_| async move {
                            if invite_loading() {
                                return;
                            }
                            let mut emails = invited_emails();
                            let current_input = email_input();
                            if !current_input.trim().is_empty() {
                                let Ok(email) = normalize_email_input(&current_input) else {
                                    toast.error(Error::InvalidEmail);
                                    return;
                                };
                                if !emails.iter().any(|value| value == &email) {
                                    emails.push(email);
                                }
                            }
                            if emails.is_empty() {
                                return;
                            }
                            invite_loading.set(true);
                            let space_id = space().id;
                            let result = invite_space_participants(
                                    space_id,
                                    InviteSpaceParticipantsRequest {
                                        emails,
                                    },
                                )
                                .await;
                            invite_loading.set(false);

                            match result {
                                Ok(_) => {
                                    invited_emails.set(vec![]);
                                    email_input.set(String::new());
                                    invitations_query.restart();
                                    toast.info(tr.participants_invited_successfully);
                                }
                                Err(err) => {
                                    toast.error(err);
                                }
                            }
                            debug!("Invited participants with emails: {:?}", invited_emails());
                        },
                        if invite_loading() {
                            {tr.inviting}
                        } else {
                            {tr.invite}
                        }
                    }
                }

                div { class: "flex flex-col gap-3 w-full",
                    div { class: "flex items-center justify-between",
                        p { class: "font-semibold font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-primary",
                            {tr.invited_accounts}
                        }
                    }

                    div { class: "overflow-hidden w-full rounded-[12px] border border-separator bg-background",
                        div { class: "grid grid-cols-[minmax(0,0.95fr)_minmax(0,1.05fr)_110px_72px] gap-3 px-4 py-3 border-b border-separator bg-card max-tablet:hidden",
                            p { class: "text-[13px] font-semibold text-text-secondary",
                                {tr.participant_name}
                            }
                            p { class: "text-[13px] font-semibold text-text-secondary",
                                {tr.invitation_email}
                            }
                            p { class: "text-[13px] font-semibold text-text-secondary text-center",
                                {tr.invitation_status}
                            }
                            p { class: "text-[13px] font-semibold text-text-secondary text-center",
                                {tr.invitation_actions}
                            }
                        }

                        if invitation_items.is_empty() && invitations_query.is_loading() {
                            div { class: "px-4 py-8 text-sm text-center text-text-secondary",
                                {tr.loading_invitations}
                            }
                        } else if invitation_items.is_empty() {
                            div { class: "px-4 py-8 text-sm text-center text-text-secondary",
                                {tr.no_invited_accounts}
                            }
                        } else {
                            div { class: "flex flex-col w-full",
                                for item in invitation_items {
                                    InvitationMemberRow {
                                        key: "{item.user_id}",
                                        space_id: space().id,
                                        item,
                                        on_deleted: move |_| {
                                            invitations_query.restart();
                                        },
                                    }
                                }
                                div { class: "px-4 py-2", {more_element} }
                            }
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
            span { class: "font-medium leading-6 font-raleway text-[15px] tracking-[0.5px] text-btn-primary-text",
                "{value}"
            }
            Button {
                class: "rounded-full disabled:opacity-50 size-5 !p-0 hover:!bg-transparent",
                size: ButtonSize::Icon,
                style: ButtonStyle::Text,
                onclick: move |evt| on_remove.call(evt),
                icons::ratel::XMarkIcon {
                    width: "12",
                    height: "12",
                    class: "w-3 h-3 [&>path]:stroke-icon-primary",
                }
            }
        }
    }
}

#[component]
fn RewardRoleCard(title: String, description: String) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1 items-start px-4 w-full text-left border h-[108px] shrink-0 rounded-[12px] border-btn-primary-outline bg-btn-primary-bg/5 py-[17px]",
            p { class: "font-semibold font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-primary",
                "{title}"
            }
            p { class: "font-normal leading-6 font-raleway text-[15px] tracking-[0.5px] text-card-meta",
                "{description}"
            }
        }
    }
}

#[component]
fn InvitationMemberRow(
    space_id: SpacePartition,
    item: SpaceInvitationListItem,
    on_deleted: EventHandler<()>,
) -> Element {
    let tr: GeneralTranslate = use_translate();
    let mut toast = use_toast();
    let mut show_action_menu = use_signal(|| false);
    let mut deleting = use_signal(|| false);
    let user_id = item.user_id.clone();

    let status_icon = match item.status {
        InvitationStatus::Accepted => rsx! {
            icons::ratel::CheckIcon {
                width: "16",
                height: "16",
                class: "w-4 h-4 text-green-600",
            }
        },
        InvitationStatus::Declined => rsx! {
            icons::ratel::XMarkIcon {
                width: "16",
                height: "16",
                class: "w-4 h-4 text-red-500",
            }
        },

        InvitationStatus::Pending | InvitationStatus::Invited => rsx! {},
    };

    rsx! {
        div { class: "grid relative grid-cols-[minmax(0,0.95fr)_minmax(0,1.05fr)_110px_72px] gap-3 items-center px-4 py-3 border-b border-separator last:border-b-0 max-tablet:flex max-tablet:flex-col max-tablet:items-start max-tablet:gap-2",
            div { class: "flex gap-3 items-center min-w-0",
                img {
                    class: "object-cover rounded-full size-8 shrink-0",
                    src: if item.profile_url.is_empty() { DEFAULT_PROFILE_IMAGE.to_string() } else { item.profile_url.clone() },
                }
                div { class: "flex flex-col min-w-0",
                    p { class: "truncate text-[14px] font-medium text-web-font-primary",
                        "{item.display_name}"
                    }
                    p { class: "truncate text-[12px] text-text-secondary", "@{item.username}" }
                }
            }

            div { class: "min-w-0 max-tablet:w-full",
                p { class: "truncate text-[14px] text-web-font-primary", "{item.email}" }
            }

            div { class: "flex justify-center items-center h-full max-tablet:justify-start",
                {status_icon}
            }

            div { class: "flex relative justify-center items-center h-full max-tablet:self-end",
                Button {
                    size: ButtonSize::Icon,
                    style: ButtonStyle::Text,
                    class: "flex justify-center items-center rounded-full size-8 !p-0",
                    onclick: move |_| {
                        show_action_menu.set(!show_action_menu());
                    },
                    lucide_dioxus::EllipsisVertical { class: "w-4 h-4 text-icon-primary [&>*]:stroke-current" }
                }

                if show_action_menu() {
                    div { class: "absolute top-9 right-0 z-10 min-w-[120px] rounded-[10px] border border-separator bg-card shadow-lg",
                        Button {
                            style: ButtonStyle::Text,
                            class: "justify-start w-full !px-3 !py-2 text-[14px] !text-web-error hover:!bg-card/80",
                            disabled: deleting(),
                            onclick: move |_| {
                                let user_id = user_id.clone();
                                let space_id = space_id.clone();
                                async move {
                                    if deleting() {
                                        return;
                                    }
                                    deleting.set(true);
                                    let result = delete_space_invitation(space_id, user_id).await;
                                    deleting.set(false);
                                    show_action_menu.set(false);

                                    match result {
                                        Ok(_) => {
                                            toast.info(tr.invitation_deleted_successfully);
                                            on_deleted.call(());
                                        }
                                        Err(err) => {
                                            toast.error(err);
                                        }
                                    }
                                }
                            },
                            {tr.invitation_delete}
                        }
                    }
                }
            }
        }
    }
}
