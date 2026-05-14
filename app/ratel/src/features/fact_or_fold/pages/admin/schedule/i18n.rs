use dioxus_translate::*;

translate! {
    FactFoldAdminScheduleTranslate;

    page_title: { en: "Schedule", ko: "스케줄" },
    upcoming_title: { en: "Upcoming", ko: "예정 라운드" },
    upcoming_count_suffix: { en: "scheduled", ko: "예약" },
    empty: { en: "No scheduled headlines — all caught up.", ko: "예약된 헤드라인이 없습니다." },

    alarm_title: { en: "Queue is running low", ko: "큐가 부족합니다" },
    alarm_body_prefix: { en: "Latest scheduled headline is only", ko: "가장 늦게 예약된 헤드라인이" },
    alarm_body_days_unit: { en: "days", ko: "일" },
    alarm_body_count_suffix: { en: "scheduled", ko: "건 예약됨" },
    alarm_threshold_prefix: { en: "Alert threshold:", ko: "알림 임계:" },
    alarm_threshold_suffix: { en: "d", ko: "일" },
    alarm_cta: { en: "Schedule a headline", ko: "헤드라인 예약" },

    healthy_title: { en: "Queue healthy", ko: "큐 양호" },
    healthy_count_suffix: { en: "scheduled headlines queued", ko: "건 예약 중" },
    healthy_days_suffix: { en: "days out", ko: "일치" },
}
