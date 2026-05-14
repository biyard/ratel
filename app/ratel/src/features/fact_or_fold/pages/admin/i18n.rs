use dioxus_translate::*;

translate! {
    FactFoldAdminLayoutTranslate;

    brand: { en: "RATEL ARCADE", ko: "라텔 오락실" },
    brand_sub: { en: "Admin · Fact or Fold", ko: "관리자 · Fact or Fold" },

    tab_headlines: { en: "Headlines", ko: "헤드라인" },
    tab_schedule: { en: "Schedule", ko: "스케줄" },
    tab_stats: { en: "Stats", ko: "통계" },
    tab_reports: { en: "Reports", ko: "신고" },
    tab_settings: { en: "Settings", ko: "설정" },

    new_headline_cta: { en: "New headline", ko: "신규 헤드라인" },

    queue_alert: {
        en: "Scheduled queue is running low — schedule new headlines soon.",
        ko: "스케줄 큐가 부족합니다 — 신규 헤드라인을 곧 등록해주세요.",
    },
    queue_days_remaining: {
        en: "{days}d remaining",
        ko: "{days}일 남음",
    },
}
