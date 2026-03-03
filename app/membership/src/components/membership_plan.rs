use crate::components::{
    CustomerInfo, MembershipCard, MembershipPlanTranslate, MembershipPurchaseModal,
    MembershipReceiptData, MembershipReceiptModal, MembershipTier, membership_plan_items,
};
use crate::*;
use ratel_auth::LoginModal;

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
                        on_click: move |_| {
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
                                    let customer_name = user_ctx()
                                        .user
                                        .as_ref()
                                        .map(|u| u.display_name.clone())
                                        .unwrap_or_default();
                                    let tier = membership_for_action.tier;
                                    let credits = membership_for_action.credits.unwrap_or_default();
                                    let mut popup_modal = popup.clone();
                                    let popup_receipt = popup.clone();
                                    popup.open(rsx! {
                                        MembershipPurchaseModal {
                                            membership: tier,
                                            display_amount,
                                            customer_name,
                                            on_cancel: move |_| {
                                                popup_modal.close();
                                            },
                                            on_confirm: move |_info: CustomerInfo| {
                                                let paid_at = common::utils::time::now();
                                                let receipt = MembershipReceiptData {
                                                    tx_id: format!("TX-{}", paid_at),
                                                    membership: tier,
                                                    amount: display_amount,
                                                    duration_days: duration_days_for_tier(tier),
                                                    credits,
                                                    paid_at,
                                                };
                                                let mut popup_receipt = popup_receipt.clone();
                                                let mut popup_receipt_close = popup_receipt.clone();
                                                popup_receipt
                                                    .open(rsx! {
                                                    MembershipReceiptModal {
                                                        receipt,
                                                        on_close: move |_| {
                                                            popup_receipt_close.close();
                                                        },
                                                    }
                                                })
                                                .with_title("Receipt");
                                            },
                                        }
                                    }).without_close();
                                }
                                MembershipTier::Free => {}
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
