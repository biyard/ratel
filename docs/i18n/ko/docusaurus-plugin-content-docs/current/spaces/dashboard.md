---
sidebar_position: 4
title: 스페이스 대시보드 (호스트)
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# 스페이스 대시보드

대시보드는 호스트의 *"내 스페이스, 오늘 어떻게 돌아가고 있나"* 화면입니다 — 스탯 카드 한 묶음과 참여자 랭킹 표가 한 화면에 모여 있고, 사람들이 활동할수록 라이브로 갱신됩니다. 위치는 다음과 같습니다.

```
/spaces/:space_id/dashboard
```

[스페이스 탭](./#스페이스-안의-탭) 에서 열거나 URL 을 직접 붙여넣어 접근합니다. 페이지는 역할에 따라 두 뷰로 분기됩니다. **호스트(Creator)** 는 호스트 전용 안내가 포함된 전체 대시보드를 보고 (예: 펀딩 안 된 인센티브 풀 카드의 *지금 설정* 버튼), **참여자와 일반 방문자** 는 같은 카드를 읽기 전용으로 보고 자신의 랭킹 행을 찾을 수 있습니다. 분기는 자동입니다 — 같은 URL, 다른 컨트롤.

## 대시보드가 보여주는 것

페이지는 한 컬럼의 스크롤 영역입니다. 카드는 화면 크기에 따라 자동으로 행을 구성하며 (데스크톱 4 열, 태블릿 2 열, 모바일 1 열), **랭킹** 표는 페이지 맨 아래에 풀폭으로 배치됩니다.

### 카드 그리드

그리드에는 다섯 가지 카드 변형이 렌더링됩니다. 각 카드는 상단의 한눈에 보이는 헤드라인 숫자 + 아래의 보조 지표 목록으로 구성돼요. 여섯 번째 — **랭킹 표** — 는 그리드 아래에 풀폭으로 별도 섹션으로 배치됩니다.

| 카드 | 헤드라인 | 무엇을 보여주는가 |
|---|---|---|
| <img src={useBaseUrl('/img/icons/users.svg')} width="16" alt="참여자" style={{verticalAlign: 'middle'}} /> **스페이스 뷰** | 총 참여자 | 총 참여자 · 총 좋아요 · 총 댓글 · 총 액션 |
| <img src={useBaseUrl('/img/icons/award.svg')} width="16" alt="풀" style={{verticalAlign: 'middle'}} /> **인센티브 풀** | 풀 총 규모 | 총 수상자 · 순위 비율 · 인센티브 풀 잔액 |
| <img src={useBaseUrl('/img/icons/grid.svg')} width="16" alt="액션" style={{verticalAlign: 'middle'}} /> **참여 액션** | 모든 유형의 총 액션 수 | 유형별 완료 진행률 (Poll · Discussion · Quiz · Follow), 각 항목은 `count / total` 과 진행 바로 표시 |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><line x1="18" y1="20" x2="18" y2="10"/><line x1="12" y1="20" x2="12" y2="4"/><line x1="6" y1="20" x2="6" y2="14"/></svg> **총 참여자** | 시간에 따른 참여자 수 | 액션 유형별 참여 추이를 보여주는 탭형 차트 |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="12" cy="8" r="7"/><polyline points="8.21 13.89 7 23 12 20 17 23 15.79 13.88"/></svg> **사용 가능 포인트** | 분배 가능한 총 포인트 | 스페이스가 분배에 쓸 수 있는 포인트의 액션별 내역 |

호스트로서 스페이스에 **인센티브 풀이 아직 설정되지 않았다면**, 풀 카드에 보라색 **지금 설정** 버튼이 표시되어 `/spaces/:space_id/apps/` 로 이동하여 [인센티브 풀 앱 *(베타)*](./apps#-incentive-pool-%EB%B2%A0%ED%83%80) 을 설치하도록 안내합니다. 같은 카드를 보는 참여자에게는 이 버튼이 보이지 않아요.

그리드는 상황에 따라 달라집니다 — 어떤 앱을 설치했는지, 얼마나 활동이 쌓였는지에 따라 표시되는 카드가 달라져요. 갓 만든 스페이스는 대부분 0 이고, 사이클 중반의 활성 스페이스는 모든 카드가 채워집니다.

### 랭킹 표

페이지 하단의 풀폭 **랭킹** 표는 이 스페이스에서 점수를 쌓은 모든 참여자를 보여줍니다. 컬럼은 세 개:

| 컬럼 | 정렬 가능 | 무엇을 보여주는가 |
|---|---|---|
| **순위(Rank)** | 아니오 — 시각 표시만 *(예정)* | 리더보드에서의 위치, 라이브로 재계산. 헤더 옆에 정렬 글리프가 보이지만 클릭으로 정렬하는 동작은 로드맵에 있습니다. |
| **참여자(Participant)** | — | 아바타 + 디스플레이 네임. |
| **점수(Score)** | 아니오 — 시각 표시만 *(예정)* | 이 스페이스에서 누적한 점수 (`i` 인포 툴팁이 무엇이 카운트되는지 설명). |

표 하단의 페이지 단위 페이저로 추가 페이지를 넘길 수 있습니다 — 리더보드가 깊은 스페이스에 유용해요.

> **팁.** 호스트는 [리포트](./reports) 를 만들기 전에 랭킹 표로 핵심 기여자를 먼저 파악합니다. 참여자 점수는 Phase 4 매출 분배가 출시되면 *(예정)* 기여자 몫의 가중치로 활용됩니다.

## 대시보드 / 오버뷰 / 리포트, 언제 무엇을?

스페이스에는 호스트가 쓰는 네 개의 탭이 있고, 약간 겹쳐 보일 수 있어서 빠른 안내를 정리했습니다.

| 탭 | URL | 언제 열까 |
|---|---|---|
| <img src={useBaseUrl('/img/icons/grid.svg')} width="14" alt="대시보드" style={{verticalAlign: 'middle'}} /> **대시보드** | `/spaces/:space_id/dashboard` | **라이브 숫자** 가 필요할 때 — 참여자 수, 액션 수, 풀 소진 상황. 활성 캠페인 중에는 하루에 몇 번씩 새로고침하기 좋아요. |
| <img src={useBaseUrl('/img/icons/file-text.svg')} width="14" alt="오버뷰" style={{verticalAlign: 'middle'}} /> **오버뷰** | `/spaces/:space_id/overview` | **내러티브를 편집** 할 때 — 이 스페이스가 무엇이고, 누구를 위한 곳이고, 왜 참여해야 하는지. 호스트가 직접 쓰는 호스트의 피치예요. |
| <img src={useBaseUrl('/img/icons/file-text.svg')} width="14" alt="리포트" style={{verticalAlign: 'middle'}} /> **리포트** | `/spaces/:space_id/report` | 활동을 종합한 **장문의 AI 보조 리포트를 발행** 할 때. [리포트](./reports) 챕터 참고. |
| <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="12" cy="12" r="10"/><polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76"/></svg> **인덱스(포털)** | `/spaces/:space_id/` | 처음 방문하는 사람이 도착하는 **공개 포털 화면을 확인** 할 때. 외부에 어떻게 보이는지 점검할 때 유용해요. |

대시보드는 활동이 들어오면 자동으로 갱신되는 유일한 탭이고, 나머지 셋은 *호스트가 사람들에게 무엇을 말하는가* 와 관련된 탭입니다.

## 오늘 시점에 빠진 것

대시보드에 아직 없는 항목들 (각각 로드맵에 있음):

- **액션 단위 드릴다운** *(예정)*. 참여 액션 카드의 진행 바는 읽기 전용 요약입니다 — 바를 눌러 *"이 폴에 응답한 12 명은 누구인가"* 같은 드릴다운은 아직 없어요. 그 용도로는 [Analyzes 앱](./apps#-analyzes) 을 사용하세요.
- **기간 선택기** *(예정)*. 현재 숫자는 누적값입니다 — *"최근 7 일"* / *"이번 사이클"* 필터는 로드맵에 있습니다.
- **익스포트** *(예정)*. 대시보드 스냅샷의 CSV / PDF 익스포트. Analyzes 앱에는 분석별 엑셀 익스포트가 이미 있습니다.

## 다음 단계

- [스페이스 앱](./apps) — 대시보드의 보상 카드를 채우는 인센티브 풀을 설치하세요.
- [호스트 액션](./host-actions) — 진행 바를 채우는 폴 · 토론 · 퀴즈 · 팔로우를 만드세요.
- [리포트](./reports) — 대시보드 활동을 발행 가능한 내러티브로 바꾸세요.
