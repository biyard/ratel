use crate::components::{
    CustomerInfo, MembershipCard, MembershipPlanTranslate, MembershipPurchaseModal,
    MembershipReceiptData, MembershipReceiptModal, MembershipTier, membership_plan_items,
};
use crate::controllers::{
    ChangeMembershipRequest, IdentificationRequest, change_membership_handler, identify_handler,
};
#[cfg(not(feature = "server"))]
use crate::interop::request_identity_verification;
use crate::models::MembershipTier as ApiMembershipTier;
use crate::models::{CardInfo, Currency};
use crate::*;
use common::use_toast;
use ratel_auth::LoginModal;

use common::wasm_bindgen::prelude::JsValue;
use common::wasm_bindgen_futures::JsFuture;

fn format_js_error(err: JsValue) -> String {
    if let Some(msg) = err.as_string() {
        msg
    } else {
        format!("{:?}", err)
    }
}

fn enterprise_contact() {
    let email = "hi@ratel.foundation";
    let subject = "Enterprise Membership Inquiry";
    let body = "Hello,%0D%0A%0D%0AI would like to learn more about the Enterprise membership plan.%0D%0A%0D%0AThank you.";
    let mailto_url = format!("mailto:{}?subject={}&body={}", email, subject, body);
    let _ = web_sys::window().and_then(|w| w.open_with_url(&mailto_url).ok());
}

fn display_amount_for_tier(tier: MembershipTier, is_ko: bool) -> i64 {
    let base_amount = match tier {
        MembershipTier::Pro => 20,
        MembershipTier::Max => 50,
        MembershipTier::Vip => 100,
        _ => 0,
    };

    if is_ko {
        base_amount * 1500
    } else {
        base_amount
    }
}

fn duration_days_for_tier(tier: MembershipTier) -> i64 {
    match tier {
        MembershipTier::Free => 0,
        MembershipTier::Pro => 30,
        MembershipTier::Max => 30,
        MembershipTier::Vip => 30,
        MembershipTier::Enterprise => 30,
    }
}

#[component]
pub fn MembershipPlan() -> Element {
    let lang = use_language();
    let is_ko = matches!(lang(), Language::Ko);
    let user_ctx = ratel_auth::hooks::use_user_context();
    let mut popup = use_popup();
    let mut toast = use_toast();
    let portone_store_id = option_env!("PORTONE_STORE_ID")
        .unwrap_or_default()
        .to_string();
    let portone_channel_key = option_env!("PORTONE_INICIS_CHANNEL_KEY")
        .unwrap_or_default()
        .to_string();

    let memberships = membership_plan_items(is_ko);
    let memberships_len = memberships.len();

    rsx! {
        div { class: "w-full max-w-desktop mx-auto px-4 py-8",
            MembershipPlanHeader {}
            div {
                class: "grid gap-2.5 mt-8",
                style: "grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));",
                for (idx , membership) in memberships.iter().cloned().enumerate() {
                    MembershipCard {
                        membership: membership.clone(),
                        variant: {if idx + 1 == memberships_len { "horizontal" } else { "vertical" }},
                        on_click: {
                            let portone_store_id = portone_store_id.clone();
                            let portone_channel_key = portone_channel_key.clone();
                            move |_| {
                                let membership_for_action = membership.clone();
                                if !user_ctx().is_logged_in() {
                                    popup.open(rsx! {
                                        LoginModal {}
                                    }).with_title("Join the Movement");
                                    return;
                                }
                                match membership_for_action.tier {
                                    MembershipTier::Enterprise => {
                                        enterprise_contact();
                                    }
                                    MembershipTier::Pro | MembershipTier::Max | MembershipTier::Vip => {
                                        let display_amount = display_amount_for_tier(
                                            membership_for_action.tier,
                                            is_ko,
                                        );
                                        let prefix = user_ctx()
                                            .user
                                            .as_ref()
                                            .map(|u| u.pk.to_string())
                                            .unwrap_or_default();
                                        let tier = membership_for_action.tier;
                                        let credits = membership_for_action.credits.unwrap_or_default();
                                        let mut popup_modal = popup.clone();
                                        let popup_receipt = popup.clone();
                                        let store_id = portone_store_id.clone();
                                        let channel_key = portone_channel_key.clone();
                                        spawn(async move {
                                            let identity_id = {
                                                #[cfg(feature = "server")]
                                                {
                                                    error!("identity verification is not available on server");
                                                    return;
                                                }
                                                #[cfg(not(feature = "server"))]
                                                {
                                                    let promise = request_identity_verification(
                                                        &store_id,
                                                        &channel_key,
                                                        &prefix,
                                                    );

                                                    match JsFuture::from(promise).await {
                                                        Ok(val) => {
                                                            match val.as_string() {
                                                                Some(id) => id,
                                                                None => {
                                                                    error!(
                                                                        "Failed to request identity verification: missing id"
                                                                    );
                                                                    return;
                                                                }
                                                            }
                                                        }
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
                                            let customer = match identify_handler(IdentificationRequest {
                                                    id: identity_id,
                                                })
                                                .await
                                            {
                                                Ok(resp) => resp,
                                                Err(err) => {
                                                    error!("Failed to verify identity: {:?}", err);
                                                    return;
                                                }
                                            };
                                            popup_modal.open(rsx! {
                                                MembershipPurchaseModal {
                                                    membership: tier,
                                                    display_amount,
                                                    customer,
                                                    on_confirm: move |info: CustomerInfo| {
                                                        let mut popup_receipt = popup_receipt.clone();
                                                        let mut popup_receipt_close = popup_receipt.clone();
                                                        let mut popup_modal = popup_modal.clone();
                                                        let mut toast = toast;
                                                        spawn(async move {
                                                            let req = ChangeMembershipRequest {
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
                                                            match change_membership_handler(req).await {
                                                                Ok(resp) => {
                                                                    let Some(membership_resp) = resp.membership else {
                                                                        toast.error("Membership response missing");
                                                                        popup_modal.close();
                                                                        return;
                                                                    };
                                                                    if let Some(receipt_resp) = resp.receipt {
                                                                        let membership_tier = match membership_resp.tier {
                                                                            ApiMembershipTier::Free => MembershipTier::Free,
                                                                            ApiMembershipTier::Pro => MembershipTier::Pro,
                                                                            ApiMembershipTier::Max => MembershipTier::Max,
                                                                            ApiMembershipTier::Vip => MembershipTier::Vip,
                                                                            ApiMembershipTier::Enterprise(_) => {
                                                                                MembershipTier::Enterprise
                                                                            }
                                                                        };
                                                                        let receipt = MembershipReceiptData {
                                                                            tx_id: receipt_resp.tx_id,
                                                                            membership: membership_tier,
                                                                            amount: receipt_resp.amount,
                                                                            duration_days: membership_resp.duration_days as i64,
                                                                            credits: membership_resp.credits,
                                                                            paid_at: receipt_resp.paid_at,
                                                                        };
                                                                        popup_receipt
                                                                            .open(rsx! {
                                                                            MembershipReceiptModal { receipt }
                                                                        });
                                                                    } else {
                                                                        toast.info("Membership downgrade successed");
                                                                        popup_modal.close();
                                                                    }
                                                                }
                                                                Err(err) => {
                                                                    error!("Failed to change membership: {:?}", err);
                                                                    toast.error("Failed to change membership");
                                                                    popup_modal.close();
                                                                }
                                                            }
                                                        });
                                                    },
                                                }
                                            }).without_close();
                                        });
                                    }
                                    MembershipTier::Free => {}
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
pub fn MembershipPlanHeader() -> Element {
    let tr: MembershipPlanTranslate = use_translate();

    rsx! {
        div { class: "flex flex-col gap-2.5 items-center text-center",
            h1 { class: "text-3xl md:text-4xl lg:text-5xl font-bold text-text-primary",
                {tr.title}
            }
            div {
                class: "text-[17px]/[20px] text-text-secondary",
                dangerous_inner_html: tr.description,
            }
        }
    }
}
