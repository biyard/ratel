use super::super::dto::*;
use super::super::*;

#[derive(Clone)]
pub struct RemovePayload {
    pub member_id: String,
    pub group_id: String,
}

#[derive(Clone)]
pub struct ChangeGroupPayload {
    pub member_id: String,
    pub from_group_id: String,
    pub to_group_id: String,
}

pub fn render_member(
    member: TeamMemberResponse,
    all_groups: Vec<(String, String)>,
    on_remove_from_group: EventHandler<RemovePayload>,
    on_change_group: EventHandler<ChangeGroupPayload>,
    removing: Signal<Option<String>>,
    changing: Signal<Option<String>>,
) -> Element {
    let member_id = member.user_id.clone();
    let profile_url = member.profile_url.clone();
    let is_owner = member.is_owner;

    let group_chips = member
        .groups
        .into_iter()
        .filter(|group| !is_blocked_text(&group.group_name))
        .map({
            let on_remove_from_group = on_remove_from_group.clone();
            let on_change_group = on_change_group.clone();
            let removing = removing.clone();
            let changing = changing.clone();
            let member_id = member_id.clone();
            let all_groups = all_groups.clone();
            move |group| {
                let remove_key = format!("remove:{}-{}", member_id, group.group_id);
                let change_key = format!("change:{}-{}", member_id, group.group_id);
                let is_removing = removing().as_ref() == Some(&remove_key);
                let is_changing = changing().as_ref() == Some(&change_key);
                let is_busy = is_removing || is_changing;

                let remove_payload = RemovePayload {
                    member_id: member_id.clone(),
                    group_id: group.group_id.clone(),
                };

                let from_group_id = group.group_id.clone();
                let member_id_for_change = member_id.clone();
                let on_change_group = on_change_group.clone();
                let all_groups = all_groups.clone();

                rsx! {
                    div {
                        key: "{group.group_id}",
                        class: "flex flex-row w-fit h-fit items-center gap-1 px-[8px] py-[4px] border border-border bg-card-bg rounded-lg font-medium text-sm text-text-primary",
                        if !is_owner && all_groups.len() > 1 {
                            select {
                                class: "bg-transparent text-sm font-medium text-text-primary border-none outline-none cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed",
                                aria_label: "Change group for {group.group_name}",
                                disabled: is_busy,
                                value: "{group.group_id}",
                                onchange: move |e| {
                                    let to_group_id = e.value();
                                    if to_group_id != from_group_id {
                                        on_change_group.call(ChangeGroupPayload {
                                            member_id: member_id_for_change.clone(),
                                            from_group_id: from_group_id.clone(),
                                            to_group_id,
                                        });
                                    }
                                },
                                for (gid, gname) in all_groups.iter() {
                                    option { value: "{gid}", "{gname}" }
                                }
                            }
                        } else {
                            span { {group.group_name} }
                        }
                        if !is_owner {
                            button {
                                class: "ml-1 hover:bg-background rounded-full p-0.5 transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                                onclick: move |_| on_remove_from_group.call(remove_payload.clone()),
                                disabled: is_busy,
                                title: "Remove from group",
                                span { class: "text-xs leading-none", "×" }
                            }
                        }
                    }
                }
            }
        });

    let avatar = if is_empty_or_test(&profile_url) {
        rsx! {
            div { class: "w-12 h-12 rounded-full bg-profile-bg" }
        }
    } else {
        rsx! {
            img {
                src: "{profile_url}",
                alt: "{member.username}",
                width: "48",
                height: "48",
                class: "rounded-lg object-cover w-12 h-12",
            }
        }
    };

    rsx! {
        div {
            key: "{member_id}",
            class: "flex flex-col w-full h-fit gap-[15px] bg-transparent rounded-sm border border-card-border p-5",
            div { class: "flex flex-row w-full h-fit gap-[15px] bg-transparent",
                {avatar}
                div { class: "flex flex-col justify-between items-start flex-1 min-w-0",
                    div { class: "font-bold text-text-primary text-base/[20px]", {member.username} }
                    div { class: "font-semibold text-desc-text text-sm/[20px]", {member.display_name} }
                    if member.is_owner {
                        div { class: "text-xs text-blue-500 font-medium", "Team owner" }
                    }
                }
            }

            div { class: "flex flex-wrap w-full justify-start items-center gap-[10px]",
                for chip in group_chips {
                    {chip}
                }
            }
        }
    }
}

pub fn is_blocked_text(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("test") || value.contains("테스트")
}

fn is_empty_or_test(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.is_empty() || is_blocked_text(trimmed)
}
