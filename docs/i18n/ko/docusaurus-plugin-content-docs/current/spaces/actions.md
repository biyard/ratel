---
sidebar_position: 3
title: 스페이스 액션
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# 스페이스 액션

[Apps](/spaces/apps) 가 호스트의 도구함이라면, **액션(Action)** 은 참여자의 퀘스트 보드입니다. 각 액션은 호스트가 본인에게 부탁하는 한 가지 일입니다 — 무언가에 투표하기, 토론에 의견 남기기, 퀴즈 풀기, 몇 명 팔로우하기, 미팅에 RSVP — 그리고 액션을 끝내면 스페이스 보상의 한 몫을 가져가면서, 본인이 무엇을 생각하는지에 대한 구조화된 신호가 Essence 에 쌓입니다.

## 액션 카루셀

URL:

```
/spaces/:space_id/actions
```

Actions 페이지는 퀘스트 카드의 **카루셀** 입니다. 카드 한 장이 액션 하나에 해당하고, 가로로 스크롤하면 화면 정중앙에 오는 카드가 활성 카드가 됩니다 — 더 크고 더 또렷하게. 양옆 카드는 흐릿해지고 작아져서, 카루셀이 산만해지지 않으면서 다음에 무엇이 있는지 항상 알 수 있게 합니다.

각 카드에는 다음이 노출됩니다:

- 액션 **타입** (poll, discussion, quiz, follow, meet)
- 무엇을 묻는지 짧게 요약한 헤드라인
- 액션에 걸린 보상 (스페이스에 [Incentive Pool](./apps.md#-incentive-pool-베타) 이 설정돼 있다면)
- 이미 참여했는지, 했다면 무엇을 했는지

카드를 탭하면 그 액션의 전체 페이지가 열립니다. 완료 후 카루셀로 돌아오면 다음 카드가 순서대로 보입니다.

:::tip 모바일 친화
카루셀은 휴대폰에서 카드를 넘기는 감각으로 설계됐습니다 — scroll-snap, 까다로운 정밀도 없이. 지금 이 스페이스에서 *무엇을 할 수 있는지* 한눈에 보는 가장 빠른 길입니다.
:::

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"/></svg> Discussion (토론)

URL: `/spaces/:space_id/actions/discussions/:discussion_id/edit`

**토론(Discussion)** 액션은 호스트가 던진 주제에 본인의 의견을 더해 달라는 초대입니다. 프롬프트, 참여 규칙, 그리고 답변을 작성할 리치 텍스트 에디터를 보게 됩니다.

토론은 의도적으로 열린 형태입니다 — 스페이스가 *"당신은 어떻게 생각하나요?"* 를 정돈된 방식으로 묻는 방법이며, 호스트는 이후 그 답변을 분석하고 인용하고 (스페이스가 리포트를 켜둔 경우) 발행되는 내러티브에 포함시킬 수 있습니다. 본인의 답변은 본인 계정에 귀속되며, 이는 다음을 의미합니다:

- 호스트가 이 스페이스에서 만드는 모든 리포트의 본인 기여 기록에 반영됩니다.
- Essence 에 해당 주제에 대한 본인 입장이 명확한 신호로 쌓입니다.
- 스페이스가 참여에 대해 보상을 지급한다면 토론의 보상 몫을 받습니다.

작성 중에 다른 참여자의 답변을 함께 읽을 수 있지만, 본인이 먼저 게시한 후에야 다른 사람의 답변이 보이도록 호스트가 설정할 수도 있습니다 — 독립적인 의견을 모으도록 설계된 스페이스는 먼저 말한 사람에게 휘둘리지 않습니다.

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/discussion.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
  브라우저가 비디오 임베드를 지원하지 않습니다. <a href={useBaseUrl('/media/discussion.mov')}>워크스루 다운로드</a>.
</video>

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><polyline points="20 6 9 17 4 12"/></svg> Poll (투표)

URL: `/spaces/:space_id/actions/polls/:poll_id`

**투표(Poll)** 액션은 한 표입니다. 질문, 선택지, 그리고 (호스트 설정에 따라) 다른 사람들이 어떻게 투표했는지에 대한 실시간 집계가 보입니다.

투표는 가장 가벼운 참여 방식입니다 — 한 번 탭하면 끝. 단순함에도 불구하고, 대부분의 스페이스에서 일하는 핵심 도구입니다: 참여자에게 빠르고, 설계하기 쉽고, 리포트로 깔끔하게 집계됩니다.

투표 후에는 일반적으로 결과가 즉시 보입니다. 어떤 스페이스는 마감 시각이 지나기 전까지는 집계를 숨깁니다 (먼저 투표한 사람에 의해 결과가 편향되지 않도록) — 호스트가 투표별로 설정합니다. 어느 쪽이든 본인의 표는 본인 계정에 기록되며 보상 몫에 반영됩니다.

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/poll.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
  브라우저가 비디오 임베드를 지원하지 않습니다. <a href={useBaseUrl('/media/poll.mov')}>워크스루 다운로드</a>.
</video>

## <img src={useBaseUrl('/img/icons/file.svg')} width="22" height="22" alt="File" style={{verticalAlign: 'middle'}} /> Quiz (퀴즈)

URL: `/spaces/:space_id/actions/quizzes/:quiz_id`

**퀴즈(Quiz)** 액션은 정답 (또는 합격선) 이 있는 일련의 질문을 묻습니다. 활용 예: 온보딩 게이트, 인증 흐름, 지식 확인 평가, 스페이스의 더 깊은 참여 단계를 풀기 전 스킬 검증.

퀴즈는 채점되며, 점수가 **합격** 여부를 결정합니다. 합격선은 호스트가 설정합니다 — 어떤 경우는 단순 P/F, 어떤 경우는 백분율, 어떤 경우는 정답 최소 개수입니다. 합격하면 보상이 잠금 해제되고, 불합격이면 호스트 설정에 따라 재시도가 허용될 수도 있습니다.

퀴즈 결과는 Essence 에도 유용한 입력이 됩니다 — 본인이 *무엇을 생각하는지* 뿐 아니라 *무엇을 알고 있는지* 에 대한 신호이기 때문입니다.

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/quiz.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
  브라우저가 비디오 임베드를 지원하지 않습니다. <a href={useBaseUrl('/media/quiz.mov')}>워크스루 다운로드</a>.
</video>

## <img src={useBaseUrl('/img/icons/users.svg')} width="22" height="22" alt="Users" style={{verticalAlign: 'middle'}} /> Follow (팔로우)

URL: `/spaces/:space_id/actions/follows/:follow_id`

**팔로우(Follow)** 액션은 특정 계정이나 계정 묶음을 팔로우해 달라고 부탁합니다. 호스트는 이를 통해 관련 크리에이터의 오디언스를 키우거나, 연결된 서브 커뮤니티를 빌드하거나, 스페이스가 중요하게 생각하는 목소리들을 본인 피드에 시드합니다.

각 팔로우 카드에는 대상 계정, 그들이 누구인지, 호스트가 왜 추천하는지가 노출됩니다. 본인이 어떤 계정을 팔로우할지 결정합니다 — 전부 팔로우할 의무는 없습니다 — 그리고 호스트의 임계값을 만족할 만큼 팔로우하면 액션이 완료 처리됩니다.

팔로우 액션은 투표나 토론과는 다른 방식으로 Essence 에 쌓입니다 — *누구* 와 정렬되어 있는지를 Essence 에 알려주어, 플랫폼 전체의 추천과 라우팅에 영향을 줍니다.

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/follow.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
  브라우저가 비디오 임베드를 지원하지 않습니다. <a href={useBaseUrl('/media/follow.mov')}>워크스루 다운로드</a>.
</video>

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="3" y="4" width="18" height="18" rx="2" ry="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg> Meet (미팅)

URL: `/spaces/:space_id/actions/meets/:meet_id`

**미팅(Meet)** 액션은 일정이 잡힌 이벤트입니다 — 라이브 스트림, 화상 회의, 워크숍, 오프라인 모임, 정기 오피스 아워. 카드에는 날짜, 시간, 장소 (또는 링크), 호스트가 첨부한 어젠다가 노출됩니다.

액션 페이지에서 RSVP 할 수 있고, 다른 참석자가 누구인지 볼 수 있으며, (호스트가 켜뒀다면) 미팅 시작 전 알림을 받을 수 있습니다. 출석이 — 검증 가능하다면 — 액션 완료로 인정되어 연결된 보상이 잠금 해제됩니다.

미팅은 디지털 참여를 시간 기반의 참여와 연결하는 유일한 액션 타입이라, 비동기 활동만이 아니라 실시간 모임을 중심으로 커뮤니티를 묶고 싶은 스페이스에 특히 유용합니다.

## 액션 보상은 어떻게 동작하는가

액션을 완료하면 두 가지가 일어납니다:

### 1. Incentive Pool 분배

스페이스의 Incentive Pool 이 펀딩되어 있다면, 호스트의 분배 규칙에 따라 본인 몫이 풀에서 인출됩니다:

- 어떤 풀은 모든 완료자에게 균등 분할합니다.
- 어떤 풀은 깊이 가중을 합니다 (잘 다듬어진 긴 토론 답변이 한 줄짜리 투표보다 더 많이 받습니다).
- 어떤 풀은 스페이스 단계별로 보상을 분할합니다.

어느 쪽이든 이 스페이스에서 누적된 보상은 본인 프로필에서 확인할 수 있고, 호스트 측에서 풀의 분배 이력은 완전히 감사 가능합니다.

### 2. Essence 누적

본인이 완료하는 모든 액션은 **EssenceSource** 가 됩니다 — 무엇에 참여했고 무엇을 기여했는지에 대한 구조화된 기록입니다. 투표, 댓글, 팔로우 선택, 퀴즈 답변, 미팅 RSVP — 모두 본인의 개인 Essence 로 흘러 들어가, Phase 0 파이프라인이 살아나면 그대로 임베딩됩니다.

:::note 예정
Phase 0 의 풀 Essence 파이프라인 — 모든 액션을 본인의 개인 지식 베이스로 임베딩하여 쿼리 · 에이전트 참여 · Essence House 구독으로 이어지게 하는 — 은 곧 들어옵니다. 출시 전까지도 활동은 일어나는 대로 기록되며, 임베딩 레이어는 이후 소급 적용되어 오늘 한 일은 무엇도 사라지지 않습니다.
:::

결과적으로, 스페이스에서 무언가를 한다는 것은 완료 시점의 보상으로 끝나지 않습니다. 모든 액션은 본인의 Essence 를 더 풍부하고 더 쿼리 가능하며 (궁극적으로) 본인의 Essence House 를 구독하는 누군가에게 더 가치 있게 만듭니다.
