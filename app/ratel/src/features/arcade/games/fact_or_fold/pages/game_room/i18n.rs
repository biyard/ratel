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

    // ── News reveal stage ─────────────────────────────────────────
    news_source_lock: {
        en: "Source hidden (revealed at settlement)",
        ko: "출처 비공개 (정산 시 공개)",
    },
    news_difficulty: { en: "Difficulty", ko: "난이도" },
    news_pill_category_default: { en: "Subject", ko: "대상" },
    news_cta_label: { en: "Auto-advances to 1st bet", ko: "1차 베팅으로 자동 진행" },
    players_card_title: { en: "Round participants", ko: "라운드 참가자" },
    players_card_count: { en: "{$count} / {$capacity} present", ko: "{$count} / {$capacity} 도착" },
    players_status_reading: {
        en: "Reading the subject · waiting for 1st bet",
        ko: "뉴스 읽는 중 · 1차 베팅 대기",
    },
    players_status_bet_pending: { en: "Placing 1st bet…", ko: "1차 베팅 진행 중…" },
    players_status_writing: { en: "Writing rationale…", ko: "근거 작성 중…" },
    players_status_revealed: { en: "Rationale revealed", ko: "근거 공개됨" },
    players_status_debating: { en: "Debate stage", ko: "토론 단계" },
    players_status_done: { en: "Round settled", ko: "라운드 종료" },
    players_status_forfeited: { en: "Forfeited", ko: "기권" },
    players_pill_waiting: { en: "Waiting", ko: "대기" },
    players_pill_done: { en: "Done", ko: "완료" },
    players_pill_forfeited: { en: "OUT", ko: "기권" },
    players_you_badge: { en: "YOU", ko: "YOU" },

    // ── First bet stage ───────────────────────────────────────────
    insider_title: { en: "INSIDER · TRUTH", ko: "INSIDER · 진실 정보" },
    insider_tip: {
        en: "Tip: if another player cites your rationale and flips their bet, you take 30% of their stake.",
        ko: "팁: 다른 참가자가 당신 근거를 인용해 베팅을 바꾸면 그 사람 베팅의 30%를 가져옵니다.",
    },
    bet_card_title: { en: "REAL or FAKE?", ko: "진짜인가, 가짜인가?" },
    bet_card_sub: {
        en: "Decide in 10s. Your rationale goes in the next stage (30s).",
        ko: "10초 안에 결정. 근거는 다음 단계에서 30초간 따로 적습니다.",
    },
    bet_option_real_label: { en: "REAL", ko: "REAL" },
    bet_option_real_sub: { en: "This subject is real.", ko: "이 뉴스는 진짜다" },
    bet_option_fake_label: { en: "FAKE", ko: "FAKE" },
    bet_option_fake_sub: { en: "This subject is fake.", ko: "이 뉴스는 가짜다" },
    bet_slider_label: { en: "Stake (RatelPoints)", ko: "베팅 RatelPoint" },
    bet_submit: { en: "Confirm bet → Rationale", ko: "베팅 확정 → 근거 작성으로" },
    bet_submit_hint: { en: "Final — can't change.", ko: "확정 후 변경 불가" },
    bet_already_placed_title: { en: "Bet locked", ko: "베팅 확정됨" },
    bet_already_placed_body: {
        en: "Your bet is in. Waiting for the next stage.",
        ko: "베팅이 등록됐어요. 다음 단계까지 대기 중.",
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
    // ── Rationale write stage ─────────────────────────────────────
    reason_prompt_label: { en: "Your 1st bet", ko: "내 1차 베팅" },
    reason_prompt_text_real: { en: "REAL · {$amount} RP — why?", ko: "REAL · {$amount} RP — 왜?" },
    reason_prompt_text_fake: { en: "FAKE · {$amount} RP — why?", ko: "FAKE · {$amount} RP — 왜?" },
    reason_textarea_label: {
        en: "One-line rationale (50–200 chars, single submission)",
        ko: "한 줄 근거 (50~200자, 한 번만 제출)",
    },
    reason_textarea_placeholder: {
        en: "Why did you judge it that way? A sentence or two…",
        ko: "왜 그렇게 판단했는지 한두 문장으로...",
    },
    reason_warn: {
        en: "⚠ Your first answer is your final answer — no rewrite. The debate stage lets you add chat + flip.",
        ko: "⚠ 첫 답이 최종 답입니다. 다시 쓰기 불가능. 토론 단계에서 발화 큐 + 채팅으로 추가 의견 가능.",
    },
    reason_submit: { en: "Submit rationale", ko: "근거 제출" },
    reason_submit_hint: {
        en: "After submit, wait until everyone is done.",
        ko: "제출 후 다른 사람이 끝낼 때까지 대기",
    },
    reason_submitted_title: { en: "Rationale submitted", ko: "근거 제출 완료" },
    reason_submitted_body: {
        en: "Waiting for the rest of the table — auto-advances on the stage timer.",
        ko: "다른 참가자 대기 중 — 타이머 만료 시 자동 진행됩니다.",
    },
    reason_no_bet_warning: {
        en: "You didn't lock a bet last stage — rationale write is closed.",
        ko: "이전 단계에서 베팅이 등록되지 않았습니다. 근거 작성이 닫혀 있습니다.",
    },

    reason_tips_title: { en: "Writing tips", ko: "근거 작성 팁" },
    reason_tips_body: {
        en: "All 4 rationales surface together next stage. The persuasive line is what wins RP — phrase analysis and source quirks pull harder than vibes.",
        ko: "다음 단계에서 4명 근거가 동시에 공개됩니다. 누가 누구의 어느 문장 때문에 마음을 바꿨는지가 정산에 영향. 설득력 있는 한 줄이 RP를 만듭니다.",
    },
    reason_others_label: { en: "Other players (LIVE)", ko: "다른 참가자 (LIVE)" },
    reason_pulse_submitted: { en: "Submitted", ko: "제출" },
    reason_pulse_writing: { en: "Writing…", ko: "작성 중" },
    reason_insider_hint: {
        en: "INSIDER: knowing the truth, your strongest weapons are formal cues (phrasing, structure) that move people who lean the other way.",
        ko: "INSIDER: 진실을 알고 있다면, 반대편 직관을 흔들 만한 *표현 분석*이나 *형식 단서*가 강한 무기입니다.",
    },

    stage_stub_reasoning_write: {
        en: "Rationale-write view rendering will be wired in step 7.",
        ko: "근거 작성 화면은 다음 단계에서 채워집니다.",
    },
    // ── Rationale reveal stage ────────────────────────────────────
    reveal_camp_real_label: { en: "◎ REAL", ko: "◎ REAL" },
    reveal_camp_fake_label: { en: "⊘ FAKE", ko: "⊘ FAKE" },
    reveal_camp_count: { en: "{$count} · {$rp} RP", ko: "{$count}명 · {$rp} RP" },
    reveal_camp_vs: { en: "VS", ko: "VS" },
    reveal_hint_prefix: { en: "Read every rationale in 20s.", ko: "20초 안에 모두의 근거를 읽으세요." },
    reveal_hint_body: {
        en: "Mark the one that feels decisive — ⌬. It'll auto-attach if you flip your bet in the live debate.",
        ko: "결정적이라고 느낀 문장에 ⌬ 표시 — 토론 단계에서 마음 변경 시 자동 첨부됩니다.",
    },
    reveal_card_bet_pill: { en: "{$side} · {$rp} RP", ko: "{$side} · {$rp} RP" },
    reveal_quote_btn: { en: "⌬ Decisive cite", ko: "⌬ 결정적 인용" },
    reveal_cta_hint: {
        en: "Auto-advances when the stage timer runs out.",
        ko: "단계 타이머가 만료되면 자동 진행됩니다.",
    },
    reveal_cta_no_quote: { en: "(no decisive cite marked yet)", ko: "(인용 표시 없음)" },
    reveal_cta_one_quote: { en: "Decisive cite: {$name}", ko: "결정적 인용: {$name}" },

    stage_stub_reasoning_reveal: {
        en: "Rationale-reveal view rendering will be wired in step 8.",
        ko: "근거 공개 화면은 다음 단계에서 채워집니다.",
    },
    // ── Live debate stage ─────────────────────────────────────────
    chat_title: { en: "Free debate · chat", ko: "자유 토론 · 채팅" },
    chat_sub: { en: "Short bursts · 80-char cap · 70s", ko: "짧게 던지기 · 80자 제한 · 70초" },
    chat_placeholder: {
        en: "Short message (Enter to send, 80-char cap)",
        ko: "짧게 (Enter로 전송, 80자 제한)",
    },
    chat_send: { en: "⏎ Send", ko: "⏎ 전송" },
    chat_empty: {
        en: "No messages yet — break the ice.",
        ko: "아직 메시지가 없습니다. 먼저 운을 떼주세요.",
    },
    final_tag: { en: "⚐ FINAL", ko: "⚐ FINAL" },
    final_text_locked: {
        en: "Final-bet flip unlocks in the last 10s of debate.",
        ko: "마지막 10초에 최종 베팅 변경 가능 (지금은 잠금)",
    },
    final_text_open_no_cite: {
        en: "Flip slot OPEN — but mark a decisive cite in stage 4 to unlock the flip.",
        ko: "변경 슬롯 열림 — 단, 4단계에서 ⌬ 인용을 표시해야 변경할 수 있어요.",
    },
    final_text_open_ready: {
        en: "Flip slot OPEN — keep or flip your bet now.",
        ko: "변경 슬롯 열림 — 베팅을 유지하거나 변경하세요.",
    },
    final_btn_keep: { en: "Keep", ko: "유지" },
    final_btn_flip: { en: "Flip", ko: "변경" },
    final_already_flipped: { en: "Flipped — locked.", ko: "변경 완료 — 잠금" },
    // ── Settlement stage ──────────────────────────────────────────
    reveal_banner_label: { en: "Truth reveal", ko: "진실 공개" },
    reveal_verdict_real: { en: "REAL", ko: "REAL" },
    reveal_verdict_fake: { en: "FAKE", ko: "FAKE" },
    reveal_pending: {
        en: "Settling — final results in a moment.",
        ko: "정산 중 — 잠시 후 최종 결과가 표시됩니다.",
    },
    exit_to_home_label: {
        en: "Settled · Back to Arcade",
        ko: "정산완료 → 홈으로",
    },
    exit_to_home_busy: {
        en: "Wrapping up…",
        ko: "마무리 중…",
    },
    reveal_source_label: { en: "Verified sources:", ko: "검증 출처:" },
    result_table_title: { en: "Final standings", ko: "4명 결과" },
    result_table_sub: {
        en: "Stake + influence + insider bonus combined",
        ko: "베팅 + 영향력 + 인사이더 보너스 합산",
    },
    result_judgement_won: { en: "Correct + bonuses", ko: "정답 + 보너스" },
    result_judgement_lost: { en: "Wrong — stake lost", ko: "오답 — 베팅 손실" },
    result_judgement_insider_won: { en: "Correct + insider bonus", ko: "정답 + 인사이더 보너스" },
    my_settlement_title: { en: "Your settlement", ko: "내 정산" },
    my_settle_base: { en: "Base refund", ko: "기본 베팅 회수" },
    my_settle_correct: { en: "Correct bonus", ko: "정답 보너스" },
    my_settle_pool: { en: "Pool share", ko: "패자 풀 분배" },
    my_settle_influence: { en: "Influence bonus", ko: "영향력 보너스" },
    my_settle_insider: { en: "Insider bonus", ko: "INSIDER 보너스" },
    my_settle_total: { en: "Total delta", ko: "총 손익" },

    essence_title: { en: "Register Essence", ko: "Essence 등록" },
    essence_sub: {
        en: "Add your rationale to your Essence index.",
        ko: "자신의 근거를 Essence 인덱스에 등록합니다.",
    },
    essence_my_rationale_label: { en: "Your rationale", ko: "내 근거" },
    essence_register: { en: "Register to Essence", ko: "Essence에 등록" },
    essence_registered: { en: "Registered ✓", ko: "등록 완료 ✓" },
    essence_ineligible: {
        en: "Your rationale was shorter than 50 chars — not Essence-eligible.",
        ko: "근거가 50자 미만이라 Essence 등록 대상이 아닙니다.",
    },

    stage_stub_settlement: {
        en: "Settlement view rendering will be wired in step 10.",
        ko: "정산 화면은 다음 단계에서 채워집니다.",
    },
}
