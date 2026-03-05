use crate::controllers::{
    create_space_reward, delete_space_reward, list_space_rewards, update_space_reward,
    CreateSpaceRewardRequest, DeleteSpaceRewardRequest, UpdateSpaceRewardRequest,
};
use crate::i18n::SpaceRewardTranslate;
use crate::*;
use space_common::models::SpaceRewardResponse;

fn behavior_label(behavior: &RewardUserBehavior, tr: &SpaceRewardTranslate) -> String {
    match behavior {
        RewardUserBehavior::RespondPoll => tr.respond_poll.to_string(),
    }
}

#[component]
fn RewardCard(
    reward: SpaceRewardResponse,
    tr: SpaceRewardTranslate,
    on_edit: EventHandler<SpaceRewardResponse>,
    on_delete: EventHandler<SpaceRewardResponse>,
) -> Element {
    rsx! {
        div { class: "border border-card-border rounded-lg p-4 bg-card-bg",
            div { class: "flex flex-col gap-3",
                div { class: "flex justify-between items-start",
                    div { class: "flex-1",
                        h4 { class: "text-lg font-semibold text-font-primary",
                            {behavior_label(&reward.behavior, &tr)}
                        }
                        if !reward.description.is_empty() {
                            p { class: "text-sm text-font-secondary mt-1",
                                {reward.description.clone()}
                            }
                        }
                    }
                    div { class: "flex gap-2",
                        button {
                            class: "px-2 py-1 text-sm rounded border border-btn-outline-outline bg-btn-outline-bg text-btn-outline-text hover:bg-card-hover",
                            onclick: {
                                let reward = reward.clone();
                                move |_| on_edit.call(reward.clone())
                            },
                            "✏️"
                        }
                        button {
                            class: "px-2 py-1 text-sm rounded border border-btn-outline-outline bg-btn-outline-bg text-btn-outline-text hover:bg-card-hover",
                            onclick: {
                                let reward = reward.clone();
                                move |_| on_delete.call(reward.clone())
                            },
                            "🗑️"
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

#[component]
fn RewardModal(
    tr: SpaceRewardTranslate,
    editing_reward: Option<SpaceRewardResponse>,
    on_close: EventHandler<()>,
    on_submit: EventHandler<(RewardUserBehavior, String, i64)>,
    is_submitting: bool,
) -> Element {
    let is_edit = editing_reward.is_some();
    let title = if is_edit {
        tr.edit_reward.to_string()
    } else {
        tr.create_reward.to_string()
    };

    let init_behavior = editing_reward
        .as_ref()
        .map(|r| r.behavior.clone())
        .unwrap_or_default();
    let init_description = editing_reward
        .as_ref()
        .map(|r| r.description.clone())
        .unwrap_or_default();
    let init_credits = editing_reward.as_ref().map(|r| r.credits).unwrap_or(1);

    let mut description = use_signal(move || init_description);
    let mut credits = use_signal(move || init_credits);

    rsx! {
        div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/50",
            onclick: move |_| on_close.call(()),

            div {
                class: "bg-card-bg border border-card-border rounded-lg p-6 w-full max-w-md mx-4",
                onclick: move |e| e.stop_propagation(),

                h3 { class: "text-lg font-semibold text-font-primary mb-4", {title} }

                div { class: "flex flex-col gap-4",
                    // Behavior (read-only display)
                    div {
                        label { class: "block text-sm font-medium text-font-secondary mb-1",
                            {tr.reward_behavior}
                        }
                        div { class: "px-3 py-2 rounded border border-card-border bg-card-bg text-font-primary text-sm",
                            {behavior_label(&init_behavior, &tr)}
                        }
                    }

                    // Credits
                    div {
                        label { class: "block text-sm font-medium text-font-secondary mb-1",
                            {tr.credits}
                        }
                        input {
                            class: "w-full px-3 py-2 rounded border border-card-border bg-card-bg text-font-primary text-sm",
                            r#type: "number",
                            min: "1",
                            value: "{credits()}",
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<i64>() {
                                    credits.set(v);
                                }
                            },
                        }
                    }

                    // Description
                    div {
                        label { class: "block text-sm font-medium text-font-secondary mb-1",
                            {tr.description}
                        }
                        input {
                            class: "w-full px-3 py-2 rounded border border-card-border bg-card-bg text-font-primary text-sm",
                            value: "{description()}",
                            placeholder: tr.description_placeholder,
                            oninput: move |e| description.set(e.value()),
                        }
                    }

                    // Buttons
                    div { class: "flex gap-3 justify-end mt-4",
                        button {
                            class: "px-4 py-2 text-sm font-semibold rounded border border-btn-outline-outline bg-btn-outline-bg text-btn-outline-text",
                            disabled: is_submitting,
                            onclick: move |_| on_close.call(()),
                            {tr.cancel}
                        }
                        button {
                            class: "px-4 py-2 text-sm font-semibold rounded bg-btn-primary-bg text-btn-primary-text disabled:opacity-50",
                            disabled: is_submitting || credits() < 1,
                            onclick: {
                                let behavior = init_behavior.clone();
                                move |_| {
                                    on_submit.call((
                                        behavior.clone(),
                                        description().trim().to_string(),
                                        credits(),
                                    ));
                                }
                            },
                            if is_submitting { {tr.loading} } else { {tr.save} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CreatorPage(space_id: SpacePartition) -> Element {
    let tr: SpaceRewardTranslate = use_translate();
    let mut rewards = use_signal(Vec::<SpaceRewardResponse>::new);
    let mut did_load = use_signal(|| false);
    let mut is_modal_open = use_signal(|| false);
    let mut editing_reward = use_signal(|| None::<SpaceRewardResponse>);
    let mut is_submitting = use_signal(|| false);

    let space_id_for_load = space_id.clone();
    let reload = move || {
        let space_id = space_id_for_load.clone();
        spawn(async move {
            if let Ok(res) = list_space_rewards(space_id, None).await {
                rewards.set(res.items);
            }
        });
    };

    {
        let reload_fn = reload.clone();
        use_effect(move || {
            if did_load() {
                return;
            }
            did_load.set(true);
            reload_fn();
        });
    }

    let reward_list = rewards();

    rsx! {
        div { class: "flex flex-col gap-4 p-4 w-full max-w-[1024px]",
            div { class: "flex justify-between items-center",
                h2 { class: "text-xl font-bold text-font-primary",
                    {tr.title}
                }
                button {
                    class: "px-4 py-2 text-sm font-semibold rounded bg-btn-primary-bg text-btn-primary-text hover:opacity-90",
                    onclick: move |_| {
                        editing_reward.set(None);
                        is_modal_open.set(true);
                    },
                    "+ {tr.create_reward}"
                }
            }

            if reward_list.is_empty() && did_load() {
                div { class: "text-center py-8 text-font-secondary text-sm",
                    {tr.no_rewards}
                }
            } else {
                div { class: "flex flex-col gap-3",
                    for reward in reward_list.iter() {
                        RewardCard {
                            key: "{reward.sk}",
                            reward: reward.clone(),
                            tr: tr.clone(),
                            on_edit: move |r: SpaceRewardResponse| {
                                editing_reward.set(Some(r));
                                is_modal_open.set(true);
                            },
                            on_delete: {
                                let space_id = space_id.clone();
                                let reload = reload.clone();
                                move |r: SpaceRewardResponse| {
                                    let space_id = space_id.clone();
                                    let reload = reload.clone();
                                    spawn(async move {
                                        let _ = delete_space_reward(
                                            space_id,
                                            DeleteSpaceRewardRequest { sk: r.sk },
                                        )
                                        .await;
                                        reload();
                                    });
                                }
                            },
                        }
                    }
                }
            }

            if is_modal_open() {
                RewardModal {
                    tr: tr.clone(),
                    editing_reward: editing_reward(),
                    on_close: move |_| {
                        is_modal_open.set(false);
                        editing_reward.set(None);
                    },
                    on_submit: {
                        let space_id = space_id.clone();
                        let reload = reload.clone();
                        move |(behavior, description, credits): (RewardUserBehavior, String, i64)| {
                            let space_id = space_id.clone();
                            let editing = editing_reward();
                            let reload = reload.clone();
                            is_submitting.set(true);

                            spawn(async move {
                                let result = if let Some(existing) = editing {
                                    update_space_reward(
                                        space_id,
                                        UpdateSpaceRewardRequest {
                                            sk: existing.sk,
                                            description,
                                            credits,
                                        },
                                    )
                                    .await
                                } else {
                                    create_space_reward(
                                        space_id,
                                        CreateSpaceRewardRequest {
                                            action_key: EntityType::SpacePoll(Default::default()),
                                            behavior,
                                            description,
                                            credits,
                                        },
                                    )
                                    .await
                                };

                                is_submitting.set(false);

                                if result.is_ok() {
                                    is_modal_open.set(false);
                                    editing_reward.set(None);
                                    reload();
                                }
                            });
                        }
                    },
                    is_submitting: is_submitting(),
                }
            }
        }
    }
}
