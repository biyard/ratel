# 라텔 오락실 (Arcade Platform) — 시스템 설계

**Roadmap**: [roadmap/fact-or-fold.md](../../roadmap/fact-or-fold.md)
**Design (FOF)**: [/designs/fact-or-fold/](../../app/ratel/assets/design/fact-or-fold/)
**선행 설계**: [2026-05-14-fact-or-fold.md](2026-05-14-fact-or-fold.md) (Stage 1+2 → Stage 3 진입 시점)
**Author / Date**: claude · 2026-05-15
**Status**: APPROVED — 사용자 합의 완료 (2026-05-15)

## Summary

Fact or Fold를 첫 게임으로 갖는 **미니게임 플랫폼** "라텔 오락실"을 `features/arcade/` 모듈로 도입한다. v1 게임은 Fact or Fold 단 1개이지만, **두 번째 게임이 같은 골격 위에 얹힐 수 있게** wallet/realtime/stage_scheduler 세 가지 이음매를 arcade-level 추상으로 둔다. 칩(chip)이라는 게임 내 토큰으로 RP를 환전해서 사용한다(카지노 메타포). 실시간 동기화는 **하이브리드**: 대부분(stage, bet, rationale, flip, lobby presence)은 2~3초 폴링, **채팅만 SSE + DDB Stream fan-out**. Lambda Function URL Response Streaming으로 각 SSE invocation이 DDB Stream을 listen해서 자기 라운드의 새 채팅 row를 broadcast hub로 publish. 스테이지 진행은 클라이언트 trigger + 서버 lazy advance 검증. `RoomChannel` trait + `InProcessChannel` hub는 PR4b에서 만들어둔 그대로 채팅 채널의 첫 활성화 사례가 된다.

## 선행 설계와의 관계

`2026-05-14-fact-or-fold.md`는 Fact or Fold를 **단일 feature** 로 본 설계였다. PR1~PR3까지는 그 설계를 그대로 따라 구현되었고(현재 main에 머지됨), PR4 step 3까지 진행한 시점에 다음 두 가지 결정이 추가되었다:

1. **앞으로 미니게임을 더 만들 계획**
2. **SSE 대신 WebSocket을 Ratel 전반에서 쓸 수 있도록 컴포넌트화**

본 문서는 그 결정들을 반영해 **모듈을 `features/fact_or_fold/`에서 `features/arcade/games/fact_or_fold/`로 재배치**하고, arcade 공통 이음매를 정식 설계한다. 본 문서의 결정이 선행 설계와 충돌할 경우 본 문서를 따른다.

## 핵심 결정 요약

| # | 결정 | 비고 |
|---|---|---|
| **A1** | **arcade 단위 모듈** — `features/arcade/`가 미니게임 플랫폼 owner | 게임은 `arcade/games/<name>/`에 살음 |
| **A2** | **realtime은 arcade-only** (Ratel-wide로 끌어올리지 않음) | trait 이름 `RoomChannel` (도메인 중립). 알림/라이브 댓글 등이 SSE/WS화될 때 `common/`으로 추출 리팩토링 |
| **A2'** | **v1 transport = polling + 채팅만 SSE. v2 = WebSocket** | v1: 대부분 (stage / bet / rationale / flip / lobby presence) 은 클라가 2~3초 `GET /rounds/{id}` 폴링. 채팅만 Lambda Function URL Response Streaming + DDB Stream fan-out으로 실시간. v2: WebSocket으로 통합 — 폴링도 채팅 SSE도 WS로 갈아끼움. `RoomChannel` trait이 transport-agnostic이라 게임 코드 변경 없이 transport 구현체만 교체. v2 시점에 인프라 결정 (API Gateway WebSocket + DDB connection state vs ECS Fargate + in-memory hub) |
| **A3** | **wallet은 arcade-only**, 칩(chip) 메타포 | trait 이름 `ArcadeWallet`. 기존 RP 코드(`space_reward.rs` 등)는 손대지 않음 |
| **A4** | **칩 환전**: v1 = 1:1, 운영자 조절 가능. 명시적 입장. 역환전은 v2 deferred (DB는 잔액 유지, 엔드포인트만 disabled) | 카지노 입장 메타포 |
| **A5** | **Buy-in 모델** — 라운드 입장 시 칩 예치, 라운드 내부는 칩 회계 무관, 정산 시 결과만큼 환원 | atomicity 별도 신경 안 씀 |
| **A6** | **Stage 진행** — 클라이언트 trigger + 서버 lazy advance 검증. EventBridge는 settlement만 트리거 | 인프라 단순. 서버 측 wall-clock 타이머 없음 |
| **A7** | **StageScheduler** — arcade-level trait + generic advance 로직, 게임은 stage enum + duration 매핑만 제공 | |
| **A8** | **trait + impl 한 파일**에 같이 — 비대해지면 리팩토링 | 첫 PR over-engineering 회피 |
| **A9** | **`ArcadeLayout`** — 헤더(칩 잔액 + 환전 버튼) + WS connection 소유 + 자식이 context 공유 | |
| **A10** | **훅은 페이지 단위로** 잘게 분할 (Ratel 컨벤션과 일치) | |
| **A11** | **i18n 통합** — `arcade/i18n.rs` 한 곳에 게임별 번역까지 다 통합 | |
| ~~**A12**~~ | ~~채팅 v1엔 휘발성~~ — **뒤집힘 (2026-05-15 옵션 X)**: 채팅은 DB write + DDB Stream fan-out + SSE 푸시. roadmap §FR-11 (영구 보존) 원래 요구사항 복귀. `FactFoldChatMessage` entity 부활 | DB write가 SSE fan-out의 자연스러운 트리거이자 영속화 부산물 |
| **A13** | **insiders 어드민 v2 deferred** — D1 결정(mafia 모드 삭제)으로 인사이더 관리 페이지 의미 약함 | stats에 흡수 |

## 디렉토리 구조

```
app/ratel/src/features/arcade/
│
├── mod.rs · route.rs · i18n.rs · layout.rs
│   ├── route.rs                       각 game의 route를 merge + arcade 자체 route
│   ├── i18n.rs                        arcade 전체 i18n (게임 번역 통합)
│   └── layout.rs                      ArcadeLayout — 헤더 + 칩 잔액 + WS connection 소유
│
├── wallet/                            ══ 이음매 1: 경제 경계 ══
│   └── wallet.rs                      ArcadeWallet trait + DdbArcadeWallet impl
│
├── realtime/                          ══ 이음매 2: 전송 (도메인 중립) ══
│   └── channel.rs                     RoomChannel trait + InProcessChannel impl
│
├── services/                          arcade-level 공통 비즈니스 로직
│   └── stage_scheduler.rs             StageScheduler trait + generic advance 로직
│
├── components/                        게임 카드, 칩 잔액 위젯 등 공유 UI
│
├── hooks/
│   ├── use_arcade_home.rs             아케이드 홈 (featured 게임 + 통계 + 랭킹)
│   └── use_arcade_wallet.rs           ChipWallet 잔액/환전/buy-in 공유 hook
│
├── pages/
│   └── home/                          /arcade/home  ← lobby.html
│
└── games/
    └── fact_or_fold/                  ══ 첫 미니게임 ══
        ├── mod.rs · route.rs
        │
        ├── models/                    durable 데이터 (DynamoEntity)
        │   ├── news_item.rs
        │   ├── round.rs
        │   ├── round_bet.rs
        │   ├── round_reasoning.rs
        │   └── round_result.rs
        │
        ├── controllers/               HTTP 엔드포인트 (얇은 어댑터)
        │   ├── lobby.rs
        │   ├── bet.rs
        │   ├── reasoning.rs
        │   ├── flip.rs
        │   ├── settlement.rs
        │   ├── essence.rs
        │   └── admin.rs
        │
        ├── services/                  FOF 전용 순수 비즈니스 로직
        │   ├── stage_machine.rs       arcade::StageScheduler 구현 (FOF 6단계)
        │   └── settle_round.rs        정산 공식 (순수 함수)
        │
        ├── realtime/                  FOF 게임 특화 채널 조율
        │   ├── lobby_presence.rs      매칭 대기 broadcast
        │   ├── stage_clock.rs         6단계 전환 결과 broadcast
        │   ├── chat.rs                토론 채팅 (v1 휘발성)
        │   └── events.rs              RoundEvent — WS payload 도메인 페이로드
        │
        ├── hooks/
        │   ├── use_fof_matching.rs
        │   ├── use_fof_round.rs
        │   ├── use_fof_admin_headlines.rs
        │   ├── use_fof_admin_new_headline.rs
        │   ├── use_fof_admin_schedule.rs
        │   ├── use_fof_admin_stats.rs
        │   ├── use_fof_admin_reports.rs
        │   └── use_fof_admin_settings.rs
        │
        ├── pages/
        │   ├── matching/              /arcade/games/fact-or-fold/matching   (신규 디자인 필요)
        │   ├── game_room/             /arcade/games/fact-or-fold/rounds/{id} ← round-stage.html
        │   │   ├── news_reveal/
        │   │   ├── first_bet/
        │   │   ├── reasoning_write/
        │   │   ├── reasoning_reveal/
        │   │   ├── live_debate/
        │   │   └── settlement/
        │   └── admin/                 ← admin-*.html (insiders 제외)
        │       ├── headlines/
        │       ├── new_headline/
        │       ├── schedule/
        │       ├── stats/
        │       ├── reports/
        │       └── settings/
        │
        └── types/
            ├── error.rs               FactOrFoldError
            └── dto.rs                 요청/응답 DTO
```

## 라우팅

```
/arcade/home                                          ArcadeHomePage
/arcade/games/fact-or-fold/matching                   FactFoldMatchingPage
/arcade/games/fact-or-fold/rounds/{id}                FactFoldGameRoomPage
/arcade/games/fact-or-fold/admin/headlines            FactFoldAdminHeadlinesPage
/arcade/games/fact-or-fold/admin/headlines/new        FactFoldAdminNewHeadlinePage
/arcade/games/fact-or-fold/admin/schedule             FactFoldAdminSchedulePage
/arcade/games/fact-or-fold/admin/stats                FactFoldAdminStatsPage
/arcade/games/fact-or-fold/admin/reports              FactFoldAdminReportsPage
/arcade/games/fact-or-fold/admin/settings             FactFoldAdminSettingsPage
```

API 엔드포인트도 동일 prefix:

```
/api/arcade/wallet/*                                  arcade-level wallet API
/api/arcade/games/fact-or-fold/*                      FOF 도메인 API
/api/arcade/ws                                        WebSocket upgrade (arcade-level)
```

## 데이터 모델

선행 설계의 entity 목록에서 다음이 변경된다:

| Entity | pk | sk | 비고 |
|---|---|---|---|
| `ArcadeWalletBalance` | `Partition::ArcadeWallet(user_id)` | `EntityType::ArcadeWalletBalance` | chip_balance, last_updated |
| `ArcadeWalletTransaction` | `Partition::ArcadeWallet(user_id)` | `EntityType::ArcadeWalletTxn(ulid)` | kind(Convert/BuyIn/Settle), amount, ref_round_id, created_at |
| `ArcadeSettings` | `Partition::Singleton` | `EntityType::ArcadeSettings` | rp_to_chip_ratio, ... |
| `FactFoldHeadline` | (선행 설계 그대로) | | `news_item.rs`로 파일명만 변경 |
| `FactFoldRound` | (선행 설계 그대로) | | `round.rs`. PR4 step 3의 stage_started_at/stage_deadline_at 유지 |
| `FactFoldBet` | (선행 설계 그대로) | | `round_bet.rs` |
| `FactFoldRationale` | (선행 설계 그대로) | | `round_reasoning.rs` |
| `FactFoldSettlement` | (선행 설계 그대로) | | `round_result.rs` |
| `FactFoldParticipant` | (선행 설계 그대로) | | `round.rs`에 sibling sk로 유지 |
| `FactFoldUserStats` | (선행 설계 그대로) | | 변동 없음 |
| `FactFoldReport` | (선행 설계 그대로) | | 변동 없음 |
| `FactFoldSettings` | (선행 설계 그대로) | | 변동 없음 |
| `FactFoldChatMessage` | `Partition::FactFold(round_id)` | `EntityType::FactFoldChat(ulid)` | author_pk, text (≤80), sent_at. **A12 뒤집힘 (옵션 X 결정)**: DB write가 SSE fan-out의 자연 트리거이자 영속화 부산물. roadmap §FR-11 만족 |

## 이음매 1: ArcadeWallet (칩)

### 트레잇

```rust
// features/arcade/wallet/wallet.rs

#[async_trait]
pub trait ArcadeWallet {
    /// 잔액 조회.
    async fn balance(&self, user: &UserId) -> Result<i64>;

    /// RP → 칩. v1 비율 1:1, 운영자 조절 가능.
    async fn convert_rp_to_chip(&self, user: &UserId, rp_amount: i64) -> Result<ChipReceipt>;

    /// 칩 → RP. v1엔 disabled (Err(NotImplemented)).
    async fn convert_chip_to_rp(&self, user: &UserId, chip_amount: i64) -> Result<RpReceipt>;

    /// 라운드 입장 시 칩 예치.
    /// 칩 잔액 -= amount, ArcadeWalletTransaction(BuyIn) 기록.
    async fn buy_in(&self, user: &UserId, round_id: &RoundId, chips: i64) -> Result<BuyInReceipt>;

    /// 라운드 종료 시 결과 환원.
    /// 칩 잔액 += chips_out, ArcadeWalletTransaction(Settle) 기록.
    async fn settle(&self, user: &UserId, round_id: &RoundId, chips_out: i64) -> Result<SettleReceipt>;
}

pub struct DdbArcadeWallet { /* ... */ }
impl ArcadeWallet for DdbArcadeWallet { /* ... */ }
```

### Buy-in 모델 (A5)

```
[로비 join 시]
    wallet.buy_in(user, round_id, default_buy_in_chips)
    → 칩 잔액 차감, Round.participants에 user 추가

[라운드 진행 중]
    bet/flip/rationale 등이 일어나도 wallet 안 건드림.
    Round 내부 회계는 FactFoldBet, FactFoldRationale row가 self-contained.

[라운드 종료 (settlement EventBridge)]
    for each participant:
        chips_out = compute_payout(round, participant)
        wallet.settle(user, round_id, chips_out)
```

라운드 안에서 칩이 "테이블 위에" 머무는 동안의 변동은 wallet 입장에선 invisible. 정산 시 한 번에 결과만큼 돌아옴.

## 이음매 2: RoomChannel (v1 = 채팅만 SSE + DDB Stream, 나머지 폴링)

### v1 transport: hybrid

대부분 라운드 상태는 **폴링**:
- 클라이언트가 2~3초마다 `GET /api/arcade/games/fact-or-fold/rounds/{round_id}` 호출
- 응답엔 stage, 참가자 진행 상태, 라운드 메타데이터 포함
- 액션 (베팅 / flip / tick / heartbeat) 은 HTTP POST → 서버는 DB만 쓰고 별도 push 안 함. 다른 참가자는 다음 폴링 사이클에 픽업
- stage 자동 진행은 각 클라가 자기 카운트다운 종료 시 `POST /tick` (PR4d)

**채팅만 SSE**:
- `GET /api/arcade/events?channel=fof.chat:{round_id}` (Lambda Function URL Response Streaming)
- 클라가 채팅 채널 구독 → 메시지 도착 시 즉시 push
- 폴링과 병행 (라운드 상태는 폴링, 채팅 메시지만 SSE)

### 채팅 fan-out: DDB Stream

Lambda multi-invocation fan-out 문제를 DDB Stream으로 우회:

```
[Player A]               [Player B/C/D]
  ↓ POST /chat            (각자 SSE invocation에 연결됨)
[Lambda invocation #5]      ↑    ↑    ↑
  ↓ DB write                │    │    │
[FactFoldChatMessage row]   │    │    │
  ↓ DDB Stream 발화         │    │    │
[Stream record]             │    │    │
  ↓ broadcast               │    │    │
[Lambda invocation #1, #2, #3 — 모두 같은 stream 받음]
  ↓ 자기 라운드의 채팅이면 hub.publish() ─────┘
  ↓ SSE Response stream에 write
```

핵심:
- DB write가 fan-out의 트리거이자 영속화의 부산물 (roadmap §FR-11 만족)
- SSE invocation 4개가 같은 stream record를 받음. 각자 자기가 들고 있는 connection이 그 라운드 채팅 채널 구독자인지 확인 후 push
- Latency 200~500ms (acceptable for chat)
- 인프라 추가: Lambda Function URL Response Streaming 모드 켜기, DDB Stream에 새 분기 (기존 essence/activity 패턴 재사용)

### 트레잇 (채팅 채널이 첫 활성화)

```rust
// features/arcade/realtime/channel.rs — PR4b에서 만들어 둠
pub trait RoomChannel: Send + Sync + 'static { /* ... */ }
pub struct InProcessChannel { /* tokio::sync::broadcast hub */ }
```

FOF가 `RoomChannel` impl 한 개 등록 (`FactFoldChatChannel`, kind = `"fof.chat"`). authorize에서 라운드 참가자 검증 + 채팅 history 최근 N개 반환. SSE invocation의 outer loop가 Stream listener + `hub.publish(channel, "chat_message", payload)`.

### v2 활성화 경로 (WebSocket으로 통합)

v2엔 v1의 폴링 + SSE 하이브리드를 **WebSocket 단일 채널로 통합**:

1. WebSocket endpoint 신설:
   - **옵션 a**: API Gateway WebSocket + DDB connection state (Lambda 유지)
   - **옵션 b**: ECS Fargate에 realtime 컨테이너 (in-memory hub 자연 동작)
   - v2 시작 시점에 채택 결정
2. `RoomChannel` trait은 그대로. `InProcessChannel`은 옵션 b 구현, 옵션 a면 `DdbConnectionChannel` 같은 다른 구현체
3. 게임 코드 변경 없음 — `hub.publish()` 호출은 동일, 트랜스포트만 교체
4. 폴링 코드 제거 + 채팅 SSE 코드를 WS 메시지로 치환

추가될 채널들 (같은 hub 위에):
- `fof.round:{id}` — stage_changed, bet_locked, flip_announced
- `fof.chat:{id}` — 채팅 (v1엔 SSE로 따로 처리, v2엔 같은 WS에 흡수)
- `user:{user_id}` — 알림 라이브 푸시 (Ratel-wide 확장 시)
- `post:{id}` — 라이브 댓글

각 채널은 자기 fan-out 메커니즘 선택 (DDB Stream / EventBridge / 메모리 hub / Redis pub-sub).

### 트레잇 (transport-agnostic)

```rust
// features/arcade/realtime/channel.rs

#[async_trait]
pub trait RoomChannel: Send + Sync + 'static {
    /// 채널 kind 식별자. 예: "fof.lobby_presence", "fof.round"
    fn kind(&self) -> &'static str;

    /// 클라가 채널 구독 요청 시 권한 검증 + 초기 상태 반환.
    /// SSE에선 GET /events?channel=... 요청 시 호출.
    async fn authorize(
        &self,
        ctx: &ChannelContext,
        channel: &ChannelId,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, ChannelError>;

    /// 도메인 측에서 채널로 broadcast할 때 hub.publish가 거치는 hook.
    /// 핸들러가 페이로드 가공/redact 가능 (예: 인사이더 정보 마스킹).
    async fn before_publish(
        &self,
        _channel: &ChannelId,
        _subscriber: &UserId,
        event: ServerEvent,
    ) -> Option<ServerEvent> {
        Some(event)
    }
}

pub struct InProcessChannel { /* in-memory hub */ }
```

`on_message`/`on_leave`는 v1엔 없음 (SSE 단방향). 양방향이 필요해지면 trait 확장.

### v1 polling cadence

```
클라 (매 2~3초):
    GET /api/arcade/games/fact-or-fold/rounds/{round_id}
    → RoundResponse (stage, deadlines, participants, ...)

클라 (countdown 종료 시):
    POST /api/arcade/games/fact-or-fold/rounds/{round_id}/tick
    → 서버 lazy advance + 새 RoundResponse

클라 (액션):
    POST /api/arcade/games/fact-or-fold/rounds/{round_id}/bets
    POST /api/arcade/games/fact-or-fold/rounds/{round_id}/rationale
    POST /api/arcade/games/fact-or-fold/rounds/{round_id}/chat
    POST /api/arcade/games/fact-or-fold/rounds/{round_id}/heartbeat
    → 결과 응답만. 다른 참가자는 다음 폴링 사이클에 알게 됨
```

### v2 활성화 경로

1. `common/realtime/`로 trait 이동 (단순 import path 변경)
2. `InProcessChannel::publish` 호출을 컨트롤러에 추가
3. SSE endpoint (`/events`) 또는 WS endpoint (`/ws`) 신설
4. 클라가 폴링 대신 EventSource / WebSocket 구독
5. Multi-invocation fan-out 결정 (DDB Stream listen / EventBridge / Redis pub-sub / ECS 컨테이너)

### 추출 친화 조건 (길 B 결정 부속)

1. `RoomChannel`은 도메인 어휘 미포함
2. `arcade/realtime/`은 게임 코드 import 안 함 (단방향 의존)
3. **transport에 묶이지 않음** — SSE/WS/polling 모두 같은 trait의 다른 구현체. 미래 WebSocket 전환 시 trait 안 건드림
4. 시나리오 2(알림/라이브 댓글도 SSE화)가 일어나면 `common/realtime/`으로 통째 이동

## 이음매 3: StageScheduler

### 트레잇

```rust
// features/arcade/services/stage_scheduler.rs

pub trait StageScheduler {
    /// 게임의 stage 식별 타입.
    type Stage: Copy + Eq;
    /// 게임의 settings/duration 컨텍스트.
    type Settings;

    /// 다음 stage. None이면 더 이상 자동 진행 안 함.
    fn next_stage(current: Self::Stage) -> Option<Self::Stage>;

    /// 해당 stage의 duration(ms). None이면 deadline 없음.
    fn stage_duration_ms(stage: Self::Stage, settings: &Self::Settings) -> Option<i64>;
}

/// Generic advance — 클라가 trigger를 보냈을 때, 또는 read 시 lazy로 호출.
/// 서버 wall-clock과 deadline 비교 후 정해진 만큼 ratchet.
pub async fn advance_if_due<S: StageScheduler>(
    round: &mut Round<S::Stage>,
    settings: &S::Settings,
    now_ms: i64,
) -> AdvanceOutcome<S::Stage> { /* ... */ }
```

### 클라 trigger + lazy advance (A6)

서버 측 wall-clock 타이머 **없음**.

```
[NewsReveal 시작 — lobby가 stamp_initial_stage]
    round.stage_started_at = now
    round.stage_deadline_at = now + 30s

[클라이언트가 카운트다운 종료 감지]
    POST /api/arcade/games/fact-or-fold/rounds/{id}/tick

[서버 tick 핸들러]
    advance_if_due<FactFoldStageMachine>(round, settings, now)
    if advanced:
        DB upsert
        WS broadcast { "name": "stage_changed", ... }

[그 외 read/write 모든 핸들러]
    위와 동일하게 advance_if_due 먼저 호출 (안전망 — PR4 step 3의 lazy advance)

[Settlement (마지막 stage)]
    EventBridge 트리거. WS는 settlement 결과 push만.
```

클라이언트가 다 disconnect되어도 settlement은 EventBridge로 발화. 그 외 stage는 다음 요청이 올 때까지 진행 안 됨(허용 트레이드오프).

## 페이지/훅 매핑

| 페이지 | hook | hook이 의존하는 것 |
|---|---|---|
| `pages/home/` | `use_arcade_home` | `use_arcade_wallet`, arcade home API |
| `games/fact_or_fold/pages/matching/` | `use_fof_matching` | `use_arcade_wallet`, lobby API, lobby_presence WS |
| `games/fact_or_fold/pages/game_room/` | `use_fof_round` | `use_arcade_wallet`(잔액 표시만), round API, round WS (stage_clock, chat, events) |
| `games/fact_or_fold/pages/admin/*` | `use_fof_admin_*` (페이지별 7개) | admin API (wallet/realtime 의존 없음) |

`ArcadeLayout`이 `use_arcade_wallet`을 호출해서 칩 잔액을 헤더에 표시 + WS connection을 establish, 자식 페이지는 context로 그 connection을 받아서 자기 채널 subscribe만 함.

## API surface

선행 설계의 endpoint 목록을 prefix만 바꿔 유지:

```
# Arcade wallet (신규)
GET    /api/arcade/wallet                                   잔액 + 최근 거래
POST   /api/arcade/wallet/convert                           RP → 칩
POST   /api/arcade/wallet/redeem                            칩 → RP (v1 disabled)

# FOF (선행 설계 + 변경)
GET    /api/arcade/games/fact-or-fold/lobby
POST   /api/arcade/games/fact-or-fold/lobby/join            (내부에서 wallet.buy_in 호출)
POST   /api/arcade/games/fact-or-fold/lobby/leave
GET    /api/arcade/games/fact-or-fold/rounds/{id}
POST   /api/arcade/games/fact-or-fold/rounds/{id}/tick      클라 trigger (advance_if_due)
POST   /api/arcade/games/fact-or-fold/rounds/{id}/bets
POST   /api/arcade/games/fact-or-fold/rounds/{id}/bets/flip
POST   /api/arcade/games/fact-or-fold/rounds/{id}/reasoning
GET    /api/arcade/games/fact-or-fold/rounds/{id}/insider-statement
POST   /api/arcade/games/fact-or-fold/rounds/{id}/heartbeat
POST   /api/arcade/games/fact-or-fold/rounds/{id}/essence
POST   /api/arcade/games/fact-or-fold/admin/headlines       (외 admin 5종)
GET    /api/arcade/games/fact-or-fold/me/stats

# Realtime (v1: polling + 채팅만 SSE)
# 라운드 상태는 클라가 GET /rounds/{id} 매 2~3초 폴링 (별도 endpoint 없음).
POST   /api/arcade/games/fact-or-fold/rounds/{id}/chat       채팅 발화 (HTTP POST, DB write)
GET    /api/arcade/events?channel=fof.chat:{round_id}        채팅 SSE 구독 (Function URL Response Streaming)
```

채팅 흐름:
1. 클라가 `POST /chat` — 서버가 `FactFoldChatMessage` row를 DDB에 write
2. DDB Stream이 발화 → SSE invocation들이 listen
3. 자기 들고 있는 connection이 `fof.chat:{round_id}` 구독자면 → `hub.publish` → SSE Response stream에 write
4. 다른 참가자 클라가 즉시 화면에 채팅 표시 (200~500ms latency)

v2: WebSocket으로 통합하면 위 4단계가 단일 fan-out으로 단순화.

## Event flow (EventBridge)

A6 결정에 따라 **stage 자동 진행은 EventBridge 사용 안 함.** 다음 이벤트만 EventBridge로 발화:

| DetailType | 발화 조건 | 핸들러 |
|---|---|---|
| `FactFoldSettlementTrigger` | 마지막 stage 진입 시 (manual emit) | 정산 공식 실행 → `FactFoldSettlement` upsert × N → wallet.settle × N → WS broadcast |
| `FactFoldEssenceRegister` | 사용자 essence opt-in 시 | essence pipeline 호출 |

선행 설계의 `FactFoldStageDeadline` variant는 **삭제** (A6).

## Test plan

### 서버 함수 테스트
- `app/ratel/src/features/arcade/wallet/`: convert, buy_in, settle 멱등성/순서
- `app/ratel/src/features/arcade/games/fact_or_fold/`: 기존 PR4 step 3 테스트 그대로 + tick 엔드포인트 추가

### WebSocket 단위 테스트
- `RoomChannel` mock impl로 envelope routing 검증
- 권한 거부 / 알 수 없는 채널 / 잘못된 페이로드

### Playwright
- 아케이드 홈 → 환전 → 매칭 → 게임룸 → 정산 전체 시나리오 (4인 multi-context)
- 클라 disconnect 시 stage 멈춤 + reconnect 시 즉시 tick 발사 → advance

## PR slicing (Stage 3 재구성)

선행 설계의 PR1~PR7 시퀀스를 본 설계에 맞춰 재배치한다. PR1~PR3는 이미 머지됨(`feature/fact-or-fold` 브랜치). PR4부터 본 설계 적용:

| PR | 범위 |
|---|---|
| ~~PR1~PR3~~ | 머지 완료 (선행 설계대로 `features/fact_or_fold/` 위치) |
| **PR4a** | **재배치**: `features/fact_or_fold/` → `features/arcade/games/fact_or_fold/`. 기능 변화 0. import path / route 갱신만 |
| **PR4b** | **arcade 이음매 스캐폴딩**: `arcade/wallet/wallet.rs`, `arcade/realtime/channel.rs`, `arcade/services/stage_scheduler.rs` trait + 최소 impl. 아직 FOF가 안 씀 |
| **PR4c** | **wallet 적용**: 로비 join이 `ArcadeWallet::buy_in` 호출. 환전 엔드포인트 + 아케이드 홈 placeholder |
| **PR4d** | **stage_scheduler 적용**: PR4 step 3의 `services/stage_machine.rs`를 `StageScheduler` impl로 갈아끼움. `/tick` 엔드포인트 추가 |
| **PR4e** | **SSE 인프라 (채팅 채널 전용)**: Lambda Function URL Response Streaming + `GET /api/arcade/events` + `RoomChannel` 라우터 + `InProcessChannel` hub와 DDB Stream listener 연결. CDK 작업 (Function URL InvokeMode=RESPONSE_STREAM, IAM, custom domain). 클라 `EventSource` 베이스 컴포넌트 (`ArcadeLayout`이 connection 소유) |
| **PR4f** | **FOF 채팅 채널 구현**: `FactFoldChatMessage` entity 부활 + `POST /api/arcade/games/fact-or-fold/rounds/{id}/chat` 핸들러 (DB write only) + DDB Stream의 `FACT_FOLD_CHAT#` 분기 → `hub.publish` + 클라가 `fof.chat:{round_id}` 채널 구독 + 게임룸 페이지에 채팅 UI 표시 |
| **PR5** | 단계 5 (live debate + flip slot) — 본 설계의 채널 위에 |
| **PR6** | 단계 6 (settlement) + EventBridge + Essence + 결과 화면 |
| **PR7** | 리더보드 + 통계 집계 |

각 PR은 독립 머지 가능. PR4a부터 PR4f까지 ~2주, PR5~PR7 ~3주 추정.

## Open questions / risks

### Resolved (2026-05-15)
- **Realtime transport**: v1 = 폴링 + 채팅만 SSE, v2 = WebSocket. (A2')
- **Multi-invocation fan-out**: DDB Stream 기반 (옵션 a). 각 SSE invocation이 Stream listen → 자기 라운드 채팅 publish.
- **채팅 영속화**: DB write + Stream fan-out으로 자동 영속. (A12 뒤집힘)

### Open
- **SSE 15분 timeout** — Lambda 최대 실행 시간. 클라가 자동 reconnect (EventSource 기본 동작). 라운드는 ~3분이라 정상 케이스엔 영향 없음. 라운드 대기 시간이 길어지면 reconnect 발생 가능 — `Last-Event-ID` 헤더로 끊김 사이의 채팅 메시지 재전송 처리 필요.
- **DDB Stream record 중복 처리** — Lambda는 at-least-once. 같은 채팅 메시지가 두 번 publish될 수 있음 — hub의 monotonic event id로 클라이언트 측 dedup. event id가 SSE id 필드로 매핑됨.
- **칩 환율 변경 시 진행 중 라운드 처리** — 라운드 시작 시 잠근 비율을 라운드에 stamp할지, 환율 변경을 즉시 반영할지 정책 정의 필요. v1엔 환율 변경 시 새 라운드부터 적용으로 가정.
- **v2 인프라 결정 (deferred)**: WebSocket을 API Gateway + DDB connection으로 갈지, ECS Fargate + in-memory hub로 갈지. v2 진입 시점에 트래픽 + 운영 부담 보고 결정.

## References

- 선행 설계: [2026-05-14-fact-or-fold.md](2026-05-14-fact-or-fold.md)
- 로드맵: [roadmap/fact-or-fold.md](../../roadmap/fact-or-fold.md)
- 디자인: [/designs/fact-or-fold/](../../app/ratel/assets/design/fact-or-fold/) (matching/ 추가 필요)
- PR4 step 3 stage machine: `app/ratel/src/features/fact_or_fold/services/stage_machine.rs` (PR4a에서 재배치)
- 관련 컨벤션: `.claude/rules/conventions/{server-functions,hooks-and-actions,html-first-components,playwright-tests}.md`
