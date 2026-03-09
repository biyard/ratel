// FIXME: this page is not working. need to test and remove unused tag.
#![allow(unused)]
use crate::features::spaces::apps::incentive_pool::components::{DistributionModeCard, IconActionButton, SectionCard, SummaryStatCard};
use crate::features::spaces::apps::incentive_pool::i18n::IncentivePoolTranslate;
use crate::features::spaces::apps::incentive_pool::interop::{copy_text as copy_to_clipboard, open_url};
use crate::features::spaces::apps::incentive_pool::models::{SpaceIncentive, SpaceIncentiveToken};
use crate::features::spaces::apps::incentive_pool::utils::format::{
    default_usdt_token_address, format_token_balance, incentive_explorer_url, is_valid_usdt_input,
    usdt_tokens,
};
use crate::features::spaces::apps::incentive_pool::utils::service::{load_incentive_and_tokens, refresh_tokens, register_incentive_pool};
use crate::features::spaces::apps::incentive_pool::*;
use crate::common::components::{Button, ButtonStyle};

const DEFAULT_RECIPIENT_COUNT: i64 = 10;
const MIX_RANKING_BPS: i64 = 7000;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DistributionMode {
    Top10RankOnly,
    HighScoreRandom,
    Mix,
}

impl DistributionMode {
    fn to_contract_mode(self) -> i64 {
        match self {
            DistributionMode::HighScoreRandom => 0,
            DistributionMode::Top10RankOnly => 1,
            DistributionMode::Mix => 2,
        }
    }

    fn to_ranking_bps(self) -> i64 {
        match self {
            DistributionMode::Mix => MIX_RANKING_BPS,
            _ => 0,
        }
    }
}

#[component]
pub fn IncentivePoolPage(space_id: SpacePartition) -> Element {
    let tr: IncentivePoolTranslate = use_translate();

    let mut distribution_mode = use_signal(|| DistributionMode::Top10RankOnly);
    let incentive = use_signal(|| Option::<SpaceIncentive>::None);
    let tokens = use_signal(Vec::<SpaceIncentiveToken>::new);
    let mut selected_token = use_signal(|| Option::<String>::None);
    let mut deposit_amount = use_signal(String::new);

    let notice = use_signal(|| Option::<String>::None);
    let mut deposit_validation = use_signal(|| Option::<String>::None);

    let load_incentive_failed_notice = tr.load_incentive_failed.to_string();

    let mut did_load = use_signal(|| false);
    let is_loading = use_signal(|| false);
    let is_registering = use_signal(|| false);
    let is_refreshing = use_signal(|| false);

    let space_id_for_load = space_id.clone();
    use_effect(move || {
        if did_load() {
            return;
        }
        did_load.set(true);

        let mut incentive = incentive.clone();
        let mut tokens = tokens.clone();
        let mut selected_token = selected_token.clone();
        let mut notice = notice.clone();
        let mut is_loading = is_loading.clone();
        let load_incentive_failed_notice = load_incentive_failed_notice.clone();
        let space_id = space_id_for_load.clone();

        spawn(async move {
            is_loading.set(true);
            match load_incentive_and_tokens(space_id).await {
                Ok((loaded_incentive, loaded_tokens)) => {
                    incentive.set(loaded_incentive);
                    selected_token.set(default_usdt_token_address(&loaded_tokens));
                    tokens.set(loaded_tokens);
                }
                Err(err) => {
                    error!("Failed to load incentive pool data: {:?}", err);
                    notice.set(Some(load_incentive_failed_notice));
                }
            }
            is_loading.set(false);
        });
    });

    let token_items = usdt_tokens(&tokens());
    let total_deposit_amount = token_items
        .first()
        .map(|item| format_token_balance(&item.balance, item.decimals))
        .unwrap_or_else(|| "0".to_string());

    let incentive_address = incentive()
        .as_ref()
        .map(|item| item.contract_address.clone())
        .unwrap_or_default();
    let show_incentive_address = if is_loading() && incentive_address.is_empty() {
        tr.loading.to_string()
    } else {
        incentive_address.clone()
    };

    let confirm_label = if is_registering() {
        tr.registering.to_string()
    } else {
        tr.confirm_setup.to_string()
    };
    let invalid_usdt_amount_notice = tr.invalid_usdt_amount.to_string();

    rsx! {
        div { class: "flex overflow-visible flex-col gap-5 self-start pb-6 w-full min-w-0 shrink-0 max-w-[1024px] max-tablet:gap-4 text-web-font-primary",
            h3 { class: "font-bold font-raleway text-[24px]/[28px] tracking-[-0.24px] text-web-font-primary",
                {tr.page_title}
            }

            if let Some(message) = notice() {
                p { class: "px-4 py-3 text-sm border rounded-[8px] border-separator bg-card text-card-meta",
                    "{message}"
                }
            }

            div { class: "p-4 w-full rounded-[12px] bg-card max-mobile:p-3",
                div { class: "flex gap-3 justify-between items-start",
                    div { class: "flex flex-col items-start min-w-0 gap-[10px]",
                        div { class: "flex justify-center items-center w-11 h-11 bg-violet-500 rounded-[10px]",
                            icons::ratel::Chest {
                                width: "24",
                                height: "24",
                                class: "text-web-font-primary [&>path]:fill-none [&>path]:stroke-current",
                            }
                        }
                        p { class: "font-bold leading-5 font-raleway text-[17px] tracking-[-0.18px] text-web-font-primary",
                            {tr.header_title}
                        }
                        p { class: "w-full font-medium leading-4 font-raleway text-[12px] tracking-[0] text-card-meta",
                            {tr.header_description}
                        }
                    }

                    button { class: "flex justify-center items-center w-6 h-6 text-web-font-neutral",
                        icons::validations::Extra {
                            width: "20",
                            height: "20",
                            class: "[&>circle]:fill-current",
                        }
                    }
                }
            }

            SectionCard {
                title: tr.section_incentive_pool.to_string(),
                title_class: "font-bold font-raleway text-[24px]/[28px] tracking-[-0.24px] text-web-font-primary",
                body_class: "flex flex-col gap-4 p-5 bg-card max-mobile:p-4",

                div { class: "flex flex-col gap-2 items-start",
                    p { class: "font-bold leading-5 font-raleway text-[17px] tracking-[-0.18px] text-web-font-primary",
                        {tr.incentive_pool_address}
                    }
                    div { class: "flex gap-2 items-center w-full max-tablet:flex-wrap",
                        div { class: "flex flex-1 items-center w-full min-w-0 h-11 border-gray-600 rounded-[8px] border-[0.5px] bg-web-input",
                            input {
                                class: "flex-1 px-3 min-w-0 h-full font-medium bg-transparent outline-none font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-primary",
                                value: "{show_incentive_address}",
                                readonly: true,
                            }

                            {
                                let address = incentive_address.clone();
                                rsx! {
                                    button {
                                        class: "flex justify-center items-center w-11 h-full shrink-0 rounded-r-[8px] text-web-font-neutral disabled:opacity-50",
                                        disabled: address.trim().is_empty(),
                                        onclick: move |_| {
                                            if let Some(url) = incentive_explorer_url(&address) {
                                                open_url(&url);
                                            }
                                        },
                                        icons::arrows::ExpandPage { width: "20", height: "20", class: "[&>path]:stroke-current" }
                                    }
                                }
                            }
                        }

                        {
                            let address = incentive_address.clone();
                            let notice = notice.clone();
                            let copied_address_notice = tr.address_copied.to_string();
                            let copy_address_failed_notice = tr.copy_address_failed.to_string();
                            rsx! {
                                IconActionButton {
                                    disabled: address.trim().is_empty(),
                                    onclick: move |_| {
                                        if address.trim().is_empty() {
                                            return;
                                        }

                                        let mut notice = notice.clone();
                                        let address = address.clone();
                                        let copied_address_notice = copied_address_notice.clone();
                                        let copy_address_failed_notice = copy_address_failed_notice.clone();
                                        spawn(async move {
                                            match copy_to_clipboard(address).await {
                                                Ok(_) => notice.set(Some(copied_address_notice)),
                                                Err(err) => {
                                                    error!("Failed to copy address: {:?}", err);
                                                    notice.set(Some(copy_address_failed_notice));
                                                }
                                            }
                                        });
                                    },
                                    icons::notes_clipboard::Clipboard { width: "24", height: "24", class: "[&>path]:stroke-current" }
                                }
                            }
                        }

                        {
                            let mut notice = notice.clone();
                            let mut is_refreshing = is_refreshing.clone();
                            let tokens = tokens.clone();
                            let selected_token = selected_token.clone();
                            let space_id = space_id.clone();
                            let has_incentive = incentive().is_some();
                            let refresh_tokens_failed_notice = tr.refresh_tokens_failed.to_string();
                            rsx! {
                                IconActionButton {
                                    disabled: is_refreshing() || !has_incentive,
                                    onclick: move |_| {
                                        if is_refreshing() || !has_incentive {
                                            return;
                                        }

                                        is_refreshing.set(true);
                                        notice.set(None);

                                        let mut notice = notice.clone();
                                        let mut is_refreshing = is_refreshing.clone();
                                        let mut tokens = tokens.clone();
                                        let mut selected_token = selected_token.clone();
                                        let space_id = space_id.clone();

                                        let refresh_tokens_failed_notice = refresh_tokens_failed_notice.clone();
                                        spawn(async move {
                                            match refresh_tokens(space_id).await {
                                                Ok(loaded_tokens) => {
                                                    selected_token.set(default_usdt_token_address(&loaded_tokens));
                                                    tokens.set(loaded_tokens);
                                                }
                                                Err(err) => {
                                                    error!("Failed to refresh incentive tokens: {:?}", err);
                                                    notice.set(Some(refresh_tokens_failed_notice));
                                                }
                                            }
                                            is_refreshing.set(false);
                                        });
                                    },
                                    icons::arrows::Repost { width: "24", height: "24", class: "[&>path]:stroke-current" }
                                }
                            }
                        }
                    }
                }

                div { class: "grid grid-cols-2 w-full gap-[10px] max-tablet:grid-cols-1",
                    SummaryStatCard {
                        title: tr.total_winners.to_string(),
                        badge: Some(tr.rank_rate.to_string()),
                        value: DEFAULT_RECIPIENT_COUNT.to_string(),
                        unit: tr.people.to_string(),
                    }
                    SummaryStatCard {
                        title: tr.total_deposit_amount.to_string(),
                        badge: None,
                        value: total_deposit_amount,
                        unit: "USDT".to_string(),
                    }
                }
            }

            SectionCard {
                title: tr.deposit_in_incentive_pool.to_string(),
                title_class: "font-bold font-raleway text-[24px]/[28px] tracking-[-0.24px] text-web-font-primary",
                body_class: "flex flex-col items-start p-5 gap-[10px] bg-card max-mobile:p-4",

                div { class: "flex flex-col gap-2 justify-center items-start w-full",
                    p { class: "font-bold leading-5 font-raleway text-[17px] tracking-[-0.18px] text-web-font-primary",
                        {tr.incentive_token}
                    }
                    select {
                        class: "flex justify-between items-center px-3 w-full h-11 font-medium border-gray-600 rounded-[8px] border-[0.5px] bg-web-input font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-primary disabled:text-web-font-neutral",
                        disabled: token_items.is_empty(),
                        value: selected_token().unwrap_or_default(),
                        onchange: move |evt| {
                            let value = evt.value().to_string();
                            if value.is_empty() {
                                selected_token.set(None);
                            } else {
                                selected_token.set(Some(value));
                            }
                        },
                        if token_items.is_empty() {
                            option { value: "", {tr.select} }
                        } else {
                            for token in token_items.iter() {
                                option {
                                    key: "{token.token_address}",
                                    value: "{token.token_address}",
                                    "{token.symbol}"
                                }
                            }
                        }
                    }
                    p { class: "font-normal text-gray-500 font-inter text-[12px] leading-[16px]",
                        {tr.token_distribution_hint}
                    }
                }

                div { class: "flex flex-col gap-2 justify-center items-start w-full",
                    p { class: "font-bold leading-5 font-raleway text-[17px] tracking-[-0.18px] text-web-font-primary",
                        {tr.deposit_amount}
                    }
                    div { class: "relative w-full",
                        {
                            let invalid_usdt_amount = invalid_usdt_amount_notice.clone();
                            rsx! {
                                input {
                                    class: "px-3 w-full h-12 font-medium text-right border-gray-600 rounded-[8px] border-[0.5px] bg-web-input pr-[68px] font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-primary",
                                    value: "{deposit_amount}",
                                    oninput: move |evt| {
                                        let value = evt.value().to_string();
                                        if is_valid_usdt_input(&value) {
                                            deposit_validation.set(None);
                                            deposit_amount.set(value);
                                        } else {
                                            deposit_validation.set(Some(invalid_usdt_amount.clone()));
                                        }
                                    },
                                }
                            }
                        }
                        span { class: "absolute right-3 top-1/2 font-normal text-gray-500 -translate-y-1/2 pointer-events-none font-inter text-[12px] leading-[16px]",
                            "USDT"
                        }
                    }
                    if let Some(msg) = deposit_validation() {
                        p { class: "font-normal text-red-400 font-inter text-[12px] leading-[16px]",
                            "{msg}"
                        }
                    } else {
                        p { class: "font-normal text-gray-500 font-inter text-[12px] leading-[16px]",
                            {tr.deposit_amount_hint}
                        }
                    }
                }
            }

            SectionCard {
                title: tr.distribution.to_string(),
                title_class: "font-semibold leading-5 text-center font-raleway text-[17px] tracking-[-0.18px] text-web-font-primary",
                body_class: "flex flex-col items-start p-5 gap-[10px] bg-card max-mobile:p-4",

                div { class: "flex flex-col gap-2 items-start w-full",
                    p { class: "font-semibold font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-primary",
                        {tr.weight_sampling}
                    }

                    div { class: "grid grid-cols-3 gap-2 w-full max-tablet:grid-cols-1",
                        DistributionModeCard {
                            selected: distribution_mode() == DistributionMode::Top10RankOnly,
                            title: tr.top_10_rank_only.to_string(),
                            description: tr.rank_100.to_string(),
                            onclick: move |_| distribution_mode.set(DistributionMode::Top10RankOnly),
                        }
                        DistributionModeCard {
                            selected: distribution_mode() == DistributionMode::HighScoreRandom,
                            title: tr.high_score_random.to_string(),
                            description: tr.ranking_lottery.to_string(),
                            onclick: move |_| distribution_mode.set(DistributionMode::HighScoreRandom),
                        }
                        DistributionModeCard {
                            selected: distribution_mode() == DistributionMode::Mix,
                            title: tr.mix.to_string(),
                            description: tr.rank_70_random_30.to_string(),
                            onclick: move |_| distribution_mode.set(DistributionMode::Mix),
                        }
                    }
                }

                div { class: "flex gap-5 items-start p-5 w-full border border-gray-600 h-[102px] rounded-[12px] bg-neutral-800 max-tablet:h-auto",
                    icons::security::ShieldLock {
                        width: "18",
                        height: "18",
                        class: "mt-0.5 shrink-0 text-card-meta [&>path]:stroke-current",
                    }

                    div { class: "flex flex-col flex-1 gap-0.5 items-start min-w-0",
                        p { class: "w-full font-semibold leading-5 font-raleway text-[17px] tracking-[-0.18px] text-web-font-primary",
                            {tr.mainnet_and_funding}
                        }
                        p { class: "w-full font-medium leading-5 font-raleway text-[13px] tracking-[0] text-card-meta",
                            {tr.network_support_note}
                        }
                        p { class: "w-full font-medium leading-5 font-raleway text-[13px] tracking-[0] text-card-meta",
                            {tr.external_funding_note}
                        }
                    }
                }

                div { class: "flex justify-end pt-3 w-full max-tablet:justify-stretch",
                    {
                        let mut is_registering = is_registering.clone();
                        let mut notice = notice.clone();
                        let incentive = incentive.clone();
                        let tokens = tokens.clone();
                        let selected_token = selected_token.clone();
                        let mut deposit_validation = deposit_validation.clone();
                        let space_id = space_id.clone();
                        let mode = distribution_mode();
                        let deposit_amount_value = deposit_amount();
                        let invalid_usdt_amount = tr.invalid_usdt_amount.to_string();
                        let incentive_pool_registered = tr.incentive_pool_registered.to_string();

                        let register_incentive_failed_notice = tr.register_incentive_failed.to_string();
                        rsx! {
                            Button {
                                class: "inline-flex justify-center items-center self-stretch border w-[146px] max-tablet:w-full",
                                style: ButtonStyle::Secondary,
                                onclick: move |_| {
                                    if is_registering() || is_loading() {
                                        return;
                                    }
                                    if !is_valid_usdt_input(&deposit_amount_value) {
                                        deposit_validation.set(Some(invalid_usdt_amount.clone()));
                                        return;
                                    }

                                    is_registering.set(true);
                                    notice.set(None);

                                    let mut is_registering = is_registering.clone();
                                    let mut notice = notice.clone();
                                    let mut incentive = incentive.clone();
                                    let mut tokens = tokens.clone();
                                    let mut selected_token = selected_token.clone();
                                    let space_id = space_id.clone();
                                    let incentive_pool_registered = incentive_pool_registered.clone();

                                    let register_incentive_failed_notice = register_incentive_failed_notice.clone();
                                    spawn(async move {
                                        match register_incentive_pool(
                                                space_id,
                                                mode.to_contract_mode(),
                                                mode.to_ranking_bps(),
                                                DEFAULT_RECIPIENT_COUNT,
                                            )
                                            .await
                                        {
                                            Ok((created, loaded_tokens)) => {
                                                selected_token.set(default_usdt_token_address(&loaded_tokens));
                                                incentive.set(Some(created));
                                                tokens.set(loaded_tokens);
                                                notice.set(Some(incentive_pool_registered));
                                            }
                                            Err(err) => {
                                                error!("Failed to register incentive pool: {:?}", err);
                                                notice.set(Some(register_incentive_failed_notice));
                                            }
                                        }
                                        is_registering.set(false);
                                    });
                                },
                                "{confirm_label}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let tr: IncentivePoolTranslate = use_translate();
    let role =
        use_loader(move || async move { Ok::<SpaceUserRole, Error>(SpaceUserRole::Creator) })?;

    if role() == SpaceUserRole::Creator {
        rsx! {
            div { class: "flex flex-col items-center justify-center w-full h-full",
                h1 { class: "text-2xl font-bold", "Incentive App" }
                p { class: "mt-2 text-gray-500", "Coming soon..." }
            }
        }
    } else {
        rsx! {
            div { class: "flex justify-center items-center w-full h-full text-web-font-primary",
                {tr.no_permission}
            }
        }
    }
}
