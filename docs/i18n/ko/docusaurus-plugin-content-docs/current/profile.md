---
sidebar_position: 7
title: 프로필
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# 프로필

프로필은 Ratel 에서 누구든지 내 이름을 누르거나 핸들을 주소창에 입력했을 때 도착하는 *공개* 페이지입니다. 주소는 다음과 같아요.

```
/<your-handle>
```

핸들이 `alex` 라면 `/alex` 입니다. 같은 핸들이 내 계정에 묶인 모든 페이지에 그대로 이어지므로, 내 URL 만 알면 내가 만든 모든 곳을 찾아갈 수 있습니다.

## `/your-handle` 가 공개로 보여주는 것

누군가 `/<your-handle>` 을 열면 **프로필 홈** 에 도착합니다. 위에서 아래로 세 가지가 쌓여 있어요.

- **배너** — 직접 업로드한 가로형 헤더 이미지. 설정하지 않았다면 기본 아레나 배너가 표시됩니다.
- **아바타, 디스플레이 네임, 소개** — 프로필 사진, 보여주고 싶은 이름, 본인을 짧게 설명하는 한 줄.
- **포스트 피드** — 발행한 게시글이 최신순으로 노출되며, 무한 스크롤로 과거 글까지 이어 봅니다.

방문자가 로그인한 상태이고 본인이 아니라면 헤더 옆에 **팔로우 / 팔로잉** 버튼이 보입니다. 누군가를 팔로우하면, 그 사람의 게시글이 팔로워의 피드에서 더 위로 가중치를 받습니다.

이름 오른쪽의 **Level 칩** 은 캐릭터 레벨을 보여줍니다 — Essence 를 채운 활동량에 대한 공개 요약이에요. ([내 Essence → Character](./my-essence#-character-mecharacter) 에서 레벨이 무엇을 의미하고 SP 를 어떻게 쓰는지 자세히 다룹니다.)

## 팔로잉과 팔로워

Ratel 의 팔로우는 어떤 프로필 헤더에서든 한 번 클릭으로 토글됩니다 — 본인의 프로필은 액션 없는 칩만 보이고, 다른 사람의 프로필에서는 (아직 팔로우하지 않았다면) **Follow** 또는 (팔로우 중이라면) **Following** 이 보여요.

팔로우가 하는 일:

- **그 사람의 게시글에 가중치를 더해** 피드에서 위로 끌어올립니다. 팔로우는 홈 피드의 *Following* 탭과 플랫폼의 추천 신호에 반영돼요.
- **상대에게 알림이 갑니다** (작은 인박스 이벤트 — 옵트아웃은 *(예정)* 입니다).
- **본인의 Essence 에 *누구와 결을 맞추는지* 신호로 기록됩니다** — 다운스트림 스페이스 추천과 (이후) 에이전트 참여에 유용해요.

팔로우를 해제하려면 같은 프로필의 **Following** 을 다시 누르세요 — 칩이 **Follow** 로 돌아갑니다. 팔로우 / 언팔로우 모두 즉시 반영됩니다.

팔로우 중인 사람과 본인을 팔로우하는 사람을 한 화면에서 보고 정리하는 별도의 **`/following`** / **`/followers`** 목록 페이지는 *(예정)* 입니다. 오늘은 자연스럽게 — 프로필을 방문하면서 팔로우 관계가 드러나고, 스페이스가 초대 제안의 한 입력값으로 팔로우를 사용합니다.

## 핸들 주변의 딥링크 페이지

`/<your-handle>` 접두사를 공유하는 페이지가 여러 개 있고, 각각 직접 링크할 수 있습니다.

| <img src={useBaseUrl('/img/icons/home.svg')} width="16" height="16" alt="홈" style={{verticalAlign: 'middle'}} /> 화면 | URL | 무엇이 있나 |
|---|---|---|
| **프로필 홈** | `/<your-handle>` | 위에서 설명한 배너 + 소개 + 최근 게시글. |
| <img src={useBaseUrl('/img/icons/file-text.svg')} width="16" height="16" alt="포스트" style={{verticalAlign: 'middle'}} /> **포스트** | `/<your-handle>/posts` | 발행한 모든 게시글의 전체 목록. |
| <img src={useBaseUrl('/img/icons/grid.svg')} width="16" height="16" alt="스페이스" style={{verticalAlign: 'middle'}} /> **스페이스** | `/<your-handle>/spaces` | 내가 호스팅하거나 활발히 참여 중인 스페이스. |
| <img src={useBaseUrl('/img/icons/users.svg')} width="16" height="16" alt="멤버십" style={{verticalAlign: 'middle'}} /> **멤버십** | `/<your-handle>/memberships` | Ratel 멤버십 등급 (Free, Pro, Max, Vip, Enterprise). |
| <img src={useBaseUrl('/img/icons/award.svg')} width="16" height="16" alt="크리덴셜" style={{verticalAlign: 'middle'}} /> **크리덴셜** | `/<your-handle>/credentials` | DID 기반 검증 가능 크리덴셜 — 본인에 대한 발급된 클레임. |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg> **초안** | `/<your-handle>/drafts` | 본인 외에는 아무도 볼 수 없는 작성 중인 글. |

페이지 왼쪽에는 **카테고리** 사이드바가 있어요. 카테고리 칩 (플랫폼이 노출하는 각 카테고리당 하나 — 사용자가 만든 게 아니라 서버에서 정의된 목록) 을 누르면 피드를 그 주제로 좁히고, **전체(All)** 를 누르면 필터를 해제합니다. 사이드바는 페이지를 바꾸지 않습니다 — 필터는 현재 페이지가 이해하는 범위에만 적용됩니다 (주로 포스트와 프로필 피드).

### 한눈에 보는 프라이버시

- **프로필 · 포스트 · 스페이스** 는 핸들 아래에서 **공개** 입니다. 링크만 있으면 로그인 여부와 무관하게 누구나 볼 수 있어요.
- **멤버십 · 크리덴셜은 세션 범위입니다.** `/<handle>/...` URL 아래에 있지만 항상 **로그인한 방문자 본인의** 데이터를 보여줍니다 — 핸들 소유자의 데이터가 아닙니다. 다른 사람의 `/handle/memberships` URL 을 방문해도 본인의 멤버십이 표시돼요. (인앱 내비게이션의 일관성을 위해 URL 이 존재할 뿐, 핸들 소유자의 정보를 공개하는 화면은 아닙니다.)
- **`/your-handle/drafts` 는 본인만 볼 수 있습니다.** 다른 사람의 초안 URL 을 열어도 아무것도 반환되지 않습니다. 내 작업 중인 글을 위한 페이지이므로, 본인이 로그인한 상태에서만 표시됩니다.

내 프로필이 *모르는 사람에게 어떻게 보이는지* 확인하고 싶다면, 시크릿 창에서 열거나 로그아웃해 보면 공개 뷰가 그대로 나옵니다.

## 프로필 편집

`/<your-handle>` 에서 보이는 프로필 필드는 설정에서 편집합니다. **디스플레이 네임**, **아바타**, **배너**, **소개** 를 바꾸려면:

1. <img src={useBaseUrl('/img/icons/settings.svg')} width="14" height="14" alt="설정" style={{verticalAlign: 'middle'}} /> **설정** 을 `/<your-handle>/settings` 에서 엽니다. (빠른 경로: 사이드바 하단의 사용자 드롭다운에서 *Settings* 클릭.)
2. **프로필** 카드의 필드를 수정합니다.
3. **저장(Save)** 을 클릭. 변경 사항은 즉시 반영됩니다.

**핸들 자체** 는 가입 시 정해지며 이후 변경이 어렵습니다 — 안정적인 이름을 고르세요. 그 외(소개, 아바타, 배너)는 언제든 수정할 수 있습니다. 모든 설정 카드의 전체 투어는 [설정](./settings) 챕터에서 다룹니다.

## 외부에서 프로필로 링크하기

내 프로필 URL 은 정식 주소이고, 어디에 공유해도 안전합니다.

```
https://ratel.foundation/<your-handle>
```

Bluesky, X, LinkedIn, Slack, Discord 등에서 URL 정규화를 거쳐도 깨지지 않고 그대로 동작하며, <img src={useBaseUrl('/img/icons/compass.svg')} width="14" height="14" alt="오픈그래프" style={{verticalAlign: 'middle'}} /> 오픈그래프 미리보기에 아바타·디스플레이 네임·소개가 노출됩니다. Ratel 게시글을 Bluesky 로 크로스포스트하면 ([포스트 → 크로스포스팅](./posts#%ED%81%AC%EB%A1%9C%EC%8A%A4%ED%8F%AC%EC%8A%A4%ED%8C%85-cross-posting) 참고), 외부에 게시된 카피에는 내 프로필의 원문 링크가 자동으로 따라 붙습니다.

특정 글이나 스페이스에 사람을 보내고 싶을 때는 — *"내 최근 글을 읽어 줘"*, *"이번 주 내가 호스트하는 스페이스에 와"* — 게시글 URL 이나 스페이스 URL 을 직접 보내세요. 프로필 페이지는 *내가 누구인지*, 딥링크는 *내가 만든 무엇인지* 를 알리는 도구입니다.

## 다음 단계

프로필 세팅이 끝났다면 자연스러운 다음 단계는 다음과 같습니다.

- [설정](./settings) — 비밀번호, 결제, MCP 서버 등 계정 나머지 부분 다듬기.
- [내 Essence](./my-essence) — 지금까지의 활동이 무엇을 만들었는지 확인.
- [리워드](./rewards) — 모든 스페이스에서 적립한 보상 추적.
