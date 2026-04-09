use crate::common::*;
use crate::features::profile::i18n::ProfileTranslate;
use crate::features::spaces::pages::actions::gamification::controllers::profile::GlobalProfileResponse;

/// A 2x2 grid of stat cards showing Total XP, Points, Dungeons, and
/// Quests counts.
#[component]
pub fn StatsGrid(profile: GlobalProfileResponse) -> Element {
    let tr: ProfileTranslate = use_translate();

    rsx! {
        div {
            class: "grid grid-cols-2 gap-3 w-full",
            "data-testid": "stats-grid",

            StatCard {
                label: "{tr.total_xp}",
                value: format_number(profile.total_xp),
                suffix: "{tr.xp_suffix}",
                color: "text-primary",
            }
            StatCard {
                label: "{tr.total_points}",
                value: format_number(profile.total_points as i64),
                suffix: "{tr.pts_suffix}",
                color: "text-accent",
            }
            StatCard {
                label: "{tr.dungeons_entered}",
                value: profile.dungeons_entered.to_string(),
                suffix: "",
                color: "text-text-primary",
            }
            StatCard {
                label: "{tr.quests_cleared}",
                value: profile.quests_cleared.to_string(),
                suffix: "",
                color: "text-text-primary",
            }
        }
    }
}

#[component]
fn StatCard(label: String, value: String, suffix: String, color: String) -> Element {
    rsx! {
        Card {
            variant: CardVariant::Outlined,
            direction: CardDirection::Col,
            class: "gap-1 p-4",

            span { class: "text-xs text-foreground-muted", "{label}" }
            Row { cross_axis_align: CrossAxisAlign::End, class: "gap-1",
                span { class: "text-2xl font-bold tabular-nums {color}", "{value}" }
                if !suffix.is_empty() {
                    span { class: "pb-0.5 text-sm text-foreground-muted", "{suffix}" }
                }
            }
        }
    }
}

/// Format a large number with comma separators for readability.
fn format_number(n: i64) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let negative = n < 0;
    let mut n = n.unsigned_abs();
    let mut parts = Vec::new();

    while n > 0 {
        parts.push(format!("{:03}", n % 1000));
        n /= 1000;
    }

    parts.reverse();
    // Remove leading zeros from the first group.
    if let Some(first) = parts.first_mut() {
        *first = first.trim_start_matches('0').to_string();
        if first.is_empty() {
            *first = "0".to_string();
        }
    }

    let result = parts.join(",");
    if negative {
        format!("-{result}")
    } else {
        result
    }
}
