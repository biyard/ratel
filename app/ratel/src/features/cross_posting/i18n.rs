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
