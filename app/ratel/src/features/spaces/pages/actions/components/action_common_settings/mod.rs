mod prerequisite_setting;
mod reward_setting;
pub use prerequisite_setting::*;
pub use reward_setting::*;

use super::*;

#[component]
pub fn ActionCommonSettings(
    space_id: ReadSignal<SpacePartition>,
    action_id: ReadSignal<String>,
    action_setting: ReadSignal<SpaceAction>,
    #[props(default)] on_date_change: EventHandler<DateTimeRange>,
    #[props(default)] on_credit_change: EventHandler<u64>,
    #[props(default)] on_prerequisite_change: EventHandler<bool>,
) -> Element {
    let tr: ActionCommonSettingsTranslate = use_translate();
    let mut toast = crate::common::use_toast();
    let mut current_credits = use_signal(move || action_setting().credits);
    let setting = action_setting();

    #[cfg(feature = "membership")]
    let space = crate::features::spaces::space_common::hooks::use_space();
    #[cfg(feature = "membership")]
    let current_space = space();
    #[cfg(feature = "membership")]
    let user_ctx = crate::features::auth::hooks::use_user_context();
    #[cfg(feature = "membership")]
    let personal_username = user_ctx
        .read()
        .user
        .as_ref()
        .map(|u| u.username.clone())
        .unwrap_or_default();
    #[cfg(feature = "membership")]
    let owner_username = current_space.author_username.clone();
    #[cfg(feature = "membership")]
    let team_detail =
        use_server_future(use_reactive((&owner_username,), |(username,)| async move {
            crate::features::social::controllers::find_team_handler(username.to_string()).await
        }))?;
    #[cfg(feature = "membership")]
    let team_detail_read = team_detail.read();
    #[cfg(feature = "membership")]
    let team_detail = team_detail_read
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned();
    #[cfg(feature = "membership")]
    let is_team_author = current_space.author_type == crate::common::UserType::Team;
    #[cfg(feature = "membership")]
    let team_username = team_detail
        .as_ref()
        .map(|team| team.username.clone())
        .unwrap_or_else(|| current_space.author_username.clone());
    #[cfg(feature = "membership")]
    let is_team_space = is_team_author || team_detail.is_some();
    #[cfg(feature = "membership")]
    let upgrade_route = if is_team_space {
        format!("/{}/team-memberships", team_username)
    } else {
        format!("/{personal_username}/memberships")
    };
    #[cfg(feature = "membership")]
    let user_membership = crate::features::auth::hooks::use_user_membership();
    #[cfg(feature = "membership")]
    let team_membership = use_server_future(use_reactive(
        (&team_username, &is_team_space),
        |(team_username, is_team_space)| async move {
            if is_team_space && !team_username.is_empty() {
                crate::features::membership::controllers::get_team_membership_handler(
                    team_username.to_string(),
                )
                .await
                .map(Some)
            } else {
                Ok(None)
            }
        },
    ))?;
    #[cfg(feature = "membership")]
    let team_membership_read = team_membership.read();
    #[cfg(feature = "membership")]
    let team_membership = team_membership_read
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .and_then(|membership| membership.clone());
    #[cfg(feature = "membership")]
    let base_is_paid = if is_team_space {
        team_membership
            .as_ref()
            .is_some_and(|membership| !membership.tier.0.contains("Free"))
    } else {
        user_membership
            .as_ref()
            .is_some_and(|membership| membership.is_paid())
    };
    #[cfg(feature = "membership")]
    let base_max_credits = if is_team_space {
        team_membership.as_ref().map_or(0, |membership| {
            membership.max_credits_per_space.max(0) as u64
        })
    } else {
        user_membership.as_ref().map_or(0, |membership| {
            membership.max_credits_per_space.max(0) as u64
        })
    };
    #[cfg(feature = "membership")]
    let base_remaining_credits = if is_team_space {
        team_membership
            .as_ref()
            .map_or(0, |membership| membership.remaining_credits.max(0) as u64)
    } else {
        user_membership
            .as_ref()
            .map_or(0, |membership| membership.remaining_credits.max(0) as u64)
    };
    #[cfg(feature = "membership")]
    let mut remaining_credits = use_signal(move || base_remaining_credits);
    #[cfg(feature = "membership")]
    let mut last_loaded_remaining_credits = use_signal(move || base_remaining_credits);
    #[cfg(feature = "membership")]
    use_effect(move || {
        if last_loaded_remaining_credits() != base_remaining_credits {
            last_loaded_remaining_credits.set(base_remaining_credits);
            remaining_credits.set(base_remaining_credits);
        }
    });

    #[cfg(feature = "membership")]
    let mut auth_ctx = use_context::<crate::features::auth::context::Context>();
    #[cfg(not(feature = "membership"))]
    let base_is_paid = false;
    #[cfg(not(feature = "membership"))]
    let base_max_credits = 0;
    #[cfg(not(feature = "membership"))]
    let remaining_credits = use_signal(|| 0u64);
    #[cfg(not(feature = "membership"))]
    let upgrade_route = String::new();

    rsx! {
        div { class: "flex flex-col gap-5 w-full",
            div { class: "flex flex-col gap-2.5",
                p { {tr.date} }
                DateAndTimePicker {
                    initial_started_at: Some(setting.started_at),
                    initial_ended_at: Some(setting.ended_at),
                    on_change: move |range: DateTimeRange| async move {
                        if let (Some(start_date), Some(end_date)) = (range.start_date, range.end_date) {
                            let started_at = date_time_to_millis(
                                start_date,
                                range.start_hour,
                                range.start_minute,
                            );
                            let ended_at = date_time_to_millis(
                                end_date,
                                range.end_hour,
                                range.end_minute,
                            );
                            let req = UpdateSpaceActionRequest::Time {
                                started_at,
                                ended_at,
                            };
                            match update_space_action(space_id(), action_id(), req).await {
                                Ok(_) => {
                                    toast.info(tr.date_updated.to_string());
                                    on_date_change.call(range);
                                }
                                Err(e) => {
                                    toast.error(e);
                                }
                            }
                        }
                    },
                }
            }

            PrerequisiteSetting {
                space_id,
                action_id,
                action_setting,
                on_change: on_prerequisite_change,
            }

            RewardSetting {
                saved_credits: current_credits,
                is_paid: base_is_paid,
                max_credits: base_max_credits,
                remaining_credits,
                upgrade_route: upgrade_route.clone(),
                on_change: move |credits: u64| async move {
                    let previous_credits = current_credits();
                    let req = UpdateSpaceActionRequest::Credits {
                        credits,
                    };
                    match update_space_action(space_id(), action_id(), req).await {
                        Ok(_) => {
                            let delta = credits as i64 - previous_credits as i64;
                            current_credits.set(credits);
                            #[cfg(feature = "membership")]
                            {
                                remaining_credits
                                    .set(
                                        (remaining_credits().saturating_sub(delta.max(0) as u64))
                                            .saturating_add((-delta).max(0) as u64),
                                    );
                                if !is_team_space {
                                    let mut user_ctx = auth_ctx.user_context.write();
                                    if let Some(membership) = user_ctx.membership.as_mut() {
                                        membership.remaining_credits = (membership.remaining_credits
                                            - delta)
                                            .max(0);
                                    }
                                }
                            }
                            toast.info(tr.reward_updated.to_string());
                            on_credit_change.call(credits);
                        }
                        Err(e) => {
                            toast.error(e);
                        }
                    }
                },
            }
        }
    }
}

translate! {
    ActionCommonSettingsTranslate;

    date: {
        en: "Date",
        ko: "참여기간",
    },
    reward_updated: {
        en: "Reward credits updated.",
        ko: "보상 크레딧이 업데이트되었습니다.",
    },
    date_updated: {
        en: "Date range updated.",
        ko: "참여기간이 업데이트되었습니다.",
    },
}

fn date_time_to_millis(date: time::Date, hour: u8, minute: u8) -> i64 {
    let datetime = date.with_hms(hour, minute, 0).expect("valid time");
    let offset_datetime = datetime.assume_utc();
    offset_datetime.unix_timestamp() * 1000
}
