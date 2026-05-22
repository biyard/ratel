use dioxus_translate::*;

translate! {
    FactFoldAdminStatsTranslate;

    page_title: { en: "Stats", ko: "통계" },
    notice_title: { en: "Stats land in PR4", ko: "통계는 PR4에 채워집니다" },
    notice_body: {
        en: "Round-level aggregations (accuracy, insider effect, flip count) need the FactFoldRound entity. The shell + KPI layout below mirrors the design mockup so the route stays navigable.",
        ko: "라운드 집계(정답률, 인사이더 효과, flip 카운트)는 FactFoldRound 엔티티가 필요합니다. 아래 KPI 레이아웃은 mockup을 따라 navigable한 상태로 유지됩니다.",
    },
    kpi_total_rounds: { en: "Total rounds", ko: "총 라운드" },
    kpi_avg_accuracy: { en: "Avg accuracy", ko: "평균 정답률" },
    kpi_insider_win_rate: { en: "Insider win rate", ko: "인사이더 승률" },
    kpi_avg_flip_count: { en: "Avg flips / round", ko: "라운드당 평균 flip" },
    no_data: { en: "no data yet", ko: "데이터 없음" },

    panel_recent: { en: "Last 30 rounds", ko: "최근 30 라운드" },
    panel_recent_sub: { en: "Accuracy trend", ko: "정답률 트렌드" },
    chart_placeholder: { en: "Chart renders once round data exists.", ko: "라운드 데이터가 들어오면 차트가 렌더링됩니다." },

    panel_breakdown: { en: "Per-subject breakdown", ko: "대상별 통계" },
    panel_breakdown_sub: { en: "Accuracy + insider win + flip count", ko: "정답률 + 인사이더 승 + flip 카운트" },
    empty: { en: "No settled rounds yet.", ko: "정산된 라운드가 아직 없습니다." },
}
