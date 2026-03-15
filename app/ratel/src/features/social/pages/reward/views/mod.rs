use super::*;
use dioxus::prelude::*;

mod admin_page;
mod viewer_page;

use admin_page::*;
use viewer_page::*;

use super::controllers::get_team_reward_permission_handler;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

#[component]
pub fn Home(username: String) -> Element {
    let resource = use_loader(use_reactive((&username,), |(name,)| async move {
        Ok::<_, super::Error>(
            get_team_reward_permission_handler(name)
                .await
                .map_err(|e| e.to_string()),
        )
    }))?;

    let data = resource.read();

    match data.as_ref() {
        Ok(ctx) => {
            let permissions: TeamGroupPermissions = ctx.permissions.into();
            let can_view = permissions.contains(TeamGroupPermission::TeamAdmin);

            if can_view {
                rsx! {
                    AdminPage {
                        team_pk: ctx.team_pk.clone(),
                        team_name: ctx.team_name.clone(),
                    }
                }
            } else {
                rsx! {
                    ViewerPage { username }
                }
            }
        }
        Err(_) => {
            rsx! {
                ViewerPage { username }
            }
        }
    }
}

translate! {
    TeamRewardsTranslate;

    title: {
        en: "This month's team points",
        ko: "이번 달 팀 포인트",
    },

    your_share: {
        en: "Team's share",
        ko: "팀 지분",
    },

    this_months_pool: {
        en: "This month's pool",
        ko: "이번 달 풀",
    },

    swap_available_message: {
        en: "Point-to-Token Swap will be available starting next month (Admin only)",
        ko: "포인트-토큰 스왑은 다음 달부터 가능합니다 (관리자 전용)",
    },

    exchange_from: {
        en: "from",
        ko: "from",
    },

    exchange_to: {
        en: "To",
        ko: "To",
    },

    point: {
        en: "Point",
        ko: "Point",
    },

    token: {
        en: "Token",
        ko: "Token",
    },

    received: {
        en: "Received",
        ko: "획득",
    },

    spent: {
        en: "Spent",
        ko: "사용",
    },

    from: {
        en: "from",
        ko: "from",
    },

    empty: {
        en: "No transactions",
        ko: "거래 내역 없음",
    },

    empty_description: {
        en: "This team has no point transactions yet",
        ko: "아직 팀 포인트 거래 내역이 없습니다",
    },

    loading: {
        en: "Loading...",
        ko: "로딩 중...",
    },

    error: {
        en: "Error loading rewards",
        ko: "리워드 로딩 오류",
    },

    load_more: {
        en: "Load more",
        ko: "더 보기",
    },

    yours: {
        en: "Team's share",
        ko: "팀 지분",
    },
}
