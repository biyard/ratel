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

General 앱은 스페이스의 **운영 설정** 화면입니다. Action 이 아닌, 호스트가 제어해야 하는 항목 — 브랜딩, 참여 자격, 관리자, 그리고 (맨 아래) "스페이스 삭제" 등 — 이 모두 이곳에 모여 있습니다. 모든 변경은 자동 저장되며, 저장이 끝나면 sticky footer 가 초록색 **Synced** 배지로 바뀝니다. 페이지 어디에도 별도 저장 버튼은 없습니다.

페이지는 하나의 스크롤 아레나로, 다음 섹션들이 순서대로 배치되어 있습니다:

1. **스페이스 로고 (Space logo)** — 정사각형 이미지 (1:1) 를 업로드하면 상단바 타일 및 스페이스로 연결되는 모든 리스트 / 카드 화면의 Ratel 심볼이 교체됩니다. 업로드되지 않았다면 스페이스 제목에서 추출한 2 글자 이니셜 placeholder 가 사용됩니다.
2. **시작 시간 (Start time)** — 스페이스가 "공식적으로" 시작되는 시점을 datetime picker 로 설정합니다. 기본값은 스페이스 생성 시점이며, 명시적인 시작 시간을 설정하면 발견 화면에서 스페이스의 배지가 _Draft_ → _Scheduled / Live_ 로 바뀝니다.
3. **공개 여부 (Visibility)** — **Public** (링크가 있는 누구나 스플래시에 도달) 또는 **Private** (초대받은 참여자와 로그인한 관리자만 접근) 중 선택합니다. 스위치는 라디오 카드 형태이며, 활성 카드는 `aria-selected` 로 표시됩니다.
4. **참여자 초대 (Invite participant)** — 이메일을 쉼표로 구분 (또는 Enter 키로 청크 분리) 해 입력하고, 칩으로 미리 본 뒤 발송합니다. 아래 목록에는 모든 미수락 초대가 상태 (`Pending` / `Accepted` 등) 와 함께 노출되며, 한 번 클릭으로 초대를 취소할 수 있습니다. 스크롤하면 추가 초대가 페이지네이션으로 로드됩니다.
5. **익명 참여 (Anonymous participation)** — 토글. 켜면 이 스페이스 내에서의 투표와 댓글이 익명 핸들로 기록됩니다 (참여자 본인의 신원은 Essence 파이프라인에는 그대로 보존되지만, 다른 참여자에게는 노출되지 않습니다).
6. **언제든지 참여 (Join anytime)** — 토글. 켜면 스페이스가 라이브가 된 이후에도 누구나 참여할 수 있습니다. 끄면 참여는 초대받았거나 사전에 승인된 패널로 한정됩니다.
7. **관리자 (Administrators)** — 다른 Ratel 사용자를 username 으로 추가합니다 (초대 흐름과 동일하게 쉼표 구분 또는 Enter 키로 청크 분리). 각 관리자는 제거 버튼이 달린 칩으로 노출됩니다. "추가" 입력란은 기존 관리자에게만 보이며, 비관리자는 read-only 리스트만 봅니다.
8. **Danger zone** — 빨간색 **Delete space** 버튼 하나. 인라인 아레나 모달이 열리며 확인을 요구합니다. 확인하면 스페이스 · 모든 액션 · 댓글 · 풀 자금이 영구 삭제됩니다.

:::tip 풍부한 글 콘텐츠는 어디로?
General 은 긴 환영 페이지나 선언문을 작성하는 곳이 **아닙니다** — 그건 Overview 패널 (스페이스 아레나의 상단바 file-text 아이콘) 의 역할입니다. General 은 스위치와 리스트, Overview 는 내러티브입니다.
:::

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/general.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
브라우저가 비디오 임베드를 지원하지 않습니다. <a href={useBaseUrl('/media/general.mov')}>워크스루 다운로드</a>.
</video>

## <img src={useBaseUrl('/img/icons/file.svg')} width="22" height="22" alt="File" style={{verticalAlign: 'middle'}} /> Files

URL: `/spaces/:space_id/apps/files`

Files 앱은 스페이스 다른 곳 (Overview 내러티브, Discussion 보드, Quiz 문항) 에 이미 첨부된 모든 파일을 모아 보여주는 **읽기 전용 집계 뷰** 입니다. 업로드 화면이 _아닙니다_ — 이 페이지에는 "파일 추가" 버튼이 없고, 파일은 원래 첨부된 화면에서 업로드되면 자동으로 여기에 뜹니다.

상단부터 차례대로 아레나 구성:

1. **탭 필터** — segmented 컨트롤 **All · Overview · Boards · Quiz**. 각 탭은 그 출처에서 첨부된 파일로 리스트를 좁힙니다. 활성 탭은 `aria-selected`. 우측 카운터 (`N files`) 는 현재 탭의 표시 개수이고, sticky footer 는 전체 탭 합계를 별도로 보여줍니다.
2. **파일 리스트** — 파일당 카드 하나, 표시 항목:
   - 타입별 색상이 적용된 확장자 배지 (`JPG`, `PNG`, `PDF`, `ZIP`, `DOC`, `PPT`, `XLS`, `MP4`, `MOV`, `MKV` — 이 10 개 확장자가 지원되는 전부)
   - 파일명과 사이즈
   - 출처 태그 칩 (해당 시) — **Overview** / **Board** / **Quiz** — 어느 화면에 첨부된 파일인지 알려줘서 컨텍스트로 돌아가기 쉽게 합니다
   - 카드를 클릭하면 새 탭에서 파일이 열립니다 (`target="_blank"`). URL 이 없는 파일은 카드가 렌더되지만 클릭은 동작하지 않습니다
3. **이미지 미리보기** — 자동 렌더링 그리드 섹션. 현재 탭에 JPG/PNG 가 하나 이상 있을 때만 노출. 썸네일마다 확장자 라벨이 붙습니다.
4. **비디오 미리보기** — 자동 렌더링 단일 컬럼 섹션 (16:7 종횡비, 네이티브 HTML5 컨트롤). 현재 탭에 MP4/MOV/MKV 가 하나 이상 있을 때만 노출.
5. **빈 상태 (Empty state)** — 활성 탭에 매칭되는 파일이 없을 때 표시. 폴더 아이콘 + "No files yet" 헤드라인 + "스페이스에서 공유된 파일이 이곳에 표시됨" 안내.

:::tip 실제로 어디에 업로드하나요?
참여자가 참고할 파일을 추가하려면 Overview 패널 (스페이스에 동봉되는 선언문/명세서), Discussion 보드 내부, 또는 Quiz 문항에 첨부하세요. Files 앱은 그렇게 첨부된 파일들을 한 화면에 모아서 둘러볼 수 있게 해줍니다.
:::

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/file.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
브라우저가 비디오 임베드를 지원하지 않습니다. <a href={useBaseUrl('/media/file.mov')}>워크스루 다운로드</a>.
</video>

## <img src={useBaseUrl('/img/icons/grid.svg')} width="22" height="22" alt="Grid" style={{verticalAlign: 'middle'}} /> Analyzes

:::note
현재 dev / staging 환경에서 사용할 수 있으며, 검증을 거쳐 프로덕션에 배포될 예정입니다.
:::

URL:

- 분석 목록 — `/spaces/:space_id/apps/analyzes`
- 새 분석 만들기 — `/spaces/:space_id/apps/analyzes/create`
- 분석 결과 보기 — `/spaces/:space_id/apps/analyzes/report/:report_id`
- 분석별 원본 레코드 — `/spaces/:space_id/apps/analyzes/report/:report_id/records`

Analyzes 앱은 Ratel 의 **교차 필터 분석 엔진** 입니다 — 장문 AI 내러티브 생성기가 _아닙니다_. (장문 AI 내러티브 쪽은 `/spaces/:space_id/report` 를 다루는 [리포트](./reports) 챕터에서 설명합니다.) 본인이 저장한 각 분석은 스페이스의 폴 · 퀴즈 · 팔로우 · 토론에 대한 저장된 **교차 필터** 이며, 매칭된 참여자별 레코드 드릴다운이 함께 제공됩니다.

### 일반적인 분석 흐름

1. **목록 보기** — `/spaces/:space_id/apps/analyzes` 의 수평 카루셀. 첫 카드는 항상 **+ 새 분석 만들기**, 그 오른쪽에는 이미 저장한 분석들. 상태 뱃지는 **분석 중(Running)** / **분석 완료(Analysis complete)** / **실패(Failed)** 입니다.
2. **만들기** — `/spaces/:space_id/apps/analyzes/create` 의 두 단계 빌더. 1 단계 (_교차 필터 선택_): 모든 폴 문항, 퀴즈 문항, 팔로우 타겟, 토론 스레드가 타일로 노출되고, 하나 이상 선택하면 각각이 칩이 됩니다. 쉼표로 구분된 **키워드** 도 지원돼요 (각 키워드가 별도 필터). 라이브 카운터가 현재 매칭되는 참여자 / 레코드 수를 보여줍니다. 2 단계 (_미리보기_): 분석명 입력, 칩과 매칭되는 원본 데이터 샘플 확인, **보고서 생성(Generate report)** 클릭.
3. **읽기** — `/spaces/:space_id/apps/analyzes/report/:report_id` 의 저장된 분석은 교차 필터 결과를 네 개 패널 (폴 · 퀴즈 · 팔로우 · 토론) 로 보여줍니다 — 모두 칩 세트 범위로 좁혀진 상태로. 칩은 **저장 시점에 frozen** 되므로, 한 달 뒤에 다시 열어도 같은 슬라이스를 그대로 봅니다.
4. **원본 레코드 드릴다운** — `/spaces/:space_id/apps/analyzes/report/:report_id/records` 는 매칭된 모든 개별 레코드 (사용자 · 문항 · 응답 · 글 · 코멘트 · 팔로우 대상) 의 페이지네이션 표입니다. 사이드바의 칩을 클릭하면 레코드 범위를 더 좁힐 수 있습니다.

소스별 데이터 (폴 분포, 퀴즈 정답률, 팔로우 타겟, 토론 토픽) 는 모두 **저장된 분석 리포트의 네 패널 안에** 노출됩니다 — 오늘 기준으로 단일 소스만 따로 여는 standalone 드릴다운 페이지는 없습니다.

:::note 아직 출시 안 됨

- 레코드 페이지의 스프레드시트 (Excel / CSV) 내보내기.
- 단일 소스 standalone 드릴다운 페이지 (폴 · 퀴즈 · 팔로우 · 토론). 모든 소스별 화면은 저장된 분석 리포트 안에서 봅니다.
  :::

### 기여 기록 — 향후 수익화의 데이터 레이어

`:report_id/records` 페이지는 **기여 기록(contribution records)** 화면이기도 합니다 — 각 응답자의 활동이 저장한 분석에 어떻게 매칭됐는지를 참여자별로 분해해 보여줍니다. 유료 리포트가 출시될 때 Phase 4 매출 분배가 이 데이터 위에서 동작합니다. 분배 방식은 [리포트 → Phase 4 매출 분배](./reports#phase-4--revenue-split-coming-soon) 에서 다룹니다 — 계획된 비율은 **플랫폼 10% · 호스트 60% · 기여자 30%**.

:::note 예정
유료 리포트와 온체인 매출 분배 엔진은 아직 출시되지 않았습니다. 분석 자체와 기여 기록 페이지는 오늘 dev / staging 에서 사용할 수 있고, 수익화 레이어는 호스트 대시보드에 출시되는 대로 나타납니다.
:::

## <img src={useBaseUrl('/img/icons/users.svg')} width="22" height="22" alt="Users" style={{verticalAlign: 'middle'}} /> Panels

URL: `/spaces/:space_id/apps/panels`

Panels 앱은 스페이스의 **인구통계 쿼터(quota) 설계** 화면입니다 — 설문 리서처가 "총 N 명을 모집할 거고, 연령·성별·학교 기준으로 이렇게 나눠서 받겠다" 고 선언할 때 쓰는 화면입니다. 연락처 리스트 / 초대 도구가 _아니라_, 유료 액션과 Incentive Pool 이 보상을 지급할 패널의 _구성 비율_ 을 정의하는 곳입니다. General 과 마찬가지로 모든 변경은 자동 저장되며, sticky footer 가 저장 중에는 **Saving…**, 저장 완료 시에는 **Saved** 로 전환됩니다.

이 아레나는 **Creator 전용** 입니다. 관리자 · 뷰어 · 참여자가 `/apps/panels` 에 진입하면 "no access" placeholder 와 Back 버튼만 보입니다 — 스페이스의 Creator 만 패널을 편집할 수 있습니다.

상단부터 차례대로:

1. **Total quotas (전체 쿼터)** — 총 응답자 수 정수 입력 한 칸 + 우측 _allocated / unassigned_ 미터. 미터는 아래 conditional table 의 쿼터 합계를 보여주며, unassigned 바는 아직 분배되지 않은 잔여분입니다.
2. **Attribute groups (속성 그룹)** — **University · Age · Gender** 세 개의 토글 카드. 카드를 켜면 해당 속성이 활성화되고, 끄면 서버 측에서 패널 행을 재구성합니다. 활성 카드는 `aria-selected="true"` 로 렌더됩니다.
3. **Collective panel (집합 패널)** _(속성 ≥1 개가 collective 모드일 때만 노출)_ — 현재 collective 상태인 속성을 chip 으로 보여줍니다. 섹션 헤더의 `+` 버튼은 드롭다운을 열어, 활성화된 속성 (총 쿼터 > 0 일 때 Age 또는 Gender) 을 conditional table 로 **승격(promote)** 시킬 수 있게 합니다.
4. **Conditional table (조건부 테이블)** _(속성 ≥1 개가 conditional 모드일 때만 노출)_ — 셀 단위 쿼터 표. 행은 축으로 분류됩니다: 순수 Age (`--age`), 순수 Gender (`--gen`), 순수 University (`--uni`), 또는 Age × Gender 복합행 (`--mix`). 각 행마다 자체 쿼터 정수를 가지며, 그 합이 1번 섹션의 _allocated_ 카운트로 반영됩니다.

요약하면: 2번에서 이 스페이스 패널에서 **어떤** 인구통계 축이 의미를 가질지 고르고, 3번은 "이 속성에서는 모두를 동일하게 셈" 하는 단순 형태를 유지하며, 4번은 "여성 18-24 세 30 명, 남성 25-34 세 50 명, …" 같은 정밀 모집이 필요할 때 사용합니다. Incentive Pool / Reward 앱은 그렇게 실제로 모집된 패널에 대해 보상을 지급합니다.

:::note 아직 구현 안 됨
현재 Panels 앱에는 "초대 보내기" 나 "다른 스페이스에서 가져오기" 흐름이 없습니다 — 관심사 태그, 평판 등급, 다른 스페이스 멤버십 필터, 액션별 오디언스 범위 지정 모두 _미구현_ 입니다. 현 빌드는 인구통계 쿼터 (학교 / 연령 / 성별) 만 모델링합니다.
:::

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/panel.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
브라우저가 비디오 임베드를 지원하지 않습니다. <a href={useBaseUrl('/media/panel.mov')}>워크스루 다운로드</a>.
</video>

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
