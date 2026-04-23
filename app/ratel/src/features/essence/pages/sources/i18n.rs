use crate::common::*;

translate! {
    EssenceSourcesTranslate;

    seo_title: { en: "Ratel — Essence Sources", ko: "Ratel — Essence 소스" },

    // Topbar (Add Source button removed for now)
    back_label: { en: "Back", ko: "뒤로" },
    topbar_eyebrow: { en: "Essence", ko: "Essence" },
    topbar_main: { en: "Sources", ko: "소스" },

    // Hero (Open My House button removed for now)
    hero_eyebrow: { en: "Your Essence", ko: "내 Essence" },
    hero_subtitle: {
        en: "Everything you've authored across Ratel — posts, comments, polls, quizzes — indexed into your Essence House.",
        ko: "내가 Ratel에 올린 모든 포스트 · 댓글 · 투표 · 퀴즈가 Essence House에 모입니다.",
    },
    hero_sources_word: { en: "sources", ko: "개 소스" },
    hero_words_word: { en: "words", ko: "단어" },

    // Breakdown cards
    kind_all: { en: "All", ko: "전체" },
    kind_notion: { en: "Notion", ko: "Notion" },
    kind_post: { en: "Posts", ko: "포스트" },
    kind_comment: { en: "Comments", ko: "댓글" },
    kind_poll: { en: "Polls", ko: "투표" },
    kind_quiz: { en: "Quizzes", ko: "퀴즈" },

    // Controls (status chips removed along with In House)
    search_placeholder: {
        en: "Search sources by title, content, or tag…",
        ko: "제목, 내용, 태그로 소스 검색…",
    },
    sort_last_edited: { en: "Last edited ↓", ko: "최근 편집 ↓" },
    sort_word_count: { en: "Word count ↓", ko: "단어 수 ↓" },
    sort_title: { en: "Title A–Z", ko: "제목순" },

    // Bulk bar
    bulk_selected_suffix: { en: "selected", ko: "개 선택됨" },
    bulk_remove: { en: "Remove", ko: "제거" },

    // Table
    col_title: { en: "Title", ko: "제목" },
    col_words: { en: "Words", ko: "단어" },
    col_last_sync: { en: "Last sync", ko: "최근 동기화" },
    row_select_label: { en: "Select", ko: "선택" },
    row_more_label: { en: "More", ko: "더보기" },

    // Row menu (`...` popover)
    menu_delete: { en: "Delete", ko: "삭제" },

    // Row kind badges — singular forms for per-row meta. Kept separate
    // from the `kind_*` strings that the filter chips use (plural:
    // "Posts", "Comments") so the row badge reads naturally.
    tag_notion: { en: "Notion", ko: "Notion" },
    tag_post: { en: "Post", ko: "포스트" },
    tag_poll: { en: "Poll", ko: "투표" },
    tag_quiz: { en: "Quiz", ko: "퀴즈" },
    tag_post_comment: { en: "Post comment", ko: "포스트 댓글" },
    tag_discussion_comment: { en: "Discussion comment", ko: "토론 댓글" },

    // Pagination
    pagination_prefix: { en: "Showing", ko: "" },
    pagination_of: { en: "of", ko: "/ 총" },
    pagination_previous: { en: "Previous", ko: "이전" },
    pagination_next: { en: "Next", ko: "다음" },

    // Empty state
    empty_title: { en: "No sources yet", ko: "아직 소스가 없습니다" },
    empty_subtitle: {
        en: "Posts, comments, polls, and quizzes you create will show up here.",
        ko: "포스트 · 댓글 · 투표 · 퀴즈를 만들면 여기에 표시됩니다.",
    },
}
