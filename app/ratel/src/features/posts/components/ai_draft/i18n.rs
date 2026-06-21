use crate::common::*;

translate! {
    AiDraftTranslate;

    button_label: {
        en: "AI Draft",
        ko: "AI 로 작성",
    },
    pro_badge: {
        en: "Pro",
        ko: "Pro",
    },

    // Upsell modal
    upsell_eyebrow: {
        en: "Pro feature",
        ko: "Pro 기능",
    },
    upsell_title_lead: {
        en: "AI draft is available on ",
        ko: "AI 초안 작성은 ",
    },
    upsell_title_accent: {
        en: "Pro or higher",
        ko: "Pro 이상",
    },
    upsell_title_tail: {
        en: "",
        ko: "에서 사용 가능합니다",
    },
    upsell_sub: {
        en: "A short form turns into a structured opinion-gathering draft. Background, purpose, content, and participation — all five sections filled automatically.",
        ko: "짧은 입력 몇 줄만으로 정리된 의견수렴 포스트 초안을 받아보세요. 추진배경부터 참여 안내까지 5개 섹션이 자동으로 채워집니다.",
    },
    upsell_benefit_1_title: {
        en: "Structured draft auto-generated",
        ko: "구조화된 초안 자동 생성",
    },
    upsell_benefit_1_desc: {
        en: "Provide topic, background, and the input you want — get five sections back in the editor.",
        ko: "주제와 배경, 듣고 싶은 의견만 짧게 입력하면 5개 섹션이 모두 채워진 초안이 에디터에 들어옵니다.",
    },
    upsell_benefit_2_title: {
        en: "10 minutes → 1 minute",
        ko: "10분 → 1분",
    },
    upsell_benefit_2_desc: {
        en: "Skip wrestling with format and focus on refining what matters.",
        ko: "백지에서 형식을 잡는 시간을 줄이고, 내용 다듬는 데 집중할 수 있습니다.",
    },
    upsell_benefit_3_title: {
        en: "Korean & English output",
        ko: "한국어 · 영어 출력 지원",
    },
    upsell_benefit_3_desc: {
        en: "Picks your UI language by default; override per-draft in the form.",
        ko: "UI 언어에 맞춰 자동 선택되며, 폼에서 다른 언어로 명시적으로 바꿀 수도 있습니다.",
    },
    upsell_cta: {
        en: "Upgrade membership",
        ko: "멤버십 업그레이드",
    },
    upsell_dismiss: {
        en: "Maybe later",
        ko: "다음에 다시 볼래요",
    },
    upsell_tier_note: {
        en: "Required tier: Pro. Also available on Max / Vip / Enterprise.",
        ko: "필요 멤버십: Pro · 더 높은 티어(Max / Vip / Enterprise)에서도 동일하게 사용 가능",
    },

    // Draft modal
    modal_title: {
        en: "AI draft for opinion gathering",
        ko: "의견수렴 포스트 초안 작성",
    },
    modal_eyebrow: {
        en: "AI Draft",
        ko: "AI Draft",
    },
    step_picker: {
        en: "Template",
        ko: "템플릿",
    },
    step_form: {
        en: "Inputs",
        ko: "입력",
    },
    step_generate: {
        en: "Generate",
        ko: "생성",
    },

    // Picker pane
    picker_title: {
        en: "Pick a template",
        ko: "템플릿을 선택하세요",
    },
    picker_sub: {
        en: "Only Opinion Gathering is available for now — more templates coming soon.",
        ko: "현재는 의견수렴 (Opinion gathering) 템플릿만 지원합니다. 다른 템플릿은 곧 추가될 예정입니다.",
    },
    template_opinion_title: {
        en: "Opinion gathering",
        ko: "의견수렴",
    },
    template_opinion_desc: {
        en: "Five-section structure for collecting community input — Background / Purpose / Content / Topics / How to Participate.",
        ko: "정책·사안에 대한 시민 의견을 모으기 위한 5개 섹션 구조 — 추진배경 / 추진목적 / 추진내용 / 의견수렴 사항 / 참여 안내.",
    },
    template_pill_available: {
        en: "Available",
        ko: "Available",
    },
    template_pill_soon: {
        en: "Coming soon",
        ko: "Coming soon",
    },

    // Form pane
    form_title: {
        en: "Give a few short details so AI can draft",
        ko: "의견수렴 초안을 위한 짧은 정보를 알려주세요",
    },
    form_sub: {
        en: "AI builds a five-section draft from these. Your inputs are not stored — they're used only for this request.",
        ko: "아래 정보를 바탕으로 AI 가 5개 섹션이 채워진 초안을 작성합니다. 입력하신 정보는 저장되지 않고 초안 생성에만 사용됩니다.",
    },
    field_topic_label: {
        en: "Topic",
        ko: "주제",
    },
    field_topic_hint: {
        en: "One line about what input you're gathering.",
        ko: "한 줄로 무엇에 대한 의견을 모으려는지 적어주세요.",
    },
    field_topic_placeholder: {
        en: "e.g. Relocating public-area smoking zones",
        ko: "예: 공공장소 흡연 구역 재배치",
    },
    field_background_label: {
        en: "Background / motivation",
        ko: "배경 / 문제의식",
    },
    field_background_hint: {
        en: "Why this needs input now — 2-3 sentences.",
        ko: "왜 지금 이 의견수렴이 필요한지, 어떤 문제가 있는지 2-3 문장으로.",
    },
    field_background_placeholder: {
        en: "e.g. Complaints about smoking zones near residential areas have increased lately…",
        ko: "예: 최근 주거지·학교 근처 흡연 구역에 대한 민원이 늘었습니다. 위치 적정성을 재검토하고 합리적 재배치 방안을 마련할 필요가 있습니다.",
    },
    field_feedback_label: {
        en: "Feedback you want",
        ko: "듣고 싶은 의견",
    },
    field_feedback_hint: {
        en: "What questions and topics you want answers on.",
        ko: "시민들에게 어떤 질문을 던지고 싶은지, 어떤 쟁점에 대한 답을 듣고 싶은지 자유롭게.",
    },
    field_feedback_placeholder: {
        en: "e.g. Evaluation of current locations, preferences among alternative sites, values to prioritize in operations…",
        ko: "예: 현재 흡연 구역 위치에 대한 평가, 대안 후보지에 대한 선호, 흡연 구역 운영 시 가장 중요한 가치(위생/접근성/안전 등).",
    },
    field_notes_label: {
        en: "Participation notes (optional)",
        ko: "참여 안내 (선택)",
    },
    field_notes_hint: {
        en: "Anything specific about how, when, or anonymously to participate.",
        ko: "참여 방법·기한·익명 옵션 등 특별히 알리고 싶은 내용이 있다면.",
    },
    field_notes_placeholder: {
        en: "e.g. Open for 2 weeks. Comments or anonymous form.",
        ko: "예: 향후 2주간 진행. 댓글 또는 익명 폼으로 참여 가능.",
    },
    field_language_label: {
        en: "Output language",
        ko: "출력 언어",
    },
    field_language_ko: {
        en: "Korean",
        ko: "한국어",
    },
    field_language_en: {
        en: "English",
        ko: "English",
    },
    required_mark: {
        en: "*",
        ko: "*",
    },

    // Loading pane
    loading_title: {
        en: "Drafting your post",
        ko: "초안을 작성하고 있습니다",
    },
    loading_sub: {
        en: "Usually 3-8 seconds. Hit cancel to discard — your quota stays untouched.",
        ko: "평균 3-8초 정도 소요됩니다. 잠시만 기다려 주세요. 취소를 누르면 사용량이 차감되지 않습니다.",
    },

    // Error pane
    error_title: {
        en: "Draft generation failed",
        ko: "초안 생성에 실패했습니다",
    },
    error_default_msg: {
        en: "We couldn't reach the model. This is likely a temporary network or model issue.",
        ko: "AI 모델 응답을 받지 못했습니다. 일시적인 네트워크 또는 모델 문제일 가능성이 높습니다.",
    },
    error_summary: {
        en: "Your inputs are preserved. If retrying keeps failing, try again later or compose manually. This failure does not consume your per-post AI allowance.",
        ko: "입력하신 내용은 그대로 유지됩니다. 다시 시도해도 같은 문제가 반복되면 잠시 후에 시도하거나 직접 작성해 주세요. 이 실패는 포스트의 1회 사용량에 차감되지 않습니다.",
    },

    // Footer
    foot_picker_info: {
        en: "One AI draft per post.",
        ko: "포스트당 1회만 사용할 수 있습니다.",
    },
    foot_form_info: {
        en: "Your inputs are not stored.",
        ko: "입력 정보는 저장되지 않습니다.",
    },
    foot_loading_info: {
        en: "Processing…",
        ko: "요청 처리 중…",
    },
    foot_error_info: {
        en: "Your quota was not consumed.",
        ko: "사용량은 차감되지 않았습니다.",
    },
    btn_cancel: {
        en: "Cancel",
        ko: "취소",
    },
    btn_close: {
        en: "Close",
        ko: "닫기",
    },
    btn_back: {
        en: "Back",
        ko: "이전",
    },
    btn_next: {
        en: "Next",
        ko: "다음",
    },
    btn_generate: {
        en: "Generate draft",
        ko: "초안 생성",
    },
    btn_retry: {
        en: "Try again",
        ko: "다시 시도",
    },

    // Confirm-overwrite dialog
    overwrite_title: {
        en: "Existing content will be replaced",
        ko: "기존 내용이 대체됩니다",
    },
    overwrite_msg: {
        en: "AI will replace the title and body currently in the editor. This cannot be undone.",
        ko: "AI 생성 결과가 현재 에디터의 제목과 본문을 대체합니다. 되돌릴 수 없습니다.",
    },
    overwrite_confirm: {
        en: "Continue",
        ko: "계속하기",
    },
    overwrite_cancel: {
        en: "Cancel",
        ko: "취소",
    },
}
