use super::super::controllers::{add_team_member_handler, find_user_handler, FindUserQueryType};
use super::super::dto::{AddTeamMemberRequest, FoundUserResponse};
use super::super::*;

use icons::validations;

#[derive(Clone)]
pub struct InviteResult {
    pub role: String,
    pub total_added: i64,
    pub failed_pks: Vec<String>,
}

#[component]
pub fn InviteMemberModal(
    team_pk: TeamPartition,
    username: String,
    on_close: EventHandler<()>,
    on_invited: EventHandler<InviteResult>,
) -> Element {
    let _username = username;
    let tr: InviteMemberTranslate = use_translate();
    let mut role_index = use_signal(|| 0usize);
    let mut selected_users = use_signal(Vec::<FoundUserResponse>::new);
    let mut search_value = use_signal(String::new);
    let mut is_searching = use_signal(|| false);
    let mut is_submitting = use_signal(|| false);
    let mut message = use_signal(|| Option::<String>::None);

    // Signal<String>으로 감싸면 async 블록에서 Copy로 캡처 가능
    let user_not_found = use_signal(|| tr.user_not_found.to_string());
    let failed_invite = use_signal(|| tr.failed_invite.to_string());

    let roles = vec![
        (tr.group_admin.to_string(), "admin".to_string()),
        (tr.group_member.to_string(), "member".to_string()),
    ];
    let roles_for_invite = roles.clone();

    // Signal은 Copy이므로 클론 없이 바로 캡처
    let on_search = move || {
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
                        let exists = selected_users.read().iter().any(|u| u.pk == user.pk);
                        if !exists {
                            selected_users.write().push(user);
                        } else {
                            message.set(Some(format!("{} is already added", user.nickname)));
                        }
                    }
                    Err(_) => {
                        message.set(Some(user_not_found()));
                    }
                }
            }
            search_value.set(String::new());
        });
    };

    let on_invite = move |_| {
        let role = roles_for_invite
            .get(role_index())
            .map(|(_, id)| id.clone())
            .unwrap_or_else(|| "member".to_string());

        if selected_users.read().is_empty() {
            message.set(Some(failed_invite()));
            return;
        }

        let user_pks: Vec<String> = selected_users.read().iter().map(|u| u.pk.clone()).collect();
        let team_pk = team_pk.clone();

        spawn(async move {
            is_submitting.set(true);
            let result = add_team_member_handler(
                team_pk,
                AddTeamMemberRequest {
                    user_pks,
                    role: role.clone(),
                },
            )
            .await;
            is_submitting.set(false);
            match result {
                Ok(resp) => {
                    on_invited.call(InviteResult {
                        role: role.clone(),
                        total_added: resp.total_added,
                        failed_pks: resp.failed_pks,
                    });
                    on_close.call(());
                }
                Err(_) => {
                    message.set(Some(failed_invite()));
                }
            }
        });
    };

    rsx! {
        div { class: "flex flex-col w-tablet min-h-[400px] max-w-[calc(100vw-90px)] min-w-[400px] max-mobile:w-full! max-mobile:max-w-full! max-mobile:min-w-0! gap-5",
            div { class: "flex flex-col w-full gap-[10px]",
                div { class: "font-bold text-[15px]/[28px] text-modal-label-text", {tr.select_group} }
                select {
                    class: "w-full px-4 py-3 rounded-[8px] border border-input-box-border bg-input-box-bg text-text-primary",
                    value: role_index(),
                    onchange: move |e| {
                        if let Ok(next) = e.value().parse::<usize>() {
                            role_index.set(next);
                        }
                    },
                    for (idx, (name, _id)) in roles.iter().enumerate() {
                        option { value: idx, "{name}" }
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
                    for user in selected_users.read().clone() {
                        div { class: "flex flex-row w-fit gap-1 justify-start items-center bg-primary rounded-[100px] px-[12px] py-[2px]",
                            div { class: "font-medium text-neutral-900 text-[15px]/[24px]",
                                {user.nickname}
                            }
                            div {
                                class: "w-fit h-fit",
                                onclick: {
                                    let pk = user.pk.clone();
                                    move |_| selected_users.write().retain(|u| u.pk != pk)
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
                    disabled: selected_users.read().is_empty(),
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
