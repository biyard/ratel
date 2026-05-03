use dioxus_translate::*;

translate! {
    ConnectionsPageTranslate;

    // Topbar
    title: { en: "Connections", ko: "연결" },
    eyebrow: { en: "Settings · Cross-posting", ko: "설정 · 크로스포스팅" },

    // Hero
    hero_eyebrow: { en: "Cross-posting", ko: "크로스포스팅" },
    hero_title: { en: "Connect once.", ko: "한 번 연결하면" },
    hero_title_accent: { en: "Reach every network.", ko: "모든 네트워크에 도달합니다." },
    hero_sub: {
        en: "Each Ratel post fans out to your connected social accounts with a backlink — readers find you everywhere, but your home stays here.",
        ko: "Ratel 에 한 번 발행하면 연결된 소셜 계정으로 자동 백링크 게시. 어디서든 발견되지만 본진은 Ratel.",
    },
    stat_connected: { en: "Connected", ko: "연결됨" },
    stat_this_month: { en: "This month", ko: "이번 달" },

    // Section heading
    section_platforms: { en: "Networks", ko: "네트워크" },
    section_meta_phase1: { en: "Phase 1 · Bluesky / LinkedIn / Threads", ko: "Phase 1 · Bluesky / LinkedIn / Threads" },

    // Status pills
    status_connected: { en: "Connected", ko: "연결됨" },
    status_not_connected: { en: "Not connected", ko: "연결 안 됨" },
    status_coming_soon: { en: "Coming soon", ko: "곧 출시" },
    status_phase2: { en: "Phase 2", ko: "Phase 2" },

    // Bluesky card
    bluesky_name: { en: "Bluesky", ko: "Bluesky" },
    bluesky_limit: { en: "300 chars · 4 images", ko: "300자 · 이미지 4장" },
    bluesky_subtitle_default: { en: "App password flow · revocable anytime", ko: "앱 비밀번호 방식 · 언제든 폐기 가능" },

    // LinkedIn card (1B)
    linkedin_name: { en: "LinkedIn", ko: "LinkedIn" },
    linkedin_limit: { en: "3,000 chars · OAuth 2.0", ko: "3,000자 · OAuth 2.0" },
    linkedin_subtitle: { en: "OAuth integration arrives in Phase 1B", ko: "OAuth 연동은 Phase 1B 에서 활성화됩니다" },

    // Threads card (1C)
    threads_name: { en: "Threads", ko: "Threads" },
    threads_limit: { en: "500 chars · Requires IG account", ko: "500자 · 인스타그램 프로페셔널 필요" },
    threads_subtitle: { en: "Meta OAuth integration arrives in Phase 1C", ko: "Meta OAuth 연동은 Phase 1C 에서 활성화됩니다" },

    // Farcaster card (Phase 2)
    farcaster_name: { en: "Farcaster", ko: "Farcaster" },
    farcaster_limit: { en: "320 chars · Web3-native", ko: "320자 · Web3 네이티브" },
    farcaster_subtitle: {
        en: "Frames integration + Agent posting in Phase 2",
        ko: "Phase 2 에서 Frames + Agent 게시 지원",
    },

    // Sub-row (connected card)
    posts_syndicated_count_label: { en: " posts syndicated", ko: " 건 게시됨" },
    auto_post: { en: "Auto-post new posts", ko: "새 글 자동 게시" },

    // Buttons
    btn_connect: { en: "Connect", ko: "연결하기" },
    btn_disconnect: { en: "Disconnect", ko: "연결 해제" },
    btn_notify: { en: "Notify me", ko: "출시 알림" },
    btn_back_aria: { en: "Back", ko: "뒤로" },
}

translate! {
    BlueskyConnectModalTranslate;

    title: { en: "Connect Bluesky", ko: "Bluesky 연결" },
    subtitle: { en: "Enter your handle + app password", ko: "핸들 + 앱 비밀번호 입력" },
    close: { en: "Close", ko: "닫기" },

    info: {
        en: "Bluesky uses app passwords, not OAuth. Generate one at bsky.app/settings/app-passwords — we store it encrypted and never see your main password.",
        ko: "Bluesky 는 OAuth 대신 앱 비밀번호 방식을 사용합니다. bsky.app/settings/app-passwords 에서 발급받아 입력하세요. 암호화 저장하며 메인 비밀번호는 절대 알 수 없습니다.",
    },

    label_handle: { en: "Handle", ko: "핸들" },
    placeholder_handle: { en: "you.bsky.social", ko: "you.bsky.social" },
    hint_handle: { en: "Your Bluesky handle — don't include @", ko: "Bluesky 핸들 — @ 빼고 입력" },

    label_app_password: { en: "App password", ko: "앱 비밀번호" },
    placeholder_app_password: { en: "xxxx-xxxx-xxxx-xxxx", ko: "xxxx-xxxx-xxxx-xxxx" },
    hint_app_password: {
        en: "Format: xxxx-xxxx-xxxx-xxxx. Revokable anytime from Bluesky settings.",
        ko: "형식: xxxx-xxxx-xxxx-xxxx. Bluesky 설정에서 언제든 폐기 가능.",
    },

    foot_hint: { en: "Encrypted at rest · revocable anytime", ko: "저장 시 암호화 · 언제든 폐기" },
    btn_cancel: { en: "Cancel", ko: "취소" },
    btn_connect: { en: "Connect", ko: "연결하기" },
}

translate! {
    ComposeSidebarTranslate;

    eyebrow: { en: "Syndication", ko: "확장 게시" },
    title: { en: "Cross-post", ko: "크로스포스트" },
    sub: {
        en: "Publish once. Reach every network your audience lives on.",
        ko: "한 번 발행하면 청중이 머무는 모든 네트워크에 닿습니다.",
    },

    reaching: { en: "Reaching", ko: "도달 범위" },
    networks_suffix: { en: "networks", ko: "개 네트워크" },

    not_connected: { en: "Not connected", ko: "연결 안 됨" },
    truncated_badge: { en: "Truncated", ko: "잘림" },

    connect_hint_bluesky: {
        en: "Connect Bluesky to cross-post to the AT Protocol network.",
        ko: "Bluesky 를 연결해 AT Protocol 네트워크에 크로스포스트하세요.",
    },
    connect_hint_linkedin: {
        en: "Connect LinkedIn to reach your professional network.",
        ko: "LinkedIn 을 연결해 비즈니스 네트워크에 도달하세요.",
    },
    connect_hint_threads: {
        en: "Connect Threads to reach Meta's audience with every post.",
        ko: "Threads 를 연결해 Meta 의 사용자에게 모든 게시물을 전달하세요.",
    },

    connect_btn_bluesky: { en: "Connect Bluesky", ko: "Bluesky 연결" },
    connect_btn_linkedin: { en: "Connect LinkedIn", ko: "LinkedIn 연결" },
    connect_btn_threads: { en: "Connect Threads", ko: "Threads 연결" },
}

translate! {
    SyndicationPanelTranslate;

    title: { en: "Syndication", ko: "확장 게시" },
    summary_succeeded: { en: "succeeded", ko: "성공" },
    summary_failed_suffix: { en: "failed", ko: "실패" },

    stat_likes: { en: "Likes", ko: "좋아요" },
    stat_comments: { en: "Comments", ko: "댓글" },
    stat_reposts: { en: "Reposts", ko: "재게시" },

    status_published: { en: "Published", ko: "게시됨" },
    status_pending: { en: "Pending", ko: "대기 중" },
    status_failed: { en: "Failed", ko: "실패" },
    status_skipped: { en: "Skipped", ko: "건너뜀" },

    queued_hint: { en: "Queued — awaiting dispatch", ko: "대기열 — 곧 발송됩니다" },
    attempts_label: { en: "Attempt", ko: "시도" },

    btn_view: { en: "View", ko: "열기" },
    btn_retry: { en: "Retry now", ko: "재시도" },

    engage_likes: { en: "likes", ko: "좋아요" },
    engage_comments: { en: "comments", ko: "댓글" },
    engage_reposts: { en: "reposts", ko: "재게시" },

    // Always-show panel additions: each platform card renders even when no
    // job exists yet so the author sees a coherent matrix instead of a
    // mounted/hidden race.
    btn_refresh_aria: { en: "Refresh", ko: "새로고침" },
    awaiting_dispatch: { en: "Awaiting dispatch", ko: "발송 대기" },
    not_connected: { en: "Not connected", ko: "연결 안 됨" },
    btn_connect_bluesky: { en: "Connect Bluesky", ko: "Bluesky 연결하기" },
    panel_coming_soon: { en: "Coming soon", ko: "곧 출시" },
    panel_linkedin_coming_soon_hint: {
        en: "LinkedIn cross-posting arrives in Phase 1B.",
        ko: "LinkedIn 크로스포스팅은 Phase 1B 에서 활성화됩니다.",
    },
    panel_threads_coming_soon_hint: {
        en: "Threads cross-posting arrives in Phase 1C.",
        ko: "Threads 크로스포스팅은 Phase 1C 에서 활성화됩니다.",
    },
}

translate! {
    OnboardingPageTranslate;

    // SeoMeta
    seo_title: { en: "Connect your networks · Ratel", ko: "네트워크 연결 · Ratel" },

    // Topbar
    topbar_skip: { en: "Skip for now →", ko: "나중에 →" },

    // Hero
    eyebrow: { en: "Connect your networks · Optional", ko: "네트워크 연결 · 선택" },
    title_lead: { en: "Your first post reaches", ko: "첫 게시글 한 번에" },
    title_accent: { en: "three networks instantly.", ko: "세 네트워크에 도달합니다." },
    sub: {
        en: "Connect the social accounts you want Ratel to cross-post to. Every post becomes a link back to your home, bringing new subscribers in.",
        ko: "Ratel 이 자동 크로스포스팅할 소셜 계정을 연결하세요. 모든 글이 본진으로 돌아오는 백링크가 됩니다.",
    },

    // Platform rows — reuse status / btn copy from ConnectionsPageTranslate.
    bluesky_meta_default: { en: "AT Protocol · 300 chars", ko: "AT Protocol · 300자" },
    bluesky_meta_connected_suffix: {
        en: " — AT Protocol · 300 chars",
        ko: " — AT Protocol · 300자",
    },
    linkedin_meta: { en: "Your professional network · 3,000 chars", ko: "전문 네트워크 · 3,000자" },
    threads_meta: { en: "Meta · ~275M users · 500 chars", ko: "Meta · 약 2.75억 명 · 500자" },
    coming_soon: { en: "Coming soon", ko: "곧 출시" },
    status_connected: { en: "Connected", ko: "연결됨" },
    btn_connect: { en: "Connect", ko: "연결" },

    // Benefits
    benefit_auto_label: { en: "Auto-sync", ko: "자동 동기화" },
    benefit_auto_hint: {
        en: "Every Ratel post publishes to your connected networks in 2–3 seconds.",
        ko: "Ratel 글이 연결된 네트워크에 2~3초 안에 자동 발행됩니다.",
    },
    benefit_backlinks_label: { en: "Backlinks", ko: "백링크" },
    benefit_backlinks_hint: {
        en: "Each cross-post links to Ratel, turning external readers into subscribers.",
        ko: "각 크로스포스트가 Ratel 백링크가 되어 외부 독자를 구독자로 전환합니다.",
    },
    benefit_secure_label: { en: "Secure", ko: "보안" },
    benefit_secure_hint: {
        en: "KMS-encrypted tokens · revocable from the platform or Ratel anytime.",
        ko: "토큰은 KMS 암호화 · 플랫폼이나 Ratel 에서 언제든 폐기 가능.",
    },

    // CTA
    cta_skip: { en: "Skip", ko: "건너뛰기" },
    cta_continue: { en: "Continue", ko: "계속" },

    // Footer note
    footer_note_pro_tip: { en: "Pro tip:", ko: "팁:" },
    footer_note_body: {
        en: "Creators who connect 2+ networks get 3.4× more subscribers in their first 30 days.",
        ko: "2개 이상의 네트워크를 연결한 크리에이터는 첫 30일에 3.4배 더 많은 구독자를 얻습니다.",
    },
}
