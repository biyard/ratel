use crate::features::spaces::pages::actions::controllers::list_actions;
use crate::features::spaces::pages::actions::controllers::update_space_action;
use crate::features::spaces::pages::actions::controllers::UpdateSpaceActionRequest;
use crate::features::spaces::pages::actions::types::SpaceActionSummary;
use crate::features::spaces::pages::apps::apps::rewards::controllers::list_space_rewards;
use crate::features::spaces::pages::apps::apps::rewards::i18n::SpaceRewardTranslate;
use crate::features::spaces::pages::apps::apps::rewards::*;
use crate::features::spaces::space_common::models::SpaceRewardResponse;
use dioxus_translate::{Translate as _, use_language};

#[component]
fn RewardCard(
    reward: SpaceRewardResponse,
    tr: SpaceRewardTranslate,
    on_delete: EventHandler<SpaceRewardResponse>,
) -> Element {
    let lang = use_language();

    rsx! {
        Card {
            class: "p-4",
            div { class: "flex flex-col gap-3",
                div { class: "flex justify-between items-start",
                    div { class: "flex-1",
                        h4 { class: "text-lg font-semibold text-font-primary",
                            {reward.behavior.translate(&lang())}
                        }
                        if !reward.description.is_empty() {
                            p { class: "text-sm text-font-secondary mt-1",
                                {reward.description.clone()}
                            }
                        }
                    }
                    div { class: "flex gap-2",
                        Button {
                            class: "px-2 py-1 text-sm",
                            style: ButtonStyle::Outline,
                            onclick: {
                                let reward = reward.clone();
                                move |_| on_delete.call(reward.clone())
                            },
                            {tr.delete_reward.to_string()}
                        }
                    }
                }

                div { class: "flex gap-6 text-sm",
                    div {
                        span { class: "text-font-secondary", "{tr.credits}: " }
                        span { class: "font-medium text-font-primary", "{reward.credits}" }
                    }
                    div {
                        span { class: "text-font-secondary", "{tr.total_claims}: " }
                        span { class: "font-medium text-font-primary", "{reward.total_claims}" }
                    }
                    div {
                        span { class: "text-font-secondary", "{tr.total_points}: " }
                        span { class: "font-medium text-font-primary", "{reward.total_points}" }
                    }
                }
            }
        }
    }
}

/// A section grouping rewards per SpaceAction (React RewardSection equivalent)
#[component]
fn RewardSection(
    action: SpaceActionSummary,
    rewards: Vec<SpaceRewardResponse>,
    tr: SpaceRewardTranslate,
    space_id: SpacePartition,
    on_reload: EventHandler<()>,
) -> Element {
    let reward_count = rewards.len();
    let has_reward = reward_count > 0;
    let mut is_submitting = use_signal(|| false);

    rsx! {
        Card {
            class: "overflow-hidden",
            // Section header
            div { class: "flex items-center justify-between px-4 py-3 bg-card-bg border-b border-card-border",
                div { class: "flex items-center gap-2",
                    h3 { class: "text-base font-semibold text-font-primary",
                        {action.title.clone()}
                    }
                    Badge {
                        "{reward_count} {tr.action_reward_count}"
                    }
                }
            }

            // Section content
            div { class: "p-4",
                if has_reward {
                    div { class: "flex flex-col gap-3",
                        for reward in rewards.iter() {
                            RewardCard {
                                key: "{reward.sk}",
                                reward: reward.clone(),
                                tr: tr.clone(),
                                on_delete: {
                                    let space_id = space_id.clone();
                                    let action_id = action.action_id.clone();
                                    let on_reload = on_reload.clone();
                                    move |_r: SpaceRewardResponse| {
                                        let space_id = space_id.clone();
                                        let action_id = action_id.clone();
                                        let on_reload = on_reload.clone();
                                        spawn(async move {
                                            let _ = update_space_action(
                                                space_id,
                                                action_id,
                                                UpdateSpaceActionRequest::Credits { credits: 0 },
                                            )
                                            .await;
                                            on_reload.call(());
                                        });
                                    }
                                },
                            }
                        }
                    }
                } else {
                    div { class: "flex items-center justify-between",
                        span { class: "text-sm text-font-secondary",
                            {tr.no_reward_configured.to_string()}
                        }
                        Button {
                            style: ButtonStyle::Primary,
                            class: "text-sm",
                            disabled: is_submitting(),
                            onclick: {
                                let space_id = space_id.clone();
                                let action_id = action.action_id.clone();
                                let on_reload = on_reload.clone();
                                move |_| {
                                    let space_id = space_id.clone();
                                    let action_id = action_id.clone();
                                    let on_reload = on_reload.clone();
                                    is_submitting.set(true);
                                    spawn(async move {
                                        let _ = update_space_action(
                                            space_id,
                                            action_id,
                                            UpdateSpaceActionRequest::Credits { credits: 1 },
                                        )
                                        .await;
                                        is_submitting.set(false);
                                        on_reload.call(());
                                    });
                                }
                            },
                            "+ {tr.create_reward}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CreatorPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceRewardTranslate = use_translate();

    let mut actions = use_loader(move || async move { list_actions(space_id()).await })?;
    let mut rewards = use_loader(move || async move { list_space_rewards(space_id(), None).await })?;

    let action_list = actions();
    let reward_list = rewards();

    rsx! {
        div { class: "flex flex-col gap-4 p-4 w-full max-w-[1024px]",
            h2 { class: "text-xl font-bold text-font-primary",
                {tr.title}
            }

            if action_list.is_empty() {
                div { class: "text-center py-8 text-font-secondary text-sm",
                    {tr.no_polls.to_string()}
                }
            } else {
                div { class: "flex flex-col gap-4",
                    for action in action_list.iter() {
                        {
                            let action_rewards: Vec<SpaceRewardResponse> = reward_list
                                .items
                                .iter()
                                .filter(|r| {
                                    r.sk.action_id.as_deref() == Some(&action.action_id)
                                })
                                .cloned()
                                .collect();

                            rsx! {
                                RewardSection {
                                    key: "{action.action_id}",
                                    action: action.clone(),
                                    rewards: action_rewards,
                                    tr: tr.clone(),
                                    space_id: space_id(),
                                    on_reload: move |_| {
                                        actions.restart();
                                        rewards.restart();
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
