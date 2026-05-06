---
sidebar_position: 8
title: 설정
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# 설정

계정에 관한 모든 것을 한곳에서 바꿀 수 있는 페이지입니다 — 어떻게 보일지, 어떻게 로그인할지, 어떤 플랜을 쓸지, AI 에이전트가 내 Ratel 데이터에 어떻게 접속할지까지.

## 설정 페이지 위치

```
/<your-handle>/settings
```

사이드바 하단의 사용자 드롭다운에서 *Settings* 를 클릭하거나, URL 을 직접 붙여넣어 엽니다. 페이지는 비공개입니다 — 본인이 로그인했을 때만 볼 수 있고, 다른 사람의 `/<their-handle>/settings` 는 열 수 없습니다.

페이지는 한 컬럼의 카드 묶음으로, 위에서 아래로 — **프로필 · 비밀번호 · 구독 및 결제 · MCP 서버** — 의 순서입니다. 크로스포스트 연결은 별도 서브 페이지 `/<your-handle>/settings/connections` 에 있습니다.

## <img src={useBaseUrl('/img/icons/user.svg')} width="20" height="20" alt="프로필" style={{verticalAlign: 'middle'}} /> 프로필

첫 번째 카드. `/<your-handle>` 에 보이는 모든 항목을 여기서 편집합니다.

- **아바타** — 원형 썸네일(또는 *Upload* 자리표시자) 을 누르고 이미지를 선택하세요. 새 아바타는 사이드바, 프로필, 작성한 모든 게시글에 즉시 반영됩니다.
- **사용자명(Username)** — 잠겨 있습니다. 가입 시 정한 핸들은 이후 변경이 어려우므로 읽기 전용으로 표시됩니다 — URL 이 어디에 묶여 있는지 한눈에 확인할 수 있도록 함께 보입니다.
- **이메일(Email)** — 잠겨 있습니다. 이메일은 계정 자체에 묶여 있어서 변경이 필요하면 Help 메뉴로 문의하세요.
- **디스플레이 네임(Display Name)** — 게시글과 프로필에 노출되는 이름. 최대 30 자.
- **소개(Description / Bio)** — 본인을 설명하는 더 긴 한 문단. 어떤 글을 쓰고, 어떤 스페이스를 호스트하며, Essence 구독자에게 무엇을 알려주고 싶은지를 적기 좋아요.

**저장(Save)** 을 누르면 적용됩니다 — 즉시 반영돼요. 플랫폼의 콘텐츠 필터에 걸리는 단어(공격적 표현 등) 가 입력되면 **저장** 버튼이 비활성화 상태로 남습니다 — 텍스트를 수정하면 다시 활성화됩니다.

## <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg> 비밀번호

두 번째 카드. 이메일·비밀번호 로그인에 사용하는 비밀번호를 설정하거나 변경합니다.

- **현재 비밀번호(Current Password)** — 지금 로그인할 때 사용하는 비밀번호.
- **새 비밀번호(New Password)** — 최소 8 자, 영문·숫자·기호 혼합 권장.
- **비밀번호 확인(Confirm Password)** — 오타 방지를 위해 새 비밀번호를 한 번 더 입력.

**비밀번호 업데이트(Update Password)** 를 누르면 적용됩니다. Google 또는 지갑으로 가입해서 비밀번호를 한 번도 설정하지 않았다면, 이 폼에서 처음으로 비밀번호를 추가하여 대체 로그인 수단을 만들 수 있어요.

현재 비밀번호를 잊었다면 로그인 화면의 **Forgot password?** 를 사용하세요 — 이메일로 재설정 링크가 발송됩니다.

## <img src={useBaseUrl('/img/icons/award.svg')} width="20" height="20" alt="구독" style={{verticalAlign: 'middle'}} /> 구독 및 결제

세 번째 카드. 활성 멤버십 등급을 확인하고 등록된 카드를 관리합니다.

- **현재 플랜(Current Plan)** — 활성 등급 (Free, Pro, Max, Vip, Enterprise) 을 표시하는 뱃지와, 실제 등급 변경이 일어나는 [/membership](./membership) 으로 이동하는 **Change Plan** 링크.
- **Credits** — 현재 사이클의 잔여 / 전체 Credit (예: `145 / 190`). Free 등급에서는 `0 / 0` 으로 표시됩니다.
- **만료(Expires)** — 현재 사이클 할당량의 만료일 (Free 에서는 Unlimited).
- **등록된 카드** — 마스킹된 카드 번호와 카드 소유자 이름, 그리고 옆의 버튼: **Add Card** (등록된 카드 없음), **Change Card** (등록된 카드 있음), 또는 카드 폼이 열려 있을 때는 **Cancel**. 결제는 **PortOne** 이 처리하므로, 거주 지역에서 PortOne 이 지원하는 결제 수단 (Visa, Mastercard, AMEX, JCB, 한국 내 결제수단 등) 을 사용할 수 있습니다.

이 카드에는 **카드 안에서의 등급 직접 변경** 이나 **결제 이력 목록** 이 포함되어 있지 않습니다 — 둘 다 다른 곳에 있어요. 등급 변경은 `/membership` 으로의 클릭 이동이고, 결제별 영수증은 결제 시점의 영수증 모달에 노출됩니다.

멤버십 결제는 **오프체인 전용** 입니다 — 등급 구독에는 온체인 정산 단계가 없습니다.

## <img src={useBaseUrl('/img/icons/grid.svg')} width="20" height="20" alt="MCP" style={{verticalAlign: 'middle'}} /> MCP 서버

네 번째 카드. AI 어시스턴트가 내 Ratel 계정에 연결할 때 사용하는 비밀 토큰을 발급/교체하는 곳입니다.

- **Generate Secret (생성)** — 처음이라면 클릭해서 토큰을 발급받으세요. Ratel 이 `https://ratel.foundation/mcp/<your-token>` 형태의 URL 을 만들고 *한 번만* 표시합니다 — 그 자리에서 즉시 복사하세요.
- **Regenerate (재생성)** — 토큰이 이미 있는데 새 URL 이 필요하다면 *Regenerate* 를 클릭. 새 URL 이 즉시 발급되고 기존 URL 은 즉시 동작을 멈춥니다. URL 은 API 키처럼 다루세요.

전체 설정 가이드(Claude Desktop, Claude Code, Cursor, 일반 JSON) 는 [AI 연결](./ai-connect) 챕터를 참고하세요. 같은 컨트롤이 별도의 아레나 레이아웃 `/my-ai` 에도 있고, [내 Essence → My AI](./my-essence#-my-ai--my-ai) 에서 자세히 다룹니다.

## <img src={useBaseUrl('/img/icons/bluesky.svg')} width="20" height="20" alt="연결" style={{verticalAlign: 'middle'}} /> 연결 (Connections — 별도 페이지)

연결 페이지는 `/settings` 의 카드가 아니라 별도의 라우트입니다.

```
/<your-handle>/settings/connections
```

Ratel 이 크로스포스트할 수 있는 플랫폼을 관리합니다. 상단 히어로 카드에는 연결된 플랫폼 수와 *이번 달 게시물* 카운터 (현재는 항상 0 으로 표시 — 정확한 카운트는 *(예정)*) 가 표시됩니다.

| 플랫폼 | 상태 | 동작 |
|---|---|---|
| <img src={useBaseUrl('/img/icons/bluesky.svg')} width="16" alt="Bluesky" style={{verticalAlign: 'middle'}} /> **Bluesky** | 사용 중 | 한 번 연결해 두면 Ratel 게시글이 Bluesky 타임라인으로 자동 동기화. 계정별로 자동 게시 토글 가능. |
| <img src={useBaseUrl('/img/icons/linkedin.svg')} width="16" alt="LinkedIn" style={{verticalAlign: 'middle'}} /> **LinkedIn** | *(예정)* | 장문 콘텐츠를 LinkedIn 피드로 크로스포스트. |
| <img src={useBaseUrl('/img/icons/threads.svg')} width="16" alt="Threads" style={{verticalAlign: 'middle'}} /> **Threads** | *(예정)* | 길이 자동 조정과 함께 Threads 로 크로스포스트. |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M15 8l-7 7-3-3"/><circle cx="12" cy="12" r="10"/></svg> **Farcaster** | *(예정)* | Farcaster 캐스트로 크로스포스트. |

각 플랫폼 카드에는 **연결(Connect) / 해제(Disconnect)** 와 **자동 게시(Auto-post)** 토글이 있어요. OAuth 는 각 플랫폼의 로그인 페이지에서 처리되므로 Ratel 은 비밀번호를 보지 않고, 같은 페이지에서 언제든 권한을 회수할 수 있습니다.

Ratel 에 처음 가입했다면 동일한 목적지가 안내된 온보딩 페이지 `/onboarding/connections` 에서도 보입니다. 지금 건너뛰고 필요할 때 다시 와도 괜찮아요.

> **참고: Notion 은 크로스포스트 대상이 아닙니다.** Notion 은 *인바운드* 쪽에 속합니다 — Notion 문서를 *Essence 로 끌어오는* Essence 소스이며, 현재 *(예정)* 단계입니다. 크로스포스트 페이지는 *아웃바운드* 발행 전용입니다.

## 언어 · 테마 · 알림은 어디 있나요?

이 세 가지는 `/settings` 안에 있지 않습니다 — 사용자가 만나는 위치에 두는 편이 더 유용하기 때문이에요.

- <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="12" cy="12" r="10"/><path d="M2 12h20"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg> **언어 토글** — **사이드바 푸터** 에 있습니다. 영어 ↔ 한국어를 한 번에 전환하고, 페이지·기기 사이에서 선택이 유지됩니다.
- <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg> **테마 토글** — 같은 **사이드바 푸터** 에. 다크 / 라이트 / 시스템을 순환합니다.
- <img src={useBaseUrl('/img/icons/bell.svg')} width="16" height="16" alt="알림" style={{verticalAlign: 'middle'}} /> **알림(Notifications)** — 상단 내비게이션 바의 종 모양 아이콘을 클릭하면 인박스가 열립니다. 항목별로 읽음 처리하거나, 패널 헤더의 *모두 읽음 처리(Mark all as read)* 를 사용하세요.

별도의 알림 환경설정 카드와 셀프서비스 계정 삭제 플로우는 *(예정)* 입니다.

## 요약 표

| 카드 | URL | 무엇을 바꾸나 |
|---|---|---|
| 프로필 | `/<your-handle>/settings` (상단 카드) | 아바타, 디스플레이 네임, 소개 |
| 비밀번호 | 동일 페이지 | 로그인 비밀번호 |
| 구독 및 결제 | 동일 페이지 | 등급 + Credit + 만료 확인, 카드 관리. 등급 변경은 `/membership` 으로 이동. |
| MCP 서버 | 동일 페이지 | AI 엔드포인트 생성 / 재생성 |
| 연결 | `/<your-handle>/settings/connections` | Bluesky (사용 중) + LinkedIn / Threads / Farcaster *(예정)* |
