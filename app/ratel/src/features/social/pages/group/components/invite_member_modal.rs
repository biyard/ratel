use super::super::controllers::*;
use super::super::controllers::{FindUserQueryType, add_member_handler, find_user_handler};
use super::super::dto::*;
use super::super::*;

use icons::validations;

#[derive(Clone)]
pub struct InviteResult {
    pub group_id: String,
    pub total_added: i64,
    pub failed_pks: Vec<String>,
}

#[component]
pub fn InviteMemberModal(
    team_pk: TeamPartition,
    username: String,
    groups: Vec<TeamGroupResponse>,
    on_close: EventHandler<()>,
    on_invited: EventHandler<InviteResult>,
) -> Element {
    let _username = username;
    let tr: TeamGroupTranslate = use_translate();
    let mut group_index = use_signal(|| 0usize);
    let mut selected_users = use_signal(Vec::<FoundUserResponse>::new);
    let mut search_value = use_signal(String::new);
    let mut is_searching = use_signal(|| false);
    let mut is_submitting = use_signal(|| false);
    let mut message = use_signal(|| Option::<String>::None);

    let groups = groups;

    let on_search = {
        let selected_users = selected_users.clone();
        let is_searching = is_searching.clone();
        let message = message.clone();
        let search_value = search_value.clone();
        let user_not_found = tr.user_not_found.to_string();
        move || {
            let mut selected_users = selected_users.clone();
            let mut is_searching = is_searching.clone();
            let mut message = message.clone();
            let mut search_value = search_value.clone();
            let user_not_found = user_not_found.clone();
            let input = search_value();
            if input.trim().is_empty() {
                return;
            }
            let identifiers: Vec<String> = input
                .split(',')
                .map(|v| v.trim().to_string())
                .filter(|v| !v.is_empty())
                .collect();
            if identifiers.is_empty() {
                return;
            }
            spawn(async move {
                for identifier in identifiers {
                    is_searching.set(true);
                    let query_type = detect_query_type(&identifier);
                    let result = find_user_handler(query_type, identifier.clone()).await;
                    is_searching.set(false);

                    match result {
                        Ok(user) => {
                            let exists = selected_users().iter().any(|u| u.pk == user.pk);
                            if !exists {
                                let mut next = selected_users();
                                next.push(user);
                                selected_users.set(next);
                            } else {
                                message.set(Some(format!("{} is already added", user.nickname)));
                            }
                        }
                        Err(_) => {
                            message.set(Some(user_not_found.clone()));
                        }
                    }
                }
                search_value.set(String::new());
            });
        }
    };

    let on_invite = {
        let mut is_submitting = is_submitting.clone();
        let mut message = message.clone();
        let selected_users = selected_users.clone();
        let team_pk = team_pk.clone();
        let on_invited = on_invited.clone();
        let on_close = on_close.clone();
        let failed_invite = tr.failed_invite.to_string();
        let groups_for_invite = groups.clone();
        move |_| {
            let team_pk = team_pk.clone();
            let group_id = groups_for_invite
                .get(group_index())
                .map(|g| g.id.clone())
                .unwrap_or_default();
            let on_invited = on_invited.clone();
            let on_close = on_close.clone();
            let failed_invite = failed_invite.clone();
            let mut is_submitting = is_submitting.clone();
            let mut message = message.clone();
            if selected_users().is_empty() {
                message.set(Some(failed_invite.clone()));
                return;
            }
            if group_id.is_empty() {
                message.set(Some(failed_invite.clone()));
                return;
            }
            let user_pks: Vec<String> = selected_users().iter().map(|u| u.pk.clone()).collect();
            spawn(async move {
                is_submitting.set(true);
                let result = add_member_handler(
                    team_pk.clone(),
                    group_id.clone(),
                    AddMemberRequest { user_pks },
                )
                .await;
                is_submitting.set(false);
                match result {
                    Ok(resp) => {
                        on_invited.call(InviteResult {
                            group_id: group_id.clone(),
                            total_added: resp.total_added,
                            failed_pks: resp.failed_pks.clone(),
                        });
                        on_close.call(());
                    }
                    Err(_) => {
                        message.set(Some(failed_invite.clone()));
                    }
                }
            });
        }
    };

    rsx! {
        div { class: "flex flex-col w-tablet min-h-[400px] max-w-tablet min-w-[400px] max-mobile:w-full! max-mobile:max-w-full! gap-5",
            div { class: "flex flex-col w-full gap-[10px]",
                div { class: "font-bold text-[15px]/[28px] text-modal-label-text", {tr.select_group} }
                select {
                    class: "w-full px-4 py-3 rounded-[8px] border border-input-box-border bg-input-box-bg text-text-primary",
                    value: group_index(),
                    onchange: move |e| {
                        if let Ok(next) = e.value().parse::<usize>() {
                            group_index.set(next);
                        }
                    },
                    for (idx, group) in groups.iter().enumerate() {
                        option { value: idx, "{group.name}" }
                    }
                }
            }

            div { class: "flex flex-col w-full",
                div { class: "font-bold text-[15px]/[28px] text-modal-label-text", {tr.email_label} }
                div { class: "mt-2.5",
                    Input {
                        value: search_value(),
                        placeholder: tr.email_hint,
                        oninput: move |e: FormEvent| search_value.set(e.value()),
                        onconfirm: move |_| on_search(),
                    }
                }
                if is_searching() {
                    div { class: "text-sm text-foreground-muted mt-2", {tr.searching} }
                }
            }

            div { class: "flex flex-col w-full gap-[10px]",
                div { class: "flex flex-wrap gap-1",
                    for user in selected_users() {
                        div { class: "flex flex-row w-fit gap-1 justify-start items-center bg-primary rounded-[100px] px-[12px] py-[2px]",
                            div { class: "font-medium text-neutral-900 text-[15px]/[24px]",
                                {user.nickname}
                            }
                            div {
                                class: "w-fit h-fit",
                                onclick: {
                                    let pk = user.pk.clone();
                                    move |_| {
                                        let mut next = selected_users();
                                        next.retain(|u| u.pk != pk);
                                        selected_users.set(next);
                                    }
                                },
                                validations::Clear {
                                    width: "24",
                                    height: "24",
                                    class: "w-6 h-6 cursor-pointer [&>path]:stroke-neutral-800",
                                }
                            }
                        }
                    }
                }
            }

            if let Some(msg) = message() {
                div { class: "text-sm text-post-required-marker", {msg} }
            }

            div { class: "flex flex-col w-full",
                Button {
                    style: ButtonStyle::Primary,
                    shape: ButtonShape::Square,
                    class: "w-full my-[15px] py-[15px]",
                    disabled: selected_users().is_empty(),
                    loading: is_submitting,
                    onclick: on_invite,
                    {tr.send}
                }
            }
        }
    }
}

fn detect_query_type(value: &str) -> FindUserQueryType {
    if value.contains('@') {
        return FindUserQueryType::Email;
    }
    let is_phone = value.chars().all(|c| c.is_ascii_digit() || c == '+');
    if is_phone && value.len() >= 8 {
        return FindUserQueryType::PhoneNumber;
    }
    FindUserQueryType::Username
}
