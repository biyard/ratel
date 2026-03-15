use super::*;
use dioxus::prelude::*;

mod admin_page;
mod viewer_page;

use admin_page::*;
use viewer_page::*;

use super::controllers::get_team_dao_handler;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

#[component]
pub fn Home(username: String) -> Element {
    let resource = use_loader(use_reactive((&username,), |(name,)| async move {
        Ok::<_, super::Error>(get_team_dao_handler(name).await.map_err(|e| e.to_string()))
    }))?;

    let data = resource.read();

    match data.as_ref() {
        Ok(ctx) => {
            let permissions: TeamGroupPermissions = ctx.permissions.into();
            let is_admin = permissions.contains(TeamGroupPermission::TeamAdmin);

            if is_admin {
                rsx! {
                    AdminPage { username, context: ctx.clone() }
                }
            } else {
                rsx! {
                    ViewerPage { username }
                }
            }
        }
        Err(_) => rsx! {
            ViewerPage { username }
        },
    }
}

translate! {
    TeamDaoTranslate;

    dao: {
        en: "DAO",
        ko: "DAO",
    },

    dao_title: {
        en: "Team DAO",
        ko: "Team DAO",
    },

    dao_description: {
        en: "Manage your team's decentralized autonomous organization",
        ko: "팀의 탈중앙화 자율 조직을 관리합니다",
    },

    register_dao: {
        en: "Register DAO",
        ko: "DAO 등록",
    },

    registering_dao: {
        en: "Registering...",
        ko: "등록 중...",
    },

    dao_address: {
        en: "DAO Address",
        ko: "DAO 주소",
    },

    view_on_explorer: {
        en: "View on Kaia Explorer",
        ko: "Kaia 탐색기에서 보기",
    },

    select_admins: {
        en: "Select DAO Admins",
        ko: "DAO 관리자 선택",
    },

    select_admins_description: {
        en: "Select at least 3 team admins with registered EVM addresses to become DAO managers",
        ko: "DAO 관리자가 될 EVM 주소를 등록한 팀 관리자를 최소 3명 이상 선택하세요",
    },

    min_admins_required: {
        en: "At least 3 admins required",
        ko: "최소 3명의 관리자 필요",
    },

    eligible_admins_count: {
        en: "{{count}} eligible admins",
        ko: "{{count}}명의 적격 관리자",
    },

    insufficient_admins: {
        en: "Insufficient eligible admins. You need at least 3 team admins with EVM addresses registered.",
        ko: "적격 관리자가 부족합니다. EVM 주소가 등록된 팀 관리자가 최소 3명 필요합니다.",
    },

    admin_requirements: {
        en: "Admin Requirements",
        ko: "관리자 요구사항",
    },

    admin_requirements_description: {
        en: "To register a DAO, you need:\n• At least 3 team members with Admin permission\n• Each admin must have an EVM address registered\n• You must be a team admin",
        ko: "DAO를 등록하려면 다음이 필요합니다:\n• 관리자 권한을 가진 팀 멤버 최소 3명\n• 각 관리자는 EVM 주소를 등록해야 함\n• 본인이 팀 관리자여야 함",
    },

    selected_count: {
        en: "{{count}} selected",
        ko: "{{count}}명 선택됨",
    },

    admin_only: {
        en: "Only team admins can access this page",
        ko: "팀 관리자만 이 페이지에 접근할 수 있습니다",
    },

    admin_only_description: {
        en: "You must be a team admin to access this page.",
        ko: "팀 관리자 권한이 있어야 이 페이지에 접근할 수 있습니다.",
    },

    wallet_connection_required: {
        en: "Please connect your wallet",
        ko: "지갑을 연결해주세요",
    },

    transaction_failed: {
        en: "Transaction failed",
        ko: "트랜잭션 실패",
    },

    cancel: {
        en: "Cancel",
        ko: "취소",
    },

    confirm: {
        en: "Confirm",
        ko: "확인",
    },

    copy_address: {
        en: "Copy address",
        ko: "주소 복사",
    },

    copied: {
        en: "Copied!",
        ko: "복사됨!",
    },

    dao_status: {
        en: "DAO Status",
        ko: "DAO 상태",
    },

    active: {
        en: "Active",
        ko: "활성",
    },

    inactive: {
        en: "Inactive",
        ko: "비활성",
    },
}
