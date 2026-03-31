use crate::common::*;
use crate::features::membership::components::*;
use crate::features::membership::controllers::{
    change_team_membership_handler, identify_team_handler,
    ChangeTeamMembershipRequest, TeamIdentificationRequest,
};
#[cfg(not(feature = "server"))]
use crate::features::membership::interop::request_identity_verification;
use crate::features::membership::models::MembershipTier as ApiMembershipTier;
use crate::features::membership::models::{CardInfo, Currency};
use crate::features::auth::LoginModal;
#[cfg(not(feature = "server"))]
use crate::common::wasm_bindgen::prelude::JsValue;
#[cfg(not(feature = "server"))]
use crate::common::wasm_bindgen_futures::JsFuture;

mod i18n;
pub use i18n::*;

#[cfg(not(feature = "server"))]
fn format_js_error(err: JsValue) -> String {
    if let Some(msg) = err.as_string() {
        msg
    } else {
        format!("{:?}", err)
    }
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

fn enterprise_contact() {
    let email = "hi@ratel.foundation";
    let subject = "Enterprise Membership Inquiry";
    let body = "Hello,%0D%0A%0D%0AI would like to learn more about the Enterprise membership plan for our team.%0D%0A%0D%0AThank you.";
    let mailto_url = format!("mailto:{}?subject={}&body={}", email, subject, body);
    let _ = web_sys::window().and_then(|w| w.open_with_url(&mailto_url).ok());
}

#[component]
pub fn SubscriptionPage(username: String) -> Element {
    let lang = use_language();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let mut popup = use_popup();
    #[allow(unused_variables)]
    let toast = use_toast();

    #[cfg(not(feature = "server"))]
    let is_ko = matches!(lang(), Language::Ko);
    #[cfg(not(feature = "server"))]
    let portone_store_id = option_env!("PORTONE_STORE_ID")
        .unwrap_or_default()
        .to_string();
    #[cfg(not(feature = "server"))]
    let portone_channel_key = option_env!("PORTONE_INICIS_CHANNEL_KEY")
        .unwrap_or_default()
        .to_string();

    let memberships = use_memo(move || membership_plan_items(matches!(lang(), Language::Ko)));
    let memberships_len = memberships.read().len();

    rsx! {
        document::Script { src: "https://cdn.portone.io/v2/browser-sdk.js" }
        div { class: "flex flex-col gap-6 w-full",
            div { class: "flex flex-col gap-2",
                MembershipPlanHeader {}
            }
            div { class: "gap-2.5 mt-4 membership-plan-grid",
                for (idx , membership) in memberships.read().iter().cloned().enumerate() {
                    MembershipCard {
                        key: "{membership.name}",
                        membership: membership.clone(),
                        variant: if idx + 1 == memberships_len { "horizontal" } else { "vertical" },
                        on_click: {
                            #[cfg(not(feature = "server"))]
                            let portone_store_id = portone_store_id.clone();
                            #[cfg(not(feature = "server"))]
                            let portone_channel_key = portone_channel_key.clone();
                            #[cfg(not(feature = "server"))]
                            let username = username.clone();
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
                                        #[cfg(not(feature = "server"))]
                                        {
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
                                            let mut popup_modal = popup.clone();
                                            let popup_receipt = popup.clone();
                                            let store_id = portone_store_id.clone();
                                            let channel_key = portone_channel_key.clone();
                                            let username = username.clone();
                                            spawn(async move {
                                                let identity_id = {
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
                                                };
                                                let customer = match identify_team_handler(
                                                        username.clone(),
                                                        TeamIdentificationRequest {
                                                            id: identity_id,
                                                        },
                                                    )
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
                                                            let mut popup_modal = popup_modal.clone();
                                                            let mut toast = toast;
                                                            let username = username.clone();
                                                            spawn(async move {
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
                                                                match change_team_membership_handler(username, req).await {
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
                                                                        error!("Failed to change team membership: {:?}", err);
                                                                        toast.error(crate::common::Error::MembershipChangeFailed);
                                                                        popup_modal.close();
                                                                    }
                                                                }
                                                            });
                                                        },
                                                    }
                                                }).without_close();
                                            });
                                        }
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
