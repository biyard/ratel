# 액션 내비게이션

**Status**: Ready for design (Stage 2)
**Slug**: `action-navigation`
**Original spec**: `roadmap/action-navigation.md`
**Primary use case**: 스페이스 참여자/관리자가 액션에 진입하고, 생성하고, 관리하는 흐름 + 데스크톱 카드 carousel surface 내비게이션

## 문제

현재 Ratel의 스페이스 액션은 사용자가 일반적인 페이지 전환을 기대하는 지점에서도 overlay, sheet, layover 형태로 열리는 경우가 많습니다. 이 구조는 기본적인 웹/앱 내비게이션 멘탈 모델을 깨뜨립니다.

구체적으로는:
- 액션을 열었을 때 "액션 페이지로 이동했다"기보다 "arena 위에 모달이 하나 더 떴다"는 느낌을 줍니다.
- 브라우저 뒤로 가기 / 앞으로 가기 동작이 예측하기 어려워집니다. 어떤 상태는 라우트 기반이고, 어떤 상태는 overlay 기반이기 때문입니다.
- 현재 보고 있는 상태를 새로고침하거나 공유하기가 어색하거나 불안정합니다. 중요한 UI 상태가 canonical URL이 아니라 임시 UI 상태에 묶여 있기 때문입니다.
- 액션 생성과 액션 설정도 layover에 의존하고 있어, 사용자는 집중이 필요한 작업을 하면서도 임시 UI 상태를 계속 관리해야 합니다.
- 데스크톱 carousel surface에서는 중심에서 벗어난 blurred 카드가 "옆에 있는 선택 가능한 카드"처럼 보이기 때문에, 사용자는 먼저 그 카드가 중앙으로 와서 focus되기를 기대합니다. 하지만 실제로는 blurred 카드도 첫 클릭에 바로 열리거나 이동합니다.
- 이 문제의 핵심은 단순한 "클릭 오작동"이 아니라 상호작용 모델 불일치입니다. 시각 시스템은 "먼저 이웃 카드를 선택하라"고 말하는데, 실제 동작은 "바로 열어버린다"로 구현되어 있습니다.

그 결과 액션은 스페이스 안의 핵심 콘텐츠 객체임에도 불구하고 일시적이고 불안정하게 느껴지고, 카드 기반 내비게이션도 신중한 선택이 아니라 튀는 점프로 느껴집니다. 사용자는 방향감, 확신, 내비게이션 통제감을 잃습니다.

## 목표

모든 액션 경험이 canonical page route를 중심으로 동작하도록 바꾸고, 데스크톱 carousel 카드가 focus-before-enter 상호작용을 따르도록 만들어서, 사용자가 안정적인 내비게이션 의미 체계 안에서 콘텐츠를 진입, 이탈, 새로고침, 공유, 관리할 수 있게 한다.

## 비목표

- **이번 단계에서 모든 액션의 비주얼 스타일을 전면 재설계하지 않는다.** 우선순위는 내비게이션 모델과 플로우 구조다.
- **액션 흐름과 직접 관련 없는 스페이스 패널을 다시 쓰지 않는다.** leaderboard, notifications, overview 등은 액션 플로우에 직접 의존하지 않는 한 범위에 포함하지 않는다.
- **모든 모달을 없애는 것을 목표로 하지 않는다.** 가벼운 확인창이나 작은 선택기 같은 일시적 인터랙션은 overlay를 유지할 수 있다.
- **안정적인 액션 라우팅에 필요하지 않은 백엔드/도메인 모델 확장은 하지 않는다.**
- **모바일 네이티브 전용 내비게이션 재설계는 하지 않는다.** 대신 같은 canonical route 모델이 모바일 웹에서도 일관되게 동작하도록 맞춘다.
- **모바일 단일 카드 레이아웃에 데스크톱식 강제 2클릭 규칙을 적용하지 않는다.** focus-before-enter 규칙은 여러 blurred 이웃 카드가 동시에 보이는 데스크톱/태블릿 carousel surface를 위한 것이다.

## 사용자 스토리

### 스페이스 참여자

- 참여자로서, 액션 카드를 클릭했을 때 실제 액션 페이지로 이동하고 싶다. 그래야 내가 지금 어디에 있는지 알 수 있다.
- 참여자로서, 새로고침과 브라우저 뒤로 가기가 상식적인 액션 내비게이션을 유지해주길 원한다. 그래야 임시 UI 안에 갇힌 느낌이 들지 않는다.
- 참여자로서, 다른 멤버에게 액션 링크를 공유했을 때 같은 액션 화면으로 도착하길 원한다.
- 데스크톱 참여자로서, blurred/off-center 액션 카드를 클릭했을 때 먼저 중앙으로 이동하길 원한다. 그래야 맞는 항목을 선택했다는 것을 확인한 뒤 진입할 수 있다.

### 스페이스 관리자

- 스페이스 관리자로서, 새 액션을 만들기 시작하면 실제 액션 편집 화면으로 이동하길 원한다. 그래야 워크플로우가 실질적이고 안전하게 느껴진다.
- 스페이스 관리자로서, 액션 설정과 편집이 페이지 수준의 내비게이션을 중심으로 구성되길 원한다. 그래야 설정이 어디에 있는지 논리적으로 파악할 수 있다.

### 카드 기반 surface의 데스크톱 사용자

- 데스크톱 사용자로서, carousel에서 blurred/off-center 카드는 즉시 진입 대상이 아니라 먼저 선택되는 이웃 카드처럼 동작하길 원한다.
- 데스크톱 사용자로서, action 카드와 Home 카드 모두 같은 centered-card 상호작용 규칙을 따르길 원한다. 그래야 제품 내부 일관성이 유지된다.

## 기능 요구사항

### FR-1: Canonical action route

1. 모든 액션 타입은 자신의 기본 읽기/보기 상태를 나타내는 canonical route를 반드시 가져야 한다.
2. 액션 목록/대시보드에서 액션에 진입할 때는 전체 화면 overlay를 여는 대신 해당 canonical route로 이동해야 한다.
3. Canonical action route는 새로고침에 안전해야 한다. 사용자가 그 URL에서 브라우저를 새로고침해도 같은 액션 화면이 복원되어야 한다.
4. Canonical action route는 공유 가능해야 한다. 권한이 있는 다른 사용자가 같은 URL을 열면 동일한 액션 화면에 도착해야 한다.

### FR-2: 내비게이션 동작

5. 액션 페이지에서 브라우저 뒤로 가기를 누르면 내부 overlay 상태를 닫는 것이 아니라 이전 내비게이션 위치로 돌아가야 한다.
6. 액션 페이지에서 돌아간 뒤 브라우저 앞으로 가기를 누르면 같은 액션 페이지가 다시 열려야 한다.
7. 특정 comment/reply target 같은 하위 상태를 deep link로 지원하는 액션은, 그 상태가 메모리 내부 overlay 상태가 아니라 URL로 표현되어야 한다.

### FR-3: 액션 생성 플로우

8. "Create action" 시작 시 가벼운 타입 선택 UI를 사용할 수는 있지만, 타입을 선택한 이후에는 반드시 해당 액션 타입의 canonical creation/editor route로 이동해야 한다.
9. 초기 생성 플로우의 핵심 작성 경험은 지속적인 half-screen/full-screen layover에 의존해서는 안 된다.
10. 액션 생성 중 새로고침하더라도 사용자는 생성/편집 route 안에 남아 있어야 하며, 해당 액션 타입이 이미 지원하는 draft 동작도 유지되어야 한다.

### FR-4: 액션 관리 플로우

11. 기존 액션 편집은 일시적 overlay 안이 아니라 canonical route에서 이루어져야 한다.
12. 액션 설정 중에서 실제로 액션 구성을 바꾸는 성격의 설정은 분리된 글로벌 layover가 아니라 페이지 route 또는 page-scoped panel에 있어야 한다.
13. 삭제 같은 파괴적 확인 동작은 modal/popup 기반을 유지할 수 있다.

### FR-5: 액션 타입 간 UX 일관성

14. Poll, Quiz, Discussion, Follow는 내부 페이지 구조가 다르더라도, 진입/이탈 내비게이션 모델은 동일해야 한다.
15. 모바일과 데스크톱은 레이아웃 차이는 허용되지만 같은 canonical route 모델을 써야 한다.
16. 스페이스 수준의 내비게이션 chrome은 사용자가 현재 메인 arena/dashboard가 아니라 action page를 보고 있다는 사실을 명확히 드러내야 한다.

### FR-6: 데스크톱 carousel 선택 규칙

17. 데스크톱/태블릿 carousel surface에서 중앙 카드가 강조되고 주변 카드가 blur/scale/opacity로 약화되어 보이는 경우, 비중앙 카드를 클릭하면 먼저 그 카드가 중앙/active 위치로 이동해야 한다.
18. 그런 surface에서 비중앙 카드는 첫 클릭에 primary navigation 또는 entry action을 실행해서는 안 된다.
19. 카드가 중앙/active 상태가 된 이후에는 카드 본문 클릭이 그 surface의 primary 방식으로 목적지를 열어야 한다.
20. 중앙 카드 내부의 명확한 CTA도 같은 목적지를 열 수 있지만, centered-card click 동작과 충돌해서는 안 된다.
21. 데스크톱/태블릿 carousel surface에서의 키보드 활성화도 같은 규칙을 따라야 한다. 비중앙 카드는 먼저 focus/center되고, 중앙 카드는 열려야 한다.
22. 비중앙 카드의 시각적 처리, hover 처리, cursor 의미는 "즉시 열기"가 아니라 "이 카드를 focus/select"라는 뜻을 전달해야 한다.
23. 이 데스크톱 상호작용 규칙은 스페이스 action carousel과 team Home post carousel 모두에 동일하게 적용되어야 한다.

## 인수 기준

- [ ] AC-1: 스페이스 액션 목록/대시보드에서 Poll 액션을 클릭하면 overlay가 아니라 Poll page URL로 이동한다.
- [ ] AC-2: 스페이스 액션 목록/대시보드에서 Quiz 액션을 클릭하면 overlay가 아니라 Quiz page URL로 이동한다.
- [ ] AC-3: 스페이스 액션 목록/대시보드에서 Discussion 액션을 클릭하면 overlay가 아니라 Discussion page URL로 이동한다.
- [ ] AC-4: 어떤 액션 페이지에서든 새로고침해도 같은 액션 페이지에 그대로 남는다.
- [ ] AC-5: 액션 URL을 복사해서 다른 권한 있는 세션에서 열면 같은 액션 페이지에 도착한다.
- [ ] AC-6: 액션 페이지에서 브라우저 뒤로 가기를 누르면 예측 가능한 방식으로 이전 페이지/상태로 돌아간다.
- [ ] AC-7: 뒤로 간 뒤 브라우저 앞으로 가기를 누르면 같은 액션 페이지로 다시 돌아온다.
- [ ] AC-8: discussion comment deep link를 열면 해당 discussion page의 올바른 target으로 진입한다.
- [ ] AC-9: "Create action" 시작 시 선택기가 먼저 보일 수는 있지만, 액션 타입을 선택하면 실제 creation/editor page로 이동한다.
- [ ] AC-10: 액션 생성 중 새로고침해도 사용자가 스페이스 arena 루트로 튕기지 않는다.
- [ ] AC-11: 기존 액션 편집은 일시적인 full-screen layover가 아니라 page-based navigation을 사용한다.
- [ ] AC-12: 삭제 확인은 popup으로 남아 있을 수 있지만, 취소하면 사용자는 같은 액션 페이지에 그대로 남는다.
- [ ] AC-13: 데스크톱에서 blurred/non-centered action 카드를 클릭하면 카드가 중앙으로 이동하고, 첫 클릭에는 열리지 않는다.
- [ ] AC-14: 데스크톱에서 centered action 카드를 활성화하면 정의된 primary 방식으로 action page/view가 열린다.
- [ ] AC-15: 데스크톱에서 blurred/non-centered Home post 카드를 클릭하면 카드가 중앙으로 이동하고, 첫 클릭에는 이동하지 않는다.
- [ ] AC-16: 데스크톱에서 centered Home post 카드를 활성화하면 정의된 primary 방식으로 post/space 목적지로 이동한다.
- [ ] AC-17: 데스크톱에서 blurred/non-centered action 카드에 키보드로 활성화(Enter/Space)하면 먼저 중앙으로 이동하고, 첫 활성화에는 열리지 않는다.
- [ ] AC-18: 데스크톱에서 centered action 카드를 키보드로 활성화하면 action page/view가 열린다.
- [ ] AC-19: 데스크톱에서 비중앙 카드는 선택/focus 의미의 hover/cursor를 사용하고, 중앙 카드는 진입/open 의미를 드러낸다.

## 제약사항

- **하위 호환성**: 기존 액션 엔티티와 권한 모델은 제품 수준의 데이터 마이그레이션 없이 계속 동작해야 한다.
- **Route 안정성**: Canonical action URL은 새로고침, deep link, 향후 notification 진입점까지 감당할 만큼 안정적이어야 한다.
- **접근 제어**: 액션 URL을 직접 여는 사용자도 해당 액션/스페이스에 이미 존재하는 권한 규칙으로 동일하게 제어되어야 한다.
- **점진적 전환 가능성**: 내부적으로는 단계적으로 이행할 수 있지만, 사용자 관점에서는 완료 시점에 액션 타입 전반에서 일관된 모델처럼 느껴져야 한다.
- **구현 편의보다 UX 명확성 우선**: canonical navigation을 해치는 overlay 기반 편법은 남겨두면 안 된다.
- **surface 간 상호작용 일관성**: 같은 centered/blurred 시각 언어를 공유하는 카드 surface끼리는 상충하는 클릭 의미를 가져서는 안 된다.

## 열린 질문

- Stage 2를 막는 열린 질문은 없다.
- Stage 2 기본 결정: 액션 타입 선택기는 가벼운 modal/sheet로 남을 수 있지만, 타입 선택 직후 반드시 canonical creation/editor route로 이동해야 한다.
- Stage 2 기본 결정: action page는 detached overlay가 아니라 compact한 space context(예: breadcrumb, back-to-space, top context)를 유지해야 한다.
- Stage 2 기본 결정: 실질적인 action settings는 글로벌 layover가 아니라 page-scoped 구조(tab/section/panel/sub-route)로 이동한다.
- Stage 2 기본 결정: Poll, Quiz, Discussion, Follow는 구현 rollout이 단계적이더라도 제품 모델은 처음부터 통일된 route 패턴을 기준으로 잡는다.
- Stage 2 기본 결정: 데스크톱 carousel surface에서는 비중앙 카드 첫 클릭은 center, 중앙 카드 클릭은 open, CTA는 같은 open 동작을 중복 제공할 수 있다.

## 참고자료

- 현재 스페이스 액션 viewer/editor 흐름: `app/ratel/src/features/spaces/pages/index/`, `app/ratel/src/features/spaces/pages/actions/`
- 현재 action carousel 구현은 비중앙 카드를 시각적으로 약화시키면서도 full-card click 진입을 그대로 유지하고 있다: `app/ratel/src/features/spaces/pages/index/action_dashboard/`
- 현재 team Home carousel도 같은 centered-card 시각 모델 위에서 full-card 즉시 이동을 사용하고 있다: `app/ratel/src/features/social/pages/home/views/`
- 기존 액션 관련 로드맵 항목: `roadmap/meet-action.md`
- 현재 브랜치 논의에서 수집된 사용자 피드백: overlay 중심 액션 내비게이션이 주요 사용자 고통점으로 지적됨
