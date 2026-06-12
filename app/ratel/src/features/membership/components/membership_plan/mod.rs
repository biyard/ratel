use super::*;
mod i18n;

pub use i18n::*;

use crate::features::membership::*;
use crate::common::use_toast;
use crate::features::membership::components::*;
use crate::features::membership::controllers::{
    change_membership_handler, identify_handler, ChangeMembershipRequest, IdentificationRequest,
};
#[cfg(not(feature = "server"))]
use crate::features::membership::interop::request_identity_verification;
use crate::features::membership::models::MembershipTier as ApiMembershipTier;
use crate::features::membership::models::{CardInfo, Currency};
use crate::features::auth::LoginModal;

use crate::common::components::popup::PopupService;
use crate::common::providers::ToastService;
use crate::common::wasm_bindgen::prelude::JsValue;
use crate::common::wasm_bindgen_futures::JsFuture;

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

#[cfg(not(feature = "server"))]
fn tier_key(tier: MembershipTier) -> &'static str {
    match tier {
        MembershipTier::Pro => "pro",
        MembershipTier::Max => "max",
        MembershipTier::Vip => "vip",
        _ => "",
    }
}

#[cfg(not(feature = "server"))]
fn tier_from_key(key: &str) -> Option<MembershipTier> {
    match key {
        "pro" => Some(MembershipTier::Pro),
        "max" => Some(MembershipTier::Max),
        "vip" => Some(MembershipTier::Vip),
        _ => None,
    }
}

/// Finalize identity (`identify_handler`) and open the purchase modal. Shared by
/// the desktop popup path (verification resolves inline) and the mobile WebView
/// path (verification redirects away and resumes here on return to /membership).
#[cfg(not(feature = "server"))]
async fn open_membership_purchase(
    tier: MembershipTier,
    is_ko: bool,
    identity_id: String,
    mut popup: PopupService,
    toast: ToastService,
) {
    let display_amount = display_amount_for_tier(tier, is_ko);
    let customer = match identify_handler(IdentificationRequest { id: identity_id }).await {
        Ok(resp) => resp,
        Err(err) => {
            error!("Failed to verify identity: {:?}", err);
            return;
        }
    };
    let popup_modal = popup;
    let popup_receipt = popup;
    popup
        .open(rsx! {
            MembershipPurchaseModal {
                membership: tier,
                display_amount,
                customer,
                on_confirm: move |info: CustomerInfo| {
                    let mut popup_receipt = popup_receipt;
                    let mut popup_modal = popup_modal;
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
                                    toast.error(crate::common::Error::MembershipResponseMissing);
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
                                    popup_receipt.open(rsx! {
                                        MembershipReceiptModal { receipt }
                                    });
                                } else {
                                    toast.info("Membership downgrade scheduled");
                                    popup_modal.close();
                                }
                            }
                            Err(err) => {
                                error!("Failed to change membership: {:?}", err);
                                toast.error(crate::common::Error::MembershipChangeFailed);
                                popup_modal.close();
                            }
                        }
                    });
                },
            }
        })
        .without_close();
}

#[component]
pub fn MembershipPlan() -> Element {
    let lang = use_language();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let mut popup = use_popup();
    let mut toast = use_toast();

    let is_ko = matches!(lang(), Language::Ko);
    let portone_store_id = option_env!("PORTONE_STORE_ID")
        .unwrap_or_default()
        .to_string();
    let portone_channel_key = option_env!("PORTONE_INICIS_CHANNEL_KEY")
        .unwrap_or_default()
        .to_string();

    let memberships = use_memo(move || membership_plan_items(matches!(lang(), Language::Ko)));
    let memberships_len = memberships.read().len();

    // Resume the purchase flow after a mobile WebView PortOne redirect: the
    // verification returns to /membership with the result in the (stripped)
    // query plus the tier we stashed before the redirect. No-op on desktop.
    #[cfg(not(feature = "server"))]
    {
        use_effect(move || {
            spawn(async move {
                if let Some((identity_id, tier_key_str)) =
                    crate::features::membership::interop::take_membership_return().await
                {
                    if let Some(tier) = tier_from_key(&tier_key_str) {
                        open_membership_purchase(tier, is_ko, identity_id, popup, toast).await;
                    }
                }
            });
        });
    }

    rsx! {
        div { class: "py-8 px-4 mx-auto w-full max-w-desktop",
            MembershipPlanHeader {}
            div { class: "gap-2.5 mt-8 membership-plan-grid",
                for (idx, membership) in memberships.read().iter().cloned().enumerate() {
                    MembershipCard {
                        key: "{membership.name}",
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
                                        let prefix = user_ctx()
                                            .user
                                            .as_ref()
                                            .map(|u| u.pk.to_string())
                                            .unwrap_or_default();
                                        let tier = membership_for_action.tier;
                                        let store_id = portone_store_id.clone();
                                        let channel_key = portone_channel_key.clone();
                                        spawn(async move {
                                            #[cfg(feature = "server")]
                                            {
                                                let _ = (
                                                    prefix,
                                                    store_id,
                                                    channel_key,
                                                    tier,
                                                    is_ko,
                                                    popup,
                                                    toast, // On mobile this navigates away (redirect) and never
                                                );
                                                error!("identity verification is not available on server");
                                            }
                                            #[cfg(not(feature = "server"))]
                                            {
                                                crate::features::membership::interop::stash_membership_tier(
                                                        tier_key(tier),
                                                    )
                                                    .await;
                                                let promise = request_identity_verification(
                                                    &store_id,
                                                    &channel_key,
                                                    &prefix,
                                                );
                                                let identity_id = match JsFuture::from(promise).await {
                                                    Ok(val) => {
                                                        match val.as_string() {
                                                            Some(id) => id,
                                                            None => {
                                                                error!("identity verification: missing id");
                                                                return;
                                                            }
                                                        }
                                                    }
                                                    Err(err) => {
                                                        error!(
                                                            "identity verification failed: {:?}", format_js_error(err)
                                                        );
                                                        return;
                                                    }
                                                };
                                                open_membership_purchase(
                                                        tier,
                                                        is_ko,
                                                        identity_id,
                                                        popup,
                                                        toast,
                                                    )
                                                    .await;
                                            }
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
            h1 { class: "text-3xl font-bold md:text-4xl lg:text-5xl text-text-primary",
                {tr.title}
            }
            div {
                class: "text-[17px]/[20px] text-text-secondary",
                dangerous_inner_html: tr.description,
            }
        }
    }
}
