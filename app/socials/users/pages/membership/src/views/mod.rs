use crate::components::{format_date, render_history};
use crate::controllers::get_membership::{MembershipResponse, get_membership_handler};
use crate::controllers::get_membership_transfer::{
    PurchaseHistoryResponse, get_purchase_history_handler,
};
use crate::*;
use common::chrono::TimeZone;
use common::lucide_dioxus::Sparkles;

#[component]
pub fn Home(username: String) -> Element {
    let tr: MembershipPageTranslate = use_translate();
    let membership_resource =
        use_server_future(move || async move { get_membership_handler().await })?;
    let history_resource =
        use_server_future(move || async move { get_purchase_history_handler(None).await })?;

    let membership_state = membership_resource.value();
    let history_state = history_resource.value();

    if membership_state.read().is_none() {
        return rsx! {
            div { class: "flex justify-center items-center min-h-screen",
                div { class: "w-12 h-12 rounded-full border-b-2 animate-spin border-primary" }
            }
        };
    }

    let membership = match membership_state.read().as_ref() {
        Some(Ok(data)) => data.clone(),
        _ => MembershipResponse::default(),
    };

    let tier_name = format_tier(&membership.tier);
    let tier_color = match tier_name.as_str() {
        "Pro" => "text-blue-500",
        "Max" => "text-purple-500",
        "Vip" => "text-amber-500",
        _ => "text-text-secondary",
    };

    rsx! {
        div { class: "flex flex-col gap-6 p-6 mx-auto w-full max-w-4xl",
            h1 { class: "text-2xl font-bold text-text-primary", "{tr.title}" }

            div { class: "p-6 rounded-lg border bg-card-bg border-card-border",
                h2 { class: "mb-4 text-xl font-semibold text-text-primary", "{tr.current_plan}" }

                div { class: "flex flex-col gap-4",
                    div { class: "flex gap-3 items-center",
                        Sparkles { class: format!("w-6 h-6 {}", tier_color) }
                        div {
                            div { class: "text-sm text-text-secondary", "{tr.tier}" }
                            div { class: format!("text-lg font-bold {}", tier_color),
                                "{tier_name}"
                            }
                        }
                    }

                    div { class: "grid grid-cols-2 gap-4",
                        div {
                            div { class: "text-sm text-text-secondary", "{tr.total_credits}" }
                            div { class: "text-lg font-semibold text-text-primary",
                                "{membership.total_credits}"
                            }
                        }
                        div {
                            div { class: "text-sm text-text-secondary", "{tr.remaining_credits}" }
                            div { class: "text-lg font-semibold text-text-primary",
                                "{membership.remaining_credits}"
                            }
                        }
                    }

                    div {
                        div { class: "text-sm text-text-secondary", "{tr.expiration}" }
                        div { class: "text-lg font-semibold text-text-primary",
                            "{format_date(membership.expired_at, tr.unlimited)}"
                        }
                    }

                    if let Some(next_membership) = membership.next_membership {
                        div { class: "p-3 rounded border bg-background-secondary border-amber-500/30",
                            div { class: "text-sm font-semibold text-amber-500",
                                "{tr.scheduled_downgrade}"
                            }
                            div { class: "text-sm text-text-secondary",
                                "{tr.next_membership}: {format_tier(&next_membership)}"
                            }
                        }
                    }
                }
            }

            div { class: "p-6 rounded-lg border bg-card-bg border-card-border",
                h2 { class: "mb-4 text-xl font-semibold text-text-primary", "{tr.purchase_history}" }

                {render_history(history_state.read().as_ref(), &tr)}
            }
        }
    }
}

fn format_tier(tier: &str) -> String {
    tier.strip_prefix("MEMBERSHIP#").unwrap_or(tier).to_string()
}

translate! {
    MembershipPageTranslate;

    title: {
        en: "Membership",
        ko: "멤버십",
    },

    current_plan: {
        en: "Current Plan",
        ko: "현재 플랜",
    },

    tier: {
        en: "Tier",
        ko: "등급",
    },

    total_credits: {
        en: "Total Credits",
        ko: "총 크레딧",
    },

    remaining_credits: {
        en: "Remaining Credits",
        ko: "남은 크레딧",
    },

    expiration: {
        en: "Expires",
        ko: "만료일",
    },

    next_membership: {
        en: "Next Membership",
        ko: "다음 멤버십",
    },

    scheduled_downgrade: {
        en: "Scheduled Downgrade",
        ko: "예정된 다운그레이드",
    },

    purchase_history: {
        en: "Purchase History",
        ko: "구매 내역",
    },

    transaction_type: {
        en: "Type",
        ko: "유형",
    },

    amount: {
        en: "Amount",
        ko: "금액",
    },

    payment_id: {
        en: "Payment ID",
        ko: "결제 ID",
    },

    date: {
        en: "Date",
        ko: "날짜",
    },

    no_purchases: {
        en: "No purchase history",
        ko: "구매 내역이 없습니다",
    },

    unlimited: {
        en: "Unlimited",
        ko: "무제한",
    },
}
