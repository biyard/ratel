mod i18n;

use crate::common::*;
use crate::features::membership::controllers::{
    PurchaseHistoryItem, get_team_membership_handler, get_team_purchase_history_handler,
};
use crate::features::membership::models::TeamMembershipResponse;
use crate::features::social::pages::user_membership::components::format_membership_tier_label;
use dioxus::prelude::*;
use i18n::TeamMembershipTranslate;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tier {
    Free,
    Pro,
    Max,
    Vip,
    Enterprise,
}

fn tier_from_label(label: &str) -> Tier {
    match label {
        s if s.eq_ignore_ascii_case("Pro") => Tier::Pro,
        s if s.eq_ignore_ascii_case("Max") => Tier::Max,
        s if s.eq_ignore_ascii_case("Vip") => Tier::Vip,
        s if s.eq_ignore_ascii_case("Free") => Tier::Free,
        _ => Tier::Enterprise,
    }
}

fn tier_modifier(t: Tier) -> &'static str {
    match t {
        Tier::Free => " current-card--free",
        Tier::Pro => " current-card--pro",
        Tier::Max => " current-card--max",
        Tier::Vip => " current-card--vip",
        Tier::Enterprise => " current-card--ent",
    }
}

fn tier_desc(t: Tier, tr: &TeamMembershipTranslate) -> &'static str {
    match t {
        Tier::Free => tr.tier_free_desc,
        Tier::Pro => tr.tier_pro_desc,
        Tier::Max => tr.tier_max_desc,
        Tier::Vip => tr.tier_vip_desc,
        Tier::Enterprise => tr.tier_enterprise_desc,
    }
}

fn format_expiry(expired_at: i64, unlimited_label: &str) -> String {
    if expired_at == i64::MAX || expired_at == 0 {
        unlimited_label.to_string()
    } else {
        use chrono::{DateTime, Utc};
        DateTime::<Utc>::from_timestamp_millis(expired_at)
            .map(|dt| dt.format("%Y · %m · %d").to_string())
            .unwrap_or_else(|| unlimited_label.to_string())
    }
}

fn format_date(ts_millis: i64) -> String {
    use chrono::{DateTime, Utc};
    DateTime::<Utc>::from_timestamp_millis(ts_millis)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

#[component]
pub fn Home(username: ReadSignal<String>) -> Element {
    let tr: TeamMembershipTranslate = use_translate();

    let membership_resource = use_loader(move || async move {
        Ok::<_, crate::common::Error>(get_team_membership_handler(username()).await.ok())
    })?;
    let history_resource = use_loader(move || async move {
        Ok::<_, crate::common::Error>(
            get_team_purchase_history_handler(username(), None)
                .await
                .ok()
                .map(|r| r.items)
                .unwrap_or_default(),
        )
    })?;

    let Some(membership): Option<TeamMembershipResponse> = membership_resource() else {
        return rsx! {
            document::Stylesheet { href: asset!("./style.css") }
            div { class: "tm-status-page",
                div { class: "hero",
                    h1 { class: "hero__title",
                        "{tr.hero_title_en}"
                        span { class: "hero__title-ko", "{tr.hero_title_ko}" }
                    }
                    p { class: "hero__desc", "{tr.no_permission}" }
                }
            }
        };
    };
    let history: Vec<PurchaseHistoryItem> = history_resource();

    let tier_name = format_membership_tier_label(&membership.tier.0, tr.enterprise_label);
    let tier = tier_from_label(&tier_name);
    let mod_class = tier_modifier(tier);
    let desc = tier_desc(tier, &tr);

    let remaining = membership.remaining_credits.max(0);
    let total = membership.total_credits.max(1);
    let pct = ((remaining as f64 / total as f64) * 100.0).clamp(0.0, 100.0);
    let pct_label = format!("{:.0}%", pct);
    let fill_style = format!("width:{:.0}%", pct);

    let expiry = format_expiry(membership.expired_at, tr.expires_unlimited);
    let show_auto_renew = membership.expired_at != i64::MAX && membership.expired_at != 0;

    let next_tier_label = membership
        .next_membership
        .as_ref()
        .map(|p| format_membership_tier_label(&p.0, tr.enterprise_label));

    let history_len = history.len();
    let count_suffix = if history_len == 1 {
        tr.history_count_suffix_one
    } else {
        tr.history_count_suffix_many
    };

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }

        div { class: "tm-status-page",
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
                    "{tr.hero_desc_suffix}"
                }
            }

            div { class: "page",

                // ── Current Plan ──
                div { class: "current-card{mod_class}",
                    div { class: "cc-tier",
                        span { class: "cc-tier__label", "{tr.current_plan_label}" }
                        div { class: "cc-tier__name", "{tier_name}" }
                        div { class: "cc-tier__desc", "{desc}" }
                    }

                    div { class: "cc-stat",
                        span { class: "cc-stat__label", "{tr.credits_label}" }
                        div { class: "cc-stat__value-row",
                            span { class: "cc-stat__value cc-stat__value--tier",
                                "{remaining}"
                            }
                            span { class: "cc-stat__suffix", "/ {total}" }
                        }
                        div {
                            div { class: "cc-progress",
                                div { class: "cc-progress__fill", style: "{fill_style}" }
                            }
                            div { class: "cc-progress__meta",
                                span { "{tr.credits_remaining_hint}" }
                                span { "{pct_label}" }
                            }
                        }
                    }

                    div { class: "cc-expire",
                        span { class: "cc-stat__label", "{tr.expires_label}" }
                        div { class: "cc-expire__date", "{expiry}" }
                        if show_auto_renew {
                            div { class: "cc-expire__hint", "{tr.expires_auto_renew}" }
                        }
                    }

                    if let Some(next) = next_tier_label {
                        div { class: "cc-footer",
                            div { class: "cc-footer__icon",
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    circle { cx: "12", cy: "12", r: "10" }
                                    line {
                                        x1: "12",
                                        y1: "8",
                                        x2: "12",
                                        y2: "12",
                                    }
                                    line {
                                        x1: "12",
                                        y1: "16",
                                        x2: "12.01",
                                        y2: "16",
                                    }
                                }
                            }
                            div { class: "cc-footer__text",
                                "{tr.downgrade_prefix}"
                                strong { "{next}" }
                                "{tr.downgrade_suffix}"
                            }
                        }
                    }
                }

                // ── Purchase History ──
                div { class: "history-card",
                    div { class: "history-card__header",
                        span { class: "history-card__title", "{tr.history_title}" }
                        span { class: "history-card__count",
                            "{history_len} {count_suffix}"
                        }
                    }

                    if history.is_empty() {
                        div { class: "history-empty",
                            div { class: "history-empty__icon",
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "1.8",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    rect {
                                        x: "3",
                                        y: "5",
                                        width: "18",
                                        height: "14",
                                        rx: "2",
                                    }
                                    line {
                                        x1: "3",
                                        y1: "10",
                                        x2: "21",
                                        y2: "10",
                                    }
                                }
                            }
                            div { class: "history-empty__title", "{tr.history_empty_title}" }
                            div { class: "history-empty__desc", "{tr.history_empty_desc}" }
                        }
                    } else {
                        table { class: "history-table",
                            colgroup {
                                col { style: "width:22%" }
                                col { style: "width:18%" }
                                col { style: "width:34%" }
                                col { style: "width:26%" }
                            }
                            thead {
                                tr {
                                    th { "{tr.th_type}" }
                                    th { class: "num", "{tr.th_amount}" }
                                    th { "{tr.th_payment_id}" }
                                    th { class: "date", "{tr.th_date}" }
                                }
                            }
                            tbody {
                                for (idx , item) in history.iter().enumerate() {
                                    tr { key: "{idx}",
                                        td { "data-label": tr.th_type,
                                            span { class: "history-chip", "{item.tx_type}" }
                                        }
                                        td { class: "num", "data-label": tr.th_amount,
                                            "₩{item.amount}"
                                        }
                                        td {
                                            class: "mono",
                                            "data-label": tr.th_payment_id,
                                            "{item.payment_id}"
                                        }
                                        td {
                                            class: "date",
                                            "data-label": tr.th_date,
                                            "{format_date(item.created_at)}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
