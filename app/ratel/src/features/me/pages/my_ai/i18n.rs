use crate::*;

translate! {
    MyAiTranslate;

    // Topbar
    page_title: { en: "My AI", ko: "마이 AI" },
    back: { en: "Back", ko: "뒤로" },
    status_online: { en: "MCP Online", ko: "MCP 연결됨" },
    status_offline: { en: "Not Configured", ko: "미설정" },
    mcp_docs: { en: "MCP Docs", ko: "MCP 문서" },

    // Hero
    hero_eyebrow_label: { en: "Personal MCP Endpoint", ko: "개인 MCP 엔드포인트" },
    hero_eyebrow_transport: { en: "Streamable HTTP", ko: "스트리머블 HTTP" },
    hero_title: { en: "Plug Ratel into your AI agent", ko: "Ratel을 AI 에이전트에 연결하세요" },
    hero_sub: {
        en: "This is your private endpoint. Add it to Claude Code, Claude Desktop, Cursor, or any MCP-compatible client to let your agent post on Ratel, vote in polls, browse spaces and discussions, and act on your behalf — securely scoped to your account.",
        ko: "이것은 당신의 개인 엔드포인트입니다. Claude Code, Claude Desktop, Cursor 또는 MCP 호환 클라이언트에 추가하여 에이전트가 Ratel에 글을 게시하고, 투표하고, 스페이스와 토론을 탐색하며, 당신의 계정에 안전하게 범위가 지정된 작업을 수행하도록 할 수 있습니다.",
    },
    endpoint_label: { en: "Endpoint", ko: "엔드포인트" },
    placeholder_existing: {
        en: "Token already generated — regenerate to view a fresh URL.",
        ko: "토큰이 이미 생성되었습니다. 새 URL을 보려면 다시 생성하세요.",
    },
    placeholder_none: {
        en: "No token yet — click \"Generate\" to create your endpoint.",
        ko: "아직 토큰이 없습니다. \"생성\"을 클릭하여 엔드포인트를 만드세요.",
    },
    note_heads_up: { en: "Heads up.", ko: "주의." },
    note_body: {
        en: "The full URL is shown only once — right after you generate or regenerate it. Copy it now and paste into your agent's config. If you lose it, you'll need to regenerate (the old token will stop working immediately).",
        ko: "전체 URL은 생성 또는 재생성 직후 한 번만 표시됩니다. 지금 복사하여 에이전트 설정에 붙여넣으세요. 분실 시 다시 생성해야 하며 기존 토큰은 즉시 작동하지 않습니다.",
    },
    btn_copy: { en: "Copy URL", ko: "URL 복사" },
    btn_copied: { en: "Copied", ko: "복사됨" },
    btn_generate: { en: "Generate", ko: "생성" },
    btn_regenerate: { en: "Regenerate", ko: "재생성" },
    btn_generating: { en: "Generating…", ko: "생성 중…" },
    btn_copy_url: { en: "Copy", ko: "복사" },
    actions_hint: {
        en: "Keep this URL secret. Treat it like an API key.",
        ko: "이 URL을 비밀로 유지하세요. API 키처럼 다루세요.",
    },

    // Capabilities
    caps_section_title: { en: "What your agent can do", ko: "에이전트가 할 수 있는 일" },
    caps_section_sub: {
        en: "All actions are scoped to your account · auditable",
        ko: "모든 작업은 당신의 계정으로 제한 · 감사 가능",
    },
    cap_posts_label: { en: "Posts & Drafts", ko: "게시물 및 초안" },
    cap_posts_value: { en: "Create, edit, like, and delete posts", ko: "게시물 작성, 수정, 좋아요, 삭제" },
    cap_polls_label: { en: "Polls & Quizzes", ko: "투표 및 퀴즈" },
    cap_polls_value: { en: "Run, respond, and analyze results", ko: "실행, 응답, 결과 분석" },
    cap_spaces_label: { en: "Spaces & Teams", ko: "스페이스 및 팀" },
    cap_spaces_value: { en: "Create spaces, manage teams, follow users", ko: "스페이스 생성, 팀 관리, 사용자 팔로우" },
    cap_discussions_label: { en: "Discussions", ko: "토론" },
    cap_discussions_value: { en: "Read, comment, and start discussions", ko: "읽기, 댓글 작성, 토론 시작" },
    cap_inbox_label: { en: "Notifications", ko: "알림" },
    cap_inbox_value: { en: "Read inbox, mark items as read", ko: "받은편지함 읽기, 항목 읽음 처리" },

    // Setup guide
    guide_section_title: { en: "Setup guide", ko: "설정 가이드" },
    guide_section_sub: { en: "Pick your client · 2-minute setup", ko: "클라이언트 선택 · 2분 설정" },
    tab_claude_code: { en: "Claude Code", ko: "Claude Code" },
    tab_claude_desktop: { en: "Claude Desktop", ko: "Claude Desktop" },
    tab_cursor: { en: "Cursor", ko: "Cursor" },
    tab_generic: { en: "Generic JSON", ko: "일반 JSON" },

    // Claude Code panel
    cc_lede_prefix: { en: "Claude Code", ko: "Claude Code" },
    cc_lede_body: {
        en: "is Anthropic's CLI agent. Add Ratel as a streamable HTTP MCP server in one command, then call /mcp inside Claude Code to confirm.",
        ko: "는 Anthropic의 CLI 에이전트입니다. Ratel을 streamable HTTP MCP 서버로 한 번의 명령으로 추가한 후, Claude Code 안에서 /mcp를 호출하여 확인하세요.",
    },
    cc_step1_title: { en: "Add the server with claude mcp add", ko: "claude mcp add로 서버 추가" },
    cc_step1_hint: {
        en: "Run this in your terminal. Replace the URL with the one you copied above.",
        ko: "터미널에서 실행하세요. URL을 위에서 복사한 URL로 교체하세요.",
    },
    cc_step2_title: { en: "Verify the connection", ko: "연결 확인" },
    cc_step2_hint: {
        en: "Inside an active Claude Code session, run /mcp. You should see a 'ratel' server with status connected and the available tools listed.",
        ko: "활성 Claude Code 세션에서 /mcp를 실행하세요. 'ratel' 서버가 연결됨 상태로 표시되고 사용 가능한 도구가 나열되어야 합니다.",
    },
    cc_step3_title: { en: "Try it out", ko: "사용해보기" },
    cc_step3_hint: {
        en: "Ask Claude Code something like: \"List my latest 5 Ratel posts and mark all unread notifications as read.\"",
        ko: "Claude Code에 다음과 같이 요청하세요: \"내 최근 Ratel 게시물 5개를 나열하고 모든 읽지 않은 알림을 읽음으로 표시하세요.\"",
    },
    cc_verify_title: { en: "Local-only token", ko: "로컬 전용 토큰" },
    cc_verify_hint: {
        en: "Claude Code stores the URL in ~/.claude.json on your machine. Don't commit this file to git.",
        ko: "Claude Code는 URL을 ~/.claude.json에 저장합니다. 이 파일을 git에 커밋하지 마세요.",
    },

    // Claude Desktop panel
    cd_lede: {
        en: "Claude Desktop reads its MCP server list from claude_desktop_config.json. Add the snippet below, save, and fully quit + relaunch Claude.",
        ko: "Claude Desktop은 claude_desktop_config.json에서 MCP 서버 목록을 읽습니다. 아래 스니펫을 추가하고 저장한 후, Claude를 완전히 종료한 다음 다시 실행하세요.",
    },
    cd_step1_title: { en: "Open the config file", ko: "설정 파일 열기" },
    cd_step1_macos: { en: "macOS", ko: "macOS" },
    cd_step1_windows: { en: "Windows", ko: "Windows" },
    cd_step2_title: { en: "Add the ratel server", ko: "ratel 서버 추가" },
    cd_step2_hint: {
        en: "Merge into \"mcpServers\" (create the key if missing).",
        ko: "\"mcpServers\"에 병합하세요 (없으면 키를 생성하세요).",
    },
    cd_step3_title: { en: "Restart Claude Desktop", ko: "Claude Desktop 재시작" },
    cd_step3_hint: {
        en: "Fully quit (not just close) then reopen. Look for the plug icon at the bottom of the chat input — 'ratel' should appear in the connected servers list.",
        ko: "완전히 종료한 후 (단순히 닫는 것이 아님) 다시 여세요. 채팅 입력 하단의 플러그 아이콘을 확인하세요 — 'ratel'이 연결된 서버 목록에 표시되어야 합니다.",
    },

    // Cursor panel
    cur_lede: {
        en: "Cursor supports MCP servers via Settings → Cursor Settings → MCP. You can either add the server through the UI or paste the JSON directly.",
        ko: "Cursor는 Settings → Cursor Settings → MCP를 통해 MCP 서버를 지원합니다. UI를 통해 서버를 추가하거나 JSON을 직접 붙여넣을 수 있습니다.",
    },
    cur_step1_title: { en: "Open Cursor Settings · MCP", ko: "Cursor Settings · MCP 열기" },
    cur_step1_hint: {
        en: "Press Cmd+, (macOS) or Ctrl+, (Windows/Linux), then search \"MCP\".",
        ko: "Cmd+, (macOS) 또는 Ctrl+, (Windows/Linux)를 누른 후 \"MCP\"를 검색하세요.",
    },
    cur_step2_title: { en: "Add a new server → HTTP / Streamable", ko: "새 서버 추가 → HTTP / Streamable" },
    cur_step2_hint: {
        en: "Name: ratel · URL: paste the endpoint you copied above.",
        ko: "이름: ratel · URL: 위에서 복사한 엔드포인트를 붙여넣으세요.",
    },
    cur_step3_title: { en: "Toggle the server on", ko: "서버 활성화" },
    cur_step3_hint: {
        en: "Cursor will show \"Loading tools…\" then list the Ratel tools (create_post, list_inbox, vote_poll, …). Open the chat sidebar and the agent can now use them.",
        ko: "Cursor에 \"Loading tools…\"가 표시된 후 Ratel 도구(create_post, list_inbox, vote_poll, …)가 나열됩니다. 채팅 사이드바를 열면 에이전트가 이제 도구를 사용할 수 있습니다.",
    },

    // Generic panel
    gen_lede: {
        en: "Any client that speaks the Model Context Protocol over Streamable HTTP can connect. The endpoint accepts JSON-RPC 2.0 requests and streams responses via Server-Sent Events.",
        ko: "Streamable HTTP를 통해 Model Context Protocol을 사용하는 모든 클라이언트가 연결할 수 있습니다. 엔드포인트는 JSON-RPC 2.0 요청을 수락하고 Server-Sent Events를 통해 응답을 스트리밍합니다.",
    },
    gen_step1_title: { en: "Smoke-test with curl", ko: "curl로 스모크 테스트" },
    gen_step1_hint: {
        en: "Confirm your endpoint is reachable and lists tools.",
        ko: "엔드포인트가 도달 가능하고 도구를 나열하는지 확인하세요.",
    },
    gen_step2_title: { en: "Use any MCP SDK", ko: "MCP SDK 사용" },
    gen_step2_hint: {
        en: "Python, TypeScript, Rust SDKs all work. Point the client's StreamableHTTPServerTransport at your endpoint URL.",
        ko: "Python, TypeScript, Rust SDK 모두 작동합니다. 클라이언트의 StreamableHTTPServerTransport를 엔드포인트 URL로 지정하세요.",
    },

    // Errors
    error_clipboard: {
        en: "Failed to copy to clipboard. Please copy it manually.",
        ko: "클립보드 복사에 실패했습니다. 수동으로 복사하세요.",
    },
}
