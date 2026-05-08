use dioxus_translate::*;

translate! {
    TeamRewardsTranslate;

    page_title: { en: "Team Rewards", ko: "팀 리워드" },
    back: { en: "Back", ko: "뒤로" },
    cycle_label: { en: "Cycle", ko: "사이클" },

    section_label_prefix: { en: "Team", ko: "팀" },
    section_label_strong: { en: "Rewards", ko: "리워드" },

    earning_this_cycle: { en: "Team earnings · this cycle", ko: "팀 이번 사이클 획득" },
    points: { en: "Points", ko: "포인트" },
    share_of_pool: { en: "Share of Pool", ko: "풀 비율" },
    of_total: { en: "of", ko: "/" },
    total_points_unit: { en: "total pts", ko: "전체 pts" },
    your_position: { en: "TEAM POSITION", ko: "팀 순위" },
    rank_of: { en: "RANK", ko: "랭크" },

    estimated_tokens: { en: "Estimated tokens", ko: "예상 토큰" },
    cycle_locks_in: { en: "Cycle locks in", ko: "사이클 종료까지" },
    claim_opens: { en: "— claim opens next cycle.", ko: "— 다음 사이클에 청구 가능" },

    treasury_rate: { en: "Treasury Rate", ko: "트레저리 환율" },
    per_token: { en: "per token", ko: "토큰당" },
    price_desc: { en: "Backed 1:1 by treasury reserve. Rate updates every block.", ko: "트레저리 준비금 1:1 담보. 매 블록 갱신." },
    stat_treasury: { en: "Treasury", ko: "트레저리" },
    stat_circulating: { en: "Circulating", ko: "유통량" },
    stat_backing: { en: "Backing", ko: "담보율" },
    backing_value: { en: "100%", ko: "100%" },
    backing_unit: { en: "collateralized", ko: "담보" },

    price_chart_title: { en: "Price · last 30 days", ko: "가격 · 최근 30일" },

    chart_points_tokens: { en: "Points", ko: "포인트" },
    chart_subtitle: { en: "Last 6 cycles · Monthly", ko: "최근 6 사이클 · 월별" },
    chart_legend_points: { en: "Points", ko: "포인트" },
    chart_legend_tokens: { en: "Tokens", ko: "토큰" },

    source_breakdown: { en: "Source Breakdown", ko: "획득 경로" },
    source_subtitle: { en: "This cycle · Earned points", ko: "이번 사이클 · 획득 포인트" },
    donut_label: { en: "POINTS · NOW", ko: "포인트 · 현재" },

    activity_title: { en: "This Cycle's Activity", ko: "이번 사이클 활동" },
    entries: { en: "entries", ko: "항목" },
    load_more: { en: "Load more", ko: "더 보기" },
    loading: { en: "Loading…", ko: "로딩 중…" },
    activity_empty: { en: "No team activity yet this cycle.", ko: "아직 팀 활동 내역이 없습니다." },

    past_cycles: { en: "Past Cycles", ko: "이전 사이클" },
    claimable: { en: "claimable", ko: "청구 가능" },
    swap_all: { en: "Swap All", ko: "전체 스왑" },
    claimed: { en: "Claimed", ko: "청구 완료" },
    stat_points: { en: "Points", ko: "포인트" },
    stat_share: { en: "Share", ko: "비율" },
    stat_tokens: { en: "Tokens", ko: "토큰" },
    past_empty_title: { en: "No past cycles yet", ko: "아직 이전 사이클이 없습니다" },
    past_empty_desc: { en: "The first cycle wraps up at month-end. Tokens become claimable on the 1st of next month.", ko: "첫 사이클은 월말에 마감됩니다." },

    pts_unit: { en: "pts", ko: "pts" },

    admin_only: { en: "Admin access required to claim team rewards.", ko: "팀 리워드 클레임은 관리자만 가능합니다." },
}
