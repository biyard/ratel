use super::super::{
    components::transaction_list,
    controllers::{list_user_transactions_handler, request_claim_signature_handler},
    views::{format_points, format_tokens, RewardsPageTranslate},
    *,
};
use crate::common::services::{MonthlySummaryItem, PointTransactionResponse};

#[component]
pub fn PastMonthsList(
    username: ReadSignal<String>,
    months: Vec<MonthlySummaryItem>,
    #[props(default)] contract_address: Option<String>,
    #[props(default)] chain_id: Option<u64>,
) -> Element {
    let tr: RewardsPageTranslate = use_translate();

    rsx! {
        div { class: "flex flex-col gap-2",
            for item in months.iter().cloned() {
                PastMonthRow {
                    key: "{item.month}",
                    username,
                    item,
                    contract_address: contract_address.clone(),
                    chain_id,
                }
            }
        }
    }
}

#[component]
fn PastMonthRow(
    username: ReadSignal<String>,
    item: MonthlySummaryItem,
    #[props(default)] contract_address: Option<String>,
    #[props(default)] chain_id: Option<u64>,
) -> Element {
    let tr: RewardsPageTranslate = use_translate();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let is_logged_in = user_ctx().is_logged_in();
    let mut expanded = use_signal(|| false);
    let mut transactions = use_signal(Vec::<PointTransactionResponse>::new);
    let mut next_bookmark = use_signal(|| Option::<String>::None);
    let mut loaded = use_signal(|| false);
    let mut is_loading = use_signal(|| false);
    let mut is_claiming = use_signal(|| false);
    let mut is_claimed = use_signal(move || item.exchanged);

    let month = item.month.clone();
    let estimated_tokens = if item.project_total_points > 0 {
        ((item.total_earned as f64 / item.project_total_points as f64)
            * item.monthly_token_supply as f64)
            .round()
    } else {
        0.0
    };

    let on_toggle = move |_| {
        let is_expanded = *expanded.read();
        if !is_expanded && !*loaded.read() {
            let month = month.clone();
            let name = username();
            is_loading.set(true);
            spawn(async move {
                let result = list_user_transactions_handler(name, Some(month), None).await;
                if let Ok(data) = result {
                    transactions.set(data.items);
                    next_bookmark.set(data.bookmark);
                }
                loaded.set(true);
                is_loading.set(false);
            });
        }
        expanded.set(!is_expanded);
    };

    let month_for_more = item.month.clone();
    let on_load_more = move |_| {
        if *is_loading.read() {
            return;
        }
        let Some(bookmark) = next_bookmark.read().clone() else {
            return;
        };
        let month = month_for_more.clone();
        let name = username();
        is_loading.set(true);
        spawn(async move {
            let result =
                list_user_transactions_handler(name, Some(month), Some(bookmark)).await;
            if let Ok(data) = result {
                let mut updated = transactions.read().clone();
                updated.extend(data.items);
                transactions.set(updated);
                next_bookmark.set(data.bookmark);
            }
            is_loading.set(false);
        });
    };

    let claim_month = item.month.clone();
    let on_claim = move |e: Event<MouseData>| {
        e.stop_propagation();

        let month = claim_month.clone();

        async move {
            if *is_claiming.read() {
                return;
            }
            is_claiming.set(true);

            #[cfg(not(feature = "server"))]
            {
                use super::super::interop::{
                    claim_tokens, connect_wallet, get_wallet_address, ClaimTokensParams,
                };

                let wallet_address = match get_wallet_address().await {
                    Some(addr) => addr,
                    None => {
                        let Some(cid) = chain_id else {
                            is_claiming.set(false);
                            return;
                        };
                        match connect_wallet(cid).await {
                            Ok(addr) => addr,
                            Err(_) => {
                                is_claiming.set(false);
                                return;
                            }
                        }
                    }
                };

                let sig_result = request_claim_signature_handler(
                    super::super::controllers::ClaimSignatureRequest {
                        month,
                        wallet_address,
                    },
                )
                .await;

                let sig = match sig_result {
                    Ok(s) => s,
                    Err(_) => {
                        is_claiming.set(false);
                        return;
                    }
                };

                let claim_result = claim_tokens(ClaimTokensParams {
                    contract_address: sig.contract_address,
                    chain_id: sig.chain_id,
                    month_index: sig.month_index,
                    amount: sig.amount,
                    max_claimable: sig.max_claimable,
                    nonce: sig.nonce,
                    deadline: sig.deadline,
                    signature: sig.signature,
                })
                .await;

                if claim_result.is_ok() {
                    is_claimed.set(true);
                }
            }

            is_claiming.set(false);
        }
    };

    let has_next = next_bookmark.read().is_some();
    let is_loading_value = *is_loading.read();
    let is_expanded = *expanded.read();
    let claiming = *is_claiming.read();
    let claimed = *is_claimed.read();

    rsx! {
        div { class: "overflow-hidden rounded-xl bg-card-bg",
            div {
                class: "flex flex-row justify-between items-center py-4 px-4 w-full cursor-pointer",
                onclick: on_toggle,
                div { class: "flex flex-col gap-1",
                    span { class: "text-sm font-semibold text-text-primary", "{item.month}" }
                    span { class: "text-xs text-foreground-muted",
                        "{format_points(item.total_earned)} P → {format_tokens(estimated_tokens)} Token"
                    }
                }
                div { class: "flex flex-row gap-3 items-center shrink-0",
                    if contract_address.is_some() && is_logged_in {
                        if claimed {
                            Badge {
                                color: BadgeColor::Grey,
                                size: BadgeSize::Normal,
                                "Claimed"
                            }
                        } else {
                            Button {
                                size: ButtonSize::Small,
                                style: ButtonStyle::Primary,
                                disabled: claiming,
                                onclick: on_claim,
                                if claiming {
                                    "{tr.loading}"
                                } else {
                                    "Claim"
                                }
                            }
                        }
                    }
                    if is_expanded {
                        lucide_dioxus::ChevronUp { class: "w-4 h-4 text-foreground-muted" }
                    } else {
                        lucide_dioxus::ChevronDown { class: "w-4 h-4 text-foreground-muted" }
                    }
                }
            }

            if is_expanded {
                div { class: "px-4 pb-4",
                    if is_loading_value && !*loaded.read() {
                        div { class: "py-4 text-sm text-center text-foreground-muted",
                            "{tr.loading}"
                        }
                    } else {
                        {transaction_list(&tr, transactions.read().as_slice(), false, false)}

                        if has_next {
                            button {
                                class: "py-2 mt-2 w-full text-sm font-medium text-center transition-colors disabled:opacity-50 text-foreground-muted hover:text-text-primary",
                                onclick: on_load_more,
                                disabled: is_loading_value,
                                if is_loading_value {
                                    "{tr.loading}"
                                } else {
                                    "{tr.load_more}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
