---
sidebar_position: 3
title: AI 연결
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# AI 연결 (MCP)

Ratel은 사용자별로 통합 MCP 엔드포인트 하나를 제공합니다. 평소 쓰는 AI 클라이언트에 연결하면, 어시스턴트가 당신의 Essence — 그리고 당신이 구독한 다른 Essence House들 — 을 일급 도구로 활용할 수 있습니다.

## MCP란?

**MCP (Model Context Protocol)** 는 Anthropic이 처음 제안한 오픈 표준으로, AI 어시스턴트가 외부 도구·데이터 소스와 균일한 프로토콜로 대화하기 위한 규격입니다. 모델마다 별도 통합을 손으로 짜는 대신, MCP 호환 클라이언트 — Claude Desktop, Cursor, Windsurf, Zed 등 — 는 어떤 MCP 서버에든 접속해서 그 서버가 제공하는 도구를 자동으로 발견할 수 있습니다.

Ratel이 MCP를 채택한 이유는 단순합니다. MCP를 통해 당신의 Essence가 단순히 "읽을 수 있는 데이터"가 아니라 어시스턴트가 *직접 행동을 취할 수 있는 도구* 가 되기 때문입니다. 한 번 연결하면, 매일 쓰는 그 AI가 당신의 House를 조회하고, 글을 작성·발행하고, 스페이스를 관리하고, 폴·퀴즈를 운영할 수 있게 됩니다. Phase 2에서는 구독 중인 모든 Essence House에까지 같은 엔드포인트로 접근할 수 있게 됩니다.

## 통합 MCP URL 받기

MCP URL은 계정 설정 안에 있습니다.

1. Ratel에 로그인 후 <img src={useBaseUrl('/img/icons/settings.svg')} width="16" height="16" alt="Settings" style={{verticalAlign: 'middle'}} /> **사용자 설정(User Settings)** 으로 이동합니다.
2. <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="2" y="2" width="20" height="8" rx="2"/><rect x="2" y="14" width="20" height="8" rx="2"/><line x1="6" y1="6" x2="6.01" y2="6"/><line x1="6" y1="18" x2="6.01" y2="18"/></svg> **MCP Server** 섹션까지 스크롤합니다.
3. <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="7.5" cy="15.5" r="5.5"/><path d="m21 2-9.6 9.6"/><path d="m15.5 7.5 3 3L22 7l-3-3"/></svg> **Generate Secret(시크릿 생성)** 버튼을 누르면 URL이 한 번만 표시됩니다 — 그 자리에서 즉시 복사하세요.
4. 새 URL이 필요할 때는 <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M3 21v-5h5"/></svg> **Regenerate(재생성)** 를 누릅니다. 새 토큰을 발급하는 즉시 이전 토큰은 무효가 됩니다.

URL은 다음과 같은 형식입니다:

```
https://ratel.foundation/mcp/<your-secret-token>
```

토큰은 *당신* 한 사람에게 고유합니다. **사용자 한 명 = MCP URL 한 개** 이며, 구독 중인 House 수와 무관합니다. Phase 2의 마켓플레이스 구독이 출시되면, 같은 URL이 자동으로 모든 구독 House에 접근할 수 있게 — 여러 엔드포인트를 따로 관리할 필요가 없게 됩니다.

:::caution
토큰은 생성된 **그 순간 한 번만** 표시됩니다. 잃어버리면 다시 보여드릴 수 없으니 **재생성** 만이 유일한 복구 수단입니다.
:::

## Claude Desktop 설정

Claude Desktop은 JSON 설정 파일에서 MCP 서버 목록을 읽어옵니다.

| OS      | 경로                                                                    |
| ------- | ----------------------------------------------------------------------- |
| macOS   | `~/Library/Application Support/Claude/claude_desktop_config.json`       |
| Windows | `%APPDATA%\Claude\claude_desktop_config.json`                           |
| Linux   | `~/.config/Claude/claude_desktop_config.json`                           |

해당 파일을 열거나 새로 만든 다음, `mcpServers` 아래에 `ratel` 항목을 추가합니다:

```json
{
  "mcpServers": {
    "ratel": {
      "url": "https://ratel.foundation/mcp/YOUR_SECRET_TOKEN"
    }
  }
}
```

저장 후 **Claude Desktop을 완전히 종료하고 다시 실행** 하세요(앱이 재시작되어야 새 MCP 서버를 인식합니다). 다시 열면 메시지 입력창의 도구 아이콘에 Ratel 도구 목록(`get_me`, `list_posts`, `create_post` 등)이 보여야 합니다. 보이지 않는다면 JSON 문법이 올바른지, 그리고 URL을 브라우저에서 열었을 때 응답이 오는지(`GET`에 405가 떠도 정상 — MCP 엔드포인트가 살아있다는 의미) 확인하세요.

## ChatGPT 설정

ChatGPT는 데스크톱·웹 앱의 **Custom Connector** 기능을 통해 MCP 서버를 지원합니다. 정확한 UI는 OpenAI 측에서 자주 갱신되므로 가장 정확한 절차는 OpenAI의 공식 문서를 참고해 주세요. 큰 흐름은 동일합니다:

1. ChatGPT에서 **Settings → Connectors** (또는 **Apps & Connectors**) 를 엽니다.
2. **Add a custom connector** (또는 **Add MCP server**) 를 선택합니다.
3. Ratel MCP URL을 그대로 붙여넣습니다.
4. 인증한 뒤, 사용할 대화·프로젝트에서 커넥터를 활성화합니다.

연결 후 *"내 Ratel 도구로 ~에 대한 글 초안 작성해줘"* 같이 요청하면 ChatGPT가 직접 당신의 Essence에 접근해 작업합니다.

## Cursor 및 그 외 클라이언트 설정

**Cursor** 는 MCP를 기본 지원합니다. `Settings → MCP → Add new MCP server` 에서 URL을 붙여넣고 저장하면 끝입니다. Cursor 자체 도구와 함께 발견된 도구 목록이 표시됩니다.

**Windsurf, Zed, Claude Code 등 모든 MCP 호환 클라이언트** 도 동일한 형식입니다 — JSON 설정 블록(Claude Desktop과 같은) 이나 설정 UI의 URL 필드 둘 중 하나를 받습니다. 같은 URL을 넣기만 하면 됩니다.

```json
{
  "mcpServers": {
    "ratel": { "url": "https://ratel.foundation/mcp/YOUR_SECRET_TOKEN" }
  }
}
```

## 제공 도구

Ratel MCP 서버는 네 영역에 걸친 도구를 제공합니다: **신원**, **포스트·피드**, **스페이스·액션**, **인사이트**. 아래는 현재 시점에서 실제로 활성화된 도구 목록이며, 클라이언트가 연결 시 자동으로 발견합니다.

### <img src={useBaseUrl('/img/icons/user.svg')} width="20" height="20" alt="User" style={{verticalAlign: 'middle'}} /> 신원과 알림

- **`get_me`** — 현재 사용자 정보와 멤버십 등급.
- **`list_teams`** — 소속된 모든 팀과 역할·권한.
- **`list_inbox`** — 알림함을 최신순으로 조회; `unread_only` 와 페이지네이션 지원.
- **`get_unread_count`** — 읽지 않은 알림 수(최대 100).

### <img src={useBaseUrl('/img/icons/edit-square.svg')} width="20" height="20" alt="Edit square" style={{verticalAlign: 'middle'}} /> 포스트·피드

- **`create_post`** — 새 초안 작성(팀 단위 게시 가능).
- **`get_post`**, **`list_posts`** — 포스트 조회와 피드 페이지네이션.
- **`update_post`** — 본문 편집·공개 범위 변경·발행.
- **`delete_post`**, **`like_post`** — 삭제와 반응.

### <img src={useBaseUrl('/img/icons/grid.svg')} width="20" height="20" alt="Grid" style={{verticalAlign: 'middle'}} /> 스페이스

- **`create_space`**, **`get_space`**, **`update_space`**, **`delete_space`** — 포스트에 연결된 스페이스 관리.
- **`install_space_app`**, **`uninstall_space_app`** — 스페이스 내 앱(General, File, Analyzes — dev/staging 전용, Panels, Incentive Pool — 베타 빌드) 설치·제거.
- **`list_actions`** — 스페이스 내의 모든 폴·퀴즈·디스커션·팔로우 액션 조회.

### <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg> 스페이스 안의 액션

- 폴 — `create_poll`, `update_poll`, `delete_poll`, `get_poll`, `respond_poll`.
- 퀴즈 — `create_quiz`, `update_quiz`, `get_quiz`, `respond_quiz`.
- 디스커션 — `create_discussion`, `update_discussion`, `delete_discussion`, `get_discussion`, `add_comment`, `list_comments`.
- 팔로우 캠페인 — `create_follow`, `get_follow`, `follow_user`.
- 미트 — `create_meet`, `get_meet`, `update_meet`, `delete_meet`.
- AI 모더레이터 — `update_ai_moderator`(프리미엄 등급 한정).

### <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><line x1="18" y1="20" x2="18" y2="10"/><line x1="12" y1="20" x2="12" y2="4"/><line x1="6" y1="20" x2="6" y2="14"/></svg> 인사이트 (Analyze 앱)

- `list_analyze_reports`, `get_analyze_report`, `create_analyze_report`, `preview_analyze_report` — 폴·퀴즈·팔로우·디스커션을 묶어 만든 교차 필터 리포트.
- `list_analyze_records`, `get_matched_users` — 필터 칩 뒤의 사용자 단위로 드릴다운.
- `list_analyze_polls`, `list_analyze_quizzes`, `list_analyze_follows`, `list_analyze_discussions` — 리포트 소스로 사용할 수 있는 액션 목록.
- `analyze_discussion`, `list_analyze_discussion_results`, `update_discussion_topics` — 디스커션 댓글에 대한 LDA / TF-IDF / 텍스트 네트워크 분석 실행과 토픽 라벨링.

### <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M9.937 15.5A2 2 0 0 0 8.5 14.063l-6.135-1.582a.5.5 0 0 1 0-.962L8.5 9.936A2 2 0 0 0 9.937 8.5l1.582-6.135a.5.5 0 0 1 .963 0L14.063 8.5A2 2 0 0 0 15.5 9.937l6.135 1.582a.5.5 0 0 1 0 .962L15.5 14.063a2 2 0 0 0-1.437 1.437l-1.582 6.135a.5.5 0 0 1-.963 0z"/><path d="M20 3v4"/><path d="M22 5h-4"/></svg> 곧 출시 (Phase 2)

- **`list_houses`** — 구독 중인 모든 Essence House 조회.
- **`query_house`** — 특정 House에 대한 자연어 질의.
- **`search_essence`** — 내 Essence와 구독 House 전체에 걸친 의미 기반 검색을 한 번에 호출.

Phase 2가 출시되면 동일한 엔드포인트에 위 도구들이 자동으로 추가됩니다 — 설정을 다시 만질 필요는 없습니다.

## 토큰·권한·키 로테이션

MCP URL에는 사용자별 시크릿 토큰이 들어 있습니다. **API 키처럼 다뤄야** 합니다.

- **노출 금지.** URL을 가진 사람은 누구든 Ratel 안에서 당신처럼 행동할 수 있습니다 — 알림 열람, 대신 게시, 스페이스 관리까지. 공개 저장소·스크린샷·공유 채팅에 절대 붙여넣지 마세요.
- **로테이션은 사용자 설정 → MCP Server → Regenerate** 에서 즉시 가능합니다. 새 URL이 발급되는 순간, 이전 토큰을 사용하던 캐시된 MCP 세션은 즉시 끊깁니다.
- **유출이 의심되면 즉시 재생성** 한 뒤, 사용 중이던 모든 클라이언트의 설정을 갱신하세요.
- **도구별 권한 스코프**(읽기 전용 토큰, 특정 House 한정 토큰 등)는 Phase 2 로드맵에 포함되어 있습니다.

이게 전부입니다 — 원하는 클라이언트에 URL만 한 번 붙여넣으면, AI 어시스턴트가 다른 모든 도구처럼 자연스럽게 당신의 Essence와 대화하게 됩니다.
