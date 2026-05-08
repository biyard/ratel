---
sidebar_position: 2
title: 시작하기
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# 시작하기

Ratel — Human Essence Platform 에 오신 것을 환영합니다. 이 챕터는 Ratel 에서 보내는 첫날을 안내합니다. 가입부터 프로필 설정, 외부 계정 연결, 그리고 화면 둘러보기까지 한 번에 짚어드릴게요.

계정이 없어도 대부분의 화면은 둘러볼 수 있습니다. 다만 글을 쓰거나, 투표하거나, 스페이스에 참여하거나, 자신의 Essence 를 쌓거나, 보상을 받으려면 로그인이 필요해요.

## 가입 & 로그인

왼쪽 사이드 메뉴 (모바일에서는 하단 바) 에서 <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/><polyline points="10 17 15 12 10 7"/><line x1="15" y1="12" x2="3" y2="12"/></svg> **Sign In** 버튼을 찾으세요. 화살표가 문 안으로 들어가는 모양의 작은 아이콘입니다. 이 버튼을 누르면 **Join the Movement** 팝업이 열리는데, 신규 가입과 재로그인 모두 여기서 시작합니다.

팝업에서는 세 가지 방법으로 들어갈 수 있어요.

- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="2" y="4" width="20" height="16" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/></svg> **이메일 & 비밀번호** — 이메일 주소를 입력하고 **Send (전송하기)** 를 누른 뒤, 메일로 받은 6 자리 인증번호를 입력해 **Verify (인증하기)** 를 누르세요. 그다음 팝업 상단의 **Create an account (계정 만들기)** 링크를 눌러 비밀번호를 정하면 가입 완료입니다. 이미 계정이 있다면 이메일과 비밀번호를 입력하고 **Continue (계속)** 를 누르면 됩니다.
- <img src={useBaseUrl('/img/icons/google.svg')} width="18" alt="Google" style={{verticalAlign: 'middle'}} /> **Continue With Google** — Google 계정으로 한 번에 로그인합니다. 첫 가입이라면 사용자 이름과 약관 동의를 한 번 받고, 이후로는 클릭만으로 로그인됩니다.
- <img src={useBaseUrl('/img/icons/walletconnect.svg')} width="18" alt="WalletConnect" style={{verticalAlign: 'middle'}} /> **Continue With Wallet** — **WalletConnect** 로 연결합니다. WalletConnect 를 지원하는 모든 지갑 (MetaMask, Rainbow, Trust Wallet, Coinbase Wallet 등) 을 QR 코드나 딥링크로 페어링할 수 있어요. 본인 인증을 위해 짧은 메시지에 서명하는 요청이 뜨는데, 트랜잭션이 아니라 가스비도 발생하지 않습니다.

비밀번호를 잊으셨다면 비밀번호 입력칸 아래의 **Forgot password? (비밀번호를 잊으셨나요?)** 를 누르면 재설정 링크가 메일로 발송됩니다.

> 💡 **팁:** 둘러보고, 글을 쓰고, 댓글을 다는 일상적인 활동에는 지갑이 필요 없습니다. 다만 일부 기능 (크리덴셜, 추후 온체인 정산 등) 은 지갑이 있어야 동작해요. 지갑은 나중에 설정에서 추가할 수도 있습니다.

## 네트워크 연결 (온보딩)

가입 직후, Ratel 은 한 단계짜리 **연결(Connections)** 온보딩 페이지를 다음 위치에 안내합니다.

```
/onboarding/connections
```

이 페이지는 [게시글 → 크로스포스팅](./posts.md#크로스포스팅-cross-posting) 의 첫 실행용 친화 버전입니다. 크로스포스팅이 무엇인지 — *"첫 게시글 한 번에 세 네트워크에 도달합니다"* — 를 설명하고, 한 번 연결해 두면 Ratel 의 모든 게시물이 다른 SNS 타임라인으로 백링크와 함께 퍼져 나가도록 목적지별 카드를 줍니다.

온보딩 페이지의 각 카드:

| 플랫폼 | 오늘 상태 | 카드 내용 |
|---|---|---|
| <img src={useBaseUrl('/img/icons/bluesky.svg')} width="16" alt="Bluesky" style={{verticalAlign: 'middle'}} /> **Bluesky** | 사용 중 | AT Protocol · 300 자. 앱 비밀번호로 연결합니다 (`bsky.app/settings/app-passwords` 에서 발급한 후 Ratel 이 암호화 저장). |
| <img src={useBaseUrl('/img/icons/linkedin.svg')} width="16" alt="LinkedIn" style={{verticalAlign: 'middle'}} /> **LinkedIn** *(예정)* | OAuth 연동은 Phase 1B 에서 활성화됩니다 | 전문 네트워크 · 3,000 자. *Connect* 대신 *Notify me (출시 알림)* 버튼이 노출됩니다. |
| <img src={useBaseUrl('/img/icons/threads.svg')} width="16" alt="Threads" style={{verticalAlign: 'middle'}} /> **Threads** *(예정)* | Meta OAuth 연동은 Phase 1C 에서 활성화됩니다 | Meta · 약 2.75 억 명 · 500 자. 출시 시 인스타그램 프로페셔널 계정이 필요합니다. |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M15 8l-7 7-3-3"/><circle cx="12" cy="12" r="10"/></svg> **Farcaster** *(예정 — Phase 2)* | Frames 연동 + 에이전트 게시는 Phase 2 | 320 자 · Web3 네이티브. *Notify me* 버튼. |

페이지 상단의 **Skip for now → (나중에)** 를 누르면 온보딩이 닫히고, Ratel 은 다시 재촉하지 않습니다. 나중에 다시 보고 싶을 때는 `/<your-handle>/settings/connections` 의 [게시글 → 크로스포스팅](./posts.md#크로스포스팅-cross-posting) 을 열면 돼요. 온보딩 페이지에서 이미 연결한 항목은 *Skip* 을 눌러도 그대로 저장되어 있습니다.

> **페이지에 등장하는 팁:** *"2 개 이상의 네트워크를 연결한 크리에이터는 첫 30 일 동안 구독자가 3.4 배 더 빠르게 늘어납니다."*

## 프로필 설정

가장 먼저 정할 것은 **핸들 (handle)** 입니다. 핸들은 Ratel 안에서 여러분의 집 주소 같은 역할을 해요. 프로필, 게시물, 스페이스, 드래프트, 보상까지 모두 `/<your-handle>` 아래에 모입니다. 예를 들어 핸들이 `alex` 라면:

- `/alex` — 프로필 홈
- `/alex/posts` — 작성한 게시물
- `/alex/spaces` — 만든/참여한 스페이스
- `/alex/rewards` — 받은 보상
- `/alex/drafts` — 작성 중인 드래프트
- `/alex/settings` — 계정 & 프로필 설정

**/your-handle/settings** 에서 다음 항목을 채워 보세요.

- **Display name** — 게시물이나 댓글에 표시되는 이름
- **Avatar** — 프로필 사진 (이미지 업로드)
- **Banner** — 프로필 페이지 상단에 보이는 가로형 헤더 이미지
- **Bio** — 자신을 소개하는 한 줄 (어떤 글을 쓰는지 등)

다 적었으면 **Save** 를 누르세요. 변경 사항은 즉시 반영됩니다.

## 외부 계정 연결

Ratel 은 여러분이 이미 쓰고 있는 다른 플랫폼들과도 연결됩니다. **/your-handle/settings/connections** 에서 관리할 수 있어요. 모든 연결은 **OAuth** 방식이므로, 인증은 해당 플랫폼에서 직접 진행되고 Ratel 은 비밀번호를 절대 보지 않습니다. 연결 해제도 같은 페이지에서 언제든 가능합니다.

| 아이콘 | 플랫폼 | 용도 |
|---|---|---|
| <img src={useBaseUrl('/img/icons/bluesky.svg')} width="16" alt="Bluesky" style={{verticalAlign: 'middle'}} /> | **Bluesky** | Ratel 게시물을 Bluesky 로 크로스포스트 |
| <img src={useBaseUrl('/img/icons/linkedin.svg')} width="16" alt="LinkedIn" style={{verticalAlign: 'middle'}} /> | **LinkedIn** *(planned / 예정)* | LinkedIn 피드로 크로스포스트 |
| <img src={useBaseUrl('/img/icons/threads.svg')} width="16" alt="Threads" style={{verticalAlign: 'middle'}} /> | **Threads** *(planned / 예정)* | Threads 로 크로스포스트 |
| <img src={useBaseUrl('/img/icons/notion.svg')} width="16" alt="Notion" style={{verticalAlign: 'middle'}} /> | **Notion** *(planned / 예정)* | Notion 문서를 Essence 의 소스로 가져오기 |

연결하고 싶은 플랫폼 옆의 **Connect** 버튼을 누르고 OAuth 안내를 따르면 됩니다. 연결한 뒤에는:

- **자동 게시 (auto-post)** 를 플랫폼별로 켜고 끌 수 있어요 (새 글을 자동으로 송출할지)
- 언제든 **Disconnect** 로 연결을 끊을 수 있어요. 모든 연결은 사용자별이며 철회 가능합니다.

> 💡 **팁:** 처음 가입한 분이라면 `/onboarding/connections` 에서 연결 안내 스텝을 만나게 됩니다. 지금 안 해도 괜찮으니 부담 없이 건너뛰고 나중에 다시 와도 됩니다.

## 화면 둘러보기

Ratel 의 주요 영역은 데스크톱과 모바일 모두 동일하지만 배치만 다릅니다.

- **상단 네비게이션** — 검색, 알림, 프로필 메뉴 (오른쪽 위). 프로필 메뉴에서 내 프로필, 드래프트, 설정, 로그아웃으로 이동할 수 있습니다.
- <img src={useBaseUrl('/img/icons/home.svg')} width="18" height="18" alt="Home" style={{verticalAlign: 'middle'}} /> **사이드바 / 왼쪽 레일** — **Home**, **Membership**, **Credentials**, **Rewards**, **Settings** 와 푸터의 언어/테마 토글로 빠르게 이동합니다. 맨 아래 접기 버튼 (<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M9 3v18"/><path d="m16 15-3-3 3-3"/></svg>) 으로 사이드바를 풀폭과 아이콘만 보이는 상태로 전환할 수 있어요.
- <img src={useBaseUrl('/img/icons/bell.svg')} width="18" height="18" alt="Bell" style={{verticalAlign: 'middle'}} /> **알림** — 종 모양 아이콘에서 답글, 멘션, 팔로우 요청, 스페이스 초대, 보상 업데이트 등 읽지 않은 활동을 확인할 수 있어요.
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg> **테마 토글** — **다크 (기본)**, **라이트**, **시스템** 모드 사이를 전환합니다. 선택은 페이지/기기 간에 유지됩니다.
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="12" cy="12" r="10"/><path d="M2 12h20"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg> **언어 토글** — **한국어** 와 **English** 를 지원합니다. 사이드바 푸터의 국기 아이콘으로 언어를 전환할 수 있어요.

모바일에서는 사이드바가 메뉴 버튼 안으로 들어가고 상단 네비게이션은 그대로 유지됩니다. 글쓰기·투표·스페이스 참여·알림 확인 등 모든 기능은 이동 중에도 동일하게 사용할 수 있어요.

## 알림 챙기기

상단 내비게이션의 <img src={useBaseUrl('/img/icons/bell.svg')} width="16" alt="알림" style={{verticalAlign: 'middle'}} /> **종 아이콘** 을 클릭하면 알림 패널이 열립니다. 새 항목이 있으면 아이콘에 미확인 카운트 뱃지가 표시됩니다 (100+ 이상은 100+ 로 표기).

패널 안에서:

- **최신순** — 받은편지함은 페이지네이션됩니다. 스크롤로 과거 항목을 더 끌어올려 보세요.
- **항목 클릭** — 해당 항목이 가리키는 곳 (게시글, 댓글 스레드, 스페이스 액션, 보상 이벤트) 으로 이동하면서 항목을 읽음으로 처리합니다.
- **모두 읽음 처리(Mark all as read)** — 패널 상단의 버튼 한 번으로 뱃지가 즉시 비워집니다.
- **읽지 않은 항목만 보기** — 토글로 이미 읽은 항목을 숨기고 새 항목에 집중할 수 있어요.

알림은 게시글에 달린 답글, 본인 핸들 멘션, 새 팔로우 관계, 참여 중인 스페이스의 초대 / 새 액션, 하위팀 신청 업데이트, 보상 이벤트 (사이클 락인, 청구) 에 대해 발생합니다. 이메일 / 푸시 전송 환경설정은 *(예정)* 이며, 오늘은 종 아이콘이 정식 알림 화면입니다.

## 첫날 체크리스트

10 분만 투자해서 아래 항목들을 처리하면 Ratel 이 한결 익숙해질 거예요.

- [ ] **핸들 정하고 프로필 채우기** — display name, 아바타, 배너, 소개
- [ ] **첫 게시물 쓰기** — `/posts` 에서 아이디어·질문·링크를 자유롭게 공유
- [ ] **다른 글에 좋아요 누르고 한 명 팔로우하기** — 좋아요와 팔로우는 추천과 Essence 형성에 모두 영향을 줍니다
- [ ] **스페이스 둘러보거나 참여하기** — 스페이스는 폴, 토론, 퀴즈, 팔로우 퀘스트 같은 공동 작업이 일어나는 곳
- [ ] **액션 한 가지 참여하기** — 폴에 투표하거나, 퀴즈를 풀거나, 토론에 의견 남기기
- [ ] **외부 계정 하나 연결하기** — `/your-handle/settings/connections` 에서 Bluesky 부터 시작해 보세요
- [ ] **내 Essence 살펴보기** — `/essence` 에서 지금 내 Essence 에 들어가는 모든 소스를 확인할 수 있어요.

여기까지 마치면 기본 세팅 완료입니다. 다음 챕터에서는 **글쓰기**와 **스페이스 참여**를 더 자세히 다룰게요.

> 💡 **팁:** 막히는 부분이 있다면 사이드바의 **Troubleshooting** 을 먼저 살펴보세요. 그래도 풀리지 않으면 프로필 메뉴의 **Help** 를 통해 알려주시면 됩니다.
