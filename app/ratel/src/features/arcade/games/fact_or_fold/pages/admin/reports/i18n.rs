use dioxus_translate::*;

translate! {
    FactFoldAdminReportsTranslate;

    page_title: { en: "Reports", ko: "신고" },
    notice_title: { en: "Reports land in PR4", ko: "신고는 PR4에 채워집니다" },
    notice_body: {
        en: "User reports require the FactFoldReport entity + chat surface (PR5). Tabs and empty list are placeholders so navigation stays consistent.",
        ko: "사용자 신고는 FactFoldReport 엔티티 + 채팅 화면(PR5)이 필요합니다. 탭과 빈 리스트는 navigation 일관성 유지를 위한 placeholder입니다.",
    },
    tab_open: { en: "Open", ko: "처리 대기" },
    tab_resolved: { en: "Resolved", ko: "해결됨" },
    tab_dismissed: { en: "Dismissed", ko: "기각" },
    empty: { en: "No reports yet.", ko: "신고가 없습니다." },
}
