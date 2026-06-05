mod i18n;

use super::controllers::{get_team_reward_permission_handler, get_team_rewards_handler};
use super::*;
use crate::common::*;
use crate::features::launchpad_partner::views::{LaunchpadTokenCard, RewardHero};
use crate::features::social::pages::reward::user::components::RewardHistorySection;
use crate::features::social::pages::team_arena::{TeamArenaTab, use_team_arena};

pub use i18n::TeamRewardsTranslate;

pub fn format_points(points: i64) -> String {
    format_with_commas(points, None)
}

pub fn format_with_commas(value: i64, suffix: Option<&str>) -> String {
    let sign = if value < 0 { "-" } else { "" };
    let digits = value.abs().to_string();
    let mut out = String::new();
    for (idx, ch) in digits.chars().rev().enumerate() {
        if idx > 0 && idx % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    let formatted: String = out.chars().rev().collect();
    if let Some(suffix) = suffix {
        format!("{}{}{}", sign, formatted, suffix)
    } else {
        format!("{}{}", sign, formatted)
    }
}

/// Team rewards page. Mirrors the user rewards layout 1:1:
/// - **Hero**: share-of-pool card using `Team.points`.
/// - **Token card**: signed-in user's launchpad token holdings
///   (intentionally user-scoped — teams don't have wallets yet, and the
///   widget is the same regardless of the page you're on).
/// - **Reward history**: per-event `UserRewardHistory` rows for this team.
///   The endpoint resolves user-or-team off the username, so the same
///   `RewardHistorySection` component renders both walls.
#[component]
pub fn Home(username: ReadSignal<String>) -> Element {
    let tr: TeamRewardsTranslate = use_translate();

    // Sync arena topbar tab.
    let mut arena = use_team_arena();
    use_effect(move || arena.active_tab.set(TeamArenaTab::Rewards));

    // Resolve the team's partition via the permission context.
    let perm_resource = use_loader(move || async move {
        Ok::<_, crate::common::Error>(get_team_reward_permission_handler(username()).await.ok())
    })?;
    let Some(perm) = perm_resource() else {
        return rsx! {
            div { class: "rewards-arena",
                div { class: "page",
                    div { class: "empty",
                        div { class: "empty-desc", "{tr.activity_empty}" }
                    }
                }
            }
        };
    };
    let team_pk = perm.team_pk;
    let team_pk_signal: Signal<TeamPartition> = use_signal(|| team_pk.clone());

    let rewards_resource = use_loader(move || async move {
        Ok::<_, crate::common::Error>(get_team_rewards_handler(team_pk_signal(), None).await.ok())
    })?;
    let team_points = rewards_resource().unwrap_or_default().team_points;

    rsx! {
        div { class: "rewards-arena",
            div { class: "page",
                // Share-of-pool hero: team points vs the current launchpad
                // round total, plus round info.
                SuspenseBoundary {
                    RewardHero { points: team_points }
                }

                // Token holdings (launchpad-backed on-chain balance),
                // isolated so the lookup doesn't block the page.
                SuspenseBoundary { LaunchpadTokenCard {} }

                // Per-event reward history. The endpoint sniffs the
                // username for User OR Team — same component, same wire.
                RewardHistorySection { username }
            }
        }
    }
}
