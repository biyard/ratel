# Hot Spaces 랭킹 — 구현 계획서

> English: [2026-04-21-hot-spaces-ranking.md](./2026-04-21-hot-spaces-ranking.md)

> **에이전트 작업자 안내:** 필수 서브 스킬 — superpowers:subagent-driven-development (권장) 또는 superpowers:executing-plans 를 사용해 본 계획을 태스크 단위로 구현하세요. 각 단계는 체크박스(`- [ ]`) 문법으로 진행 상황을 추적합니다.

**목표:** 현재 `list_hot_spaces_handler`의 요청 시점 서버 사이드 활동 점수 계산 방식을, 이벤트 기반으로 사전 계산된 랭킹으로 교체한다. 이로써 홈 캐러셀은 공개 스페이스 총 개수와 무관하게 DynamoDB 호출 상수 회수만으로 전역 Hot 스페이스를 노출한다.

**아키텍처:** `by-macros`를 확장해 Number 타입 GSI sort key를 지원하게 한 뒤, 신규 엔티티 2개(`SpaceHotScore`, `SpaceActionCount`)를 도입하고 기존 + 신규 EventBridge 파이프라인으로 유지한다. 읽기는 **GSI7 쿼리 1회 + 병렬 `batch_get` 2회**로 끝난다. 쓰기는 이벤트(참여자 가입, 액션 생성/삭제)마다 기존의 `increase_*`/`decrease_*` setter로 원자적 증분을 수행한다. 기존 `count_actions` N+1 스캔과 요청 시점 `activity_score` 정렬은 폐기한다.

**기술 스택:** Rust, DynamoEntity 매크로(Number sort key 확장), DynamoDB GSI(네이티브 Number sort key), EventBridge Pipes + Rules, CDK.

**선결 조건:** 현재 PR `feature/home-sort`는 이미 (a) 탭 전환 블러 해결, (b) My Spaces 정렬 수정, (c) 50개 fetch window 내 요청 시점 `activity_score` 정렬 도입을 마쳤다. 본 계획은 (c)를 확장 가능한 설계로 대체한다.

---

## 현재 PR의 알려진 한계 (본 계획이 해결하는 것)

| 문제 | 현재 PR | 본 계획 적용 후 |
|---|---|---|
| Fetch window 50개 상한 | GSI6의 50번째 이후에 있는 Hot 후보 누락 | GSI7이 모든 공개 스페이스를 전역 랭킹 |
| N+1 `count_actions` | 홈 로드마다 50회 순차 `SpaceAction::find_by_space` 스캔 | 읽기 시 스캔 0회 — `SpaceActionCount.batch_get`로 조회 |
| 읽기 시점 정렬 비용 | CPU + DynamoDB quota가 fetch window에 비례 증가 | 랭킹은 사전 계산됨 — 읽기 경로는 페이지 크기에 O(1) |
| 페이지네이션 bookmark 의미 | 매 페이지 재정렬 → 페이지네이션 불일치 | DynamoDB GSI7 기반 bookmark가 안정적 |
| `SpaceCommon.hot_score` 추가 시 피드백 루프 위험 | — | 점수는 별도 엔티티에 → `SpaceCommon` MODIFY 트리거 없음 |

---

## 파일 구조

### 신규 파일

| 파일 | 목적 |
|------|------|
| `app/ratel/src/features/activity/models/space_hot_score.rs` | `SpaceHotScore` 엔티티 + GSI7 선언 |
| `app/ratel/src/features/activity/models/space_action_count.rs` | `SpaceActionCount` 엔티티 (비정규화 카운트) |
| `app/ratel/src/features/activity/services/hot_score.rs` | `bump_hot_score()`, `SCORE_DELTA_*` 상수 |
| `app/ratel/src/features/activity/services/action_count.rs` | 액션 카운트 증가/감소 헬퍼 (`bump_action_count`) |
| `app/ratel/src/features/admin/controllers/migrations/backfill_hot_scores.rs` | 기존 스페이스들에 `SpaceHotScore` + `SpaceActionCount`를 시딩하는 1회성 관리자 엔드포인트 |
| `app/ratel/src/tests/hot_score_tests.rs` | 파이프라인 + 핸들러 통합 테스트 |

### 수정 파일

| 파일 | 변경 내용 |
|------|-----------|
| `scripts/create-indexes.sh` | `gsi7` 추가 (PK=String, SK=**Number**); 인덱스별 `AttributeType` 파라미터화 |
| `packages/by-macros/src/dynamo_entity/mod.rs` | `#[dynamo(index = "...", sk, as_number)]` 플래그 지원 → 해당 sort key를 `AttributeValue::N`으로 직렬화 |
| `packages/by-macros/src/query_builder_functions.rs` 또는 동등 위치 | `as_number` sort key인 `i64` 필드에도 `increase_*` / `decrease_*` setter 생성 보장 |
| `app/ratel/src/common/types/error.rs` 또는 동등 위치 | `EntityType::SpaceHotScore`, `EntityType::SpaceActionCount` 추가 |
| `cdk/lib/dynamo-stream-event.ts` | `SpaceParticipantJoinPipe`, `SpaceActionCountPipe` + 룰 추가 |
| `app/ratel/src/common/types/event_bridge_envelope.rs` | `DetailType::SpaceParticipantJoin`, `DetailType::SpaceActionCountUpdate` variant + `proc()` 분기 추가 |
| `app/ratel/src/common/stream_handler.rs` | 로컬 개발 패리티: `SpaceParticipant` INSERT와 `SpaceAction` INSERT/REMOVE 핸들러 미러링 |
| `app/ratel/src/features/spaces/space_common/controllers/list_hot_spaces.rs` | 본문을 GSI7 쿼리 + `SpaceCommon.batch_get` + `SpaceActionCount.batch_get`로 교체; `activity_score()`와 `count_actions()` 제거 |

### 폐기 (Phase 5에서 삭제)

- `list_hot_spaces.rs` 의 `fn activity_score(...)`
- `list_hot_spaces.rs` 와 `list_my_home_spaces.rs` 의 `fn count_actions(...)` (`SpaceActionCount.batch_get`으로 대체)
- `list_hot_spaces.rs` 의 `limit(50)` / `items.truncate(10)`

---

## 엔티티 설계

### A. `SpaceHotScore` — 랭킹 엔티티

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceHotScore {
    pub pk: Partition,        // "SPACE#{space_id}"
    pub sk: EntityType,       // EntityType::SpaceHotScore

    /// 랭킹 버킷
    /// - "PUB_PUB"  : Published + Public (홈 Hot 캐러셀 노출 대상)
    /// - "HIDDEN"   : Private / Draft (visibility 변경 시 재배치를 위해 보존)
    #[dynamo(prefix = "HOT", name = "find_hot_by_bucket", index = "gsi7", pk)]
    pub bucket: String,

    /// 누적 hot score. GSI7 sort key(Number 타입)로 사용.
    /// `increase_hot_score(delta)` / `decrease_hot_score(delta)`로 원자 갱신 — read-modify-write 없음.
    #[dynamo(index = "gsi7", sk, as_number)]
    pub hot_score: i64,

    /// 점수를 마지막으로 움직인 이벤트(참여/액션)의 타임스탬프.
    /// 주기적 decay 잡이 유휴 row를 식별할 때 사용.
    pub last_activity_at: i64,

    /// 읽기 시점 batch_get용 비정규화 키.
    pub space_pk: Partition,

    pub updated_at: i64,
}
```

#### 왜 Number SK인가

원자 `ADD`와 누적 카운터를 결합하는 방식은 DynamoDB 리더보드의 표준 패턴이다. sort key가 곧 카운터 그 자체이므로 `ADD hot_score :delta` 한 번으로 메인 속성과 GSI projection이 동시에 갱신된다. 이 코드베이스의 기존 `Post.likes` + `increase_likes(1)` 패턴을 랭킹 GSI로 확장한 것.

String sort key로 가면 (a) 카운터보다 뒤처지는 `hot_score_sk: String` 필드를 별도로 유지하면서 동기화 시점에 lost update race를 감수하거나, (b) 이벤트마다 read-modify-write를 수행해야 한다. 둘 다 복잡도가 커지고 동시 쓰기의 `ADD` 원자성 보장을 포기하게 된다. `by-macros`의 작은 확장(Phase 0) 비용 한 번이면 훨씬 깔끔한 시스템이 되고, 이후의 다른 랭킹 엔티티도 그대로 재사용 가능하다.

#### 점수 전략 — 누적 증분

각 이벤트가 고정된 가중치 delta를 기여한다. read-modify-write 없음, race 없음:

| 이벤트 | `hot_score`에 적용되는 delta |
|---|---|
| `SpaceParticipant` INSERT | `+10` |
| `SpaceAction` INSERT      | `+20` |
| `SpaceAction` REMOVE      | `-20` |
| (향후: 댓글 INSERT)       | `+3`  |

```rust
// 상수는 services/hot_score.rs에 정의
pub const SCORE_DELTA_PARTICIPANT: i64 = 10;
pub const SCORE_DELTA_ACTION: i64      = 20;
```

#### 갱신 계약 — 원자 증분

```rust
pub async fn bump_hot_score(
    cli: &aws_sdk_dynamodb::Client,
    space: &SpaceCommon,
    delta: i64,
    now: i64,
) -> Result<()> {
    let (pk, sk) = SpaceHotScore::keys(&space.pk);
    let mut updater = SpaceHotScore::updater(&pk, &sk)
        .with_bucket(bucket_for(space))
        .with_space_pk(space.pk.clone())
        .with_last_activity_at(now)
        .with_updated_at(now);

    // by-macros는 i64 필드에 대해 increase_*/decrease_* setter를 생성함.
    // REMOVE 이벤트에서 음수 delta가 분기 없이 들어오도록 부호에 따라 분기.
    if delta >= 0 {
        updater = updater.increase_hot_score(delta);
    } else {
        updater = updater.decrease_hot_score(-delta);
    }
    updater.execute(cli).await
}
```

모든 쓰기가 Number 속성에 `ADD`를 쓰므로 **동시 쓰기가 lost update를 일으킬 수 없다**. 같은 밀리초에 두 파이프라인이 발화해도 두 delta가 합산될 뿐이다.

#### 주기적 decay (선택, Phase 4 후속)

순수 누적만 하면 한때 인기였지만 죽은 스페이스가 영구 상위를 차지한다. 옵션:

- **EventBridge Scheduler**(CDK `Schedule`)가 24시간마다 Lambda 트리거
- Lambda가 `HOT#PUB_PUB` 버킷을 스캔해 `ADD hot_score :decay` 적용 (예: `:decay = -floor(row.hot_score * 0.05)` — 하루 5% 감쇠) 또는 `last_activity_at` 기준 N일 이상 유휴인 row에만 고정 음수 offset
- Decay 자체도 `ADD` → 여전히 원자적

**초기 롤아웃에는 포함하지 않는다**. decay가 없어도 랭크 인플레이션은 활발한 스페이스의 이벤트 유입 속도로 제한됨; 랭킹이 stale하게 느껴질 때만 재검토.

### B. `SpaceActionCount` — 비정규화 액션 카운트

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceActionCount {
    pub pk: Partition,        // "SPACE#{space_id}"
    pub sk: EntityType,       // EntityType::SpaceActionCount

    pub poll_count: i64,
    pub discussion_count: i64,
    pub quiz_count: i64,
    pub follow_count: i64,
    pub total_actions: i64,

    pub updated_at: i64,
}
```

GSI 없음 — 항상 (space_pk, EntityType::SpaceActionCount)로 `get` 또는 `batch_get` 접근.

#### 증감 계약

`by-macros`가 `i64` 필드에 대해 이미 생성해주는 `increase_*` / `decrease_*` setter로 원자 증감 (기존 선례: `features/posts/models/post.rs:289` 의 `Post::increase_likes(1)`):

```rust
pub async fn bump_action_count(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    action_type: SpaceActionType,
    delta: i64, // 생성 시 +1, 삭제 시 -1
    now: i64,
) -> Result<()> {
    let (pk, sk) = SpaceActionCount::keys(space_pk);
    let mut u = SpaceActionCount::updater(&pk, &sk).with_updated_at(now);

    let abs = delta.unsigned_abs() as i64;
    let inc = delta > 0;

    u = match action_type {
        SpaceActionType::Poll            => if inc { u.increase_poll_count(abs) }       else { u.decrease_poll_count(abs) },
        SpaceActionType::TopicDiscussion => if inc { u.increase_discussion_count(abs) } else { u.decrease_discussion_count(abs) },
        SpaceActionType::Quiz            => if inc { u.increase_quiz_count(abs) }       else { u.decrease_quiz_count(abs) },
        SpaceActionType::Follow          => if inc { u.increase_follow_count(abs) }     else { u.decrease_follow_count(abs) },
    };
    u = if inc { u.increase_total_actions(abs) } else { u.decrease_total_actions(abs) };

    u.execute(cli).await
}
```

read-modify-write 없음. 동시 쓰기는 DynamoDB가 합산.

---

## AWS 레벨 변경 사항

### 1. 메인 DynamoDB 테이블의 GSI7

sort key 속성 타입이 **Number**여야 `hot_score`가 네이티브로 저장·정렬된다. `scripts/create-indexes.sh`를 인덱스별 SK 타입을 받도록 수정한 뒤:

```bash
INDEXES=(
  # ... 기존 gsi1~gsi6은 "S S" 유지 ...
  "gsi7_pk gsi7_sk gsi7-index  S  N"   # PK=String, SK=Number
)
```

현재 스크립트는 두 키 모두 `AttributeType="S"`로 하드코딩되어 있으므로 ([scripts/create-indexes.sh:45](../../scripts/create-indexes.sh#L45)), 각 엔트리가 자신의 `PK_TYPE`과 `SK_TYPE`을 받도록 파라미터화한다.

각 복제 리전에서 순차 실행:

```
ap-northeast-2 → us-east-1 → eu-central-1
```

인덱스 빌드 비용은 `gsi7_pk`/`gsi7_sk` 속성을 가진 row 수에 비례한다. `SpaceHotScore` row만(백필 동안 생성, 단기 ≤ 10k row 예상) 해당 속성을 가지므로 리전당 ACTIVE 대기는 시간이 아닌 분 단위일 것이다. 다른 엔티티는 영향받지 않는다.

### 2. EventBridge 파이프라인

세 개의 서로 다른 쓰기 경로가 `SpaceHotScore` + `SpaceActionCount`를 업데이트해야 한다:

| 트리거 | 파이프 | DetailType | 핸들러 동작 |
|---|---|---|---|
| `SpaceParticipant` INSERT (가입) | **신규**: `SpaceParticipantPipe` | `SpaceParticipantJoin` | `bump_hot_score(space, +SCORE_DELTA_PARTICIPANT)` |
| `SpaceAction` INSERT (액션 생성) | **신규**: `SpaceActionCountPipe` | `SpaceActionCountUpdate` | `bump_action_count(..., +1)` + `bump_hot_score(space, +SCORE_DELTA_ACTION)` |
| `SpaceAction` REMOVE (액션 삭제) | 같은 파이프, OldImage REMOVE 필터 | `SpaceActionCountUpdate` | `bump_action_count(..., -1)` + `bump_hot_score(space, -SCORE_DELTA_ACTION)` |
| `SpaceCommon` MODIFY (`participants`) | 기존 `PopularSpacePipe` + `fan_out_popular_space` | (기존) | (선택) `SpaceHotScore` 버킷이 최신인지만 확인 — 참여 경로는 이미 `SpaceParticipantPipe`로 증분되므로 중복 카운트 없음 |
| `SpaceCommon` MODIFY (`visibility`/`publish_state`) | (초기: 스킵 — 다음 참여/액션 이벤트에서 rebucket; 후속: 전용 파이프 추가) | — | `with_bucket(...)`로 버킷만 갱신하고 `hot_score`는 건드리지 않음 |

CDK 파일: [cdk/lib/dynamo-stream-event.ts](../../cdk/lib/dynamo-stream-event.ts). 기존 패턴(Pipe + Rule → `props.lambdaFunction`) 재사용. Rust 쪽 `DetailType` enum에 대응되는 variant 추가 필요.

### 3. 로컬 개발 스트림 핸들러 패리티

[app/ratel/src/common/stream_handler.rs](../../app/ratel/src/common/stream_handler.rs)가 각 EventBridge 경로를 미러링해야 `make run`이 EventBridge 인프라 없이도 동작한다. 기존의 per-`sk`-prefix 분기 구조를 따른다.

---

## 읽기 경로

```rust
#[get("/api/home/hot-spaces?bookmark")]
pub async fn list_hot_spaces_handler(
    bookmark: Option<String>,
) -> Result<ListResponse<HotSpaceResponse>> {
    let cli = ServerConfig::default().dynamodb();

    // 1) 랭킹 쿼리 — GSI7, scan_index_forward=false (hot_score 내림차순)
    let opts = SpaceHotScore::opt_with_bookmark(bookmark).limit(10);
    let (scores, next_bookmark) =
        SpaceHotScore::find_hot_by_bucket(cli, "PUB_PUB".into(), opts).await?;

    if scores.is_empty() {
        return Ok((vec![], next_bookmark).into());
    }

    // 2) 병렬 보강 — batch_get 3회
    let space_keys: Vec<_> = scores.iter()
        .map(|s| (s.space_pk.clone(), EntityType::SpaceCommon)).collect();
    let count_keys: Vec<_> = scores.iter()
        .map(|s| (s.space_pk.clone(), EntityType::SpaceActionCount)).collect();
    let post_keys: Vec<_> = scores.iter()
        .filter_map(|s| s.space_pk.clone().to_post_key().ok())
        .map(|pk| (pk, EntityType::Post)).collect();

    let (spaces_r, counts_r, posts_r) = tokio::join!(
        SpaceCommon::batch_get(cli, space_keys),
        SpaceActionCount::batch_get(cli, count_keys),
        Post::batch_get(cli, post_keys),
    );
    let spaces = spaces_r.unwrap_or_default();
    let counts = counts_r.unwrap_or_default();
    let posts  = posts_r.unwrap_or_default();

    // 3) 랭킹 순서를 유지하며 조립; 오래된 HIDDEN row는 제외
    let space_map: HashMap<_, _> = spaces.into_iter().map(|s| (s.pk.to_string(), s)).collect();
    let count_map: HashMap<_, _> = counts.into_iter().map(|c| (c.pk.to_string(), c)).collect();
    let post_map:  HashMap<_, _> = posts.into_iter().map(|p| (p.pk.to_string(), p)).collect();

    let items: Vec<HotSpaceResponse> = scores.iter().enumerate()
        .filter_map(|(idx, score)| {
            let space = space_map.get(&score.space_pk.to_string())?;
            if !space.is_public() || !space.is_published() { return None; }
            let default_count = SpaceActionCount::default();
            let count = count_map.get(&space.pk.to_string()).unwrap_or(&default_count);
            let post = space.pk.clone().to_post_key().ok()
                .and_then(|pk| post_map.get(&pk.to_string()));
            Some(HotSpaceResponse {
                space_id: space.pk.clone().into(),
                rank: idx as i64 + 1,
                participants: space.participants,
                poll_count: count.poll_count,
                discussion_count: count.discussion_count,
                quiz_count: count.quiz_count,
                follow_count: count.follow_count,
                total_actions: count.total_actions,
                heat: derive_heat(space.participants),
                // ... SpaceCommon + Post에서 title/description/logo/author 등 ...
                ..Default::default()
            })
        })
        .collect();

    Ok((items, next_bookmark).into())
}
```

**홈 로드당 DynamoDB 호출 수**: 3회 (Query 1 + BatchGetItem 3 병렬 실행 → 레이턴시 = 셋 중 최댓값). 전체 공개 스페이스 수와 무관. `count_actions`는 사라진다.

---

## 백필

`Role::Admin`으로 보호되는 1회성 관리자 엔드포인트. [backfill_space_score_rank.rs](../../app/ratel/src/features/admin/controllers/migrations/backfill_space_score_rank.rs) 패턴 참고.

```rust
#[post("/api/admin/migrations/backfill-hot-scores", role: Role::Admin)]
pub async fn backfill_hot_scores_handler() -> Result<String> {
    // GSI6 페이지네이션으로 SpaceCommon row 전수 순회.
    // 각 스페이스에 대해:
    //   - SpaceAction::find_by_space로 액션 카운트 (1회성이므로 N+1 허용)
    //   - SpaceActionCount row를 절대값으로 시딩 (최초 생성, ADD 아님)
    //   - participants*SCORE_DELTA_PARTICIPANT + total_actions*SCORE_DELTA_ACTION
    //     으로 hot_score 산출, SpaceHotScore row 시딩
    //   - last_activity_at = space.updated_at (근사값)
    // 처리/실패 건수 반환.
}
```

실행 순서: **3리전 모두 GSI7 ACTIVE → 앱 배포(이벤트 핸들러 가동) → 관리자가 백필 엔드포인트 호출**. 백필은 `with_hot_score(...)` / `with_*_count(...)` (절대값 setter)를 사용하므로 재실행 시 멱등 — 시드값을 덮어쓸 뿐 중복 카운트가 발생하지 않는다. 백필 중 들어오는 라이브 이벤트는 시드값 위에 ADD 되므로 저트래픽 윈도우를 선택하거나 이벤트 누적으로 희석되는 유계 overcounting을 수용한다.

---

## 동시성 보장

DynamoDB `UpdateItem`의 `ADD` 액션은 Number 속성에 대해 원자적이다. `by-macros`가 생성하는 `increase_*(delta)` / `decrease_*(delta)` setter가 이를 컴파일해 낸다. 이로써:

- **Lost update 없음.** 같은 밀리초에 두 파이프라인이 발화해도 두 delta가 합산된다. 핫 패스 어디에도 read-modify-write 없음.
- **GSI sort key 일관성 유지.** `hot_score` 자체가 sort key 속성이므로 한 번의 `ADD`가 메인 아이템과 GSI projection을 원자적으로 함께 이동시킨다. 별도 `hot_score_sk` 필드가 초래했을 "점수는 갱신됐는데 인덱스는 아직" 구간이 없다.
- **이벤트 순서 비의존성.** 순서가 뒤바뀐 이벤트도 delta 부호만 맞으면 결과가 정확하다 (REMOVE = `-20`). 늦게 도착한 이벤트는 현재 값을 그냥 변화시킬 뿐.
- **부호 오류만 유계 영향.** 핸들러가 부호를 잘못 적용해도 이벤트당 `|delta|` 이내로만 오차 발생. cascading 부패 없음.

원자 ADD로 처리하지 **않는** 경계 케이스:

- **재시도 시 멱등성.** EventBridge는 가끔 같은 이벤트를 두 번 전달한다. 중복된 `SpaceAction INSERT`는 이중 카운트가 될 수 있다. 구현 시점 결정 옵션: (a) 이벤트 `id` 필드를 TTL 있는 소형 `ProcessedEvent` DynamoDB 엔티티의 dedupe key로 사용, (b) EventBridge at-least-once를 랭킹 수준 노이즈로 수용. MVP는 (b) — 관측 상 랭크 drift가 눈에 띌 때만 (a) 도입.
- **백필 vs 라이브 이벤트 교차.** 백필 섹션 참고 — 백필은 절대값 setter, 라이브는 `ADD`. 백필 윈도우 중 라이브 이벤트가 시드값 위에 더해지면 영구적으로 보정되지 않는 유계 overcounting. 저트래픽 윈도우에 실행하거나 drift를 수용.

---

## Phase

### Phase 0 — `by-macros` Number sort key 지원

나머지 전체의 선결 조건. 매크로 변경을 독립 리뷰하기 위해 **별도 PR로 분리**해서 머지.

- [ ] [packages/by-macros/src/dynamo_entity/mod.rs](../../packages/by-macros/src/dynamo_entity/mod.rs)의 속성 파서에 `as_number` 플래그 추가 → `#[dynamo(index = "...", sk, as_number)]`
- [ ] `as_number`가 설정되면 SK 쓰기 시 `AttributeValue::S(...)` 대신 `AttributeValue::N(sk.to_string())` 생성, 읽기 시에도 N을 기대 (`AttributeValue::S(sk` grep 결과 약 15곳)
- [ ] Number SK에서도 `scan_index_forward=false`가 정상 동작하는지 검증 (이미 기본값 — 코드 변경 없이 확인만)
- [ ] `scripts/create-indexes.sh`에 인덱스별 SK 타입(`S` 또는 `N`) 지원 추가; 기본 `S`라서 기존 GSI 엔트리는 영향 없음
- [ ] `packages/by-macros/tests/`에 Number SK를 가진 토이 엔티티로 round-trip(put → desc query → 순서 검증) 테스트 1개 추가
- [ ] `cargo check -p by-macros` 통과; `app-shell` `cargo check --features server` 통과 (기존 엔티티 회귀 없음)

### Phase 1 — 엔티티와 순수 함수 (AWS 변경 없음)

- [ ] `EntityType::SpaceHotScore`, `EntityType::SpaceActionCount` enum variant 추가 (EntityType이 선언된 위치)
- [ ] `app/ratel/src/features/activity/models/space_hot_score.rs` 생성 — 구조체, `keys()`, `bucket_for()`
- [ ] `app/ratel/src/features/activity/models/space_action_count.rs` 생성 — 구조체와 `keys()`
- [ ] `app/ratel/src/features/activity/services/hot_score.rs` 생성 — `bump_hot_score()`와 `SCORE_DELTA_*` 상수
- [ ] `app/ratel/src/features/activity/services/action_count.rs` 생성 — `bump_action_count()`
- [ ] `bump_*` 래퍼 유닛 테스트 (부호 처리, `SpaceActionType`별 올바른 필드 라우팅) — DynamoDB mock 또는 `ratel-local` 통합
- [ ] `cargo check --features server` 가 경고 0으로 통과

### Phase 2 — AWS 인프라

- [ ] `scripts/create-indexes.sh`에 `"gsi7_pk gsi7_sk gsi7-index S N"` 추가 (PK=String, SK=Number)
- [ ] `ratel-dev` 대상으로 먼저 실행; `gsi7-index` 상태 `ACTIVE` 확인
- [ ] 프로덕션: `ap-northeast-2` → `us-east-1` → `eu-central-1` (스크립트가 리전별 완료 대기)
- [ ] `cdk/lib/dynamo-stream-event.ts`에 `SpaceParticipantPipe` + `SpaceActionCountPipe` + 대응 `events.Rule` 추가
- [ ] `cd cdk && npx tsc --noEmit` 통과
- [ ] dev 스택에 CDK 배포; 파이프가 `RUNNING` 상태인지 확인

### Phase 3 — 이벤트 핸들러

- [ ] `event_bridge_envelope.rs`에 `DetailType::SpaceParticipantJoin`, `DetailType::SpaceActionCountUpdate` variant + `proc()` 분기 추가
- [ ] 참여자 가입 핸들러: `bump_hot_score(space, +SCORE_DELTA_PARTICIPANT, now)` 호출
- [ ] 액션 카운트 핸들러: `bump_action_count(..., delta)` 후 `bump_hot_score(space, delta * SCORE_DELTA_ACTION, now)` (단일 delta 부호가 양쪽을 결정)
- [ ] `stream_handler.rs`에 두 분기 미러링
- [ ] 결정 사항: `fan_out_popular_space`/`aggregate_score`는 수정 없이 유지 (참여자 경로는 이미 `SpaceParticipantPipe`에서 반영) → 중복 카운트 방지
- [ ] `cargo check --features "server,lambda"` 통과

### Phase 4 — 백필

- [ ] `features/admin/controllers/migrations/backfill_hot_scores.rs`에 `backfill_hot_scores_handler` 구현 — 절대값 `with_*` setter 사용
- [ ] admin 라우터에 라우트 등록
- [ ] 시드 데이터셋이 있는 `ratel-dev` 대상으로 실행; ≥ 20개 스페이스에서 랭킹 쿼리가 예상 순서를 반환하는지 확인
- [ ] 저트래픽 윈도우에 프로덕션 실행

### Phase 5 — 읽기 경로 컷오버

- [ ] `list_hot_spaces_handler`를 GSI7 쿼리 + 3× 병렬 `batch_get` 구현으로 재작성
- [ ] `list_hot_spaces.rs`에서 `activity_score()` 삭제
- [ ] `list_hot_spaces.rs`와 `list_my_home_spaces.rs`에서 `count_actions()` 삭제
- [ ] `list_my_home_spaces_handler`가 `count_actions` 대신 `SpaceActionCount::batch_get`을 쓰도록 업데이트
- [ ] `app/ratel/src/tests/hot_score_tests.rs` 통합 테스트: 콜드 스타트 → 빈 응답; 참여 가입이 랭크 증가; 액션 INSERT/REMOVE가 랭크 이동; HIDDEN 버킷 row는 제외
- [ ] `cargo check --features web` + `dx check --web` 통과
- [ ] `cd playwright && CI=true npx playwright test tests/web/home.spec.js` 통과

### Phase 6 — 정리

- [ ] 선행 PR에서 `list_hot_spaces.rs` / `list_my_home_spaces.rs`에 추가한 TODO 주석 제거
- [ ] `.claude/rules/conventions/`에 `SpaceHotScore` + `SpaceActionCount` 패턴 문서화 (원조 선례로 `Post::increase_likes` 참조)
- [ ] 3개 리전 모두에서 `dev` 브랜치 Hot 캐러셀 동작 확인

---

## 미결 질문 (구현 중 해소)

1. **점수 가중치 튜닝.** `+10`(참여자) / `+20`(액션)은 초기 추정치. 론칭 후 실제 분포를 관측해 `services/hot_score.rs` 상수만 조정 — DB 마이그레이션 불필요(이후 이벤트에만 적용).
2. **`PopularSpacePipe` 재사용 vs 신규 `SpaceParticipantPipe`?** 기존 파이프는 `SpaceCommon` MODIFY의 `participants` 변경에서 발화한다. 참여자 가입 경로가 이미 `SpaceCommon`을 업데이트한다면 그 이벤트에 편승해 hot-score 로직을 얹어 신규 파이프를 제거할 수 있다. 대신 hot-score 로직이 타임라인 fan-out과 결합된다. Phase 2 초반에 `create_participant`를 확인해 결정.
3. **`gsi6` 재사용 vs 신규 `gsi7`?** `SpaceHotScore`는 고유 PK prefix(`HOT#PUB_PUB`)를 쓰면 `SpaceCommon`/`Post`/기타와 `gsi6`을 공유할 수 있음 — DB 변경 제로. 트레이드오프: 이미 바쁜 GSI의 파티션 hot-spot, `gsi6` SK가 String 타입이라 Number SK 장점 상실. 기본은 신규 `gsi7` — Phase 0 매크로 PR이 막힐 때만 대안 고려.
4. **Visibility 변경 rebucket.** Phase 1–5는 보류: 스페이스가 Public ↔ Private/Draft 전환 시 `SpaceHotScore.bucket`은 다음 참여/액션 이벤트까지 오래된 값을 유지. 런칭 시점엔 수용 가능(대부분의 전환은 일방향 Public). 가시 버그가 되면 후속으로 `SpaceCommonVisibilityChangePipe` 전용 파이프 추가.
5. **주기적 decay.** 감쇠 없이 한때 인기였던 죽은 스페이스가 잔존. 후속 과제: 실사용 1–2개월 후 랭킹이 stale하게 느껴지면 EventBridge Scheduler → decay Lambda 추가.
6. **중복 이벤트 처리.** EventBridge는 at-least-once. MVP에서는 재시도 시 유계 overcounting 수용 — 실측 drift가 문제되면 그때 재검토.

---

## 롤아웃 안전성

- 모든 쓰기는 **신규** 엔티티로 향한다. `SpaceCommon`, `SpaceAction`, `SpaceParticipant`의 기존 읽기/쓰기는 변하지 않는다.
- 기존 EventBridge 필터 중 신규 엔티티를 매치하는 것은 없다 — 기존 파이프라인은 영향받지 않는다.
- Phase 0(매크로 변경)은 격리되어 있고 현재 엔티티에 런타임 영향 없음; `as_number`를 옵트인한 엔티티만 새 경로를 사용.
- 백필은 재실행 시 시드값을 덮어쓴다 (절대값 setter) → 조용한 윈도우 내에서 멱등.
- 롤백 경로: Phase 5 커밋 revert → 홈은 선행 PR의 요청 시점 정렬로 폴백. 고아가 된 `SpaceHotScore`/`SpaceActionCount` row는 포워드 롤 때까지 무해.
