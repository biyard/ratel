---
sidebar_position: 5
title: 호스트 액션 (편집기)
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# 호스트 액션

이 챕터는 [스페이스 액션](./actions) 의 **호스트 측** 짝입니다. 참여자 챕터가 _액션이 참여자에게 어떻게 보이는지_ 를 다뤘다면, 이 챕터는 호스트인 당신이 각 액션 유형을 _어떻게 만들고 편집_ 하는지를 안내합니다.

## 액션 만들기

스페이스의 액션 페이지 `/spaces/:space_id/actions` 에서 액션 카루셀의 **+ 만들기** 영역을 누르세요 — 그때 열리는 모달이 모든 액션 작성의 진입점입니다.

모달은 먼저 **어떤 종류의 액션인가?** 를 묻고, 네 개의 타일 중 하나를 고르게 합니다.

| 타일                                                                                                                                                                                                                                                                                                                                                                                                              | 무엇을 만드는가                                        |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><polyline points="20 6 9 17 4 12"/></svg> **Poll (투표)**                                                                                                                                                                        | 빠른 투표 — 단일 선택, 다중 선택, 주관식, 선형 척도.   |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg> **Quiz (퀴즈)**                                                                                                  | 채점되는 문항과 통과 기준선.                           |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"/></svg> **Discussion (토론)** | 호스트의 질문과 리치텍스트로 작성하는 답변.            |
| <img src={useBaseUrl('/img/icons/users.svg')} width="16" alt="팔로우" style={{verticalAlign: 'middle'}} /> **Follow (팔로우)**                                                                                                                                                                                                                                                                                    | 큐레이션된 사용자 목록을 팔로우하도록 요청하는 캠페인. |

타일을 선택하면 오른쪽에 빈 카드의 라이브 **미리보기** 가 나타나고, **만들기(Create)** 를 누르면 새 액션의 편집기 URL 로 바로 이동합니다 — 이후는 거기서 채워 넣으면 됩니다.

> **Meet 에 대해.** 다섯 번째 액션 유형인 **Meet (RSVP 가 가능한 일정 이벤트)** 는 플랫폼에 존재하고 자체 뷰어 `/spaces/:space_id/actions/meets/:meet_id` 도 있지만, 호스트 편집기 진입점은 _(예정)_ 입니다. 오늘은 [MCP API](../essence/my-essence#-my-ai--my-ai) 의 `create_meet` 도구로 Meet 를 만들고, 다른 액션처럼 참여자 카루셀에 노출시키는 방식으로 사용합니다.

## 편집기 URL 의 구조

모든 액션 편집기는 다음의 안정적인 URL 패턴을 갖습니다.

| 유형       | URL                                                         |
| ---------- | ----------------------------------------------------------- |
| Discussion | `/spaces/:space_id/actions/discussions/:discussion_id/edit` |
| Poll       | `/spaces/:space_id/actions/polls/:poll_id` (관리자 뷰)      |
| Quiz       | `/spaces/:space_id/actions/quizzes/:quiz_id` (관리자 뷰)    |
| Follow     | `/spaces/:space_id/actions/follows/:follow_id` (관리자 뷰)  |
| Meet       | `/spaces/:space_id/actions/meets/:meet_id` (관리자 뷰)      |

**Discussion 은 전용 `/edit` URL 을 갖고, Poll · Quiz · Follow · Meet 은 관리자 뷰와 참여자 뷰가 같은 URL 을 공유** 합니다 — 뒤의 네 가지에 대해 Ratel 이 방문자가 관리자인지 감지하여 페이지를 편집기 모드로 전환합니다. 참여자에게는 참여 카드가, 호스트에게는 편집기가 보여요.

편집은 디바운스를 거쳐 **자동 저장** 됩니다. 푸터의 **Save** 버튼은 저장 대기 중인 변경 사항을 즉시 반영하고 _저장됨(Saved)_ 토스트를 띄웁니다. 별도의 "발행" 버튼은 없습니다 — 저장과 발행이 같은 동작입니다.

## 공통 설정

모든 편집기는 상단의 **Content (내용)** 카드와 하단의 **Configuration (설정)** 카드로 나뉩니다. 설정 카드의 섹션은 네 가지 액션 유형 모두 공통으로 다음과 같습니다.

| 섹션                                       | 무엇을 설정하는가                                                                                                                            |
| ------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------- |
| **일정 (Schedule)**                        | _시작_ / _종료_ 일시. 이 구간을 벗어나면 액션이 활성화되지 않습니다.                                                                         |
| **참여 및 보상 (Participation & Rewards)** | 스페이스의 [인센티브 풀](./apps#-incentive-pool-%EB%B2%A0%ED%83%80) 에서 가져오는 **Credits (CR)** 보상. 완료 시 참여자 포인트로 환산됩니다. |
| **선행 액션 (Dependency Actions)**         | 참여자가 이 액션을 풀기 전에 먼저 완료해야 하는 다른 액션.                                                                                   |
| **상태 (Status)**                          | 초안 / 활성 / 종료 토글.                                                                                                                     |
| **위험 영역 (Danger zone)**                | 액션 삭제. 제출/응답이 함께 사라지며, 이미 지급된 보상은 환불되지 않습니다.                                                                  |

Discussion 은 **조정 (Moderation)** 섹션을 더 가집니다 (오프토픽 답변을 가릴 수 있는 권한 지정); Poll 은 **투표 규칙 (Voting rules)** 섹션을 더 가집니다 (아래 참고).

### 보상과 Credits 는 누가 수정할 수 있나

설정은 **Creator (스페이스 생성자) 전용** 입니다. 저장할 때마다 서버 측에서 역할을 검사합니다.

| 역할                       | 가능한 작업                                                                                                                                        |
| -------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Creator**                | 편집기 풀 액세스 — 내용 · 일정 · 보상 · 의존 · 상태 · 삭제. Creator 는 스페이스를 만든 사람 (게시물을 스페이스로 승격한 작성자) 입니다.            |
| **Member** _(팀 스페이스)_ | 팀 스페이스 안에서 _본인이 만든_ 액션을 작성/편집할 수 있지만, 보상 금액 변경이나 인센티브 풀에서의 인출은 못 합니다 — 그건 Creator 만 가능합니다. |
| **Participant**            | 액션 페이지 읽기 전용. Configuration 카드는 보이지 않습니다.                                                                                       |
| **Viewer**                 | 스페이스 스플래시 + Overview 만 읽기 전용. 액션 편집기에 진입 불가.                                                                                |

가장 많이 부딪히는 곳이 **Credits (CR)** 필드입니다 — 보상 금액을 바꾸면 스페이스 인센티브 풀에서 돈이 빠지므로 Creator (또는 명시적으로 권한이 부여된 팀 admin) 만 손댈 수 있습니다. Configuration 카드가 안 보이거나 Credits 필드가 비활성화되어 있다면, 보상을 수정할 권한이 없는 역할로 보고 있는 것입니다.

### 익명 참여 (Anonymous participation)

스페이스에는 **General → Anonymous participation** 토글이 있습니다 (Settings → Apps → General → Settings 경로). 이 토글이 두 가지를 동시에 바꿉니다.

- **신원 연결.** _Off_ (기본) 일 때는 모든 투표 · 댓글 · 제출이 참여자 핸들에 묶입니다. _On_ 일 때는 액션 카루셀에서 참여자에게 익명 핸들이 표시되고, 제출은 그 익명 신원으로 기록됩니다. Creator 는 집계 수치를 볼 수 있지만 개별 매핑은 볼 수 없습니다.
- **동의 문구.** 처음 참여 시 보이는 ConsentModal 이 익명 변형으로 바뀝니다 — "응답이 개인에게 귀속되지 않지만 집계 분석 (Panels, Analyzes) 은 동작한다" 는 점을 명확히 안내합니다.

:::tip 언제 켤까
민감한 설문 (직장 만족도, 건강 스크리닝, 내부 비평) 처럼 신원 노출 시 답변이 위축될 수 있는 경우 익명 참여를 켜세요. 누가 어디에 표를 던졌는지 그 자체가 가치인 커뮤니티 폴은 끄는 게 좋습니다.
:::

익명 모드는 [Panels](./apps.md#-panels-%EB%B2%A0%ED%83%80) 의 인구통계 수집을 끄지 **않습니다** — 익명성은 _제출_ 단위이고, 호스트가 Panels 앱을 켜놓았다면 참여자에게 연령대 · 성별 · 지역 등은 여전히 묻습니다 (Creator 의 집계 슬라이싱 용). Panel 속성은 제출 신원과 별도로 저장돼서 둘이 공존합니다.

## 스페이스 게시 (Publishing)

액션을 편집한다고 해서 스페이스가 자동으로 게시되지는 않습니다. 초안 (Draft) 상태의 스페이스는 Creator 와 팀 admin 에게만 보이고, _게시_ 한 시점에 비로소 **Index 페이지**, **Overview**, **Dashboard**, **액션 카루셀** 이 참여자 (그리고 Public 으로 설정 시 비로그인 방문자) 에게 도달 가능해집니다.

### 게시 버튼

스페이스 Index 페이지 (`/spaces/:space_id/`) 를 엽니다. 아레나 상단바에 종이비행기 아이콘의 **게시 (Publish)** 버튼이 있습니다. 클릭하면 **공개 범위 모달 (Space visibility modal)** 이 열립니다.

| 옵션                 | 누가 볼 수 있나                                                                                                                                                                  |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **공개 (Public)**    | URL 을 가진 누구나 — 비로그인 방문자 포함. Hot Spaces 랭킹, 검색, 공유 링크 모두 노출합니다. 공개 발견 피드에 노출시키려는 스페이스라면 필수.                                    |
| **비공개 (Private)** | 초대된 멤버와 팀 admin 만. 초대 안 된 방문자는 "찾을 수 없음" 페이지를 봅니다. URL 은 General 앱의 _Invite Participant_ 흐름으로 추가한 사람들에게만 비공개 링크처럼 동작합니다. |

공개 범위를 고르고 **게시** 를 누르면 모달이 스페이스를 _Draft_ 에서 해당 상태로 전환합니다. 상단바의 게시 버튼이 **시작 (Start)** 버튼으로 바뀌며 (실제 참여 개시 시점에 _Open_ → _Ongoing_ 전환), 제목 옆에 _진행 중 (In progress)_ 상태 칩이 표시됩니다.

### 게시 후 공개 범위 변경

게시 후에도 공개 범위는 수정 가능합니다 — 상단바의 **Settings → Status** 항목에서 모달을 다시 열어 다른 옵션을 고르면 됩니다. Private → Public 으로 바꾸면 소급 발견 가능해지고, Public → Private 으로 바꾸면 새 방문자에게는 즉시 숨겨지지만 이미 참여한 사람은 쫓겨나지 않습니다.

:::warning 게시는 초대 메일 발송의 트리거입니다
General 앱의 _Invite Participant_ 흐름으로 게시 _전_ 에 이메일을 추가해 두었다면, 그 초대는 **게시 시점에 한꺼번에 발송됩니다** — 이메일을 추가한 시점이 아닙니다. 그러므로 콘텐츠 / 앱 / 액션이 다 준비된 다음 토글을 넘기세요. 게시한 뒤 수정한 스페이스는 참여자에게 이미 발송된 초대 메일에 옛 모습으로 남아 있게 됩니다.
:::

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"/></svg> Discussion 편집기

URL: `/spaces/:space_id/actions/discussions/:discussion_id/edit`

Content 카드는 다음을 담습니다.

- **제목 (Title)** — 토론 카드 상단에 노출되는 헤드라인.
- **설명 / 본문 (Description / body)** — 마크다운 지원 리치 텍스트. 이것이 참여자가 답할 _프롬프트_ 입니다.
- **첨부 (Attachments)** — 드래그앤드롭 드롭존. 허용 확장자: `.pdf`, `.docx`, `.pptx`, `.xlsx`, `.png`, `.jpg`, `.jpeg`, `.mp4`, `.mov`. 업로드된 파일은 편집기 아래에 행으로 (아이콘 · 이름 · 크기 · 제거 버튼) 노출되며, Files 앱의 **Boards** 탭에 자동 링크되어 참여자도 파일 라이브러리에서 찾을 수 있습니다.

Configuration 카드는 공통 섹션에 더해 **조정 (Moderation)** 섹션을 가집니다. 토론이 활성화된 후 오프토픽 답변을 가릴 수 있는 권한을 여기서 지정합니다.

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/discussion_editor.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
브라우저가 비디오 임베드를 지원하지 않습니다. <a href={useBaseUrl('/media/discussion_editor.mov')}>워크스루 다운로드</a>.
</video>

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><polyline points="20 6 9 17 4 12"/></svg> Poll 편집기

URL: `/spaces/:space_id/actions/polls/:poll_id`

Content 카드는 다음을 담습니다.

- **제목 (Title)** — 폴의 헤드라인.
- **질문 (Questions)** — 원하는 만큼 추가할 수 있습니다. 각 카드는 자체 제목 + 선택적 설명을 가지며, **+ 질문 추가** 메뉴에서 네 가지 유형 중 하나를 고릅니다.
  - **Single (단일)** — 라디오 리스트, 응답당 한 옵션.
  - **Multi (다중)** — Single 과 동일한 옵션 리스트지만 여러 옵션을 동시에 선택 가능.
  - **Subjective (주관식)** — 장문 자유 텍스트 응답. 질문의 description 이 입력창 placeholder 힌트로 동작합니다.
  - **Linear (선형)** — `min_value` / `max_value` 경계와 양 끝의 선택적 anchor 라벨 (`min_label`, `max_label`) 을 가진 숫자 스케일.
- **"Other" 옵션 허용** _(Single / Multi 한정)_ — 참여자가 직접 입력할 수 있는 자유 텍스트 "Other" 선택지를 추가합니다. 토글을 끄면 `Other` 텍스트가 포함된 응답이 거부됩니다.

Configuration 카드는 공통 섹션에 더해 **투표 규칙 (Voting rules)** 섹션을 가집니다.

- **응답 수정 허용 (Allow response editing)** — 켜면 폴 진행 중에 참여자가 응답을 수정할 수 있습니다. 끄면 제출 즉시 응답이 확정됩니다. 암호화 업로드가 켜져 있으면 자동으로 비활성화됩니다.
- **암호화 업로드 (Encrypted upload)** — 켜면 투표 결과가 암호화되어 온체인 (canister) 에 업로드되며, 제출 후 응답 수정이 불가능합니다. 감사 가능성이 중요한 고비중 투표에 사용하세요.

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/poll_editor.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
브라우저가 비디오 임베드를 지원하지 않습니다. <a href={useBaseUrl('/media/poll_editor.mov')}>워크스루 다운로드</a>.
</video>

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg> Quiz 편집기

URL: `/spaces/:space_id/actions/quizzes/:quiz_id`

Content 카드는 다음을 담습니다.

- **제목 (Title)** 과 **설명 (Description)** — 참여자가 시작하기 전에 보는 프레이밍.
- **질문 (Questions)** — **3 ~ 20 개**, 각각 Single 또는 Multi 선택. 정답을 표시합니다.
- **합격 점수 (Pass Score)** — 통과에 필요한 최소 점수 (전체 문항 대비).
- **재시도 횟수 (Retry Count)** — 참여자별 재응시 가능 횟수.
- **첨부 (Attachments)** — 참여자가 참고할 자료: PDF / PNG / JPG, 각 25MB 까지.

합격하면 보상이 잠금 해제되고, 불합격하면 (Retry Count 가 허용하는 한) 다시 응시할 수 있습니다.

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/quiz_editor.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
브라우저가 비디오 임베드를 지원하지 않습니다. <a href={useBaseUrl('/media/quiz_editor.mov')}>워크스루 다운로드</a>.
</video>

## <img src={useBaseUrl('/img/icons/users.svg')} width="22" height="22" alt="팔로우" style={{verticalAlign: 'middle'}} /> Follow 편집기

URL: `/spaces/:space_id/actions/follows/:follow_id`

Targets 카드는 다음을 담습니다.

- **제목 (Title)** — 캠페인의 이름.
- **타겟 (Targets)** — 참여자가 팔로우해야 할 **1 ~ 20 명** 의 사용자 계정. 각 타겟은 참여자 뷰에서 인라인 행으로 렌더링되고, 그 자리에 _팔로우_ 버튼이 함께 보입니다.

Configuration 카드는 공통 섹션 — 일정, 참여 및 보상, 선행 액션, 상태, 위험 영역 — 만 갖습니다.

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/follow_editor.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
브라우저가 비디오 임베드를 지원하지 않습니다. <a href={useBaseUrl('/media/follow_editor.mov')}>워크스루 다운로드</a>.
</video>

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="3" y="4" width="18" height="18" rx="2" ry="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg> Meet 편집기 _(예정)_

URL: `/spaces/:space_id/actions/meets/:meet_id` (관리자 뷰)

라이브스트림 · 화상 통화 · 워크숍 · 오프라인 모임 같은 일정 이벤트입니다. 이 URL 의 참여자 뷰는 라이브이지만, 만들기 모달의 호스트 편집기 진입점은 _(예정)_ 입니다. 오늘 시점에 Meet 가 필요한 호스트는 MCP API 의 `create_meet` 도구를 사용하세요 ([MCP 연결](../essence/my-essence#-my-ai--my-ai) 참고).

## 팁

- **보상 예산은 적절한 액션에 배정하세요.** Credit 은 스페이스의 인센티브 풀에서 빠집니다. 온보딩용 퀴즈는 무보상으로 두고, 최종 리포트의 핵심이 되는 토론에 Credit 을 집중하는 식으로요.
- **Dependencies 로 시퀀스를 짜세요.** 흔한 패턴: 빠른 _사전(Sample)_ 폴 → 깊은 _최종(Final)_ 폴. 최종 폴이 사전 폴에 의존하도록 설정하면 참여자가 워밍업을 마치고 본 질문에 답하게 됩니다.
- **참여자 뷰로 테스트하세요.** 같은 URL 을 시크릿 창이나 다른 세션에서 열어보세요 — 참여자가 보는 화면이 그대로 드러납니다. 관리자 / 참여자 전환은 자동입니다.

## 다음 단계

- [스페이스 액션](./actions.md) — 같은 다섯 액션 유형의 참여자 측 레퍼런스.
- [스페이스 앱 → 인센티브 풀](./apps.md#-incentive-pool-%EB%B2%A0%ED%83%80) — 액션 보상이 빠져나오는 Credit 풀을 펀딩하세요.
- [리포트](./reports.md) — 액션 결과를 발행 가능한 내러티브로 바꾸세요.
