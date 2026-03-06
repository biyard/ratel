use crate::*;
use self::action_chip::ActionSettingsActionChip;
use self::fields::{DateField, TimeZoneField};
use self::i18n::ActionSettingsModalTranslate;
use self::reward_cards::{RewardSummaryCard, RewardsCreditsCard, RewardsInfoCard};
use self::utils::{
    action_label, apply_selected_action_dates, available_actions, resolve_action_time_range,
    reward_credit_summary, reward_preview_items, selected_actions, supports_action_settings,
};

mod action_chip;
mod fields;
mod i18n;
mod reward_cards;
mod utils;

#[component]
pub fn ActionSettingsModal(
    space_id: SpacePartition,
    actions: Vec<SpaceAction>,
    on_applied: EventHandler<()>,
) -> Element {
    let tr: ActionSettingsModalTranslate = use_translate();
    let lang = use_language();
    let layover = use_layover();
    let toast = use_toast();

    let initial_action_ids = actions
        .iter()
        .filter(|action| supports_action_settings(action))
        .map(|action| action.action_id.clone())
        .collect::<Vec<_>>();

    let selected_action_ids = use_signal(move || initial_action_ids);
    let mut is_add_menu_open = use_signal(|| false);
    let mut is_date_enabled = use_signal(|| true);
    let mut is_rewards_enabled = use_signal(|| false);
    let mut start_date = use_signal(String::new);
    let mut end_date = use_signal(String::new);
    let mut time_zone = use_signal(String::new);
    let is_applying = use_signal(|| false);

    let current_lang = lang();
    let selected_ids = selected_action_ids();
    let selected_actions = selected_actions(&actions, &selected_ids);
    let available_actions = available_actions(&actions, &selected_ids);
    let reward_previews = reward_preview_items(&selected_actions);
    let (credit_usage, remaining_credits) = reward_credit_summary();

    let select_dates_error = tr.select_dates_error.to_string();
    let select_time_zone_error = tr.select_time_zone_error.to_string();
    let invalid_date_range_error = tr.invalid_date_range_error.to_string();
    let unsupported_time_zone_error = tr.unsupported_time_zone_error.to_string();
    let applied_success = tr.applied_success.to_string();

    let close_modal = {
        let mut layover = layover;
        move |_| layover.close()
    };

    let apply_settings = {
        let mut layover = layover;
        let mut toast = toast;
        let mut is_applying = is_applying;
        let selected_actions = selected_actions.clone();
        let space_id = space_id.clone();
        let on_applied = on_applied.clone();

        move |_| {
            if is_applying() {
                return;
            }

            if !is_date_enabled() {
                layover.close();
                return;
            }

            if start_date().is_empty() || end_date().is_empty() {
                toast.warn(select_dates_error.clone());
                return;
            }

            if time_zone().is_empty() {
                toast.warn(select_time_zone_error.clone());
                return;
            }

            let (started_at, ended_at) = match resolve_action_time_range(
                &start_date(),
                &end_date(),
                &time_zone(),
                &select_dates_error,
                &invalid_date_range_error,
                &unsupported_time_zone_error,
            ) {
                Ok(range) => range,
                Err(err) => {
                    toast.error(err);
                    return;
                }
            };

            let selected_actions = selected_actions.clone();
            let space_id = space_id.clone();
            let on_applied = on_applied.clone();
            let applied_success = applied_success.clone();
            let mut toast = toast;
            let mut layover = layover;
            is_applying.set(true);

            spawn(async move {
                let result =
                    apply_selected_action_dates(space_id, selected_actions, started_at, ended_at)
                        .await;

                match result {
                    Ok(()) => {
                        toast.info(applied_success);
                        on_applied.call(());
                        layover.close();
                    }
                    Err(err) => {
                        toast.error(err);
                    }
                }

                is_applying.set(false);
            });
        }
    };

    rsx! {
        div { class: "flex min-h-full w-full justify-end overflow-y-auto px-6 py-8 text-web-font-primary max-tablet:px-4 max-tablet:py-4 max-mobile:px-0 max-mobile:py-0",
            div { class: "flex w-full max-w-[337px] shrink-0 flex-col gap-5 rounded-[24px] bg-neutral-950 p-6 shadow-[0_8px_20px_0_rgba(20,26,62,0.25)] max-tablet:max-w-full max-tablet:p-5 max-mobile:min-h-full max-mobile:gap-4 max-mobile:rounded-none max-mobile:p-4 max-mobile:shadow-none",
                div { class: "flex items-center gap-3 max-mobile:gap-2.5",
                    Button {
                        size: ButtonSize::Icon,
                        style: ButtonStyle::Text,
                        shape: ButtonShape::Square,
                        class: "flex size-5 items-center justify-center rounded-none p-0 text-web-font-primary hover:bg-transparent",
                        onclick: close_modal,
                        icons::arrows::LineArrowLeft {
                            width: "20",
                            height: "20",
                            class: "[&>path]:stroke-current",
                        }
                    }

                    h4 { class: "font-bold font-raleway text-[15px]/[20px] tracking-[0.5px] text-web-font-primary max-mobile:text-[14px]/[18px]",
                        {tr.title}
                    }
                }

                div { class: "flex flex-col gap-2.5",
                    for action in selected_actions.iter() {
                        ActionSettingsActionChip {
                            key: "{action.action_id}",
                            label: action_label(action, &current_lang, &tr.untitled),
                            on_remove: {
                                let action_id = action.action_id.clone();
                                let mut selected_action_ids = selected_action_ids;
                                let mut is_add_menu_open = is_add_menu_open;
                                move |_| {
                                    selected_action_ids.with_mut(|ids| ids.retain(|id| id != &action_id));
                                    is_add_menu_open.set(false);
                                }
                            },
                        }
                    }
                }

                div { class: "relative w-full",
                    Button {
                        size: ButtonSize::Medium,
                        style: ButtonStyle::Ghost,
                        shape: ButtonShape::Square,
                        class: if available_actions.is_empty() { "flex h-11 w-full items-center gap-2.5 rounded-[10px] bg-neutral-800 py-2.5 pr-5 pl-2.5 text-left hover:bg-neutral-800 cursor-not-allowed opacity-50" } else { "flex h-11 w-full items-center gap-2.5 rounded-[10px] bg-neutral-800 py-2.5 pr-5 pl-2.5 text-left hover:bg-neutral-800" },
                        disabled: available_actions.is_empty(),
                        onclick: move |_| is_add_menu_open.set(!is_add_menu_open()),
                        icons::validations::Add {
                            width: "20",
                            height: "20",
                            class: "text-web-font-neutral [&>path]:stroke-current",
                        }
                        span { class: "min-w-0 truncate font-bold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-web-font-neutral",
                            if available_actions.is_empty() {
                                {tr.no_available_actions}
                            } else {
                                {tr.select_action}
                            }
                        }
                    }

                    if is_add_menu_open() && !available_actions.is_empty() {
                        div { class: "absolute left-0 top-full z-10 mt-2 flex w-full flex-col overflow-hidden rounded-[10px] border border-yellow-400 bg-neutral-800 shadow-[0_8px_20px_0_rgba(20,26,62,0.25)] max-mobile:max-h-56 overflow-y-auto",
                            for action in available_actions.iter() {
                                Button {
                                    key: "{action.action_id}",
                                    size: ButtonSize::Medium,
                                    style: ButtonStyle::Text,
                                    shape: ButtonShape::Square,
                                    class: "flex min-h-11 w-full items-start justify-start rounded-none px-3 py-2.5 text-left font-medium font-raleway text-[15px]/[18px] whitespace-normal break-words text-web-font-primary hover:bg-neutral-900",
                                    onclick: {
                                        let action_id = action.action_id.clone();
                                        let mut selected_action_ids = selected_action_ids;
                                        let mut is_add_menu_open = is_add_menu_open;
                                        move |_| {
                                            selected_action_ids.with_mut(|ids| ids.push(action_id.clone()));
                                            is_add_menu_open.set(false);
                                        }
                                    },
                                    {action_label(action, &current_lang, &tr.untitled)}
                                }
                            }
                        }
                    }
                }

                div { class: "h-px w-full bg-neutral-800" }

                div { class: "flex flex-col gap-4",
                    div { class: "flex items-center justify-between gap-3",
                        div { class: "flex items-center gap-2.5 text-web-font-primary",
                            icons::calendar::CalendarToday {
                                width: "20",
                                height: "20",
                                class: "[&>path]:stroke-current",
                            }
                            span { class: "font-bold font-raleway text-[17px]/[20px] tracking-[-0.18px] max-mobile:text-[15px]/[18px]",
                                {tr.date}
                            }
                        }

                        Switch {
                            active: is_date_enabled(),
                            on_toggle: move |_| is_date_enabled.set(!is_date_enabled()),
                        }
                    }

                    if is_date_enabled() {
                        div { class: "flex flex-col gap-2.5",
                            div { class: "flex flex-col gap-2.5",
                                DateField {
                                    label: tr.start_date.to_string(),
                                    value: start_date(),
                                    placeholder: tr.date_placeholder.to_string(),
                                    min: None,
                                    onchange: move |value| start_date.set(value),
                                }

                                DateField {
                                    label: tr.end_date.to_string(),
                                    value: end_date(),
                                    placeholder: tr.date_placeholder.to_string(),
                                    min: if start_date().is_empty() { None } else { Some(start_date()) },
                                    onchange: move |value| end_date.set(value),
                                }
                            }

                            TimeZoneField {
                                value: time_zone(),
                                placeholder: tr.time_zone.to_string(),
                                onchange: move |value| time_zone.set(value),
                            }
                        }
                    }
                }

                div { class: "h-px w-full bg-neutral-800" }

                div { class: "flex flex-col gap-4 py-1",
                    div { class: "flex items-center justify-between gap-3",
                        div { class: "flex items-center gap-2.5 text-web-font-primary",
                            icons::shopping::Gift {
                                width: "20",
                                height: "20",
                                class: "[&>path]:stroke-current",
                            }
                            span { class: "font-bold font-raleway text-[17px]/[20px] tracking-[-0.18px] max-mobile:text-[15px]/[18px]",
                                {tr.rewards}
                            }
                        }

                        Switch {
                            active: is_rewards_enabled(),
                            on_toggle: move |_| is_rewards_enabled.set(!is_rewards_enabled()),
                        }
                    }

                    if is_rewards_enabled() {
                        div { class: "flex flex-col gap-5 max-mobile:gap-4",
                            if reward_previews.is_empty() {
                                Card { class: "w-full rounded-[8px] border-web-card-stroke bg-web-card-bg px-[17px] py-[17px] font-medium font-raleway text-[13px]/[20px] text-web-font-neutral max-mobile:px-4 max-mobile:py-4",
                                    {tr.no_rewards_configured}
                                }
                            } else {
                                for (idx , reward) in reward_previews.iter().enumerate() {
                                    RewardSummaryCard {
                                        key: "reward-preview-{idx}",
                                        reward: reward.clone(),
                                        action_point_reward: tr.action_point_reward.to_string(),
                                        boost_unit: tr.boost_unit.to_string(),
                                    }
                                }
                            }

                            Card { class: "flex w-full items-center justify-between gap-3 rounded-[8px] border-web-card-stroke bg-web-card-bg px-[17px] py-[17px] text-left max-mobile:px-4 max-mobile:py-4",
                                span { class: "flex-1 font-bold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-web-font-neutral",
                                    {tr.boost_multiplier_settings}
                                }
                                icons::arrows::ExpandPage {
                                    width: "20",
                                    height: "20",
                                    class: "shrink-0 text-web-font-neutral [&>path]:stroke-current",
                                }
                            }

                            RewardsCreditsCard {
                                credit_usage,
                                remaining_credits,
                                credit_usage_label: tr.credit_usage.to_string(),
                                remaining_credits_label: tr.remaining_credits.to_string(),
                                credit_unit_singular: tr.credit_unit_singular.to_string(),
                                credit_unit_plural: tr.credit_unit_plural.to_string(),
                            }

                            RewardsInfoCard {
                                title: tr.point_boost.to_string(),
                                line_one: tr.point_boost_line_one.to_string(),
                                line_two: tr.point_boost_line_two.to_string(),
                                line_three: tr.point_boost_line_three.to_string(),
                                membership: tr.membership.to_string(),
                            }
                        }
                    }
                }

                div { class: "mt-auto pt-2",
                    Button {
                        size: ButtonSize::Medium,
                        style: ButtonStyle::Primary,
                        shape: ButtonShape::Square,
                        class: "flex w-full justify-center rounded-[10px] font-raleway text-[15px]/[18px] tracking-[-0.16px]",
                        disabled: selected_actions.is_empty() || is_applying(),
                        onclick: apply_settings,
                        if is_applying() {
                            {tr.applying}
                        } else {
                            {tr.apply}
                        }
                    }
                }
            }
        }
    }
}
