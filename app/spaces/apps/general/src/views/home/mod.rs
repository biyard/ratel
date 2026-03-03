use crate::controllers::{
    delete_space, get_space_administrator, invite_space_participants,
    InviteSpaceParticipantsRequest, SpaceAdministratorResponse,
};
use crate::i18n::GeneralTranslate;
use crate::*;

const DEFAULT_PROFILE_IMAGE: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

fn normalize_email_input(raw: &str) -> Option<String> {
    let email = raw.trim().to_ascii_lowercase();
    if email.is_empty() || !email.contains('@') {
        return None;
    }
    Some(email)
}

#[component]
pub fn GeneralPage(space_id: SpacePartition) -> Element {
    let navigator = use_navigator();
    let tr: GeneralTranslate = use_translate();
    let failed_to_load_administrator = tr.failed_to_load_administrator.to_string();
    let participants_invited_successfully = tr.participants_invited_successfully.to_string();
    let failed_to_invite_participants = tr.failed_to_invite_participants.to_string();
    let space_deleted_successfully = tr.space_deleted_successfully.to_string();
    let failed_to_delete_space = tr.failed_to_delete_space.to_string();
    let space_id_for_admin = space_id.clone();
    let space_id_for_invite = space_id.clone();
    let space_id_for_delete = space_id.clone();

    let mut email_input = use_signal(String::new);
    let mut invited_emails = use_signal(Vec::<String>::new);
    let administrator = use_signal(|| Option::<SpaceAdministratorResponse>::None);
    let mut notice = use_signal(|| Option::<String>::None);
    let mut invite_loading = use_signal(|| false);
    let mut delete_loading = use_signal(|| false);
    let mut did_load_admin = use_signal(|| false);

    use_effect(move || {
        if did_load_admin() {
            return;
        }
        did_load_admin.set(true);

        let space_id = space_id_for_admin.clone();
        let mut administrator = administrator.clone();
        let mut notice = notice.clone();
        let failed_prefix = failed_to_load_administrator.clone();

        spawn(async move {
            match get_space_administrator(space_id).await {
                Ok(admin) => administrator.set(Some(admin)),
                Err(err) => notice.set(Some(format!("{}: {}", failed_prefix, err))),
            }
        });
    });

    rsx! {
        div { class: "flex overflow-visible flex-col gap-5 self-start pb-6 w-full min-w-0 shrink-0 max-w-[1024px] max-tablet:gap-4 text-font-primary",
            h3 { class: "font-bold sp-dash-font-raleway text-[24px]/[28px] tracking-[-0.24px] text-font-primary",
                {tr.space_setting}
            }

            div { class: "overflow-visible w-full shrink-0 rounded-[12px] bg-card",
                div { class: "flex justify-between items-center self-stretch px-5 py-4 border-b border-separator",
                    p { class: "font-semibold text-center sp-dash-font-raleway text-[17px]/[20px] tracking-[-0.18px] text-font-primary",
                        {tr.invite_participant}
                    }
                }

                div { class: "flex flex-col gap-5 items-start self-stretch p-5 bg-card max-mobile:p-4",
                    div { class: "flex items-start w-full gap-[10px] max-tablet:flex-col",
                        div { class: "flex flex-col flex-1 gap-2 justify-center items-start",
                            p { class: "font-semibold sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-font-primary",
                                {tr.email_address}
                            }
                            input {
                                class: "flex flex-col justify-center items-start px-3 py-2.5 w-full font-medium leading-6 border-gray-600 rounded-[8px] border-[0.5px] bg-web-input sp-dash-font-raleway text-[15px] tracking-[0.5px] text-font-primary placeholder:text-card-more-muted",
                                placeholder: tr.email_placeholder.to_string(),
                                value: email_input(),
                                oninput: move |evt| {
                                    email_input.set(evt.value().to_string());
                                },
                                onkeydown: move |evt| {
                                    if evt.key() != Key::Enter {
                                        return;
                                    }
                                    let Some(email) = normalize_email_input(&email_input()) else {
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
                                    {
                                        let value = value.clone();
                                        rsx! {
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
                            }
                        }

                        div { class: "flex flex-col flex-1 gap-2 justify-center items-start w-full",
                            p { class: "font-semibold sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-font-primary",
                                {tr.default_reward}
                            }
                            RewardRoleCard {
                                title: tr.participant.to_string(),
                                description: tr.can_participate_in_this_space.to_string(),
                            }
                        }
                    }

                    div { class: "flex justify-end w-full max-tablet:justify-stretch",
                        button {
                            class: "flex flex-col justify-center items-center self-stretch w-fit max-tablet:w-full gap-[10px] rounded-full px-5 py-3 border bg-btn-primary-bg border-btn-primary-outline text-web-font-ab-bk hover:bg-btn-primary-hover-bg hover:border-btn-primary-hover-outline hover:text-btn-primary-hover-text disabled:bg-btn-primary-disable-bg disabled:border-btn-primary-disable-outline disabled:text-btn-primary-disable-text sp-dash-font-raleway text-[14px]/[16px] font-bold",
                            disabled: invite_loading(),
                            onclick: move |_| {
                                if invite_loading() {
                                    return;
                                }

                                let emails = invited_emails();
                                if emails.is_empty() {
                                    return;
                                }

                                invite_loading.set(true);
                                notice.set(None);

                                let mut invite_loading = invite_loading.clone();
                                let mut invited_emails = invited_emails.clone();
                                let mut notice = notice.clone();
                                let space_id = space_id_for_invite.clone();
                                let success_text =
                                    participants_invited_successfully.clone();
                                let failed_prefix = failed_to_invite_participants.clone();

                                spawn(async move {
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
                                            notice.set(Some(success_text));
                                        }
                                        Err(err) => {
                                            notice.set(Some(format!("{}: {}", failed_prefix, err)));
                                        }
                                    }
                                });
                            },
                            if invite_loading() {
                                {tr.inviting}
                            } else {
                                {tr.invite}
                            }
                        }
                    }

                    if let Some(message) = notice() {
                        p { class: "w-full text-sm text-card-meta", "{message}" }
                    }
                }
            }

            div { class: "overflow-visible w-full shrink-0 rounded-[12px] bg-card",
                div { class: "flex justify-between items-center self-stretch px-5 py-4 border-b border-separator",
                    p { class: "font-bold sp-dash-font-raleway text-[24px]/[28px] tracking-[-0.24px] text-font-primary",
                        {tr.administrator}
                    }
                }

                div { class: "flex flex-col items-start self-stretch p-5 gap-[10px] bg-card max-mobile:p-4",
                    if let Some(admin) = administrator() {
                        AdministratorRow {
                            name: admin.display_name,
                            username: admin.username,
                            profile_url: admin.profile_url,
                        }
                    } else {
                        p { class: "font-medium leading-6 sp-dash-font-raleway text-[15px] tracking-[0.5px] text-card-meta",
                            {tr.administrator_not_found}
                        }
                    }
                }
            }

            div { class: "flex justify-end pt-5 w-full max-tablet:justify-stretch",
                button {
                    class: "flex flex-col justify-center items-center px-5 py-3 font-bold bg-transparent rounded-full border w-fit max-tablet:w-full gap-[10px] border-web-error text-web-error hover:bg-transparent hover:border-web-error hover:text-web-error disabled:border-web-error/40 disabled:text-web-error/40 sp-dash-font-raleway text-[14px]/[16px]",
                    disabled: delete_loading(),
                    onclick: move |_| {
                        if delete_loading() {
                            return;
                        }

                        delete_loading.set(true);
                        notice.set(None);

                        let mut delete_loading = delete_loading.clone();
                        let mut notice = notice.clone();
                        let space_id = space_id_for_delete.clone();
                        let navigator = navigator.clone();
                        let success_text = space_deleted_successfully.clone();
                        let failed_prefix = failed_to_delete_space.clone();

                        spawn(async move {
                            let result = delete_space(space_id).await;
                            delete_loading.set(false);

                            match result {
                                Ok(_) => {
                                    notice.set(Some(success_text));
                                    navigator.push("/");
                                }
                                Err(err) => {
                                    notice.set(Some(format!("{}: {}", failed_prefix, err)));
                                }
                            }
                        });
                    },
                    if delete_loading() {
                        {tr.deleting}
                    } else {
                        {tr.delete_space}
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
                "×"
            }
        }
    }
}

#[component]
fn RewardRoleCard(title: String, description: String) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1 items-start px-4 w-full text-left border h-[108px] shrink-0 rounded-[12px] border-btn-primary-outline bg-btn-primary-bg/5 py-[17px]",
            p { class: "font-semibold sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-font-primary",
                "{title}"
            }
            p { class: "font-normal leading-6 sp-dash-font-raleway text-[15px] tracking-[0.5px] text-card-meta",
                "{description}"
            }
        }
    }
}

#[component]
fn AdministratorRow(name: String, username: String, profile_url: String) -> Element {
    let profile = if profile_url.trim().is_empty() {
        DEFAULT_PROFILE_IMAGE.to_string()
    } else {
        profile_url
    };

    rsx! {
        div { class: "flex justify-between items-center px-4 py-3 w-full border rounded-[12px] border-separator bg-card max-tablet:flex-col max-tablet:items-start max-tablet:gap-3",
            div { class: "flex items-center gap-[10px]",
                img {
                    src: "{profile}",
                    alt: "{name}",
                    class: "object-cover w-12 h-12 rounded-full shrink-0",
                }

                div { class: "flex flex-col gap-1 items-start",
                    div { class: "flex gap-1 items-center",
                        p { class: "font-bold leading-5 sp-dash-font-raleway text-[17px] tracking-[-0.18px] text-font-primary",
                            "{name}"
                        }
                        icons::shapes::Badge2 { width: "18", height: "18", class: "" }
                    }
                    p { class: "font-semibold leading-4 sp-dash-font-raleway text-[13px] tracking-[-0.14px] text-web-font-neutral",
                        "@{username}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn AppGeneralPage(space_id: SpacePartition) -> Element {
    let tr: GeneralTranslate = use_translate();
    let role =
        use_loader(move || async move { Ok::<SpaceUserRole, Error>(SpaceUserRole::Creator) })?;

    if role() == SpaceUserRole::Creator {
        rsx! {
            GeneralPage { space_id }
        }
    } else {
        rsx! {
            div { class: "flex justify-center items-center w-full h-full text-font-primary",
                {tr.no_permission}
            }
        }
    }
}
