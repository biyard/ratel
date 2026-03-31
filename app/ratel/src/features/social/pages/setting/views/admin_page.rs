use super::super::components::DeleteTeamPopup;
use super::super::controllers::TeamResponse;
use super::super::controllers::{delete_team_handler, update_team_handler, UpdateTeamRequest};
use super::super::layout::SettingsSaveContext;
use super::super::*;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};
use dioxus::prelude::*;

fn format_last_saved(ts_millis: i64) -> String {
    if ts_millis == 0 {
        return String::new();
    }
    use chrono::{TimeZone, Utc};
    let dt = match Utc.timestamp_millis_opt(ts_millis).single() {
        Some(dt) => dt,
        None => return String::new(),
    };

    #[cfg(not(feature = "server"))]
    {
        // Get browser's local timezone offset (minutes) and apply it
        let offset_minutes = js_sys::Date::new_0().get_timezone_offset() as i64;
        let local_dt = dt - chrono::Duration::minutes(offset_minutes);
        local_dt.format("%Y-%m-%d %H:%M").to_string()
    }

    #[cfg(feature = "server")]
    {
        dt.format("%Y-%m-%d %H:%M UTC").to_string()
    }
}

#[component]
pub fn AdminPage(username: ReadSignal<String>, team: TeamResponse) -> Element {
    let tr: TeamSettingsTranslate = use_translate();
    let mut popup = use_popup();
    let navigator = use_navigator();
    let mut team_ctx = use_team_context();

    let mut toast = use_toast();
    let mut save_ctx = use_context::<SettingsSaveContext>();
    let mut is_saving = save_ctx.is_saving;

    let mut team_state = use_signal(|| team);
    let mut message = use_signal(|| Option::<String>::None);

    let mut thumbnail_url = use_signal(|| team_state().thumbnail_url.clone().unwrap_or_default());
    let mut profile_url = use_signal(|| team_state().profile_url.clone().unwrap_or_default());
    let mut nickname = use_signal(|| team_state().nickname.clone());
    let mut html_contents = use_signal(|| team_state().html_contents.clone());
    let mut allow_invite = use_signal(|| team_state().allow_invite);
    let mut allow_create_space = use_signal(|| team_state().allow_create_space);

    // Translation strings captured before use_effect
    let validation_nickname_required = tr.validation_nickname_required;
    let success_update_team = tr.success_update_team;
    let failed_update_team = tr.failed_update_team;

    // Wire header Save button → execute save
    use_effect(move || {
        let trigger = (save_ctx.save_trigger)();
        if trigger == 0 {
            return;
        }
        // Reset trigger immediately to prevent re-firing on other signal changes
        save_ctx.save_trigger.set(0);
        let display_name = nickname().trim().to_string();
        let description = html_contents().trim().to_string();
        let username = username();

        if display_name.is_empty() {
            message.set(Some(validation_nickname_required.to_string()));
            return;
        }

        spawn(async move {
            is_saving.set(true);
            message.set(None);

            let result = update_team_handler(
                username,
                UpdateTeamRequest {
                    nickname: Some(display_name),
                    description: Some(description),
                    profile_url: Some(profile_url()),
                    dao_address: None,
                    thumbnail_url: Some(thumbnail_url()),
                    allow_invite: Some(allow_invite()),
                    allow_create_space: Some(allow_create_space()),
                },
            )
            .await;

            is_saving.set(false);
            match result {
                Ok(updated) => {
                    team_state.set(updated);
                    toast.info(success_update_team.to_string());
                }
                Err(err) => {
                    message.set(Some(format!("{}: {}", failed_update_team, err)));
                }
            }
        });
    });

    let mut on_open_delete = move |_evt: MouseEvent| {
        let on_cancel = move |_evt: MouseEvent| {
            popup.close();
        };
        let on_confirm = move |_evt: MouseEvent| async move {
            match delete_team_handler(username()).await {
                Ok(_response) => {
                    team_ctx.teams.with_mut(|teams| {
                        teams.retain(|t| t.username != username());
                    });

                    debug!("Team delete: {:?}", team_ctx.teams());
                    navigator.push(Route::Index {});
                }
                Err(e) => {
                    toast.error(e);
                }
            }
            popup.close();
        };
        popup.open(rsx! {
            DeleteTeamPopup { on_confirm, on_cancel }
        });
    };

    let permissions: TeamGroupPermissions = team_state().permissions.unwrap_or(0).into();
    let delete_team_permission = permissions.contains(TeamGroupPermission::TeamAdmin);
    let last_saved = format_last_saved(team_state().updated_at);

    rsx! {
        div { class: "flex flex-col gap-8 w-full",
            // Thumbnail (banner)
            div { class: "flex flex-col gap-2",
                span { class: "text-sm font-semibold text-text-primary", "{tr.thumbnail}" }
                FileUploader {
                    on_upload_success: move |url: String| thumbnail_url.set(url),
                    accept: "image/*",
                    class: "w-full",
                    if !thumbnail_url().is_empty() {
                        img {
                            src: "{thumbnail_url()}",
                            alt: "Thumbnail",
                            class: "object-cover w-full h-40 cursor-pointer rounded-[10px]",
                        }
                    } else {
                        div { class: "flex flex-col gap-2 justify-center items-center w-full h-40 border-2 border-dashed transition-colors cursor-pointer rounded-[10px] border-border bg-card-bg hover:bg-white/5",
                            lucide_dioxus::ImagePlus { class: "w-6 h-6 [&>path]:stroke-foreground-muted [&>line]:stroke-foreground-muted [&>polyline]:stroke-foreground-muted [&>circle]:stroke-foreground-muted" }
                            span { class: "text-sm text-foreground-muted", "{tr.upload_banner}" }
                        }
                    }
                }
                span { class: "text-xs text-foreground-muted", "{tr.thumbnail_hint}" }
            }

            // Team Logo
            div { class: "flex flex-col gap-2",
                span { class: "text-sm font-semibold text-text-primary", "{tr.team_logo}" }
                FileUploader {
                    on_upload_success: move |url: String| profile_url.set(url),
                    accept: "image/*",
                    if !profile_url().is_empty() {
                        img {
                            src: "{profile_url()}",
                            alt: "Team Logo",
                            class: "object-cover w-20 h-20 cursor-pointer rounded-[10px]",
                        }
                    } else {
                        div { class: "flex flex-col gap-1 justify-center items-center w-20 h-20 transition-colors cursor-pointer rounded-[10px] bg-card-bg hover:bg-white/5",
                            lucide_dioxus::ImagePlus { class: "w-5 h-5 [&>path]:stroke-foreground-muted [&>line]:stroke-foreground-muted [&>polyline]:stroke-foreground-muted [&>circle]:stroke-foreground-muted" }
                        }
                    }
                }
                span { class: "text-xs text-foreground-muted", "{tr.team_logo_hint}" }
            }

            // Team name
            div { class: "flex flex-col gap-3",
                label { class: "text-sm font-semibold text-text-primary", "{tr.team_name}" }
                Input {
                    variant: InputVariant::Default,
                    r#type: InputType::Text,
                    placeholder: tr.display_name_hint.to_string(),
                    value: nickname(),
                    oninput: move |e: FormEvent| nickname.set(e.value()),
                }
            }

            // Description + Last saved
            div { class: "flex flex-col gap-2",
                label { class: "text-sm font-semibold text-text-primary", "Description" }
                TextArea {
                    placeholder: tr.team_description_hint.to_string(),
                    value: html_contents(),
                    oninput: move |e: FormEvent| html_contents.set(e.value()),
                    class: "w-full resize-y min-h-[120px]",
                }
                if !last_saved.is_empty() {
                    div { class: "flex justify-end",
                        span { class: "text-xs text-foreground-muted", "{tr.last_saved_at} {last_saved}" }
                    }
                }
            }

            // Checkboxes
            div { class: "flex flex-col gap-4",
                label { class: "flex gap-3 items-start cursor-pointer",
                    input {
                        r#type: "checkbox",
                        checked: allow_invite(),
                        onchange: move |e: FormEvent| allow_invite.set(e.checked()),
                        class: "mt-0.5 w-4 h-4 cursor-pointer accent-primary shrink-0",
                    }
                    div { class: "flex flex-col gap-0.5",
                        span { class: "text-sm font-medium text-text-primary", "{tr.allow_invite}" }
                        span { class: "text-xs text-foreground-muted", "{tr.allow_invite_description}" }
                    }
                }
                label { class: "flex gap-3 items-start cursor-pointer",
                    input {
                        r#type: "checkbox",
                        checked: allow_create_space(),
                        onchange: move |e: FormEvent| allow_create_space.set(e.checked()),
                        class: "mt-0.5 w-4 h-4 cursor-pointer accent-primary shrink-0",
                    }
                    div { class: "flex flex-col gap-0.5",
                        span { class: "text-sm font-medium text-text-primary", "{tr.allow_create_space}" }
                        span { class: "text-xs text-foreground-muted",
                            "{tr.allow_create_space_description}"
                        }
                    }
                }
            }

            if let Some(msg) = message() {
                div { class: "text-sm text-destructive", "{msg}" }
            }

            // Delete team — bottom right
            if delete_team_permission {
                div { class: "flex justify-end pt-4 border-t border-separator",
                    Button {
                        size: ButtonSize::Medium,
                        style: ButtonStyle::Outline,
                        shape: ButtonShape::Square,
                        class: "text-destructive border-destructive hover:bg-destructive/10",
                        onclick: move |e| on_open_delete(e),
                        "{tr.delete_team}"
                    }
                }
            }
        }
    }
}
