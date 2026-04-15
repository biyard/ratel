use crate::features::social::pages::user_draft::*;

translate! {
    UserDraftsTranslate;

    page_title: { en: "Drafts", ko: "초안" },
    total_label: { en: "total", ko: "개" },
    back: { en: "Back", ko: "뒤로" },
    new_post: { en: "New Post", ko: "새 게시물" },

    stat_total: { en: "Total drafts", ko: "전체 초안" },
    stat_words: { en: "Words written", ko: "작성 단어" },
    stat_last: { en: "Last edited", ko: "마지막 편집" },
    unit_words: { en: "words", ko: "단어" },
    time_ago: { en: "ago", ko: "전" },
    time_just_now: { en: "just now", ko: "방금" },
    time_never: { en: "—", ko: "—" },

    filter_all: { en: "All", ko: "전체" },
    filter_today: { en: "Today", ko: "오늘" },
    filter_week: { en: "This Week", ko: "이번 주" },
    filter_older: { en: "Older", ko: "이전" },
    filter_space: { en: "Space-enabled", ko: "스페이스 사용" },

    sort_label: { en: "Sort", ko: "정렬" },
    sort_recent: { en: "Recently edited", ko: "최근 편집순" },
    sort_oldest: { en: "Oldest first", ko: "오래된 순" },
    sort_title: { en: "Title A → Z", ko: "제목순" },
    sort_words: { en: "Most words", ko: "단어 많은순" },

    section_today: { en: "Today", ko: "오늘" },
    section_week: { en: "This Week", ko: "이번 주" },
    section_older: { en: "Older", ko: "이전" },

    untitled: { en: "Untitled draft", ko: "제목 없는 초안" },
    empty_excerpt: { en: "Tell your story… Use the toolbar to format text, add links, and embed images.", ko: "이야기를 들려주세요…" },
    badge_space: { en: "Space", ko: "스페이스" },
    badge_writing: { en: "Writing now", ko: "작성 중" },
    meta_images: { en: "images", ko: "이미지" },
    meta_saved: { en: "Saved", ko: "저장됨" },

    resume: { en: "Resume", ko: "이어쓰기" },
    more_options: { en: "More options", ko: "더 보기" },
    menu_resume: { en: "Resume editing", ko: "이어서 편집" },
    menu_duplicate: { en: "Duplicate", ko: "복제" },
    menu_export: { en: "Export as Markdown", ko: "마크다운 내보내기" },
    menu_delete: { en: "Delete draft", ko: "초안 삭제" },

    empty_title: { en: "No drafts yet", ko: "초안이 없습니다" },
    empty_desc: { en: "Start writing — every post is autosaved as a draft until you publish, so you can pick up exactly where you left off.", ko: "게시 전까지 모든 글이 초안으로 자동 저장됩니다." },
    empty_cta: { en: "Start a draft", ko: "초안 작성" },
    empty_filtered_title: { en: "No matching drafts", ko: "조건에 맞는 초안이 없습니다" },
    empty_filtered_desc: { en: "Try a different filter to see other drafts.", ko: "다른 필터를 적용해 보세요." },
}
