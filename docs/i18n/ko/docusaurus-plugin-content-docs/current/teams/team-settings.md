---
sidebar_position: 4
title: 팀 설정
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# 팀 설정

팀 설정(Team Settings) 은 Team 의 관리자가 Team 자체를 관리하는 곳입니다 — 공개 정체성, 멤버와 역할, 매월 Credit 을 지급하는 구독, 그리고 (필요하다면) 삭제 흐름까지. 이 화면은 Team 의 **관리자(administrator) 시점** 입니다 — 공개 프로필과 피드는 [팀 생성](./create.md) 을 참고하세요.

## 팀 설정 위치

세 개의 URL 이 화면을 구성하며, 모두 Team 핸들 아래에 있습니다.

```
/<team-handle>/team-settings              # 일반 설정 + 결제 대시보드 카드 (소유자 전용)
/<team-handle>/team-settings/members      # 멤버 관리
/<team-handle>/team-settings/subscription # 등급 업그레이드 흐름 (5 카드 플랜 그리드, /membership 과 동일)
```

보통 Team 사이드바 하단의 사용자 드롭다운에서 *Settings* 를 눌러 도착하지만, URL 은 안정적이고 공유 가능합니다.

### 누가 접근 가능한가

팀 설정은 **관리자 전용** 입니다. 동작에 따라 세부 권한이 다릅니다.

- **소유자(Owner)** 는 모든 작업을 수행할 수 있습니다 — 일반 페이지의 **결제 대시보드(Billing dashboard)** 카드 운영, (예정) 소유권 이전, Team 삭제 포함.
- **관리자(Admin)** 는 Team 프로필 필드 편집과 멤버 관리를 할 수 있습니다. 결제 대시보드 카드와 Team 삭제 액션은 **소유자 전용** 입니다 — 소유자가 아닌 관리자에게는 이 카드들이 아예 보이지 않습니다.
- 일반 **멤버(Member)** 가 `/team-settings` 를 열려고 하면 페이지 자리에 *"You don't have permission to view team settings"* 이 표시됩니다. URL 은 그대로 유지되고, 리디렉션은 일어나지 않아요. `/team-settings/members` 의 Members 탭에는 일반 멤버용 읽기 전용 변형이 있지만, 초대 / 역할 변경 / 제거는 관리자 게이트입니다.

DAO 활성화 시의 DAO Address 와 온체인 거버넌스 설정도 소유자 전용입니다.

## <img src={useBaseUrl('/img/icons/settings.svg')} width="20" height="20" alt="일반 설정" style={{verticalAlign: 'middle'}} /> 일반 설정

메인 `/team-settings` 페이지는 한 컬럼의 관리자 컨트롤로 구성됩니다. 각 섹션은 편집하면 자동 저장되며, 상단의 *마지막 저장(Last saved)* 칩이 가장 최근 저장 시각을 알려줍니다.

### Team Profile (팀 프로필)

첫 번째 카드. Team 의 공개 프로필에 보이는 모든 항목을 여기서 편집합니다.

- **팀 로고(Team Logo)** — 정사각형 업로드, 권장 256 × 256 (1:1), 최대 2MB. 로고는 Team 프로필, 피드, Team 핸들로 발행한 게시글 옆에 노출됩니다.
- **배너 이미지(Banner image)** — 가로형 헤더 업로드, 권장 1200 × 400 (16:9), 최대 5MB.
- **팀 이름 (디스플레이 네임)** — UI 에 노출되는 이름. 최대 30 자.
- **설명(Description)** — Team 이 무엇을 하는지를 설명하는 더 긴 한 문단. 최소 10 자, 플랫폼의 콘텐츠 필터가 테스트 키워드와 공격적 표현을 거부합니다.
- **사용자명 (Username / 핸들)** — 잠겨 있습니다. 핸들은 Team 생성 시 정해지며 이후 변경이 어렵습니다 — 꼭 변경이 필요하면 운영팀에 문의하세요. URL 이 어디에 묶여 있는지 한눈에 보이도록 읽기 전용으로 표시됩니다.
- **DAO Address** *(DAO 활성화 시)* — 이 Team 에 연결된 온체인 트레저리 / 거버넌스 주소. 여기서는 읽기 전용이고, DAO 활성화는 [팀 생성 → DAO](./create.md#dao--집단-거버넌스) 흐름에서 이뤄집니다.

디스플레이 네임, 설명, 로고, 배너의 변경은 자동 저장 후 모든 곳에 즉시 반영됩니다.

### 위험 영역 — Team 삭제

일반 페이지 하단에서 소유자(만) **위험 영역(Danger Zone)** 카드 하나를 봅니다 — 단일 액션 **Team 삭제(Delete team)** 가 있어요. 클릭하면 확인 다이얼로그 (*"This will permanently delete \<Team name\>, including members, posts, and any associated governance data. This action cannot be undone."*) 와 **Confirm** 버튼이 열립니다.

알아두면 좋은 점:

- **소유자 전용.** 관리자(소유자가 아닌) 는 이 카드를 아예 보지 않습니다.
- **하위팀 링크에 캐스케이드.** 이 Team 이 인정된 하위팀의 상위팀이라면, 삭제 시 그 링크들의 캐스케이드 등록 해제를 시도하여 하위팀이 독립으로 돌아가게 합니다. 어떤 이유로든 캐스케이드가 실패하더라도 Team 자체는 삭제됩니다 — 하위팀의 `parent_team_id` 정리는 수동으로 필요할 수 있어요.
- **되돌릴 수 없음.** 복구 기능은 없습니다 — 스크린샷에서 다시 만드는 것이 사실상 유일한 방법입니다.

## <img src={useBaseUrl('/img/icons/users.svg')} width="20" height="20" alt="멤버" style={{verticalAlign: 'middle'}} /> 멤버 & 역할

Members 탭은 `/team-settings/members` 에 있습니다. 현재 Team 멤버 전원과 관리자 컨트롤이 표시돼요.

### 역할

오늘 출시된 세 가지 역할 (권한 등급순):

- **Owner (소유자)** — 최초 생성자 (또는 소유권이 이전된 사람). 모든 작업 가능: 설정 편집, 멤버 관리, 역할 변경, Team 삭제. 한 시점에 정확히 한 명만 존재합니다.
- **Admin (관리자)** — Team 설정 편집, 멤버 초대 / 제거, Member ↔ Admin 승급 / 강등 (Owner 는 제거 / 강등 불가), 구독 운영 가능.
- **Member (멤버)** — Team 핸들로 게시글 공동 저자, 초안 기여, DAO 제안 투표, Team 귀속 보상 수령 가능.

Team 은 여러 명의 관리자를 둘 수 있어요 — 행 단위 메뉴에서 **Make admin** 으로 멤버를 승급, **Make member** 로 강등합니다.

### 멤버 추가

**멤버 추가(Add members)** 버튼은 초대 흐름을 엽니다. 사용자명 또는 이메일로 초대를 보내고, 초대받은 사람은 인박스에서 **수락 / 거절** 을 선택합니다. 수락하면 즉시 Member 로 멤버 목록에 나타납니다.

아직 수락되지 않은 초대는 별도의 대기(Pending) 섹션에 노출됩니다 — 관리자는 수락 전에 초대를 회수할 수 있어요.

### 멤버 제거

행 단위 메뉴에는 **팀에서 제거(Remove from team)** 도 있습니다. 제거된 멤버는 향후 활동에서 Team 귀속을 잃지만, 개인 계정은 그대로 유지됩니다. 관리자는 Owner 를 제거할 수 없습니다.

### 소유권 이전 *(예정)*

오늘 시점에 Owner 역할은 Team 생성 시 설정되며 UI 를 통해 이전할 수 없습니다. *"이 관리자를 새 Owner 로 지정"* 같은 셀프서비스 이전 흐름은 로드맵에 있어요. 출시 전에는 운영팀에 문의해 소유권을 이전합니다.

## <img src={useBaseUrl('/img/icons/award.svg')} width="20" height="20" alt="결제" style={{verticalAlign: 'middle'}} /> 일반 설정의 결제 대시보드 (소유자 전용)

Team 구독의 *대시보드* — 현재 상태와 카드 — 는 `/team-settings/subscription` 이 아니라 **일반 설정 페이지(`/team-settings`)** 의 추가 카드로 자리잡고 있고, **소유자 전용** 입니다. 소유자가 아닌 관리자에게는 이 카드가 아예 보이지 않아요.

카드에 표시되는 항목:

- **현재 플랜 뱃지(Current Plan badge)** — Free / Pro / Max / VIP / Enterprise, 그리고 *Change Plan* / *View Membership* 링크가 `/team-settings/subscription` (또는 `/membership`) 으로 이동시켜 실제 등급 변경을 진행하게 합니다.
- **Credits** — 현재 사이클의 잔여 / 전체 Credit (예: `145 / 190`). Free 등급에서는 `0 / 0` 으로 표시되며 카드가 업그레이드를 권합니다.
- **만료(Expires)** — 현재 사이클 할당량의 만료일. Free 는 *Unlimited* 로 표시.
- **등록된 카드** — 구독 결제에 사용되는 마스킹된 카드 번호와 카드 소유자 이름. 등록된 카드가 없을 때 **Add Card**, 변경할 때 **Change Card**.

카드 폼은 사용자가 보는 PortOne KR 체크아웃과 동일합니다 (카드 번호 / 유효기간 MM / YY / 생년월일 또는 사업자 등록번호 / 카드 비밀번호 앞 2 자리). PortOne 은 Visa, Mastercard, AMEX, JCB, 그리고 한국 내 결제 수단을 지원해요.

## <img src={useBaseUrl('/img/icons/award.svg')} width="20" height="20" alt="구독" style={{verticalAlign: 'middle'}} /> 팀 구독 — `/team-settings/subscription`

`/team-settings/subscription` 라우트는 **업그레이드 흐름 그 자체** 입니다 — `/membership` 과 동일한 다섯 카드짜리 플랜 그리드 (Free / Pro / Max / VIP / Enterprise) 와 동일한 **Get Pro** / **Get Max** / **Get VIP** / **Contact Us** CTA 버튼을 노출합니다. 등급 변경은 개인이 사용하는 동일한 결제 모달을 거치되, 결제가 개인 카드가 아니라 Team 의 결제 카드로 이뤄진다는 점만 다릅니다.

`/<team-handle>/team-settings/subscription` URL 은 Team 용 플랜 그리드로 바로 이동하고 싶을 때 사용하세요 — 예를 들어 소유자에게 업그레이드를 권하는 링크를 보낼 때. 등급을 *바꾸지 않고* 현재 플랜을 *읽고*, Credit 사용량을 보고, 카드를 관리할 때는 `/team-settings` 의 **결제 대시보드** 카드 (소유자 전용) 를 사용하세요.

### 등급 — 개인과 동일한 다섯 등급

Team 구독은 사용자 구독과 동일한 다섯 등급을 사용하며, Credit 할당량과 스페이스당 한도도 같습니다. 전체 기능 매트릭스는 [용어 정리 → 멤버십 티어](../intro/glossary#membership-tier-멤버십-티어) 를 참고하세요.

- **Free** — 모든 Team 기본 포함.
- **Pro** — 월 40 Credit, 보상 스페이스당 최대 2 Credit, 월 30,000 원.
- **Max** — 월 190 Credit, 보상 스페이스당 최대 10 Credit, Trusted Creator 뱃지 *(예정 — Phase 1)*, 월 75,000 원.
- **VIP** — 월 1,360 Credit, 보상 스페이스당 최대 100 Credit, 참여자 원본 데이터 열람, 월 150,000 원.
- **Enterprise** — 완전 맞춤형, 컨택 전용, 월 1,000,000 원 이상.

Team 에 한정된 몇 가지 추가 사항:

- **Credit 은 한 번 결제되어 공유됩니다.** Team 의 월간 Credit 은 Team 구독으로 펀딩되며, 개별 멤버에게서 풀링되지 않습니다. 어떤 관리자가 보상 스페이스에 ([호스트 액션](../spaces/host-actions) 을 통해) Credit 을 사용하면 Team 의 할당량에서 빠져나갑니다.
- **소유자 전용 구매.** Team 의 등급 업그레이드는 소유자만 완료할 수 있습니다 — 결제 흐름은 소유자 전용 결제 대시보드의 등록 카드를 사용합니다.
- **결제는 오프체인 전용.** 개인 구독과 동일하게, 등급 결제에는 온체인 정산 단계가 없습니다.

## 이 페이지에 없는 것

Team 단위 컨트롤 중 `/team-settings` 에 속하지 않는 항목들:

- **DAO 활성화** — Team 홈 / DAO 흐름에 있고, 설정에 있지 않습니다.
- **하위팀** (신청, 관리, 등록 해제, 이탈) — [하위팀](./sub-teams.md) 참고. 거버넌스 흐름이지 계정 설정이 아닙니다.
- **드래프트, 포스트, 스페이스, Team 이 판매하는 멤버십** — Team 의 자체 아레나 탭에서 관리, 설정에서 관리하지 않음.
- **Team 단위 알림 환경설정** — *(예정)*.
- **관리자 동작 감사 로그(Audit log)** — *(예정)*.

## 요약 표

| URL | 무엇을 바꾸나 |
|---|---|
| `/<team-handle>/team-settings` | 로고, 배너, 이름, 설명 (관리자+). 결제 대시보드 카드 — 현재 등급, Credit, 만료, 카드 관리 (소유자 전용). Team 삭제 (소유자 전용). |
| `/<team-handle>/team-settings/members` | 초대, 제거, Admin ↔ Member 승급 / 강등 (관리자+). |
| `/<team-handle>/team-settings/subscription` | 다섯 등급 플랜 그리드 — *Get Pro* / *Get Max* / *Get VIP* / *Contact Us* 업그레이드 버튼, Team 의 결제 카드로 청구 (구매 완료는 소유자 전용). |

## 다음 단계

- [팀 생성](./create.md) — 멤버, 초안, DAO 기초를 다루는 기본 Team 워크스루.
- [하위팀](./sub-teams.md) — 상위 / 하위 Team 관계의 전체 거버넌스 라이프사이클.
- [용어 정리 → 멤버십 티어](../intro/glossary#membership-tier-멤버십-티어) — Team 의 보상 스페이스를 결제할 월간 Credit 등급을 선택하세요.
