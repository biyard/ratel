use dioxus_translate::*;

translate! {
    FactFoldAdminNewSubjectTranslate;

    page_title: { en: "New subject", ko: "신규 대상" },

    section_truth_title: { en: "01 · Truth + difficulty", ko: "01 · 정답 + 난이도" },
    section_truth_sub: { en: "Operator-only — hidden from players until settlement.", ko: "운영자 전용 — 정산 전까지 참가자에게 비공개." },
    verdict_label: { en: "Verdict", ko: "정답" },
    verdict_hint: { en: "Pick what the round will reveal as truth at settlement.", ko: "정산 시 진실로 공개될 정답을 선택." },
    difficulty_label: { en: "Difficulty (1–5)", ko: "난이도 (1–5)" },

    section_text_title: { en: "02 · Subject text", ko: "02 · 대상 텍스트" },
    section_text_sub: { en: "What players read in stage 1.", ko: "참가자가 단계 1에서 읽는 내용." },
    headline_text: { en: "Subject", ko: "대상" },
    headline_text_placeholder: { en: "e.g. \"Bank of Korea cuts rate 0.5% on June 4\"", ko: "예: \"한국은행, 6월 4일 0.5%p 인하\"" },
    body_excerpt: { en: "Body excerpt (200–500 chars)", ko: "본문 발췌 (200–500자)" },
    body_excerpt_placeholder: { en: "Quote the article body — players will skim this in 30 seconds.", ko: "기사 본문 발췌 — 참가자가 30초 안에 훑어봅니다." },

    section_meta_title: { en: "03 · Source + tags", ko: "03 · 출처 + 태그" },
    section_meta_sub: { en: "Surfaced alongside the subject.", ko: "대상과 함께 표시됨." },
    source_label: { en: "Source label", ko: "출처 라벨" },
    source_label_placeholder: { en: "e.g. \"Korea Times · 5/13\"", ko: "예: \"한국경제 · 5/13\"" },
    category_tags: { en: "Category tags (comma-separated)", ko: "카테고리 태그 (쉼표로 구분)" },
    category_tags_placeholder: { en: "e.g. economy, monetary-policy", ko: "예: 경제, 통화정책" },

    section_insider_title: { en: "04 · Insider", ko: "04 · 인사이더" },
    section_insider_sub: { en: "v1 — single TRUTH-KNOWER per round (D1 fixed).", ko: "v1 — 라운드당 진실 인사이더 1명 (D1 고정)." },
    insider_statement: { en: "Truth statement (delivered privately)", ko: "진실 진술 (비공개 전달)" },
    insider_statement_placeholder: { en: "1 sentence the insider sees at round start. Used as their persuasion lever.", ko: "라운드 시작 시 인사이더에게 표시되는 한 문장. 설득 근거로 사용." },
    insider_hint: {
        en: "v1 captures only the truth statement — no \"possibly false\" lying-insider field (mafia mode v2 deferred).",
        ko: "v1은 진실 진술만 수집 — \"거짓 가능\" 인사이더 필드 없음 (마피아 모드는 v2).",
    },

    section_reveal_title: { en: "05 · Settlement reveal", ko: "05 · 정산 공개" },
    section_reveal_sub: { en: "Shown to all players when the round ends.", ko: "라운드 종료 시 모든 참가자에게 표시." },
    reveal_summary: { en: "Truth summary", ko: "진실 요약" },
    reveal_summary_placeholder: { en: "2–3 sentences explaining why the verdict is what it is.", ko: "정답이 그러한 이유를 2–3문장으로 설명." },
    reveal_sources: { en: "Verification sources (0–5)", ko: "검증 출처 (0–5개)" },
    reveal_source_label_placeholder: { en: "Label", ko: "라벨" },
    reveal_source_add: { en: "Add source", ko: "출처 추가" },

    section_publish_title: { en: "06 · Publish", ko: "06 · 발행" },
    section_publish_sub: { en: "Save as draft, or schedule a publish time.", ko: "초안으로 저장하거나 발행 시각 예약." },
    schedule_label: { en: "Scheduled publish at (local time)", ko: "발행 예약 시각 (로컬 시간)" },
    schedule_hint: { en: "Leave empty to save as Draft.", ko: "비워두면 초안으로 저장됩니다." },

    fields_incomplete: { en: "Fill required fields above to enable Save / Schedule.", ko: "위 필수 항목을 채우면 저장 / 예약이 활성화됩니다." },
    save_draft: { en: "Save draft", ko: "초안 저장" },
    schedule_publish: { en: "Schedule publish", ko: "예약 발행" },
}
