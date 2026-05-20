use crate::features::spaces::pages::report::*;

translate! {
    ReportListTranslate;

    // ── Topbar ─────────────────────────────────────────
    back_aria: {
        en: "Back",
        ko: "뒤로",
    },
    workspace_label: {
        en: "Climate Action DAO · @climate_action",
        ko: "Climate Action DAO · @climate_action",
    },
    workspace_logo: {
        en: "R",
        ko: "R",
    },
    workspace_title: {
        en: "Reports",
        ko: "보고서",
    },
    new_report_aria: {
        en: "New report",
        ko: "새 보고서",
    },
    new_report_btn: {
        en: "New Report",
        ko: "새 보고서",
    },

    // ── Section heading + stats ───────────────────────
    section_title: {
        en: "Reports",
        ko: "보고서",
    },
    stat_total: {
        en: "Total",
        ko: "전체",
    },
    stat_drafts: {
        en: "Drafts",
        ko: "초안",
    },
    stat_published: {
        en: "Published",
        ko: "발행됨",
    },

    // ── Filter chips ──────────────────────────────────
    filter_all: {
        en: "All",
        ko: "전체",
    },
    filter_drafts: {
        en: "Drafts",
        ko: "초안",
    },
    filter_published: {
        en: "Published",
        ko: "발행됨",
    },

    // ── Create card ───────────────────────────────────
    create_title: {
        en: "New report",
        ko: "새 보고서",
    },
    create_sub_prefix: {
        en: "Start from a blank document — type ",
        ko: "빈 문서에서 시작 — ",
    },
    create_sub_suffix: {
        en: " to insert a chart from your analyzes",
        ko: " 입력 시 analyze에서 만든 데이터로 차트 삽입",
    },
    create_cta: {
        en: "Create",
        ko: "만들기",
    },

    // ── Report card ───────────────────────────────────
    card_menu_aria: {
        en: "Options",
        ko: "옵션",
    },
    card_open_cta: {
        en: "Open",
        ko: "열기",
    },
    status_draft: {
        en: "Draft",
        ko: "초안",
    },
    status_published: {
        en: "Published",
        ko: "발행됨",
    },

    // ── Relative time buckets (client-side formatted) ─
    time_just_now: {
        en: "just now",
        ko: "방금",
    },
    time_minutes_suffix: {
        en: "m ago",
        ko: "분 전",
    },
    time_hours_suffix: {
        en: "h ago",
        ko: "시간 전",
    },
    time_days_suffix: {
        en: "d ago",
        ko: "일 전",
    },
    time_weeks_suffix: {
        en: "w ago",
        ko: "주 전",
    },
    time_months_suffix: {
        en: "mo ago",
        ko: "개월 전",
    },
    time_years_suffix: {
        en: "y ago",
        ko: "년 전",
    },

    // ── Card overflow menu ────────────────────────────
    menu_delete: {
        en: "Delete",
        ko: "삭제하기",
    },

    // ── Delete confirmation modal ────────────────────
    modal_eyebrow: {
        en: "Destructive · Cannot be undone",
        ko: "Destructive · 되돌릴 수 없음",
    },
    modal_title: {
        en: "Delete report",
        ko: "보고서 삭제",
    },
    modal_body: {
        en: "Delete this report?",
        ko: "이 보고서를 삭제하시겠습니까?",
    },
    modal_note: {
        en: "The body, charts, and aggregate mappings cannot be recovered once deleted.",
        ko: "한 번 삭제하면 본문·차트·집계 매핑 모두 복구할 수 없습니다.",
    },
    modal_cancel: {
        en: "Cancel",
        ko: "취소",
    },
    modal_confirm: {
        en: "Delete",
        ko: "삭제",
    },
}

translate! {
    ReportTranslate;

    title_editor: {
        en: "Report Editor",
        ko: "보고서 편집기",
    },

    btn_toggle_edit: {
        en: "Toggle Edit",
        ko: "편집 전환",
    },

    btn_generate_report: {
        en: "Generate AI Report",
        ko: "AI 레포트 생성하기",
    },

    btn_edit: {
        en: "Edit",
        ko: "편집",
    },

    btn_save: {
        en: "Save",
        ko: "저장",
    },

    status_readonly: {
        en: "Read-only",
        ko: "읽기 모드",
    },

    status_editable: {
        en: "Editable",
        ko: "편집 모드",
    },

    placeholder: {
        en: "AI report will appear here...",
        ko: "AI 보고서가 여기에 표시됨...",
    },

    generating: {
        en: "Generating...",
        ko: "생성 중...",
    },

    generate_failed: {
        en: "Failed to generate",
        ko: "생성 실패",
    }
}
