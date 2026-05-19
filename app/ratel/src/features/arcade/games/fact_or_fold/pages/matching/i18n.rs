use dioxus_translate::*;

translate! {
    FactFoldMatchingTranslate;

    eyebrow_waiting: { en: "Matching · auto-starts at {$capacity}", ko: "매칭 중 · {$capacity}명이 모이면 자동 시작" },
    eyebrow_full: { en: "Matched · entering the round", ko: "매칭 완료 · 라운드 시작" },
    eyebrow_no_round: { en: "No active round — head back home.", ko: "참여 중인 라운드가 없습니다 — 홈으로 돌아가세요." },

    title: { en: "FACT OR FOLD", ko: "FACT OR FOLD" },
    subtitle: {
        en: "Today's subject is revealed when the round begins. One of you will be the truth insider — the other three must judge.",
        ko: "오늘의 뉴스는 라운드 시작 시 공개됩니다. 4명 중 1명이 무작위로 진실 인사이더가 되고, 나머지 3명이 베팅과 근거로 판별합니다.",
    },
    buyin_note: { en: "Buy-in {$chips} chips already escrowed · refunded if you cancel", ko: "buy-in {$chips} 칩 이미 예치됨 · 라운드 종료 시 결과만큼 환원" },

    slot_empty: { en: "Waiting…", ko: "대기 중…" },
    slot_pill_empty: { en: "EMPTY", ko: "EMPTY" },
    slot_pill_ready: { en: "READY", ko: "READY" },
    slot_seat: { en: "SEAT {$n}", ko: "SEAT {$n}" },
    you_tag: { en: "YOU", ko: "YOU" },

    progress_label: { en: "Lobby fill", ko: "모임 진행" },
    progress_count: { en: "{$current} / {$capacity}", ko: "{$current} / {$capacity}" },

    hint_timing_strong: { en: "~3 min", ko: "약 3분" },
    hint_timing_body: { en: ": news 30s · bet 10s · rationale 30s · reveal 20s · debate 70s · settle.", ko: ": 뉴스 30s · 베팅 10s · 근거 30s · 공개 20s · 토론 70s · 정산" },
    hint_insider_strong: { en: "1 insider", ko: "1명은 인사이더" },
    hint_insider_body: { en: ": knows the truth, must sway the other 3 to score the bonus.", ko: ": 진실을 알지만 다른 3명을 흔들어야 보너스를 얻습니다." },
    hint_persuade_strong: { en: "Persuasive rationale", ko: "설득력 있는 근거" },
    hint_persuade_body: { en: " wins RP. One bet-flip slot in the last 10s of debate.", ko: "가 RP를 만듭니다. 토론 마지막 10초에 베팅 변경 슬롯 1회." },

    cancel_btn: { en: "Cancel · refund chips", ko: "매칭 취소 · 칩 환불" },
    cancel_hint: {
        en: "Round auto-starts the moment the seats fill — you'll jump straight in.",
        ko: "4명 모이는 즉시 자동으로 라운드 화면으로 이동합니다.",
    },
    leave_error: { en: "Couldn't cancel — try again.", ko: "취소에 실패했습니다. 다시 시도해주세요." },
}
