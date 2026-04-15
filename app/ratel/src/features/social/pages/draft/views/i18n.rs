use dioxus_translate::*;

translate! {
    TeamDraftTranslate;

    drafts_label: { en: "Drafts", ko: "초안" },
    drafts_title: { en: "Drafts", ko: "초안" },
    drafts_subhead: {
        en: "unpublished · resume editing any time",
        ko: "개의 미발행 글 · 언제든 다시 편집할 수 있습니다",
    },
    new_post: { en: "New Post", ko: "새 글 작성" },

    badge_draft: { en: "Draft", ko: "초안" },
    edit: { en: "Edit", ko: "편집" },
    untitled: { en: "(Untitled draft)", ko: "(제목 없는 초안)" },

    updated_just_now: { en: "Just now", ko: "방금 전" },
    updated_minutes: { en: "min ago", ko: "분 전" },
    updated_hours: { en: "h ago", ko: "시간 전" },
    updated_days: { en: "d ago", ko: "일 전" },

    empty_title: { en: "No drafts available", ko: "초안이 없습니다" },
    empty_desc: {
        en: "Start a new post — your in-progress work will appear here.",
        ko: "새 글을 작성하면 작업 중인 초안이 여기에 표시됩니다.",
    },

    load_more: { en: "Load more", ko: "더 보기" },

    delete_title: { en: "Delete Draft?", ko: "초안을 삭제하시겠습니까?" },
    delete_desc_pre: {
        en: "Are you sure you want to delete this draft? This action ",
        ko: "이 초안을 삭제하시겠습니까? 이 작업은 ",
    },
    delete_desc_strong: {
        en: "cannot be undone",
        ko: "되돌릴 수 없습니다",
    },
    delete_desc_post: { en: ".", ko: "." },
    cancel: { en: "Cancel", ko: "취소" },
    confirm: { en: "Confirm", ko: "확인" },
    delete_success: { en: "Draft deleted", ko: "초안이 삭제되었습니다" },
}
