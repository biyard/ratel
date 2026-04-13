use super::super::controllers::{get_team_rewards_handler, list_team_point_transactions_handler};
use super::super::dto::TeamRewardsResponse;
use super::super::*;
use crate::common::services::PointTransactionResponse;
use crate::features::social::pages::user_reward::components::{
    exchange_preview_card, points_summary_card, transaction_list,
};
use crate::features::social::pages::user_reward::dto::RewardsResponse;
use crate::features::social::pages::user_reward::RewardsPageTranslate;
use dioxus::prelude::*;

#[component]
pub fn AdminPage(team_pk: TeamPartition) -> Element {
    let tr: RewardsPageTranslate = use_translate();

    let rewards_resource = use_loader(use_reactive((&team_pk,), |(team_pk,)| async move {
        Ok::<_, super::super::Error>(
            get_team_rewards_handler(team_pk, None)
                .await
                .map_err(|e| e.to_string()),
        )
    }))?;
    let transactions_resource = use_loader(use_reactive((&team_pk,), |(team_pk,)| async move {
        Ok::<_, super::super::Error>(
            list_team_point_transactions_handler(team_pk, None, None)
                .await
                .map_err(|e| e.to_string()),
        )
    }))?;

    let rewards_state = rewards_resource.read().clone();
    let transactions_state = transactions_resource.read().clone();

    let mut transactions = use_signal(Vec::<PointTransactionResponse>::new);
    let mut next_bookmark = use_signal(|| Option::<String>::None);
    let mut transactions_loaded = use_signal(|| false);
    let mut transactions_error = use_signal(|| false);
    let mut is_fetching_next = use_signal(|| false);

    {
        let transactions_state = transactions_state.clone();
        let mut transactions = transactions.clone();
        let mut next_bookmark = next_bookmark.clone();
        let mut transactions_loaded = transactions_loaded.clone();
        let mut transactions_error = transactions_error.clone();

        use_effect(move || {
            if *transactions_loaded.read() {
                return;
            }

            match transactions_state.as_ref() {
                Ok(data) => {
                    transactions.set(data.items.clone());
                    next_bookmark.set(data.bookmark.clone());
                    transactions_error.set(false);
                }
                Err(_) => {
                    transactions_error.set(true);
                }
            }
            transactions_loaded.set(true);
        });
    }

    let rewards: TeamRewardsResponse = match rewards_state.as_ref() {
        Ok(data) => data.clone(),
        Err(err) => {
            return rsx! {
                div { class: "w-full max-w-desktop mx-auto px-4 py-8",
                    div { class: "bg-card-bg border border-card-border rounded-lg p-8",
                        div { class: "text-center text-destructive text-text-primary",
                            "{tr.error}: {err}"
                        }
                    }
                }
            };
        }
    };

    let rewards = RewardsResponse {
        project_name: rewards.project_name,
        token_symbol: rewards.token_symbol,
        month: rewards.month,
        total_points: rewards.total_points,
        points: rewards.team_points,
        monthly_token_supply: rewards.monthly_token_supply,
        chain_id: None,
        contract_address: None,
    };

    let estimated_tokens = if rewards.total_points > 0 {
        ((rewards.points as f64 / rewards.total_points as f64)
            * rewards.monthly_token_supply as f64)
            .round()
    } else {
        0.0
    };

    let has_next = next_bookmark.read().is_some();
    let is_loading_transactions = !*transactions_loaded.read();
    let is_fetching_next_value = *is_fetching_next.read();
    let transactions_error_value = *transactions_error.read();
    let month = rewards.month.clone();

    let on_load_more = {
        let team_pk = team_pk.clone();
        let transactions = transactions.clone();
        let next_bookmark = next_bookmark.clone();
        let mut is_fetching_next = is_fetching_next.clone();
        let month = month.clone();

        move |_| {
            let month = month.clone();
            if *is_fetching_next.read() {
                return;
            }

            let Some(bookmark) = next_bookmark.read().clone() else {
                return;
            };

            is_fetching_next.set(true);
            let team_pk = team_pk.clone();
            let mut transactions = transactions.clone();
            let mut next_bookmark = next_bookmark.clone();
            let mut is_fetching_next = is_fetching_next.clone();
            spawn(async move {
                let result = list_team_point_transactions_handler(
                    team_pk.clone(),
                    Some(month),
                    Some(bookmark),
                )
                .await;
                if let Ok(data) = result {
                    let mut updated = {
                        let current = transactions.read();
                        current.clone()
                    };
                    updated.extend(data.items);
                    transactions.set(updated);
                    next_bookmark.set(data.bookmark);
                }
                is_fetching_next.set(false);
            });
        }
    };

    rsx! {
        div { class: "w-full max-w-desktop mx-auto px-4 py-6",
            {points_summary_card(&tr, &rewards, estimated_tokens)}
            {exchange_preview_card(&tr, &rewards, estimated_tokens)}

            div { class: "mt-6",
                {
                    transaction_list(
                        &tr,
                        transactions.read().as_slice(),
                        is_loading_transactions,
                        transactions_error_value,
                    )
                }

                if has_next && !transactions_error_value {
                    button {
                        class: "mt-4 py-3 text-center text-sm font-medium text-text-primary hover:text-white transition-colors disabled:opacity-50",
                        onclick: on_load_more,
                        disabled: is_fetching_next_value,
                        if is_fetching_next_value {
                            {tr.loading}
                        } else {
                            {tr.load_more}
                        }
                    }
                }
            }
        }
    }
}
