---
sidebar_position: 1
slug: /spaces
title: 스페이스
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# 스페이스 (Spaces)

**스페이스**는 Ratel 활동이 일어나는 아레나입니다. 토론, 투표, 퀴즈, 팔로우 퀘스트, 일정이 잡힌 미팅 — 커뮤니티를 측정 가능한 대화로 바꾸는 모든 활동이 스페이스 안에서 벌어집니다. 스페이스에서 호스트는 참여자에게 보여줄 모습을 만들고, 참여자는 활동을 통해 Essence 를 쌓습니다.

모든 스페이스는 안정적인 URL 을 갖습니다:

```
/spaces/:space_id
```

링크가 있다면 바로 들어갈 수 있고, 없다면 보통 크리에이터 프로필, 피드, 알림을 통해 도달하게 됩니다.

## 두 개의 역할, 하나의 아레나

스페이스 안에는 사실상 두 가지 역할만 있습니다:

- **호스트 (Host)** 는 스페이스의 정체성을 결정합니다. [Apps](/spaces/apps) — 안내 페이지, 파일 라이브러리, 투표, 퀴즈, 분석 — 를 끼워 넣고 보상이 어떻게 흐를지 설정합니다.
- **참여자 (Participant)** 는 *행동하기 위해* 들어옵니다. 액션 카루셀을 스크롤하고, 투표하고, 의견을 남기고, 사람을 팔로우하고, 미팅에 RSVP 합니다. 모든 액션은 본인의 Essence 가 되고, 보상이 걸린 스페이스라면 Incentive Pool 에서 한 몫을 받게 됩니다.

스페이스를 운영해 본 적이 없다면, 먼저 참여자로 시작하세요. 참여자 흐름을 직접 경험해 보는 것이 호스트가 반대편에서 무엇을 설정하고 있는지를 가장 빠르게 이해하는 길입니다.

## 스페이스 안의 탭

스페이스를 열면 같은 `:space_id` 가 여러 화면의 루트가 되어, 탭을 바꿔 가며 볼 수 있습니다:

| 아이콘 | 탭 | URL | 용도 |
|---|---|---|---|
| <img src={useBaseUrl('/img/icons/compass.svg')} width="18" height="18" alt="Compass" style={{verticalAlign: 'middle'}} /> | **Index** (아레나 / 포털) | `/spaces/:space_id/` | 뷰어 스플래시. 처음 보이는 화면 — 브랜딩, 헤드라인, 참여 / 로그인 카드. |
| <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="3" y="3" width="4.5" height="14" rx="0.5"/><rect x="10" y="9" width="4.5" height="8" rx="0.5"/><rect x="10" y="3" width="4.5" height="2.5" rx="0.5"/></svg> | **Dashboard** | `/spaces/:space_id/dashboard` | 한눈에 보는 활동 현황. 진행 중인 액션, 참여 수, 최근 무슨 일이 있었는지. |
| <img src={useBaseUrl('/img/icons/file-text.svg')} width="18" height="18" alt="File text" style={{verticalAlign: 'middle'}} /> | **Overview** | `/spaces/:space_id/overview` | 내러티브 탭. 호스트의 피치 — 이 스페이스가 무엇이고 누구를 위한 것이며 왜 관심을 가져야 하는지. |
| <img src={useBaseUrl('/img/icons/file-text.svg')} width="18" height="18" alt="File text" style={{verticalAlign: 'middle'}} /> | **Report** | `/spaces/:space_id/report` | 호스트가 커뮤니티에 공개한 분석 · 리포트. Phase 4 의 매출 분배(플랫폼 10% · 호스트 60% · 기여자 30%)가 이곳을 통해 흐릅니다. |

알아둘 만한 딥링크 두 가지도 있습니다:

- `/spaces/:space_id/discussions/:discussion_id` — 단일 토론을 그 자체의 페이지로 엽니다 (SNS 공유에 좋습니다).
- `/spaces/:space_id/discussions/:discussion_id/comments/:comment_id` — 토론을 열고 특정 댓글까지 자동으로 스크롤하면서 해당 댓글을 하이라이트합니다. *바로 그 답변* 을 누군가에게 보내고 싶을 때 사용하세요.

:::tip 공유 팁
토론 딥링크는 URL 정규화 후에도 유지됩니다 — Bluesky, X, LinkedIn, Slack 어디에 붙여 넣어도 그대로 동작합니다. 댓글 딥링크도 마찬가지입니다.
:::

## 스페이스 발견하기

자연스러운 진입점이 몇 가지 있습니다:

- <img src={useBaseUrl('/img/icons/user.svg')} width="18" height="18" alt="User" style={{verticalAlign: 'middle'}} /> **프로필에서.** 모든 사용자는 자신이 호스트하거나 활발히 참여 중인 스페이스를 노출합니다 (`/<your-handle>/spaces`).
- <img src={useBaseUrl('/img/icons/home.svg')} width="18" height="18" alt="Home" style={{verticalAlign: 'middle'}} /> **피드에서.** 게시물이 스페이스를 알릴 수 있고, 그 링크는 곧장 아레나로 데려다 줍니다.
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg> **검색에서.** 주제나 스페이스 제목으로 검색하세요.
- <img src={useBaseUrl('/img/icons/bell.svg')} width="18" height="18" alt="Bell" style={{verticalAlign: 'middle'}} /> **알림에서.** 참여 중인 스페이스에 호스트가 새 액션을 추가하면, 그 퀘스트로 직접 연결되는 알림이 옵니다.

## 스페이스에 참여하기

스페이스에 참여하는 순간, 그 안에서의 활동에서 Essence 가 구조화된 신호를 모으기 시작합니다. Index 페이지에서 **Participate** 카드를 보게 됩니다. 로그인하거나 (호스트가 검증을 요구한다면) 지갑을 연결하세요. 그러면 들어가게 됩니다.

"참여" 가 실제로 의미하는 것:

- 호스트가 게시한 모든 액션을 열 수 있습니다.
- 투표 · 댓글 · 팔로우 선택 · 퀴즈 답변이 본인 계정에 묶이고, 본인이 소유하는 EssenceSource 가 됩니다.
- 스페이스에 **Incentive Pool** 이 있다면, 액션 완료 시 분배 대상이 됩니다.
- 호스트가 새 액션을 게시하거나 리포트를 공개하거나 본인 댓글에 답글을 달면 알림을 받습니다.

:::note 예정
Phase 0 의 풀 Essence 파이프라인 — 모든 스페이스 액션을 본인의 개인 지식 베이스로 임베딩 — 은 곧 들어옵니다. 출시 전까지도 활동은 정상적으로 기록되고 보상이 지급되며, 임베딩 레이어는 이후에 소급 적용됩니다.
:::

## 다음 페이지

이 챕터는 역할별 / 호스트 워크플로우별로 여러 하위 페이지를 갖습니다.

- **[스페이스 앱](/spaces/apps)** — 호스트의 도구함. 안내 페이지, 파일, AI 보조 리포트, Panels, Incentive Pool.
- **[스페이스 액션](/spaces/actions)** — 참여자의 퀘스트 보드. 토론, 투표, 퀴즈, 팔로우, 미팅 — 그리고 그것들이 Essence 에 어떻게 쌓이는지.
- **[스페이스 대시보드](/spaces/dashboard)** — 호스트의 라이브 통계: 카드 그리드 + 참여자 랭킹.
- **[호스트 액션](/spaces/host-actions)** — 토론 · 투표 · 퀴즈 · 팔로우를 만드는 호스트 측 편집기.
- **[리포트](/spaces/reports)** — AI 보조 장문 리포트와 그것을 뒷받침하는 교차 필터 Analyzes.
