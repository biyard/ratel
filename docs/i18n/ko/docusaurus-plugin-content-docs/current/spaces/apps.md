---
sidebar_position: 2
title: 스페이스 앱 (호스트 도구)
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# 스페이스 앱 — 호스트 도구

스페이스의 호스트라면, **앱(App)** 은 스페이스를 만들기 위해 끼워 넣는 빌딩 블록입니다. 각 앱은 한 가지 일을 담당합니다 — 안내 게시, 파일 공유, 참여자에게 투표 요청, AI 보조 리포트 생성, 보상 분배 — 오늘 필요 없는 앱은 그냥 빼두면 됩니다.

앱 패널은 다음 URL 에서 접근할 수 있습니다:

```
/spaces/:space_id/apps
```

각 앱은 아래 섹션에 적힌 자체 URL 을 가지고 있어, 필요한 설정 화면으로 직접 딥링크할 수 있습니다.

:::tip 더하기 식 설계
모든 앱을 켜야 할 의무는 없습니다. 단순한 스페이스라면 **General** 과 **Files** 만 써도 충분합니다. 유료 리포트를 운영하는 플래그십 스페이스라면 **Analyzes** 와 **Incentive Pool** 을 함께 사용하게 됩니다.
:::

## <img src={useBaseUrl('/img/icons/settings.svg')} width="22" height="22" alt="Settings" style={{verticalAlign: 'middle'}} /> General

URL: `/spaces/:space_id/apps/general`

General 앱은 스페이스의 글 형식 콘텐츠가 사는 곳입니다. 참여자가 처음 보는 환영 페이지, 참여 규칙, 이미지/영상 임베드, 외부 링크 — 모두 이곳에 둡니다. 스페이스의 풍부 콘텐츠 백본 이라고 보면 됩니다 — 참여자가 참여 여부를 결정하기 전에 가볍게 훑게 되는 화면입니다.

미션 선언문, FAQ, 어젠다 개요, 행동 강령, 파트너 크레딧처럼 정적이고 오래 가는 자료에 General 을 사용하세요. 시간에 묶인 항목 (단일 투표, 일회성 미팅) 은 [Actions](/spaces/actions) 에 넣어, 액션 카루셀에서 참여자가 찾을 수 있도록 하세요.

## <img src={useBaseUrl('/img/icons/file.svg')} width="22" height="22" alt="File" style={{verticalAlign: 'middle'}} /> Files

URL: `/spaces/:space_id/apps/files`

Files 앱은 스페이스의 공용 자산 라이브러리입니다. PDF, 슬라이드 덱, CSV, 디자인 익스포트, 오디오 녹음 — 참여자가 활동 중에 참고할 자료라면 무엇이든 업로드할 수 있습니다. 첨부된 파일은 스페이스에 참여한 모든 사람에게 보이며, 추가/제거 권한은 호스트가 갖습니다.

Files 는 특히 *문서 한 건* 을 중심으로 스페이스를 운영할 때 유용합니다 — 초안 제안서, 연구 데이터셋, 명세서 — 모든 참여자가 같은 원본을 토대로 토론하거나 투표하도록 하고 싶을 때입니다.

## <img src={useBaseUrl('/img/icons/grid.svg')} width="22" height="22" alt="Grid" style={{verticalAlign: 'middle'}} /> Analyzes

:::note
현재 dev / staging 환경에서 사용할 수 있으며, 검증을 거쳐 프로덕션에 배포될 예정입니다.
:::

URL:
- 분석 목록 — `/spaces/:space_id/apps/analyzes`
- 새 분석 만들기 — `/spaces/:space_id/apps/analyzes/create`
- 분석 결과 보기 — `/spaces/:space_id/apps/analyzes/report/:report_id`
- 분석별 원본 레코드 — `/spaces/:space_id/apps/analyzes/report/:report_id/records`
- 단일 투표 드릴다운 — `/spaces/:space_id/apps/analyzes/poll/:poll_id`
- 단일 토론 드릴다운 — `/spaces/:space_id/apps/analyzes/discussion/:discussion_id`

Analyzes 앱은 Ratel 의 **교차 필터 분석 엔진** 입니다 — 장문 AI 내러티브 생성기가 *아닙니다*. (장문 AI 내러티브 쪽은 `/spaces/:space_id/report` 를 다루는 [리포트](./reports.md) 챕터에서 설명합니다.) 본인이 저장한 각 분석은 스페이스의 폴 · 퀴즈 · 팔로우 · 토론에 대한 저장된 **교차 필터** 이며, 매칭된 참여자별 레코드 드릴다운이 함께 제공됩니다.

### 일반적인 분석 흐름

1. **목록 보기** — `/spaces/:space_id/apps/analyzes` 의 수평 카루셀. 첫 카드는 항상 **+ 새 분석 만들기**, 그 오른쪽에는 이미 저장한 분석들. 상태 뱃지는 **분석 중(Running)** / **분석 완료(Analysis complete)** / **실패(Failed)** 입니다.
2. **만들기** — `/spaces/:space_id/apps/analyzes/create` 의 두 단계 빌더. 1 단계 (*교차 필터 선택*): 모든 폴 문항, 퀴즈 문항, 팔로우 타겟, 토론 스레드가 타일로 노출되고, 하나 이상 선택하면 각각이 칩이 됩니다. 쉼표로 구분된 **키워드** 도 지원돼요 (각 키워드가 별도 필터). 라이브 카운터가 현재 매칭되는 참여자 / 레코드 수를 보여줍니다. 2 단계 (*미리보기*): 분석명 입력, 칩과 매칭되는 원본 데이터 샘플 확인, **보고서 생성(Generate report)** 클릭.
3. **읽기** — `/spaces/:space_id/apps/analyzes/report/:report_id` 의 저장된 분석은 교차 필터 결과를 보여줍니다 — 폴 응답의 분포 차트, 퀴즈 점수 요약, 팔로우 카운트, 댓글 빈도 — 모두 칩 세트 범위로 좁혀진 상태로. 칩은 **저장 시점에 frozen** 되므로, 한 달 뒤에 다시 열어도 같은 슬라이스를 그대로 봅니다.
4. **원본 레코드 드릴다운** — `/spaces/:space_id/apps/analyzes/report/:report_id/records` 는 매칭된 모든 개별 레코드 (사용자 · 문항 · 응답 · 글 · 코멘트 · 팔로우 대상) 의 페이지네이션 표입니다. 칩을 누르면 추가로 좁힐 수 있고, 상단의 **엑셀 다운로드(Download Excel)** 버튼은 매칭 레코드를 스프레드시트로 내보냅니다.
5. **단일 소스 드릴다운** — 교차 필터가 필요 없을 때는 두 딥링크가 단일 소스 뷰로 바로 이동시킵니다 — `/apps/analyzes/poll/:poll_id` (폴 하나), `/apps/analyzes/discussion/:discussion_id` (토론 하나). 저장된 분석 안에서 사용하는 동일한 차팅 화면을 한 소스 범위로 좁힌 것입니다.

### 기여 기록과 Phase 4 매출 분배

`:report_id/records` 페이지는 **기여 기록(contribution records)** 화면이기도 합니다 — 각 응답자의 활동이 저장한 분석에 어떻게 매칭됐는지를 참여자별로 분해해 보여줍니다. 이것이 Phase 4 매출 분배의 근거가 됩니다 — 유료 [리포트](./reports.md) 를 발행하면 매출은 **플랫폼 10% · 호스트 60% · 기여자 30%** 로 나뉘고, 기여자 몫은 최종 리포트에 대한 관련도 가중치로 분배됩니다.

:::note 예정
Phase 4 의 풀 매출 분배 엔진 (온체인 정산 옵트인 포함) 은 에이전트 경제와 함께 점진적으로 출시됩니다. 분석 자체는 오늘 dev / staging 에서 사용할 수 있고, 수익화 레이어는 호스트 대시보드에 출시되는 대로 나타납니다.
:::

## <img src={useBaseUrl('/img/icons/users.svg')} width="22" height="22" alt="Users" style={{verticalAlign: 'middle'}} /> Panels

URL: `/spaces/:space_id/apps/panels`

Panels 앱은 *맞는 사람* 에게 도달하기 위한 도구입니다. 관심사, 평판 등급, 과거 참여 이력, 다른 스페이스에서의 멤버십 등으로 타겟 오디언스를 빌드하고, 맞춤 초대를 보낼 수 있습니다. 또한 액션을 *모든 사람* 에게가 아니라 일부 참여자에게만 노출하고 싶을 때, 그 범위를 정하는 수단이기도 합니다.

스페이스의 가치가 *누가 오는가* 에 달린 경우 가장 먼저 손이 가는 앱입니다 — 전문가 리뷰, 배심형 토론, 멤버 게이팅 투표, 파트너 한정 아웃리치.

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M8 19c.53 0 1.04-.21 1.41-.59.38-.37.59-.88.59-1.41V9c0-1.06-.42-2.08-1.17-2.83C8.08 5.42 7.06 5 6 5M6 5c-1.06 0-2.08.42-2.83 1.17C2.42 6.92 2 7.94 2 9v8c0 .53.21 1.04.59 1.41.37.38.88.59 1.41.59h16c.53 0 1.04-.21 1.41-.59.38-.37.59-.88.59-1.41V9c0-1.06-.42-2.08-1.17-2.83C19.92 5.42 18.94 5 18 5H6zM2 11h20M16 11v3"/></svg> Incentive Pool (베타)

URL: `/spaces/:space_id/apps/incentive-pool`

베타 빌드에서 사용할 수 있으며, 곧 프로덕션 빌드로 순차 출시됩니다.

Incentive Pool 앱은 액션 완료자가 받게 될 보상 풀을 펀딩하고 설정하는 곳입니다. 풀 사이즈, 액션당 배분, 분배 규칙 — 균등 분할, 참여 깊이 가중, 스페이스 단계별 분배 — 을 정합니다.

참여자가 보상이 걸린 액션을 완료하면, 이 풀에서 자동으로 몫이 빠져 지급됩니다. 풀 크기, 현재 잔액, 분배 이력이 모두 같은 화면에 노출되어, 보상 경제를 한 화면에서 감사할 수 있습니다.

:::tip Analyzes 와 짝지어 쓰세요
유료 리포트를 발행하는 스페이스는 보통 Incentive Pool 도 함께 운영합니다 — 풀이 참여를 끌어내고, 그 결과로 만들어진 리포트의 매출이 다시 기여자에게 분배됩니다.
:::

:::note 호스트 보상에 관한 안내
호스트 보상 분배는 사용자 보상 페이지(`/your-handle/rewards`)에서 추적합니다 — 오늘 기준으로는 별도의 호스트용 Rewards 앱이 없습니다. 발행한 리포트의 호스트 몫, 플랫폼 분배 보너스, 본인 계정으로 라우팅된 보상 크레딧을 본인이 호스트하는 모든 스페이스를 합쳐 한 화면에서 확인할 수 있습니다.
:::

## 함께 묶어 쓰기

보상이 걸린 전형적인 스페이스는 다음 앱들을 함께 사용합니다:

1. **General** 로 프레이밍과 규칙을 게시합니다.
2. 호스트가 **액션** (투표, 토론, 퀴즈, 팔로우, 미팅 — [스페이스 액션](/spaces/actions) 참고) 을 추가합니다.
3. **Incentive Pool** 이 보상을 펀딩하고, **Analyzes** 가 활동을 리포트로 변환합니다.

모든 앱은 선택 사항이지만, 함께 쓰면 참여자 커뮤니티를 발행 가능하고 수익화 가능한 산출물로 바꾸는 생산 라인이 됩니다 — 그 산출물을 만든 사람들에 대한 연결고리를 잃지 않으면서.
