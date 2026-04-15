use super::super::dto::*;
use super::super::*;

#[derive(Clone)]
pub struct RemovePayload {
    pub member_id: String,
}

pub fn render_member(
    member: TeamMemberResponse,
    on_remove: EventHandler<RemovePayload>,
    removing: Signal<Option<String>>,
) -> Element {
    let member_id = member.user_id.clone();
    let profile_url = member.profile_url.clone();
    let is_owner = member.is_owner;
    let role = member.role.clone();

    let remove_key = format!("remove:{}", member_id);
    let is_removing = removing().as_ref() == Some(&remove_key);

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

    let role_class = match role {
        TeamRole::Owner => "bg-primary/20 text-primary border-primary/30",
        TeamRole::Admin => "bg-primary/10 text-primary border-primary/20",
        TeamRole::Member => "bg-card-bg text-text-primary border-border",
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
                    if is_owner {
                        div { class: "text-xs text-blue-500 font-medium", "Team owner" }
                    }
                }
                div { class: "flex flex-row items-center gap-2 ml-auto",
                    div { class: "flex items-center px-[8px] py-[4px] border rounded-lg font-medium text-sm {role_class}",
                        "{role}"
                    }
                    if !is_owner {
                        Button {
                            style: ButtonStyle::Text,
                            size: ButtonSize::Icon,
                            disabled: is_removing,
                            onclick: move |_| {
                                on_remove
                                    .call(RemovePayload {
                                        member_id: member_id.clone(),
                                    })
                            },
                            lucide_dioxus::X { class: "w-4 h-4 [&>path]:stroke-foreground-muted" }
                        }
                    }
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
