use crate::common::*;

translate! {
    PostDetailSyndicatedTranslate;

    topbar_eyebrow: { en: "Post", ko: "포스트" },
    topbar_main: { en: "Detail", ko: "상세" },
    btn_edit: { en: "Edit", ko: "편집" },
    btn_share: { en: "Share", ko: "공유" },
    btn_back_aria: { en: "Back", ko: "뒤로" },

    action_likes_suffix: { en: "likes", ko: "좋아요" },
    action_comments_suffix: { en: "comments", ko: "댓글" },

    share_link_copied: { en: "Link copied to clipboard", ko: "링크가 복사되었습니다" },
    share_link_copy_failed: { en: "Failed to copy link", ko: "링크 복사에 실패했습니다" },

    drawer_comments_title: { en: "Comments", ko: "댓글" },
    drawer_close: { en: "Close", ko: "닫기" },
    comment_placeholder: { en: "Add a comment…", ko: "댓글을 남기세요…" },
    post_btn: { en: "Post", ko: "등록" },
    reply_label: { en: "Reply", ko: "답글" },
    reply_placeholder: { en: "Reply…", ko: "답글 남기기…" },
    replies_label: { en: "replies", ko: "답글" },

    syn_title: { en: "Syndication", ko: "크로스 포스팅" },
    syn_summary_prefix: { en: "succeeded", ko: "성공" },

    stat_external_reads: { en: "External reads", ko: "외부 열람수" },
    stat_reactions: { en: "Reactions", ko: "반응" },
    stat_backlink_clicks: { en: "Backlink clicks", ko: "백링크 클릭" },

    status_coming_soon: { en: "Coming soon", ko: "준비 중" },
    card_coming_soon_hint: {
        en: "Integration will launch soon",
        ko: "연동 준비 중입니다",
    },

    // AC-17/18 — signed-out backlink-landing additions.
    refer_close_aria: { en: "Close", ko: "닫기" },
    refer_text_bluesky: {
        en: "You arrived from Bluesky — see the full thinking and subscribe to the author on Ratel.",
        ko: "Bluesky 에서 오셨군요. Ratel 에서 글 전체를 읽고 작성자를 구독해 보세요.",
    },
    refer_text_linkedin: {
        en: "You arrived from LinkedIn — see the full thinking and subscribe to the author on Ratel.",
        ko: "LinkedIn 에서 오셨군요. Ratel 에서 글 전체를 읽고 작성자를 구독해 보세요.",
    },
    refer_text_threads: {
        en: "You arrived from Threads — see the full thinking and subscribe to the author on Ratel.",
        ko: "Threads 에서 오셨군요. Ratel 에서 글 전체를 읽고 작성자를 구독해 보세요.",
    },
    refer_text_generic: {
        en: "You're reading this on Ratel — every post is a queryable surface, not just a feed item.",
        ko: "이 글은 Ratel 에서 읽고 계십니다. 모든 글은 단순 피드가 아닌 쿼리 가능한 자산입니다.",
    },

    brand_signin: { en: "Sign in", ko: "로그인" },
    brand_get_started: { en: "Get started", ko: "시작하기" },

    subscribe_cta_eyebrow: { en: "Want more from this author?", ko: "이 작성자의 글이 더 궁금하신가요?" },
    subscribe_cta_title: {
        en: "Create a free Ratel account to follow them.",
        ko: "Ratel 계정을 무료로 만들어 팔로우해 보세요.",
    },
    subscribe_cta_sub: {
        en: "Reading is always free. Connect your social accounts to bring your own audience along.",
        ko: "읽기는 항상 무료입니다. 소셜 계정을 연결해 자신의 독자도 함께 데려와 보세요.",
    },
    subscribe_cta_primary: { en: "Create free account", ko: "무료 계정 만들기" },
    subscribe_cta_secondary: { en: "I have an account", ko: "이미 계정이 있어요" },

    // Sidebar — Essence House card.
    house_card_eyebrow: { en: "Essence House", ko: "에센스 하우스" },
    house_card_hero_title_suffix: { en: "'s House", ko: "님의 하우스" },
    house_card_hero_sub: { en: "Author on Ratel", ko: "Ratel 작성자" },
    house_card_pitch: {
        en: "Subscribe to follow this author and get every new Ratel post — queryable from ChatGPT, Claude Desktop, or any MCP client.",
        ko: "이 작성자를 구독하면 Ratel 새 글을 모두 받아볼 수 있고, ChatGPT / Claude Desktop / MCP 클라이언트에서도 쿼리 가능합니다.",
    },
    house_card_cta: { en: "Subscribe to House", ko: "하우스 구독" },
    house_card_note: {
        en: "One unified MCP endpoint across every House you subscribe to. Cancel anytime.",
        ko: "구독한 모든 하우스에 단일 MCP 엔드포인트로 접근. 언제든 취소 가능.",
    },

    // Sidebar — MCP info card.
    mcp_card_title: { en: "MCP-ready", ko: "MCP 지원" },
    mcp_card_body_lead: {
        en: "Register",
        ko: "ChatGPT 또는 Claude Desktop 에",
    },
    mcp_card_body_endpoint: { en: "mcp.ratel.foundation", ko: "mcp.ratel.foundation" },
    mcp_card_body_tail: {
        en: "in ChatGPT or Claude Desktop. One endpoint, every subscribed House — query across minds like a single corpus.",
        ko: "을 등록하세요. 구독한 모든 하우스를 단일 코퍼스처럼 한 엔드포인트에서 쿼리할 수 있습니다.",
    },
}
