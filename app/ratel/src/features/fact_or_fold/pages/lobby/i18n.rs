use dioxus_translate::*;

translate! {
    FactFoldLobbyTranslate;

    brand: { en: "RATEL ARCADE", ko: "라텔 오락실" },
    brand_sub: { en: "Fact or Fold · v1", ko: "Fact or Fold · v1" },
    tagline: {
        en: "4 players. 1 news headline. Bet your RatelPoints + a sharp rationale to judge whether it's REAL or FAKE — in ~3 minutes.",
        ko: "4명이 모여 뉴스 1건의 진실성을 RP 베팅 + 근거로 ~3분 안에 판별합니다.",
    },

    tag_open: { en: "OPEN · Headline ready", ko: "OPEN · 헤드라인 준비됨" },
    tag_waiting: { en: "WAITING · Lobby filling", ko: "WAITING · 로비 모집 중" },
    tag_in_progress: { en: "LIVE · Round in progress", ko: "LIVE · 라운드 진행 중" },
    tag_settled: { en: "SETTLED · Round complete", ko: "SETTLED · 라운드 종료" },
    tag_closed: { en: "CLOSED · No headlines queued", ko: "CLOSED · 예정 헤드라인 없음" },

    meta_waiting: { en: "Lobby", ko: "로비" },
    meta_min_bet: { en: "Min bet", ko: "최소 베팅" },
    meta_round_time: { en: "Round duration", ko: "라운드 길이" },
    meta_round_time_value: { en: "~3 min", ko: "약 3분" },
    meta_cycle: { en: "Cycle", ko: "라운드 주기" },
    meta_cycle_value: { en: "1 / day", ko: "하루 1회" },

    cta_join: { en: "Join round →", ko: "라운드 입장 →" },
    cta_leave: { en: "Leave lobby", ko: "로비 나가기" },
    cta_disabled: { en: "Join round →", ko: "라운드 입장 →" },
    cta_in_progress: { en: "Round in progress", ko: "라운드 진행 중" },

    status_can_join_new: { en: "You'll be the first joiner — round starts when 4 are in.", ko: "첫 참가자가 됩니다 — 4명이 모이면 라운드가 시작됩니다." },
    status_can_join_existing: { en: "Players already waiting", ko: "현재 대기 중" },
    status_already_joined: { en: "You're in. Round will start as soon as the lobby fills up.", ko: "참여 완료. 로비가 채워지면 자동으로 시작됩니다." },
    status_no_headline: { en: "No published headline right now. Check back when admin schedules one.", ko: "지금 발행된 헤드라인이 없습니다. 운영자가 헤드라인을 등록하면 다시 확인해주세요." },
    status_round_in_progress: { en: "A round is already underway. Wait for it to settle to join the next one.", ko: "이미 라운드가 진행 중입니다. 다음 라운드를 기다려주세요." },

    rules_title: { en: "Round rules (v1):", ko: "라운드 규칙 (v1):" },
    rule_capacity: { en: "Round starts the moment 4 players join.", ko: "4명이 모이면 즉시 라운드 시작." },
    rule_insider: { en: "1 of the 4 is randomly chosen as the truth-knowing insider — the other 3 deduce.", ko: "4명 중 1명이 무작위로 진실 인사이더가 되고, 나머지 3명이 추리." },
    rule_stake: { en: "Bet RatelPoints on REAL vs FAKE; cite a teammate's argument to flip in the last 10s.", ko: "REAL/FAKE에 RP 베팅; 마지막 10초에 동료의 근거를 인용해 베팅 변경 가능." },
    rule_settle: { en: "Winners take losers' stakes (×1.6 default) + insider bonus + flip-cite influence.", ko: "정답자가 패자의 stake (기본 ×1.6) + 인사이더 보너스 + flip 인용 영향력 보너스를 가져감." },
}
