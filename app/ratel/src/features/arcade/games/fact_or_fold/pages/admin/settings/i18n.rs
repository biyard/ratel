use dioxus_translate::*;

translate! {
    FactFoldAdminSettingsTranslate;

    page_title: { en: "Settings", ko: "설정" },
    loading: { en: "Loading settings…", ko: "설정 불러오는 중…" },
    save: { en: "Save", ko: "저장" },
    saving: { en: "Saving…", ko: "저장 중…" },
    saved: { en: "Saved.", ko: "저장됨." },

    // Section 01 — Round
    section_round_title: { en: "01 · Round", ko: "01 · 라운드" },
    section_round_sub: { en: "Capacity + per-stage timing", ko: "정원 + 단계별 시간" },
    round_capacity: { en: "Round capacity", ko: "라운드 정원" },
    round_capacity_desc: {
        en: "Players per round (v1 fixed at 4 in spec; this knob is for ops experiments).",
        ko: "라운드당 인원 (v1 스펙 고정 4명; 운영 실험용 노브).",
    },
    round_capacity_solo_warning: {
        en: "Solo mode (capacity 1) bypasses the insider dynamic — use for dev / smoke tests only.",
        ko: "솔로 모드 (정원 1)는 인사이더 게임을 우회해요 — 개발/스모크 테스트 용도로만 사용하세요.",
    },
    stage_news_reveal_sec: { en: "Stage 1 — News reveal (sec)", ko: "단계 1 — 뉴스 공개 (초)" },
    stage_bet_sec: { en: "Stage 2 — First bet (sec)", ko: "단계 2 — 1차 베팅 (초)" },
    stage_rationale_sec: { en: "Stage 3 — Rationale (sec)", ko: "단계 3 — 근거 작성 (초)" },
    stage_reveal_sec: { en: "Stage 4 — Reveal (sec)", ko: "단계 4 — 근거 공개 (초)" },
    stage_debate_sec: { en: "Stage 5 — Debate (sec)", ko: "단계 5 — 토론 (초)" },
    stage_sec_desc: { en: "Seconds before auto-advance.", ko: "자동 진행까지의 초." },

    // Section 02 — Economy
    section_economy_title: { en: "02 · Economy", ko: "02 · 경제" },
    section_economy_sub: { en: "RatelPoint stakes + bonuses", ko: "RP 베팅 + 보너스" },
    min_bet_rp: { en: "Min bet", ko: "최소 베팅" },
    min_bet_rp_desc: { en: "Lowest RP a player may stake per round.", ko: "한 라운드 최저 베팅 RP." },
    max_bet_rp: { en: "Max bet", ko: "최대 베팅" },
    max_bet_rp_desc: { en: "Stake cap per round.", ko: "한 라운드 베팅 상한." },
    correct_multiplier: { en: "Correct-side multiplier", ko: "정답 보너스 배율" },
    correct_multiplier_desc: {
        en: "Stored as basis points (10000 = 1.0×); UI shows the human ×.",
        ko: "bps로 저장됨 (10000 = 1.0×); UI는 ×로 표시.",
    },
    insider_bonus: { en: "Insider correct bonus", ko: "인사이더 정답 보너스" },
    insider_bonus_desc: {
        en: "Extra ×stake when the insider bets the truth and wins.",
        ko: "인사이더가 정답 진영으로 이겼을 때 추가 ×stake.",
    },
    influence_bonus: { en: "Influence (flip-cite) bonus", ko: "인용 영향력 보너스" },
    influence_bonus_desc: {
        en: "Share of the cited player's stake the citer takes when the flip wins.",
        ko: "인용한 사람이 flip해서 이겼을 때 인용된 사람이 가져오는 stake 비율.",
    },
    signup_rp: { en: "Signup bonus RP", ko: "가입 보너스 RP" },
    signup_rp_desc: { en: "One-time grant on account creation.", ko: "계정 생성 시 1회 지급." },

    // Section 03 — Insider
    section_insider_title: { en: "03 · Insider", ko: "03 · 인사이더" },
    section_insider_sub: { en: "v1: TRUTH-KNOWER 1명 fixed", ko: "v1: TRUTH-KNOWER 1명 고정" },
    insider_note: {
        en: "v1 ships a single truth-knowing insider per round (D1/D2). Mafia mode (false-statement insider) deferred to v2 — no admin knob until then.",
        ko: "v1은 라운드당 진실 인사이더 1명 고정(D1/D2). 거짓 인사이더(mafia mode)는 v2 도입 — 그때까지 관리자 노브 없음.",
    },

    // Section 04 — Operations
    section_ops_title: { en: "04 · Operations", ko: "04 · 운영" },
    section_ops_sub: { en: "Reconnect + queue health", ko: "재접속 + 큐 상태" },
    reconnect_grace: { en: "Reconnect grace (sec)", ko: "재접속 유예 (초)" },
    reconnect_grace_desc: {
        en: "Drop window before a disconnected player auto-forfeits and gets refunded.",
        ko: "연결 끊긴 참가자가 자동 기권 + 환불되기까지의 유예 시간.",
    },
    queue_alert: { en: "Queue alert threshold (days)", ko: "큐 알림 임계 (일)" },
    queue_alert_desc: {
        en: "When the latest scheduled subject is closer than this, the admin sees a low-queue banner (FR-45).",
        ko: "최신 스케줄 대상이 이 일수보다 가까우면 관리자에게 큐 부족 배너 표시(FR-45).",
    },

    // Mockup-only deferred items
    deferred_title: { en: "Deferred (mockup mentions, not yet wired):", ko: "보류 (mockup만 있고 아직 미반영):" },
    deferred_auto_publish: {
        en: "Auto-publish KST hour-of-day — needs a scheduler decision.",
        ko: "KST 자동 발행 시각 — 스케줄러 결정 필요.",
    },
    deferred_auto_backfill: {
        en: "Auto-backfill from insider pool when queue is empty — out of v1 scope.",
        ko: "큐가 비면 인사이더 풀에서 자동 백필 — v1 스코프 외.",
    },
    deferred_essence_policy: {
        en: "Essence opt-in policies (default opt-in, min length, false-side handling) land with PR6.",
        ko: "Essence opt-in 정책 (기본 opt-in, 최소 길이, 거짓 진영 처리)은 PR6에서.",
    },
    deferred_danger_zone: {
        en: "Danger zone (force-end round, export, grant admin) — separate ops surface, not this form.",
        ko: "위험 영역 (강제 종료, 내보내기, 관리자 부여) — 이 폼이 아닌 별도 운영 화면에서.",
    },

    // Units
    unit_people: { en: "people", ko: "명" },
    unit_sec: { en: "sec", ko: "초" },
    unit_rp: { en: "RP", ko: "RP" },
    unit_day: { en: "days", ko: "일" },
}
