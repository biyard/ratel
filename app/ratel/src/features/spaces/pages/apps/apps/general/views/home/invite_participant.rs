use super::*;

#[component]
pub fn InviteParticipant() -> Element {
    let space = use_space();
    let tr: GeneralTranslate = use_translate();

    let mut toast = use_toast();

    let mut email_input = use_signal(String::new);
    let mut invited_emails = use_signal(Vec::<String>::new);
    let mut invite_loading = use_signal(|| false);

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
                            let emails = invited_emails();
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
