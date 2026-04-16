mod i18n;

pub use i18n::*;

use crate::common::*;
use crate::common::wasm_bindgen::prelude::JsValue;
use crate::common::wasm_bindgen_futures::JsFuture;
use crate::features::auth::LoginModal;
use crate::features::membership::components::*;
use crate::features::membership::controllers::{
    ChangeTeamMembershipRequest, TeamIdentificationRequest, change_team_membership_handler,
    get_team_membership_handler, identify_team_handler,
};
#[cfg(not(feature = "server"))]
use crate::features::membership::interop::request_identity_verification;
use crate::features::membership::models::MembershipTier as ApiMembershipTier;
use crate::features::membership::models::{CardInfo, Currency};

fn format_js_error(err: JsValue) -> String {
    err.as_string().unwrap_or_else(|| format!("{:?}", err))
}

fn display_amount_for_tier(tier: MembershipTier, is_ko: bool) -> i64 {
    let base_amount = match tier {
        MembershipTier::Pro => 20,
        MembershipTier::Max => 50,
        MembershipTier::Vip => 100,
        _ => 0,
    };
    if is_ko { base_amount * 1500 } else { base_amount }
}

fn enterprise_contact() {
    let email = "hi@ratel.foundation";
    let subject = "Enterprise Membership Inquiry";
    let body = "Hello,%0D%0A%0D%0AI would like to learn more about the Enterprise membership plan for our team.%0D%0A%0D%0AThank you.";
    let mailto_url = format!("mailto:{}?subject={}&body={}", email, subject, body);
    let _ = web_sys::window().and_then(|w| w.open_with_url(&mailto_url).ok());
}

fn api_tier_to_ui(tier: &ApiMembershipTier) -> MembershipTier {
    match tier {
        ApiMembershipTier::Free => MembershipTier::Free,
        ApiMembershipTier::Pro => MembershipTier::Pro,
        ApiMembershipTier::Max => MembershipTier::Max,
        ApiMembershipTier::Vip => MembershipTier::Vip,
        ApiMembershipTier::Enterprise(_) => MembershipTier::Enterprise,
    }
}

fn partition_tier_to_ui(raw: &str) -> MembershipTier {
    let tier = raw.strip_prefix("MEMBERSHIP#").unwrap_or(raw);
    if tier.starts_with("ENTERPRISE#") || tier.eq_ignore_ascii_case("enterprise") {
        MembershipTier::Enterprise
    } else if tier.eq_ignore_ascii_case("pro") {
        MembershipTier::Pro
    } else if tier.eq_ignore_ascii_case("max") {
        MembershipTier::Max
    } else if tier.eq_ignore_ascii_case("vip") {
        MembershipTier::Vip
    } else {
        MembershipTier::Free
    }
}

#[component]
pub fn SubscriptionPage(username: ReadSignal<String>) -> Element {
    // Only admins/owners can access subscription page — read from team context.
    let team_ctx = crate::common::contexts::use_team_context();
    let is_admin = {
        let teams = team_ctx.teams.read();
        teams.iter().find(|t| t.username == username()).map_or(false, |t| {
            let mut mask = 0i64;
            for v in &t.permissions {
                mask |= 1i64 << (*v as i32);
            }
            crate::features::social::pages::member::dto::TeamRole::from_legacy_permissions(mask)
                .is_admin_or_owner()
        })
    };
    if !is_admin {
        return rsx! {
            super::ViewerPage { username: username() }
        };
    }

    let tr: TeamSubscriptionTranslate = use_translate();
    let lang = use_language();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let mut popup = use_popup();
    let mut toast = use_toast();

    let is_ko = matches!(lang(), Language::Ko);
    let portone_store_id = option_env!("PORTONE_STORE_ID").unwrap_or_default().to_string();
    let portone_channel_key = option_env!("PORTONE_INICIS_CHANNEL_KEY")
        .unwrap_or_default()
        .to_string();

    // Fetch the team's current membership so we can mark the matching tier
    // card as the current plan (disabled button).
    let mut current_membership = use_loader(move || async move {
        Ok::<_, crate::common::Error>(get_team_membership_handler(username()).await.ok())
    })?;
    let current_tier: MembershipTier = current_membership()
        .map(|m| partition_tier_to_ui(&m.tier.0))
        .unwrap_or(MembershipTier::Free);

    let on_tier_click = use_callback(move |tier: MembershipTier| {
        let portone_store_id = portone_store_id.clone();
        let portone_channel_key = portone_channel_key.clone();
        async move {
        if !user_ctx().is_logged_in() {
            popup
                .open(rsx! {
                    LoginModal {}
                })
                .with_title("Join the Movement");
            return;
        }
        match tier {
            MembershipTier::Enterprise => {
                enterprise_contact();
            }
            MembershipTier::Pro | MembershipTier::Max | MembershipTier::Vip => {
                let display_amount = display_amount_for_tier(tier, is_ko);
                let prefix = user_ctx()
                    .user
                    .as_ref()
                    .map(|u| u.pk.to_string())
                    .unwrap_or_default();
                let mut popup_modal = popup;
                let mut popup_receipt = popup;
                let team_username = username();
                let identity_id = {
                    #[cfg(feature = "server")]
                    {
                        error!("identity verification is not available on server");
                        return;
                    }
                    #[cfg(not(feature = "server"))]
                    {
                        let promise = request_identity_verification(
                            &portone_store_id,
                            &portone_channel_key,
                            &prefix,
                        );
                        match JsFuture::from(promise).await {
                            Ok(val) => match val.as_string() {
                                Some(id) => id,
                                None => {
                                    error!("Failed to request identity verification: missing id");
                                    return;
                                }
                            },
                            Err(err) => {
                                error!(
                                    "Failed to request identity verification: {:?}",
                                    format_js_error(err)
                                );
                                return;
                            }
                        }
                    }
                };
                let customer = match identify_team_handler(
                    team_username.clone(),
                    TeamIdentificationRequest { id: identity_id },
                )
                .await
                {
                    Ok(resp) => resp,
                    Err(err) => {
                        error!("Failed to verify identity: {:?}", err);
                        return;
                    }
                };
                popup_modal
                    .open(rsx! {
                        MembershipPurchaseModal {
                            membership: tier,
                            display_amount,
                            customer,
                            on_confirm: move |info: CustomerInfo| {
                                let team_username = team_username.clone();
                                async move {
                                    let req = ChangeTeamMembershipRequest {
                                        membership: match tier {
                                            MembershipTier::Pro => ApiMembershipTier::Pro,
                                            MembershipTier::Max => ApiMembershipTier::Max,
                                            MembershipTier::Vip => ApiMembershipTier::Vip,
                                            MembershipTier::Enterprise => {
                                                ApiMembershipTier::Enterprise("Enterprise".to_string())
                                            }
                                            MembershipTier::Free => ApiMembershipTier::Free,
                                        },
                                        currency: Currency::Krw,
                                        card_info: Some(CardInfo {
                                            card_number: info.card_number,
                                            expiry_year: info.expiry_year,
                                            expiry_month: info.expiry_month,
                                            birth_or_business_registration_number: info.birth_or_biz,
                                            password_two_digits: info.card_password,
                                        }),
                                    };
                                    match change_team_membership_handler(team_username, req).await {
                                        Ok(resp) => {
                                            let Some(membership_resp) = resp.membership else {
                                                toast.error(crate::common::Error::MembershipResponseMissing);
                                                popup_modal.close();
                                                return;
                                            };
                                            // Refresh the current-tier loader so the
                                            // "Current Plan" badge moves to the new tier.
                                            current_membership.restart();
                                            if let Some(receipt_resp) = resp.receipt {
                                                let receipt = MembershipReceiptData {
                                                    tx_id: receipt_resp.tx_id,
                                                    membership: api_tier_to_ui(&membership_resp.tier),
                                                    amount: receipt_resp.amount,
                                                    duration_days: membership_resp.duration_days as i64,
                                                    credits: membership_resp.credits,
                                                    paid_at: receipt_resp.paid_at,
                                                };
                                                popup_receipt.open(rsx! {
                                                    MembershipReceiptModal { receipt }
                                                });
                                            } else {
                                                toast.info("Membership downgrade scheduled");
                                                popup_modal.close();
                                            }
                                        }
                                        Err(err) => {
                                            error!("Failed to change team membership: {:?}", err);
                                            toast.error(crate::common::Error::MembershipChangeFailed);
                                            popup_modal.close();
                                        }
                                    }
                                }
                            },
                        }
                    })
                    .without_close();
            }
            MembershipTier::Free => {}
        }
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        document::Script { src: "https://cdn.portone.io/v2/browser-sdk.js" }

        div { class: "ts-sub-page",
            // Section label
            div { class: "section-label",
                span { class: "section-label__dash" }
                span { class: "section-label__title",
                    "{tr.section_label_prefix} "
                    strong { "{tr.section_label_strong}" }
                }
                span { class: "section-label__dash" }
            }

            // Hero
            div { class: "hero",
                h1 { class: "hero__title",
                    "{tr.hero_title_en}"
                    span { class: "hero__title-ko", "{tr.hero_title_ko}" }
                }
                p { class: "hero__desc",
                    "{tr.hero_desc_prefix}"
                    strong { "{tr.hero_desc_credits}" }
                    "{tr.hero_desc_middle}"
                    em { "{tr.hero_desc_reward_space}" }
                    "{tr.hero_desc_suffix}"
                }
            }

            // Page
            div { class: "page",
                div { class: "tier-grid",
                    // Free
                    TierCard {
                        tier: MembershipTier::Free,
                        current_tier,
                        name: tr.free_name.to_string(),
                        desc: tr.free_desc.to_string(),
                        features: vec![
                            FeatureLine::Plain(tr.free_feat_1.to_string()),
                            FeatureLine::Plain(tr.free_feat_2.to_string()),
                            FeatureLine::Plain(tr.free_feat_3.to_string()),
                            FeatureLine::Plain(tr.free_feat_4.to_string()),
                        ],
                        price: PriceDisplay::Amount(tr.free_price_amount.to_string()),
                        on_click: move |_| {},
                    }

                    // Pro
                    TierCard {
                        tier: MembershipTier::Pro,
                        current_tier,
                        name: tr.pro_name.to_string(),
                        desc: tr.pro_desc.to_string(),
                        features: vec![
                            FeatureLine::Plain(tr.pro_feat_1.to_string()),
                            FeatureLine::Strong {
                                prefix: tr.pro_feat_2_prefix.to_string(),
                                strong: tr.pro_feat_2_strong.to_string(),
                                suffix: tr.pro_feat_2_suffix.to_string(),
                            },
                            FeatureLine::Plain(tr.pro_feat_3.to_string()),
                            FeatureLine::Plain(tr.pro_feat_4.to_string()),
                        ],
                        price: PriceDisplay::Monthly {
                            prefix: tr.price_prefix.to_string(),
                            amount: "30,000".to_string(),
                            suffix: tr.price_suffix_krw.to_string(),
                        },
                        cta_label: tr.cta_apply_pro.to_string(),
                        on_click: move |_| on_tier_click(MembershipTier::Pro),
                    }

                    // Max
                    TierCard {
                        tier: MembershipTier::Max,
                        current_tier,
                        name: tr.max_name.to_string(),
                        desc: tr.max_desc.to_string(),
                        features: vec![
                            FeatureLine::Plain(tr.max_feat_1.to_string()),
                            FeatureLine::Strong {
                                prefix: tr.max_feat_2_prefix.to_string(),
                                strong: tr.max_feat_2_strong.to_string(),
                                suffix: tr.max_feat_2_suffix.to_string(),
                            },
                            FeatureLine::Plain(tr.max_feat_3.to_string()),
                            FeatureLine::Plain(tr.max_feat_4.to_string()),
                            FeatureLine::Plain(tr.max_feat_5.to_string()),
                        ],
                        price: PriceDisplay::Monthly {
                            prefix: tr.price_prefix.to_string(),
                            amount: "75,000".to_string(),
                            suffix: tr.price_suffix_krw.to_string(),
                        },
                        cta_label: tr.cta_apply_max.to_string(),
                        on_click: move |_| on_tier_click(MembershipTier::Max),
                    }

                    // VIP
                    TierCard {
                        tier: MembershipTier::Vip,
                        current_tier,
                        name: tr.vip_name.to_string(),
                        desc: tr.vip_desc.to_string(),
                        features: vec![
                            FeatureLine::Plain(tr.vip_feat_1.to_string()),
                            FeatureLine::Strong {
                                prefix: tr.vip_feat_2_prefix.to_string(),
                                strong: tr.vip_feat_2_strong.to_string(),
                                suffix: tr.vip_feat_2_suffix.to_string(),
                            },
                            FeatureLine::Plain(tr.vip_feat_3.to_string()),
                            FeatureLine::Plain(tr.vip_feat_4.to_string()),
                            FeatureLine::Plain(tr.vip_feat_5.to_string()),
                            FeatureLine::Plain(tr.vip_feat_6.to_string()),
                        ],
                        price: PriceDisplay::Monthly {
                            prefix: tr.price_prefix.to_string(),
                            amount: "150,000".to_string(),
                            suffix: tr.price_suffix_krw.to_string(),
                        },
                        cta_label: tr.cta_apply_vip.to_string(),
                        ribbon: Some(tr.vip_ribbon.to_string()),
                        on_click: move |_| on_tier_click(MembershipTier::Vip),
                    }
                }

                // Enterprise wide card
                EnterpriseCard {
                    name: tr.enterprise_name.to_string(),
                    desc: tr.enterprise_desc.to_string(),
                    feat_1: tr.enterprise_feat_1.to_string(),
                    feat_2: tr.enterprise_feat_2.to_string(),
                    price_prefix: tr.enterprise_price.to_string(),
                    price_amount: tr.enterprise_price_amount.to_string(),
                    cta_label: tr.enterprise_cta.to_string(),
                    on_click: move |_| on_tier_click(MembershipTier::Enterprise),
                }

                // Footer note
                div { class: "footer-note", "{tr.footer_note}" }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum FeatureLine {
    Plain(String),
    Strong {
        prefix: String,
        strong: String,
        suffix: String,
    },
}

#[derive(Clone, PartialEq)]
enum PriceDisplay {
    Amount(String),
    Monthly {
        prefix: String,
        amount: String,
        suffix: String,
    },
}

#[component]
fn TierCard(
    tier: MembershipTier,
    current_tier: MembershipTier,
    name: String,
    desc: String,
    features: Vec<FeatureLine>,
    price: PriceDisplay,
    #[props(default)] cta_label: Option<String>,
    #[props(default)] ribbon: Option<String>,
    on_click: EventHandler<()>,
) -> Element {
    let tr: TeamSubscriptionTranslate = use_translate();
    let is_current = tier == current_tier;

    let mod_class = match tier {
        MembershipTier::Pro => " tier-card--pro",
        MembershipTier::Max => " tier-card--max",
        MembershipTier::Vip => " tier-card--vip",
        _ => "",
    };

    rsx! {
        div { class: "tier-card{mod_class}",
            if let Some(label) = ribbon.as_ref() {
                div { class: "tier-card__ribbon", "{label}" }
            }

            div { class: "tier-card__head",
                div { class: "tier-card__name", "{name}" }
                div { class: "tier-card__desc", "{desc}" }
            }

            div { class: "tier-card__features",
                for (idx , line) in features.iter().enumerate() {
                    div { key: "{idx}", class: "feature",
                        span { class: "feature__dot" }
                        match line {
                            FeatureLine::Plain(s) => rsx! {
                                span { "{s}" }
                            },
                            FeatureLine::Strong { prefix, strong, suffix } => rsx! {
                                span {
                                    "{prefix}"
                                    strong { "{strong}" }
                                    "{suffix}"
                                }
                            },
                        }
                    }
                }
            }

            div { class: "tier-card__price",
                match &price {
                    PriceDisplay::Amount(amt) => rsx! {
                        span { class: "tier-card__price-amt", "{amt}" }
                    },
                    PriceDisplay::Monthly { prefix, amount, suffix } => rsx! {
                        span { class: "tier-card__price-prefix", "{prefix}" }
                        span { class: "tier-card__price-amt", "{amount}" }
                        span { class: "tier-card__price-suffix", "{suffix}" }
                    },
                }
            }

            if is_current {
                button { class: "tier-btn tier-btn--current", disabled: true,
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "20 6 9 17 4 12" }
                    }
                    "{tr.cta_current}"
                }
            } else if let Some(label) = cta_label.as_ref() {
                button {
                    class: "tier-btn",
                    r#type: "button",
                    onclick: move |_| on_click.call(()),
                    "{label}"
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line {
                            x1: "5",
                            y1: "12",
                            x2: "19",
                            y2: "12",
                        }
                        polyline { points: "12 5 19 12 12 19" }
                    }
                }
            }
        }
    }
}

#[component]
fn EnterpriseCard(
    name: String,
    desc: String,
    feat_1: String,
    feat_2: String,
    price_prefix: String,
    price_amount: String,
    cta_label: String,
    on_click: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "enterprise-card",
            div { class: "enterprise-card__left",
                div { class: "enterprise-card__name", "{name}" }
                div { class: "enterprise-card__desc", "{desc}" }
            }
            div { class: "enterprise-card__features",
                div { class: "feature",
                    span { class: "feature__dot" }
                    span { "{feat_1}" }
                }
                div { class: "feature",
                    span { class: "feature__dot" }
                    span { "{feat_2}" }
                }
            }
            div { class: "enterprise-card__cta",
                div { class: "enterprise-card__price",
                    "{price_prefix}"
                    strong { "{price_amount}" }
                }
                button {
                    class: "tier-btn",
                    r#type: "button",
                    onclick: move |_| on_click.call(()),
                    "{cta_label}"
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2.5",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" }
                        polyline { points: "22,6 12,13 2,6" }
                    }
                }
            }
        }
    }
}
