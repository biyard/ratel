use dioxus_translate::*;

translate! {
    FactFoldRoomTranslate;

    // Top bar
    brand: { en: "RATEL ARCADE", ko: "라텔 오락실" },
    nav_arcade: { en: "Arcade", ko: "오락실" },
    nav_leaderboard: { en: "Leaderboard", ko: "리더보드" },

    // Sidebar stage labels (mapped 1:1 to RoundStatus)
    stage_news_reveal_name: { en: "News Reveal", ko: "뉴스 공개" },
    stage_news_reveal_time: { en: "30s", ko: "30초" },
    stage_bet_name: { en: "1st Bet", ko: "1차 베팅" },
    stage_bet_time: { en: "10s", ko: "10초" },
    stage_rationale_name: { en: "Rationale", ko: "근거 작성" },
    stage_rationale_time: { en: "30s", ko: "30초" },
    stage_reveal_name: { en: "Reveal", ko: "근거 공개" },
    stage_reveal_time: { en: "20s", ko: "20초" },
    stage_debate_name: { en: "Live Debate", ko: "실시간 토론" },
    stage_debate_time: { en: "70s", ko: "70초" },
    stage_settlement_name: { en: "Settlement", ko: "정산" },
    stage_settlement_time: { en: "Auto", ko: "자동" },

    // Sidebar live timer
    timer_section_label: { en: "⏵ LIVE · CURRENT STAGE", ko: "⏵ LIVE · 현재 단계" },
    timer_remaining_sub: { en: "remaining", ko: "남음" },
    timer_done_sub: { en: "settled", ko: "정산됨" },
    timer_waiting_sub: { en: "waiting", ko: "대기" },
    timer_next: { en: "next:", ko: "다음:" },

    // Waiting state shown when status is still Waiting / Settlement transition
    waiting_for_players: {
        en: "Waiting for players to fill the lobby — the round starts as soon as it's full.",
        ko: "참가자가 모이는 중입니다. 로비가 채워지면 라운드가 시작됩니다.",
    },
    settling: {
        en: "Settling the round — results land in a moment.",
        ko: "라운드 정산 중 — 잠시 후 결과가 표시됩니다.",
    },

    // Coming-soon banner shown inside each sub-component until its
    // detailed RSX lands. Sub-components replace this string with their
    // own translations as they're implemented step-by-step.
    stage_stub_news_reveal: {
        en: "News-reveal view rendering will be wired in step 5.",
        ko: "뉴스 공개 화면은 다음 단계에서 채워집니다.",
    },
    stage_stub_first_bet: {
        en: "First-bet view rendering will be wired in step 6.",
        ko: "1차 베팅 화면은 다음 단계에서 채워집니다.",
    },
    stage_stub_reasoning_write: {
        en: "Rationale-write view rendering will be wired in step 7.",
        ko: "근거 작성 화면은 다음 단계에서 채워집니다.",
    },
    stage_stub_reasoning_reveal: {
        en: "Rationale-reveal view rendering will be wired in step 8.",
        ko: "근거 공개 화면은 다음 단계에서 채워집니다.",
    },
    stage_stub_live_debate: {
        en: "Live-debate view rendering will be wired in step 9.",
        ko: "실시간 토론 화면은 다음 단계에서 채워집니다.",
    },
    stage_stub_settlement: {
        en: "Settlement view rendering will be wired in step 10.",
        ko: "정산 화면은 다음 단계에서 채워집니다.",
    },
}
