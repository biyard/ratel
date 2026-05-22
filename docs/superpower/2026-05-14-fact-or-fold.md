# Fact or Fold — System Design

**Roadmap**: [roadmap/fact-or-fold.md](../../roadmap/fact-or-fold.md)
**Design**: [app/ratel/assets/design/fact-or-fold/](../../app/ratel/assets/design/fact-or-fold/)
**Author / Date**: claude · 2026-05-14
**Status**: APPROVED — PO 결정 반영 완료 (2026-05-14), PR1 진입 가능

## Summary

Ratel Arcade의 첫 게임 *Fact or Fold*: 4인이 모여 ~3분 라운드 안에 뉴스 1건의 진실성(REAL/FAKE)을 RP 베팅 + 근거 작성으로 판별. 결과는 (1) RP 정산 (2) Essence opt-in 등록 두 갈래로 흐른다. v1은 단일 글로벌 로비, 정원 채워지면 즉시 시작, **1인 진실 인사이더만**(거짓 인사이더 v2 deferred), 정확도 기준 글로벌 리더보드 포함(시즌/명예 배지는 v2 deferred). 모든 단계 전환과 정산은 EventBridge로 비동기 처리, 실시간 UI 갱신은 SSE.

## Resolved decisions (2026-05-14)

스펙 vs. mockup 불일치 5건 PO 결정 — 본 문서·코드·mockup 정리 모두 아래 결정에 따른다.

| # | 결정 | 영향 |
|---|---|---|
| **D1** | **거짓 인사이더 완전 삭제 (mafia mode 전부 v2 deferred)** | mockup에서 "REAL-KNOWER 거짓 진술" 필드, "LIAR · 거짓" 배지, "거짓말 성공률" 통계 제거. 데이터 모델은 `is_insider` boolean + 진실 statement 하나만 보유. |
| **D2** | **인사이더 1명 고정** | `FactFoldRound.insider_pk: Partition` (단일). admin form 안내 문구를 "1명"으로 고정. |
| **D3** | **글로벌 리더보드 v1 포함, 시즌·명예 배지 v2 deferred** | 정확도 기준 lifetime 리더보드 페이지 PR7로 추가. lobby.html의 시즌/뱃지/백분위 섹션은 mockup에서 제거. |
| **D4** | **lobby의 "진행 시간: 24시간" 라벨을 "라운드 주기: 하루 1회" 등으로 정정** | mockup 텍스트 변경만. |
| **D5** | **상단바 "라운드 생성하기" 버튼은 admin role conditional render** | RSX 변환 시 `is_admin()` guard. |

이후 섹션은 모두 위 5건 결정 반영 상태.

## Scope & PR slicing

v1 surface는 single PR로 가져갈 수 없는 크기(서버 30+ endpoint, 8+ entity, 10+ frontend view). PR을 단계로 자른다 — 각 PR은 독립 배포 가능하고 다음 PR이 빌드 위에 쌓는다.

| PR | 범위 | 베이스 |
|---|---|---|
| **PR1** | 모듈 스캐폴드 + 데이터 모델 + 어드민 헤드라인 CRUD (서버 + 어드민 페이지: headlines, new-headline) | `feature/fact-or-fold` |
| **PR2** | 어드민 schedule / stats / reports / settings (CRUD + 큐 알람) | PR1 |
| **PR3** | 로비 + 매칭 (단일 글로벌 로비, 4인 채워지면 시작, 잔액 가드) + 로비 뷰 | PR2 |
| **PR4** | 라운드 엔진 단계 1–4 (뉴스 공개 → 1차 베팅 → 근거 작성 → 근거 공개) + EventBridge 단계 전환 + SSE | PR3 |
| **PR5** | 단계 5 (실시간 토론 채팅 + 마지막 10초 flip) + 인용 검증 + 채팅 모더레이션 frame | PR4 |
| **PR6** | 단계 6 (정산) + RP 페이아웃 + 인사이더 보너스 + idempotency + Essence opt-in 등록 + 결과 화면 | PR5 |
| **PR7** | **리더보드** (정확도 기준 lifetime 글로벌) + `FactFoldUserStats` 집계 SSE/주기 갱신 | PR6 |

시즌·명예 배지는 v2 (PR8+) deferred.

PR1–PR7 ≈ 6–7주 작업. 각 PR은 자체 Playwright suite + cargo 테스트 포함.

## Data model (DynamoDB single-table)

전 entity는 `Partition` + `EntityType` enum 확장 + `#[derive(DynamoEntity)]` + `#[dynamo(prefix)]` 따른다.

| Entity | pk | sk | 핵심 필드 |
|---|---|---|---|
| `FactFoldHeadline` | `Partition::FactFold(headline_id)` | `EntityType::FactFoldHeadline(headline_id)` | verdict (REAL/FAKE), headline_text, body_excerpt, insider_statement, reveal_summary, sources[], category_tags[], difficulty, scheduled_at, status (Draft/Scheduled/Live/Settled), creator_pk |
| `FactFoldRound` | `Partition::FactFold(round_id)` | `EntityType::FactFoldRound(round_id)` | headline_pk, stage (NewsReveal..Settlement), stage_started_at, stage_deadline, participants[], insider_pk, started_at, settled_at, status |
| `FactFoldParticipant` | `Partition::FactFold(round_id)` | `EntityType::FactFoldParticipant(user_id)` | user_pk, joined_at, is_insider, last_seen_at (reconnect grace), forfeited |
| `FactFoldBet` | `Partition::FactFold(round_id)` | `EntityType::FactFoldBet(user_id)` | side (REAL/FAKE), amount_rp, locked_at, flipped_to, flip_cite_user_pk |
| `FactFoldRationale` | `Partition::FactFold(round_id)` | `EntityType::FactFoldRationale(user_id)` | text (50–200 char), submitted_at, essence_eligible (>= 50 chars), essence_registered |
| `FactFoldChatMessage` | `Partition::FactFold(round_id)` | `EntityType::FactFoldChat(msg_id)` | author_pk, text (≤80), sent_at |
| `FactFoldSettlement` | `Partition::FactFold(round_id)` | `EntityType::FactFoldSettlement(user_id)` | base_refund, correct_bonus, influence_bonus, insider_bonus, total_delta, idempotency_key |
| `FactFoldUserStats` | `Partition::User(user_pk)` | `EntityType::FactFoldUserStats` | total_rounds, correct_count, lifetime_delta_rp, last_played_at |
| `FactFoldReport` | `Partition::FactFold(report_id)` | `EntityType::FactFoldReport(report_id)` | reporter_pk, round_pk, target_pk, reason, created_at, status |
| `FactFoldSettings` | `Partition::Singleton` | `EntityType::FactFoldSettings` | (admin-tunable params from spec table) |

**Insider protection** (§Constraints): `is_insider` + `insider_statement_delivered` 필드는 인사이더 본인 요청에만 응답에 포함. 나머지는 redact. controller-level guard.

**Idempotency** (§Constraints): `FactFoldSettlement.idempotency_key = round_pk + user_pk` 로 conditional put. 중복 페이아웃 차단.

## API surface

`features/fact_or_fold/controllers/` 아래 endpoint group. 모두 SubPartition 타입 사용 (path/DTO에서 prefix 제거).

### Admin
- `POST   /api/fact-or-fold/admin/headlines` — create draft/scheduled
- `GET    /api/fact-or-fold/admin/headlines` — list with status filter
- `PATCH  /api/fact-or-fold/admin/headlines/{id}` — edit (verdict frozen once Live)
- `DELETE /api/fact-or-fold/admin/headlines/{id}` — soft-delete
- `POST   /api/fact-or-fold/admin/headlines/{id}/publish` — publish now
- `GET    /api/fact-or-fold/admin/stats` — round stats aggregate
- `GET    /api/fact-or-fold/admin/reports?bookmark`
- `GET    /api/fact-or-fold/admin/settings` / `PATCH ...` — tunable params

### Player (lobby + round)
- `GET    /api/fact-or-fold/lobby` — current lobby state (waiting count, ETA, can-join)
- `POST   /api/fact-or-fold/lobby/join` — 잔액 가드 + 단일 라운드 가드
- `POST   /api/fact-or-fold/lobby/leave`
- `GET    /api/fact-or-fold/rounds/{round_id}` — full state for player (insider redact)
- `POST   /api/fact-or-fold/rounds/{round_id}/bets` — 1차 베팅 (side + amount)
- `POST   /api/fact-or-fold/rounds/{round_id}/bets/flip` — 마지막 10초 flip + cite
- `POST   /api/fact-or-fold/rounds/{round_id}/rationale` — 근거 단발 제출
- `POST   /api/fact-or-fold/rounds/{round_id}/chat` — 채팅 (80자)
- `GET    /api/fact-or-fold/rounds/{round_id}/insider-statement` — 인사이더만 응답
- `POST   /api/fact-or-fold/rounds/{round_id}/heartbeat` — reconnect grace 갱신
- `POST   /api/fact-or-fold/rounds/{round_id}/report` — 인사이더 자폭/룰 위반 신고
- `POST   /api/fact-or-fold/rounds/{round_id}/essence` — opt-in 등록 (selected items)
- `GET    /api/fact-or-fold/rounds/{round_id}/events` — SSE stream

### Stats
- `GET    /api/fact-or-fold/me/stats` — 본인 lifetime 통계 (정답률, 누적 손익)

전 endpoint는 typed error enum (`FactFoldError`) + `crate::error!` 서버 로깅 패턴.

## Event flow (EventBridge)

폴링 없이 단계 전환과 정산은 모두 EventBridge로 비동기 처리 (§Constraints).

### 단계 자동 전환 (FR-7, FR-9)
- 1차 베팅 잠금 시 (또는 라운드 시작 시) deadline 계산해서 **scheduled event** 발행 — `FactFoldStageDeadline { round_pk, stage, fire_at }`
- 해당 시각에 핸들러가 라운드 상태 확인 후 다음 stage로 advance. 미제출자 처리(§FR-8).
- 구현: CDK Scheduler + EventBridge Rule → app-shell Lambda.

### 정산 (FR-10, FR-33)
- 단계 6 진입 시 `FactFoldSettlementTrigger { round_pk }` 발행.
- 핸들러가 정산 공식 계산 → 각 참가자별 `FactFoldSettlement` upsert (idempotency_key conditional put) → `User.points` 갱신 → SSE 결과 푸시.

### Pipe + Rule + DetailType 추가
- `DetailType` enum에 `FactFoldStageDeadline`, `FactFoldSettlementTrigger`, `FactFoldEssenceRegister` 추가.
- `EventBridgeEnvelope::proc()`에 match 분기.
- `common/stream_handler.rs`에 local-dev 분기.
- 참조: `conventions/implementing-event-bridge.md`.

## Realtime channel

SSE (Server-Sent Events) — Axum 내부, 외부 의존성 없음 (§Constraints).

- 엔드포인트: `GET /api/fact-or-fold/rounds/{round_id}/events`
- 이벤트 타입: `stage_changed`, `bet_locked` (마스킹), `rationale_submitted` (마스킹), `chat_message`, `flip_announced` (cited_user_pk), `settled`
- 각 참가자 connection을 round별 broadcast channel (tokio::sync::broadcast)에 구독. 단일 app-shell 인스턴스 가정 (§MVP concurrency).
- 짧은 폴 fallback: SSE 실패 시 `GET /rounds/{id}` 2–3초 폴 (RSX-side 자동 전환).
- 인사이더 식별 데이터는 SSE 페이로드에서도 제거 (channel별로 보내기 전 redact).

## Frontend architecture

### Feature module
`app/ratel/src/features/fact_or_fold/` — 신규.

```
fact_or_fold/
├── mod.rs, route.rs, layout.rs, i18n.rs
├── controllers/    (서버 함수: 위 API surface)
├── models/         (#[cfg(feature = "server")] DynamoEntity)
├── components/     (재사용 UI: BetCard, RationaleEditor, ChatMessage, …)
├── pages/          (HTML-first 페이지 — 디자인 mockup 1:1 매핑)
│   ├── lobby/, round/, leaderboard/(deferred), admin/{headlines,new_headline,schedule,insiders,stats,reports,settings}
├── hooks/
│   ├── use_lobby.rs       — UseLobby 컨트롤러
│   ├── use_round.rs       — UseRound 컨트롤러 (SSE 구독, stage signals, mutations)
│   └── use_admin_headlines.rs ...
└── types/error.rs, types/dto.rs
```

### UseRound 컨트롤러 (대표 패턴, `conventions/hooks-and-actions.md`)
- 필드: `stage: Signal<RoundStage>`, `participants: Signal<Vec<Participant>>`, `my_bet: Loader<Option<Bet>>`, `rationales: Loader<Vec<Rationale>>`, `chat: Signal<Vec<ChatMessage>>`, `time_remaining: Signal<i64>`, `insider_statement: Loader<Option<String>>` (본인이 인사이더일 때만 Some)
- 메서드 (async fn on context, default): `submit_bet(side, amount)`, `submit_rationale(text)`, `flip_bet(target_side, cite_user_pk)`, `send_chat(text)`, `register_essence(items)`, `heartbeat()`
- SSE 구독: `use_effect`에서 EventSource 생성, 이벤트별로 해당 signal 갱신.

### HTML→RSX 변환
`conventions/html-first-components.md` 따른다 — 클래스명·ID 보존, CSS는 `app/ratel/assets/main.css`에 추가, JS는 `script.js` per-page.

### Route enum
- `Route::FactFoldLobbyPage`
- `Route::FactFoldRoundPage { round_id: FactFoldPartition }`
- `Route::FactFoldAdminHeadlinesPage`, ...

## Settlement formula (FR-28, FR-29, FR-30)

스펙 그대로:
1. 승자 base = stake refund + stake × 0.6 (correct multiplier 1.6× default)
2. 패자 stake 전액 몰수 → bonus pool
3. bonus pool 승자 stake 비례 분배
4. flip 인용 보너스: A flip하고 승리 시 cited B에게 A.stake × 30%
5. 인사이더 정답 시 본인 stake × 0.5 추가

전 계산은 `services/settle_round.rs`에 순수 함수로. idempotency_key conditional put으로 중복 차단.

## Essence integration (FR-34–38)

기존 `essence/services/indexer.rs` 패턴 재사용 — 새 `index_fact_fold_rationale(rationale)` 추가.
- `EssenceSourceKind`에 `FactFoldRationale` variant 추가.
- 사용자가 "Register to Essence" 체크한 항목만 `Essence::upsert_for_source` 호출.
- 라운드 데이터 자체는 영구 보존 (§FR-11) — Essence 미등록 항목도 `FactFoldRationale` row는 남음.

## Test plan

### Server function tests (`app/ratel/src/tests/fact_or_fold_tests.rs`)
각 endpoint별 success / error / unauth. 정산 공식은 순수 함수 단위 테스트로 6+ 시나리오 (단순 승/패, flip 보너스, 인사이더 보너스, 모두 같은 side, 0스테이크 fallback, idempotency 재실행).

### Playwright (`playwright/tests/web/fact-or-fold.spec.js`)
PR별 추가. PR6까지 끝나면 acceptance criteria 16건 전수 커버.
- 로비 4인 자동 시작
- stage deadline 자동 진행
- 50자 미만 근거 Essence 후보 제외
- flip 인용 없으면 거부
- 인사이더 진실 statement 본인만 조회
- 정산 breakdown 4행
- Essence 등록 화면 체크 토글

### EventBridge dry-run
local stream_handler 분기로 stage transition + settlement 트리거 수동 호출 테스트.

## Open questions

스펙 §Open questions 5건 + 위 D1–D5 합쳐 결정 필요. **D1, D2, D3은 데이터 모델·정산·UI 큰 분기를 만들기 때문에 PR1 시작 전 결론 필요.** 나머지는 PR3 이후 진행 중 결정 가능.

## References

- 데이터/로직 baseline: `space_common/models/space_reward.rs` (RP 페이아웃 패턴)
- EventBridge: `common/types/event_bridge_envelope.rs`, `common/stream_handler.rs`, `features/activity/services/aggregate_score.rs`
- Essence: `features/essence/services/indexer.rs`, `features/essence/models/essence.rs`
- 컨벤션: `.claude/rules/conventions/{server-functions,dynamodb-patterns,hooks-and-actions,html-first-components,implementing-event-bridge,playwright-tests}.md`
