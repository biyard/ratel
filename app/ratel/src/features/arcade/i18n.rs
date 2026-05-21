use dioxus_translate::*;

translate! {
    ArcadeHomeTranslate;

    // Featured card
    tag_live: { en: "LIVE · Round in progress", ko: "LIVE · 오늘의 라운드 진행 중" },
    tag_waiting: { en: "WAITING · Lobby filling", ko: "WAITING · 로비 모집 중" },
    tag_open: { en: "OPEN · Subject ready", ko: "OPEN · 대상 준비됨" },
    tag_closed: { en: "CLOSED · No subjects queued", ko: "CLOSED · 예정 대상 없음" },
    tag_settled: { en: "SETTLED · Round complete", ko: "SETTLED · 라운드 종료" },

    featured_title: { en: "Fact or Fold", ko: "Fact or Fold" },
    featured_tagline: {
        en: "Four players, one news headline. One of them knows the truth. Your RatelPoints and rationale are your weapons.",
        ko: "4명이 모여 하루 한 건의 뉴스를 판별합니다. 둘 중 한 명은 거짓을 알고 있죠. 당신의 RatelPoint와 근거가 무기입니다.",
    },

    meta_capacity: { en: "Round size", ko: "라운드 인원" },
    meta_capacity_value: { en: "{$count} players", ko: "{$count}명" },
    meta_min_bet: { en: "Bet floor", ko: "베팅 단위" },
    meta_min_bet_value: { en: "{$rp} RP+", ko: "{$rp} RP+" },
    meta_cycle: { en: "Cycle", ko: "라운드 주기" },
    meta_cycle_value: { en: "1 / day", ko: "하루 1회" },
    meta_stage_now: { en: "Stage", ko: "현재 단계" },
    meta_stage_idle: { en: "idle", ko: "대기" },

    cta_join: { en: "Join round →", ko: "라운드 입장 →" },
    cta_resume: { en: "Resume round →", ko: "라운드 복귀 →" },
    cta_in_progress: { en: "Round in progress", ko: "라운드 진행 중" },
    cta_no_subject: { en: "No subject queued", ko: "예정된 대상 없음" },

    status_can_join_new: {
        en: "Be the first to sit — the round auto-starts at 4 players.",
        ko: "첫 참가자가 됩니다 — 4명이 모이면 자동 시작됩니다.",
    },
    status_can_join_existing: {
        en: "● Live · {$count} players already in",
        ko: "● 라이브 · 현재 {$count}명 대기 중",
    },
    status_already_joined: {
        en: "You're in. Round starts as soon as the lobby fills.",
        ko: "참여 완료. 로비가 채워지면 자동 시작.",
    },
    status_no_subject: {
        en: "Operator hasn't queued a subject yet. Check back soon.",
        ko: "대상이 발행되지 않았습니다. 잠시 후 다시 확인해주세요.",
    },
    status_in_progress: {
        en: "A round is already underway. Wait for it to settle.",
        ko: "이미 라운드가 진행 중입니다. 다음 라운드를 기다려주세요.",
    },

    // Side cards
    stats_card_title: { en: "Your stats", ko: "나의 통계" },
    stats_card_sub: { en: "Lifetime", ko: "누적 (lifetime)" },
    stats_rounds: { en: "Rounds played", ko: "참여 라운드" },
    stats_accuracy: { en: "Accuracy", ko: "정답률" },
    stats_delta: { en: "Lifetime chips", ko: "누적 칩" },
    stats_last_played: { en: "Last played", ko: "마지막 플레이" },
    stats_never: { en: "—", ko: "—" },

    history_card_title: { en: "Recent rounds", ko: "최근 라운드" },
    history_card_sub: { en: "Coming soon", ko: "곧 공개" },
    history_empty: {
        en: "Per-round history lands when chips → RP redemption ships (v2).",
        ko: "라운드별 기록은 칩→RP 역환전과 함께 v2에서 활성화됩니다.",
    },

    catalog_title: { en: "Other games", ko: "다른 게임" },
    catalog_sub: { en: "Ratel Arcade · {$count} game", ko: "라텔 오락실 · {$count}개" },
    catalog_tile_status_playing: { en: "PLAYING", ko: "PLAYING" },
    catalog_tile_status_idle: { en: "IDLE", ko: "IDLE" },
    catalog_tile_meta: { en: "Live · 4 players · ~3min", ko: "라이브 · 4인 · 약 3분" },

    // Leaderboard tab
    lb_section_title: { en: "Accuracy ranking · lifetime", ko: "정확도 랭킹 · 누적" },
    lb_section_sub: {
        en: "10+ rounds played · lifetime",
        ko: "최소 10라운드 참여자 · lifetime",
    },
    lb_head_rank: { en: "#", ko: "#" },
    lb_head_name: { en: "Player", ko: "이름" },
    lb_head_stats: { en: "Participation", ko: "참여" },
    lb_head_accuracy: { en: "Accuracy", ko: "정답률" },
    lb_head_chips: { en: "Lifetime chips", ko: "누적 칩" },
    lb_row_stat: { en: "{$total} rounds · {$correct} correct", ko: "{$total}라운드 · 정답 {$correct}회" },
    lb_empty: {
        en: "No leaderboard entries yet — be the first to settle a round.",
        ko: "리더보드 데이터가 없습니다. 첫 라운드를 끝내고 등재되어 보세요.",
    },

    error_join: { en: "Couldn't join the round.", ko: "라운드 입장에 실패했습니다." },
}

translate! {
    ArcadeLayoutTranslate;

    brand: { en: "RATEL ARCADE", ko: "RATEL ARCADE" },
    brand_sub: { en: "Ratel Arcade · v1", ko: "라텔 오락실 · v1" },
    tab_home: { en: "Arcade", ko: "오락실" },
    tab_leaderboard: { en: "Leaderboard", ko: "리더보드" },
    chip_unit: { en: "chips", ko: "칩" },
    chip_aria: { en: "Open chip exchange", ko: "칩 환전 열기" },
    admin_create_round: { en: "Create round", ko: "라운드 생성" },
    menu_open: { en: "Open menu", ko: "메뉴 열기" },
    menu_close: { en: "Close menu", ko: "메뉴 닫기" },
}

translate! {
    ArcadeExchangeModalTranslate;

    title: { en: "Exchange RP → chips", ko: "RP → 칩 환전" },
    subtitle: {
        en: "Convert your RatelPoints into arcade chips. Chips can be staked across arcade games.",
        ko: "RatelPoint를 아케이드 칩으로 환전합니다. 칩은 오락실의 모든 게임에 사용됩니다.",
    },
    input_label: { en: "RP to convert", ko: "환전할 RP" },
    ratio_label: { en: "Current rate", ko: "현재 환율" },
    ratio_value: { en: "1 RP → {$chips} chips", ko: "1 RP → {$chips} 칩" },
    receive_label: { en: "You'll receive", ko: "받을 칩" },
    receive_value: { en: "{$chips} chips", ko: "{$chips} 칩" },
    confirm_btn: { en: "Confirm exchange", ko: "환전 확정" },
    cancel_btn: { en: "Cancel", ko: "취소" },
    redeem_disabled_note: {
        en: "Chip → RP redemption isn't available yet (v2).",
        ko: "칩 → RP 역환전은 v2에 활성화됩니다.",
    },
    error_amount: { en: "Enter an amount above zero.", ko: "0보다 큰 값을 입력하세요." },
    success_summary: {
        en: "Exchanged. New balance: {$balance} chips.",
        ko: "환전 완료. 잔액: {$balance} 칩.",
    },
}
