---
sidebar_position: 10
title: 내 Essence
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# 내 Essence

Essence 는 추상적인 개념이 아닙니다 — 앱 안에 두 개의 전용 페이지가 있어서, 내가 쌓은 것과 그것이 가져다주는 것을 직접 확인할 수 있어요. **My AI** (`/my-ai`) 는 Ratel 을 내 AI 어시스턴트에 연결하는 곳입니다. **Character** (`/me/character`) 는 내 활동이 레벨, 스킬 포인트, 스탯 부스트로 변하는 곳입니다.

## 왜 Essence 에 별도 페이지가 있는가

[소개 챕터](./)에서 Essence 는 "참여할수록 자라는 개인 지식 베이스"로 설명했습니다. 이 두 페이지는 그 개념이 손에 잡히는 형태로 드러나는 곳이에요. 한 탭은 Essence 를 평소 쓰는 AI 에 꽂아주고, 다른 탭은 그 Essence 를 만든 활동을 눈에 보이는 레벨과 업그레이드 가능한 스킬로 바꿉니다.

## <img src={useBaseUrl('/img/icons/grid.svg')} width="20" height="20" alt="My AI" style={{verticalAlign: 'middle'}} /> My AI — `/my-ai`

My AI 페이지는 개인 MCP 컨트롤 패널입니다. 차례로 세 가지를 합니다.

### 엔드포인트 생성과 복사

상단의 히어로 카드에는 개인 MCP URL — Claude Code, Claude Desktop, Cursor 같은 MCP 호환 클라이언트에 붙여넣을 URL — 이 표시됩니다. 형식은 다음과 같아요:

```
https://ratel.foundation/mcp/<your-token>
```

처음에는 **Generate (생성)** 을 눌러 토큰을 발급받습니다. 전체 URL 은 생성 직후 단 한 번만 표시되니, 그 자리에서 즉시 복사하세요. 토큰을 교체하려면 **Regenerate (재생성)** 을 누르면 됩니다 — 새 URL 이 나오는 즉시 기존 URL 은 동작을 멈춥니다. API 키처럼 다루세요.

페이지 상단 바의 상태 뱃지는 한눈에 — 토큰을 발급한 상태(**MCP Online / MCP 연결됨**)인지, 아직 발급 전(**Not Configured / 미설정**)인지 — 알려줍니다.

### 에이전트가 할 수 있는 일 보기

엔드포인트 아래의 **What your agent can do (에이전트가 할 수 있는 일)** 섹션은 토큰 한 장으로 연결된 클라이언트가 무엇을 할 수 있는지 보여줍니다.

- **Posts & Drafts (게시물 및 초안)** — 게시물 작성·수정·좋아요·삭제.
- **Polls & Quizzes (투표 및 퀴즈)** — 실행, 응답, 결과 분석.
- **Spaces & Teams (스페이스 및 팀)** — 스페이스 생성, 팀 관리, 사용자 팔로우.
- **Discussions (토론)** — 읽기, 댓글, 토론 시작.
- **Notifications (알림)** — 받은편지함 읽기, 읽음 처리.

모든 작업은 내 계정 범위로 한정되며 감사 가능합니다 — 에이전트는 *나로서* 행동하지, 다른 사람으로 행동하지 않습니다.

### 클라이언트 선택과 설정 가이드

기능 카드 아래의 **Setup guide (설정 가이드)** 는 탭형 안내입니다 — 클라이언트마다 한 탭씩 있어요.

- **Claude Code** — `claude mcp add` 한 줄, 활성 세션 안에서 `/mcp` 로 확인.
- **Claude Desktop** — `claude_desktop_config.json` 에 JSON 스니펫 추가, 앱 완전 종료 후 재실행.
- **Cursor** — `Settings → Cursor Settings → MCP → Add new server (HTTP / Streamable)`.
- **Generic JSON (일반 JSON)** — `curl` 스모크 테스트 + Python · TypeScript · Rust 등 모든 MCP SDK 가 사용하는 JSON-RPC 2.0 / SSE 형식.

각 탭은 번호가 매겨진 세 단계로 이뤄져 있고, 그대로 복사해 쓸 수 있는 명령어와 JSON 스니펫이 들어 있습니다. [AI 연결 챕터](./ai-connect.md)를 이미 읽으셨다면 같은 화면입니다 — `/my-ai` 는 그 내용의 인앱 도착지입니다.

## <img src={useBaseUrl('/img/icons/award.svg')} width="20" height="20" alt="Character" style={{verticalAlign: 'middle'}} /> Character — `/me/character`

Character 페이지는 Essence 를 쌓는 활동을 게임으로 바꿉니다. Essence 를 자라게 하는 그 게시물·댓글·스페이스 활동이 똑같이 XP 가 됩니다. XP 는 레벨이 되고, 레벨은 스킬 포인트(SP) 를 줍니다. 스킬 포인트는 다음 보상을 부풀려 줄 업그레이드를 사는 데 씁니다.

### 히어로 — 레벨, XP, SP

상단의 아레나 히어로는 세 숫자를 보여줍니다.

- **Level (레벨)** — 현재 캐릭터 레벨.
- **Character XP (캐릭터 XP)** — 누적 XP, 그리고 다음 레벨까지 남은 XP.
- **Skill Points / SP (스킬 포인트)** — 스킬 트리에서 사용할 수 있는 포인트.

히어로는 또한 *지금 쓸 SP 가 있는지*, *XP 를 더 모아야 하는지* 도 같이 알려줍니다.

### 스킬 트리

각 스킬의 상한은 10 단계입니다. 한 단계당 효과는 **+5%** 씩, **L10 에서 +50%** 가 최대치예요. 비용은 단계가 오를수록 늘어납니다 — 첫 단계에 5 SP, 이후 한 단계마다 +4 SP (5 → 9 → 13 → … → 41 at L10, **한 스킬을 완전히 마스터하려면 약 230 SP**).

**현재 사용 가능**

- **Money Tree (머니트리)** — 모든 스페이스 보상에서 받는 RatelPoint 지급액에 곱셈 부스트를 적용. 잔액에 적립되기 전에 가산됩니다.
- **Ranker (랭커)** — 모든 스페이스 활동에서 적립되는 보너스 XP 를 부스트합니다. 복리 효과가 있어요 — 활동당 더 많은 XP → 더 빠른 레벨업 → 다음 스킬을 위한 더 많은 SP.

**v2 에서 추가 예정** *(예정)*

- **Influencer (인플루언서)** — *v2 디자인:* 내가 소유한 스페이스가 핫(Hot) 에 노출되는 데 필요한 참여자 수를 완화 (예: L6 에서 기본 10 명 대신 4 명만 있으면 노출).
- **Sweeper (싹쓸이)** — *v2 디자인:* 내가 소유한 스페이스에서 참여자가 보상을 청구할 때 받는 소유자 보너스를 키움 (예: L6 에서 기본 10% 대신 40% 를 가져감).

각 카드는 현재 레벨, 다음 단계의 효과, **Level Up (레벨 업)** 버튼 (또는 L10 도달 시 **Maxed (최대치)**) 을 보여줍니다. v2 스킬은 **v2 · 곧 출시** 뱃지가 붙고, 아직 레벨업할 수 없습니다.

## My AI 와 Character 가 서로를 키운다

두 페이지는 같은 활동 흐름을 공유합니다.

- Ratel 에서 한 모든 활동 — 게시물 작성, 토론 댓글, 폴 호스팅, 스페이스 퀘스트 완료 — 은 **Essence** (데이터 레이어) 를 채우고 동시에 **Character XP** (게임플레이 레이어) 를 올립니다.
- MCP 로 연결된 에이전트가 나 대신 글을 올리면 — 그 작업이 내 명의로 기록되는 한 — Essence 도 풍성해지고 Character 도 레벨업합니다.
- Character 에서 산 스킬 업그레이드(현재는 Money Tree) 는 스페이스와 보상에서 실제로 가져가는 금액에 직접 영향을 줍니다.

다시 말해, My AI 는 Essence 가 *바깥으로* 닿는 통로이고, Character 는 Essence 가 *안으로* 보상을 돌려주는 통로입니다. 같은 엔진의 두 측면이에요.

## 다음 단계

*(예정)* Phase 2 에서는 **Essence House** 마켓플레이스가 열립니다 — 내 Essence 가 다른 사용자가 단일 통합 MCP URL 로 구독할 수 있는 자산이 됩니다. 그때가 되면 기존 My AI 엔드포인트가 자동으로 구독한 모든 House 까지 라우팅을 확장하기 때문에, 클라이언트 설정을 다시 만질 필요가 없습니다.

그때까지 My AI 는 내 엔드포인트, Character 는 내 점수판입니다. URL 을 발급해서 AI 에 꽂고, XP 를 모으기 시작하세요.
