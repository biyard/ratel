---
sidebar_position: 5
title: 게시글
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# 게시글 (Posts)

## 게시글이 중요한 이유

게시글은 당신의 Essence 에 가장 직접적으로 기여하는 통로입니다. 작성한 글, 남긴 댓글, 누른 반응 하나하나가 모두 **EssenceSource** — 당신의 House 가 학습하는 원료가 됩니다. Ratel 에 글을 많이 쓸수록 Essence 는 풍성해지고, 당신을 구독한 사람들에게 House 는 더 유용해집니다.

## 피드 둘러보기

피드는 `/` (홈) 와 `/posts` 에서 만날 수 있습니다. 다른 사람들이 무슨 생각을 하고 있는지 발견하고, 흥미로운 대화에 끼어들 수 있는 가장 중심이 되는 공간입니다.

- <img src={useBaseUrl('/img/icons/users.svg')} width="18" height="18" alt="Users" style={{verticalAlign: 'middle'}} /> **팔로잉(Following)** — 팔로우 중인 사람·팀·House 의 게시글
- <img src={useBaseUrl('/img/icons/compass.svg')} width="18" height="18" alt="Compass" style={{verticalAlign: 'middle'}} /> **발견(Discover)** — 네트워크 전체에서 폭넓게 추천되는 게시글. 새로운 것을 찾을 때 유용합니다
- **무한 스크롤** — 스크롤만 내리면 다음 게시글이 자동으로 로드됩니다. "다음 페이지" 버튼을 누를 필요가 없습니다
- **정렬 · 필터** — 피드 상단에서 최신순 / 인기순을 바꾸거나 주제별로 좁힐 수 있습니다

:::tip
피드가 너무 조용하다면 발견(Discover) 탭에서 몇 명을 더 팔로우해 보세요. 팔로우한 계정의 글이 가장 우선적으로 노출됩니다.
:::

## 게시글 작성

앱 어디서든 <img src={useBaseUrl('/img/icons/edit.svg')} width="18" height="18" alt="Edit" style={{verticalAlign: 'middle'}} /> **작성** 버튼을 누르거나 `/posts` 에서 새 글쓰기를 시작하면 에디터가 열립니다. Ratel 의 에디터는 **Tiptap** 기반의 리치 텍스트 에디터입니다. 마크다운 문법을 외울 필요는 없지만, 글을 진지하게 쓰는 데 필요한 기능은 모두 갖춰져 있습니다.

지원하는 기능:

- **제목(H1–H3)** — 글의 구조 잡기
- **굵게**, **기울임**, **취소선**
- **글머리 기호 · 번호 매기기 목록**
- **인용 블록** — 다른 사람의 말을 인용할 때
- **코드 블록** — 문법 강조 포함, 기술적인 내용에 유용합니다
- **인라인 링크** — 웹 어디든 연결 가능
- **이미지** — 드래그 앤 드롭 · 클립보드 붙여넣기 · 파일 업로드
- **임베드** — YouTube 영상, X 게시글 등 지원되는 링크를 붙이면 자동으로 미리보기가 만들어집니다

또한 **해시태그**(`#essence`) 로 발견을 돕고, **@멘션** 으로 다른 사용자를 대화에 부를 수 있습니다.

작성을 마치면 두 가지 선택지가 있습니다 — 나중에 다듬기 위해 **임시 저장(Save as draft)** 하거나, 바로 타임라인과 팔로워 피드에 노출되도록 **게시(Publish)** 합니다.

## 임시저장 (Drafts)

임시저장은 **`/your-handle/drafts`** 에 모입니다 — `your-handle` 은 본인의 사용자명으로 바꾸면 됩니다. 아직 공개할 준비가 안 된 글들의 개인 작업 공간이고, 페이지는 본인 전용 — 다른 사람의 drafts URL 을 방문해도 아무것도 반환되지 않습니다.

### 자동 저장 동작

에디터에 입력하기 시작하는 순간부터 모든 글은 자동으로 임시저장됩니다. 별도의 "저장" 버튼을 기억할 필요가 없어요 — 탭을 닫거나, 연결이 끊기거나, 다른 기기로 옮기더라도 작업 중이던 글이 그대로 남아 있습니다. 다른 탭에서 활발히 편집 중인 글은 임시저장 목록에서 **Writing now (작성 중)** 뱃지로 표시돼요.

### 통계 헤더와 필터

`/your-handle/drafts` 상단에는 통계 스트립이 있습니다 — **전체 임시저장(Total drafts)**, **작성 단어(Words written)**, **마지막 편집(Last edited)** — *"이번 달에 얼마나 썼지?"* 를 한눈에 보기 좋아요.

아래의 필터 칩으로 목록을 좁힐 수 있습니다.

- **전체(All)** · **오늘(Today)** · **이번 주(This Week)** · **이전(Older)** — 최근성 기준.
- **스페이스 사용(Space-enabled)** — 스페이스가 첨부된 임시저장 (게시글 발행과 동시에 스페이스를 만드는 임시저장).

**정렬(Sort)** 드롭다운으로 목록을 재배열합니다 — *최근 편집순(Recently edited)* (기본), *오래된 순(Oldest first)*, *제목순(Title A → Z)*, *단어 많은순(Most words)*. 목록은 *오늘 / 이번 주 / 이전* 섹션으로 자동 묶여 시각적으로 훑기 좋습니다.

### 임시저장별 액션

각 임시저장 타일에는 썸네일, 제목 (또는 *Untitled draft*), 발췌, 마지막 편집부터의 경과 시간, 첨부된 이미지가 있다면 이미지 카운트가 표시돼요. 타일의 **`…`** 메뉴에서 다음 작업이 가능합니다.

- **이어서 편집(Resume editing)** — 임시저장을 게시글 에디터에서 다시 열기.
- **복제(Duplicate)** — 임시저장을 새 untitled 임시저장으로 복사 (반복 게시 템플릿에 유용).
- **마크다운 내보내기(Export as Markdown)** — 본문을 `.md` 파일로 다운로드. (본문만 대상 — 이미지 첨부는 Ratel 에 그대로 남습니다.)
- **임시저장 삭제(Delete draft)** — 영구 삭제, 되돌릴 수 없음. (휴지통 복구는 *(예정)* 입니다.)

### 팀 임시저장

협업 글을 위한 별도 **팀 임시저장** 도 `/your-handle/team-drafts` 에서 관리할 수 있어요. Team 의 관리자 / 멤버가 Team 핸들로 발행하기 전에 함께 임시저장을 다듬을 수 있습니다. 자동 저장과 임시저장별 액션은 같지만, 작업 공간은 Team 의 관리자급과 공유됩니다.

## 게시글 상세 페이지

모든 게시글은 자체 URL 을 가집니다 — **`/posts/:post_id`**. 특정 글을 누군가에게 공유할 때 사용하는 주소입니다.

상세 페이지에서 할 수 있는 일:

- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M7 10v12"/><path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H7V10l4.5-9.5L13 1.5l2 1.94v.01l.5 1.93z"/></svg> **좋아요** — 동의·공감을 빠르게 표현. Ratel 은 엄지척 모양 아이콘을 사용합니다.
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="3" y="3" width="18" height="14" rx="2"/><path d="M3 17l4 4v-4"/></svg> **댓글** — 새 스레드를 시작하거나 다른 댓글에 답글 달기
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="18" cy="5" r="3"/><circle cx="6" cy="12" r="3"/><circle cx="18" cy="19" r="3"/><line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/><line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/></svg> **공유** — 링크 복사 후 어디든 보내거나 즐겨 쓰는 플랫폼으로 공유
- <img src={useBaseUrl('/img/icons/edit-square.svg')} width="18" height="18" alt="Edit square" style={{verticalAlign: 'middle'}} /> **편집** (작성자 한정) **`/posts/:post_id/edit`** — 오타 수정, 논지 다듬기, 사실 업데이트 등 언제든 수정 가능

댓글도 게시글과 동일한 Tiptap 포맷을 지원하므로 답글 안에서 인용·링크·임베드를 모두 사용할 수 있습니다.

## 크로스포스팅 (Cross-posting)

Ratel 의 대표 기능 중 하나입니다. 한 번 쓴 글을 중요한 모든 곳에 동시에 발행할 수 있습니다. 현재 Ratel 은 <img src={useBaseUrl('/img/icons/bluesky.svg')} width="16" alt="Bluesky" style={{verticalAlign: 'middle'}} /> **Bluesky** 로 게시글을 자동 동기화합니다. <img src={useBaseUrl('/img/icons/linkedin.svg')} width="16" alt="LinkedIn" style={{verticalAlign: 'middle'}} /> **LinkedIn**, <img src={useBaseUrl('/img/icons/threads.svg')} width="16" alt="Threads" style={{verticalAlign: 'middle'}} /> **Threads** 연결은 UI 에 준비되어 있고, 곧 순차적으로 출시됩니다.

### 한 번만 연결하면 됩니다

**`/your-handle/settings/connections`** 에서 각 플랫폼을 연결합니다. 각 플랫폼의 공식 로그인 플로우로 인증하므로, 당신의 비밀번호는 Ratel 서버에 저장되지 않습니다. 처음 가입한 사용자는 **`/onboarding/connections`** 에서 이 과정을 안내받습니다.

### 게시글마다 켜고 끄기

게시글을 작성할 때 에디터 사이드바에 연결된 플랫폼이 표시됩니다. 발행하고 싶은 곳만 켜고, 나머지는 끄면 됩니다 — Ratel 에만 올리고 싶은 글은 어떤 외부 플랫폼도 켜지 않으면 됩니다. 글마다 자유롭게 조합할 수 있습니다.

### 글자 수 제한

각 플랫폼은 고유한 콘텐츠 규칙이 있고, Ratel 이 자동으로 맞춰 줍니다.

- <img src={useBaseUrl('/img/icons/bluesky.svg')} width="16" alt="Bluesky" style={{verticalAlign: 'middle'}} /> **Bluesky** — 300자 제한이라 긴 글은 자연스럽게 잘라내고 Ratel 의 원문 링크가 붙습니다
- <img src={useBaseUrl('/img/icons/linkedin.svg')} width="16" alt="LinkedIn" style={{verticalAlign: 'middle'}} /> **LinkedIn** *(예정)* — 장문에 친화적이라 대부분 원문 그대로 발행됩니다
- <img src={useBaseUrl('/img/icons/threads.svg')} width="16" alt="Threads" style={{verticalAlign: 'middle'}} /> **Threads** *(예정)* — 500자 제한, Bluesky 와 비슷한 방식으로 처리됩니다

### 발행 시점에 목적지 선택

게시글을 작성할 때 에디터 우측의 **크로스포스트(Cross-post)** 사이드바에 연결된 모든 목적지가 토글이 있는 행으로 표시됩니다. 기본값은 목적지별 **자동 게시(Auto-post)** 설정 (`/your-handle/settings/connections` 에서 관리) 을 따르지만, 게시글마다 토글을 바꿀 수 있어요 — Ratel 전용 글이라면 끄고, 일회성 공지라면 켭니다.

행 위쪽에 *N 개 네트워크에 도달(Reaching N networks)* 헤더가 보이고, 어떤 목적지의 글자 수 제한에 글이 잘리는 경우 **Truncated (잘림)** 뱃지가 노출됩니다. 발행 전에 외부에 노출되는 사본이 잘릴지 미리 확인할 수 있어요.

### 글자 수 제한 처리

각 목적지에는 단단한 상한선이 있어요 — Bluesky 300 자, Threads 500 자 (출시 시), Farcaster 320 자 (출시 시), LinkedIn 3,000 자 (출시 시). Ratel 게시글이 한도를 넘으면:

> 1,500 자짜리 Ratel 게시글은 Bluesky 에서 280 자 발췌 + `… → ratel.foundation/posts/<id>` 형태로 발행됩니다 — 외부 독자는 링크를 눌러 Ratel 의 원문을 읽습니다.

자르기는 단어 단위로 이뤄지며 (단어 중간을 자르지 않음), 외부 독자가 정식 사본을 찾을 수 있도록 백링크는 항상 보존됩니다.

### 발행 후 게시글 상세에 표시되는 것

발행된 게시글의 상세 페이지 (`/posts/:post_id`) 에는 본문 아래에 **확장 게시(Syndication)** 패널이 추가됩니다. 연결된 각 목적지가 다음을 포함한 행으로 렌더링돼요.

- 상태 뱃지 — **Published (게시됨)** 녹색 체크 + 외부 게시글로 가는 *View* 링크, **Pending (대기 중)** (큐 — 발송 대기), **Failed (실패)** 빨간색, **Retry now (재시도)** 버튼 포함, 또는 **Skipped (건너뜀)** 발행 시점에 토글을 껐을 때.
- 상단의 *N succeeded · N failed* 요약.
- 시간이 지나며 들어오는 목적지의 인게이지먼트 — 좋아요, 댓글, 재게시.
- Failed 행의 **Attempt** 카운터로 재시도가 이미 한 번 일어났는지 확인 가능.

패널의 작은 **Refresh** 버튼은 라이브 인게이지먼트 카운트를 즉시 다시 가져옵니다.

### 실패 처리

크로스포스트 실패의 대부분은 두 가지로 나뉩니다.

- **토큰 만료** — 외부 플랫폼이 Ratel 의 접근 권한을 회수한 경우. `/<your-handle>/settings/connections` 에서 재연결하면 행의 **Retry now** 버튼이 다시 동작합니다.
- **목적지 플랫폼 장애** — 패널이 *Failed* 와 시도 카운터를 표시합니다. 플랫폼이 복구되면 **Retry now** 를 누르세요. Ratel 은 무한정 자동 재시도하지 않습니다 — 통제권은 본인에게 있어요.

:::tip
크로스포스트가 실패했다면 (플랫폼 장애, 토큰 만료 등) `/your-handle/settings/connections` 에서 재연결한 뒤, 실패한 행의 **Retry now** 를 눌러 다시 발송하세요. 연결이 복구된 시점부터 새 게시물은 정상 동기화됩니다.
:::

## 게시글이 Essence 에 어떻게 쌓이는가

발행한 게시글, 작성한 댓글, 남긴 반응은 모두 **EssenceSource** 로 수집됩니다. 이것들은 시간이 지나며 임베딩으로 변환되어 당신의 개인 지식 베이스 — 당신 House 의 토대 — 로 엮입니다. 더 많이 쓸수록 House 는 당신을 더 잘 이해하게 되고, 구독자들이 던지는 질문에 더 유용한 답을 줄 수 있게 됩니다.

:::note
Essence 파이프라인의 전체 기능 (임베딩 · 검색 · House Q&A) 은 단계별로 출시될 예정입니다 (예정). 오늘 발행하는 게시글은 이미 EssenceSource 로 수집되어 후속 단계에 활용됩니다.
:::
