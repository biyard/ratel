# Reward System Design

## Overview

Ratel의 Reward 시스템은 사용자의 특정 행동에 대해 포인트 보상을 지급하는 시스템입니다. 실제 포인트 관리와 토큰 발급은 **Biyard** 서비스에서 담당하며, Ratel은 Biyard API를 호출하여 포인트를 지급하고 트랜잭션 정보를 저장합니다.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         Ratel API                            │
│                                                              │
│  ┌──────────────┐      ┌──────────────┐     ┌────────────┐ │
│  │ Space Reward │─────▶│ User Reward  │────▶│   Biyard   │ │
│  │   (Config)   │      │  (Instance)  │     │  Service   │ │
│  └──────────────┘      └──────────────┘     └────────────┘ │
│         │                      │                    │       │
│         │                      │                    │       │
│    [DynamoDB]            [DynamoDB]          [External API] │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Space Reward (스페이스 보상 설정)

Space 별로 어떤 행동에 대해 얼마의 포인트를 지급할지 설정합니다.

**Location**: `packages/main-api/src/features/spaces/rewards/models/space_reward.rs`

**Data Structure**:
```rust
pub struct SpaceReward {
    pub pk: CompositePartition,        // SPACE#{space_id}#REWARD
    pub sk: RewardType,                // 보상 타입 (예: RespondPoll#POLL123)

    pub created_at: i64,
    pub updated_at: i64,
    pub label: String,                 // 보상 라벨 (예: "Poll Response Reward")
    pub description: String,           // 보상 설명
    pub amount: i64,                   // 지급할 포인트 양
}
```

**Key Features**:
- Space 관리자가 생성/수정/삭제 가능
- 각 Space마다 독립적인 보상 설정
- RewardType에 따라 자동으로 기본 포인트 계산 (`reward_type.point() * credit`)

### 2. Reward Type (보상 유형)

**Location**: `packages/main-api/src/features/spaces/rewards/types/reward_type.rs`

```rust
pub enum RewardType {
    None,
    RespondPoll(String),  // Poll SK를 포함
    // 향후 확장 가능:
    // CreatePost,
    // WriteComment,
    // AttendMeeting,
    // CompleteTask,
}
```

**Point Calculation**:
- `RespondPoll`: 기본 10,000 포인트
- Space Reward 생성 시 `amount = reward_type.point() * credit`

### 3. User Reward (사용자 보상 기록)

사용자가 실제로 받은 보상을 기록합니다.

**Location**: `packages/main-api/src/features/spaces/rewards/models/user_reward.rs`

**Data Structure**:
```rust
pub struct UserReward {
    pub pk: CompositePartition,        // USER#{user_id}#REWARD
    pub sk: EntityType,                // UserReward(space_pk, reward_type)

    pub created_at: i64,
    pub month: String,                 // "2024-12" 형식
    pub transaction_id: String,        // Biyard 트랜잭션 ID
}
```

**Key Method**:
```rust
impl UserReward {
    pub async fn award(
        cli: &aws_sdk_dynamodb::Client,
        biyard: &Biyard,
        user_pk: Partition,
        space_reward: SpaceReward,
    ) -> Result<UserReward>
}
```

### 4. Biyard Service Integration

**Location**: `packages/main-api/src/services/biyard/biyard.rs`

Biyard는 외부 포인트 관리 서비스로, 다음 기능을 제공합니다:

**Configuration** (`config/biyard_config.rs`):
```rust
pub struct BiyardConfig {
    pub api_secret: &'static str,      // BIYARD_API_SECRET
    pub project_id: &'static str,      // BIYARD_PROJECT_ID
    pub base_url: &'static str,        // BIYARD_API_URL
}
```

**Main Methods**:

1. **포인트 지급** (`award_points`)
   ```rust
   pub async fn award_points(
       &self,
       user_pk: Partition,
       points: i64,
       description: String,
       month: Option<String>,
   ) -> Result<AwardPointResponse>
   ```
   - POST `/projects/{project_id}/points`
   - Returns: `transaction_id`, `month`

2. **잔액 조회** (`get_user_balance`)
   ```rust
   pub async fn get_user_balance(
       &self,
       user_pk: Partition,
       month: String,
   ) -> Result<UserPointBalanceResponse>
   ```
   - GET `/projects/{project_id}/points/{user_id}?month={month}`
   - Returns: balance, total_earned, total_spent

3. **트랜잭션 내역** (`get_user_transactions`)
   ```rust
   pub async fn get_user_transactions(
       &self,
       user_pk: Partition,
       month: String,
       bookmark: Option<String>,
       limit: Option<i32>,
   ) -> Result<ListItemsResponse<UserPointTransactionResponse>>
   ```
   - GET `/projects/{project_id}/points/{user_id}/transactions?month={month}`
   - Supports pagination with bookmark

## API Endpoints

### Space Reward Management

**Base Path**: `/v3/spaces/{space_pk}/rewards`

1. **POST /** - Create Reward
   - Permission: `SpaceWrite`
   - Body:
     ```json
     {
       "reward_type": "RespondPoll#POLL123",
       "label": "Poll Response Reward",
       "description": "Reward for responding to poll",
       "credit": 100  // amount = 10,000 * 100 = 1,000,000
     }
     ```

2. **GET /** - List Rewards
   - Permission: `SpaceRead`
   - Query: `?bookmark={bookmark}`
   - Returns: List of SpaceRewardResponse

3. **PUT /** - Update Reward
   - Permission: `SpaceEdit`
   - Body:
     ```json
     {
       "reward_type": "RespondPoll#POLL123",
       "label": "Updated Label",
       "description": "Updated Description",
       "amount": 2000000
     }
     ```

4. **DELETE /** - Delete Reward
   - Permission: `SpaceDelete`
   - Body:
     ```json
     {
       "reward_sk": "RespondPoll#POLL123"
     }
     ```

### User Points

**Base Path**: `/v3/me/points`

1. **GET /balance** - Get My Point Balance
   - Query: `?month=2024-12`
   - Returns: UserPointBalanceResponse (from Biyard)

2. **GET /transactions** - List My Transactions
   - Query: `?month=2024-12&bookmark={bookmark}`
   - Returns: ListItemsResponse<UserPointTransactionResponse>

## Reward Flow

### 보상 지급 프로세스

```
1. User performs action (예: 설문 응답)
   ↓
2. Action Handler에서 SpaceReward 조회
   ↓
3. UserReward::award() 호출
   ↓
4. Biyard API를 통해 포인트 지급
   ↓
5. transaction_id 받아서 UserReward 생성
   ↓
6. DynamoDB에 UserReward 저장
   ↓
7. 완료
```

### Example Implementation

```rust
// 설문 응답 시 보상 지급 예시
pub async fn respond_to_poll_handler(
    State(AppState { dynamo, biyard, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<RespondPollRequest>,
) -> Result<Json<PollResponseData>> {
    // 1. 설문 응답 처리
    let poll_response = process_poll_response(&dynamo.client, &user, &req).await?;

    // 2. 해당 행동에 대한 SpaceReward 조회
    let reward_type = RewardType::RespondPoll(poll_response.poll_sk.clone());
    if let Some(space_reward) = SpaceReward::get_reward(
        &dynamo.client,
        &space_pk,
        &reward_type
    ).await? {
        // 3. 보상 지급
        match UserReward::award(
            &dynamo.client,
            &biyard,
            user.pk.clone(),
            space_reward,
        ).await {
            Ok(user_reward) => {
                tracing::info!("Rewarded user: {} with transaction_id: {}",
                    user.pk, user_reward.transaction_id);
            },
            Err(e) => {
                tracing::error!("Failed to reward user: {:?}", e);
                // 보상 실패해도 메인 로직은 성공 처리
            }
        }
    }

    Ok(Json(poll_response))
}
```

## Database Schema

### DynamoDB Tables

**Main Table**: `{prefix}-main`

**SpaceReward Records**:
```
PK: SPACE#{space_id}#REWARD
SK: RespondPoll#{poll_sk}
Attributes: created_at, updated_at, label, description, amount
```

**UserReward Records**:
```
PK: USER#{user_id}#REWARD
SK: UserReward#{space_pk}#{reward_type}
Attributes: created_at, month, transaction_id
```

## Permission Model

### Space Reward Management
- **SpaceWrite**: Space Reward 생성 권한
- **SpaceRead**: Space Reward 목록 조회 권한
- **SpaceEdit**: Space Reward 수정 권한
- **SpaceDelete**: Space Reward 삭제 권한

### User Points
- 자신의 포인트 정보만 조회 가능 (No special permission needed)

## Error Handling

### Biyard API 에러
- 네트워크 오류: Retry 로직 없음 (보상 실패로 처리)
- API 오류: Error 로그 기록, 메인 로직은 계속 진행
- 타임아웃: 60초 (reqwest default)

### DynamoDB 에러
- 조회 실패: 빈 리스트 반환
- 생성 실패: Error 반환
- 중복 생성: ConditionalCheckFailedException (이미 보상 받음)

## Testing

**Test Location**: `packages/main-api/src/controllers/v3/spaces/rewards/tests.rs`

**Test Coverage**:
- ✅ Create reward (authenticated/unauthenticated)
- ✅ List rewards (authenticated/unauthenticated)
- ✅ Update reward (authenticated/unauthenticated)
- ✅ Delete reward (authenticated/unauthenticated)
- ✅ Pagination
- ✅ Space isolation (각 Space의 보상 독립성)

**Run Tests**:
```bash
cd packages/main-api
cargo test rewards::tests -- --nocapture
```

## Future Enhancements

### 1. Additional Reward Types
```rust
pub enum RewardType {
    RespondPoll(String),
    CreatePost,                    // 게시글 작성
    WriteComment,                  // 댓글 작성
    AttendMeeting,                 // 미팅 참석
    CompleteTask(String),          // 과제 완료
    DailyCheckIn,                  // 일일 출석
    InviteMember,                  // 멤버 초대
}
```

### 2. Reward Conditions
```rust
pub struct SpaceReward {
    // ... existing fields
    pub conditions: Option<RewardConditions>,
}

pub struct RewardConditions {
    pub max_per_user: Option<i64>,        // 사용자당 최대 지급 횟수
    pub max_per_day: Option<i64>,         // 일일 최대 지급 횟수
    pub valid_from: Option<i64>,          // 유효 시작 시간
    pub valid_until: Option<i64>,         // 유효 종료 시간
    pub required_role: Option<String>,    // 필요한 역할
}
```

### 3. Reward Analytics
- Space별 보상 통계
- 사용자별 보상 수령 내역
- 월별 보상 지급 리포트

### 4. Bulk Reward
```rust
pub async fn award_bulk(
    cli: &aws_sdk_dynamodb::Client,
    biyard: &Biyard,
    user_pks: Vec<Partition>,
    space_reward: SpaceReward,
) -> Result<Vec<UserReward>>
```

### 5. Reward Notification
- 보상 지급 시 알림 전송
- 이메일/텔레그램 통합
- In-app notification

## Security Considerations

### 1. Rate Limiting
- 동일 사용자가 짧은 시간에 같은 보상을 여러 번 받는 것 방지
- UserReward 생성 시 DynamoDB Conditional Check 사용

### 2. Idempotency
- 같은 행동에 대해 중복 보상 방지
- UserReward PK/SK로 중복 체크

### 3. Authorization
- Space 보상 설정은 Space 관리자만 가능
- 포인트 조회는 본인만 가능

### 4. Validation
- 포인트 양은 양수여야 함
- Month 형식 검증 ("YYYY-MM")
- RewardType 형식 검증

## Monitoring & Logging

### Key Metrics
- 보상 지급 성공률
- Biyard API 응답 시간
- 보상 지급 실패 건수
- Space별 보상 지급 통계

### Logging Points
```rust
// 보상 지급 성공
tracing::info!(
    user_pk = %user.pk,
    space_pk = %space_pk,
    reward_type = %reward_type,
    amount = space_reward.amount,
    transaction_id = %user_reward.transaction_id,
    "User rewarded successfully"
);

// 보상 지급 실패
tracing::error!(
    user_pk = %user.pk,
    space_pk = %space_pk,
    reward_type = %reward_type,
    error = ?e,
    "Failed to reward user"
);
```

## Environment Variables

```bash
# Biyard Service Configuration
BIYARD_API_SECRET=your_api_secret_here
BIYARD_PROJECT_ID=ratel_project_id
BIYARD_API_URL=https://api.biyard.co

# For development
BIYARD_API_URL=https://dev.biyard.co

# For local testing
BIYARD_API_URL=http://localhost:3003
```

## Migration Notes

현재 구현된 기능:
- ✅ SpaceReward CRUD
- ✅ UserReward 생성
- ✅ Biyard API 통합
- ✅ 사용자 포인트 조회 API
- ✅ 트랜잭션 내역 API

구현 필요 사항:
- ⬜ 실제 액션(설문 응답 등)과 보상 지급 연결
- ⬜ 중복 보상 방지 로직
- ⬜ Reward 조건 시스템
- ⬜ 보상 알림 시스템
- ⬜ 통계 및 리포트 기능

---

# Point System Design (New Architecture)

## Overview

기존 Reward 시스템을 개선하여:
- **Space 단위 배수 설정**: Credit을 사용해 Space 전체 Reward에 배수 적용
- **빌드 타임 Point 관리**: 코드 레벨에서 base point 정의 및 변경 이력 추적
- **배수 설정 시점 제한**: Space 시작 전에만 배수 설정/변경 가능
- **단순화된 구조**: RespondPoll만 우선 구현, 향후 확장 가능하도록 설계

## Core Concepts

### 1. Point (포인트)
- **정의**: 서비스 개발 주체가 빌드 타임에 특정 행위에 대한 보상을 고정한 재화
- **예시**:
  - 투표 참여 (RespondPoll) = 10,000 Point
  - 게시글 작성 (CreatePost) = 50,000 Point
- **관리**: `RewardType::point()` 메서드로 정의
- **변경 이력**: Git history + 코드 상수로 추적

### 2. Credit (크레딧)
- **정의**: Space 관리자가 사용하는 유료 재화
- **용도**: Space 전체 Reward 배수 설정
- **저장**: `SpaceCommon.rewards` 필드에 Credit 값 저장
- **사용 시점**: Admin이 배수 설정 시 즉시 차감
- **관리**: `UserMembership`에서 Credit 보유량 관리

### 3. Feature (기능 그룹)
- **정의**: Reward를 그룹화하는 계층
- **예시**:
  - Poll Feature → RespondPoll, CommentPoll
  - Post Feature → CreatePost, CreateComment, LikePost
  - SprintLeague Feature → JoinSprint, WinSprint

### 4. SpaceReward (Space별 Reward 활성화)
- **On/Off 관리**: 레코드 존재 여부로 활성화 상태 표현
  - On: SpaceReward 레코드 존재
  - Off: SpaceReward 레코드 없음
- **Default**: 모든 RewardType은 기본적으로 On

### 5. Amount Calculation (보상 계산)
```
final_amount = reward_type.point() × space_common.rewards
```
- 예: RespondPoll (10,000) × Space Credit (100) = 1,000,000 Point
- **조건**: SpaceReward가 존재해야 지급 (On 상태)

## Architecture (3-Layer)

```
┌─────────────────────────────────────────────────────────────┐
│                      Feature Layer                           │
│   ┌────────┐    ┌────────┐    ┌──────────────┐             │
│   │  Poll  │    │  Post  │    │ SprintLeague │             │
│   └────────┘    └────────┘    └──────────────┘             │
│        │             │                 │                     │
└────────┼─────────────┼─────────────────┼─────────────────────┘
         │             │                 │
┌────────┼─────────────┼─────────────────┼─────────────────────┐
│        ▼             ▼                 ▼  RewardType Layer   │
│  ┌──────────┐  ┌──────────┐      ┌──────────┐              │
│  │RespondPoll  │CreatePost│      │JoinSprint│              │
│  └──────────┘  └──────────┘      └──────────┘              │
│       │              │                  │                    │
│       └──────────────┴──────────────────┘                    │
│                      │                                       │
└──────────────────────┼───────────────────────────────────────┘
                       │
┌──────────────────────┼───────────────────────────────────────┐
│                      ▼  SpaceReward Layer                    │
│         ┌─────────────────────────────┐                     │
│         │ SpaceCommon.rewards (배수)  │                     │
│         └─────────────────────────────┘                     │
│                      │                                       │
│         ┌────────────┴────────────┐                         │
│         │                         │                         │
│    ┌────▼─────┐            ┌─────▼────┐                    │
│    │SpaceReward│            │SpaceReward│                   │
│    │(On/Off)  │            │(On/Off)  │                    │
│    └──────────┘            └──────────┘                    │
└─────────────────────────────────────────────────────────────┘
```

## Data Models

### RewardType (Extended)

**Location**: `packages/main-api/src/features/spaces/rewards/types/reward_type.rs`

```rust
pub enum RewardType {
    #[default]
    None,

    // Poll Feature
    RespondPoll(String),      // Poll SK

    // Post Feature (향후 추가)
    CreatePost,
    CreateComment(String),    // Post SK
    LikePost(String),        // Post SK
}

impl RewardType {
    /// Base point amount (빌드 타임 고정)
    pub fn point(&self) -> i64 {
        match self {
            RewardType::None => 0,
            RewardType::RespondPoll(_) => 10_000,
            RewardType::CreatePost => 50_000,
            RewardType::CreateComment(_) => 5_000,
            RewardType::LikePost(_) => 1_000,
        }
    }

    /// Feature 이름 반환
    pub fn feature(&self) -> &'static str {
        match self {
            RewardType::None => "None",
            RewardType::RespondPoll(_) => "Poll",
            RewardType::CreatePost | RewardType::CreateComment(_)
                | RewardType::LikePost(_) => "Post",
        }
    }

    /// 버전 관리 (빌드 타임 변경 추적)
    pub fn version(&self) -> u32 {
        match self {
            RewardType::RespondPoll(_) => 1,  // Point 변경 시 증가
            RewardType::CreatePost => 1,
            _ => 0,
        }
    }

    /// Point 변경 이력 (디버깅/감사용)
    /// Format: (version, effective_date, point_amount, change_reason)
    pub fn point_history(&self) -> &'static [(u32, &'static str, i64, &'static str)] {
        match self {
            RewardType::RespondPoll(_) => &[
                (1, "2024-12-09", 10_000, "Initial implementation"),
                // 향후 변경:
                // (2, "2025-01-15", 15_000, "Increased to boost engagement"),
            ],
            _ => &[],
        }
    }

    /// Feature별 모든 RewardType 반환
    pub fn all_for_feature(feature: &str) -> Vec<RewardType> {
        match feature {
            "Poll" => vec![
                RewardType::RespondPoll(String::new()),
            ],
            "Post" => vec![
                RewardType::CreatePost,
                RewardType::CreateComment(String::new()),
                RewardType::LikePost(String::new()),
            ],
            _ => vec![],
        }
    }

    /// UI 표시용 메타데이터
    pub fn metadata(&self) -> RewardTypeMetadata {
        match self {
            RewardType::RespondPoll(_) => RewardTypeMetadata {
                type_name: "RespondPoll",
                label: "투표 참여",
                description: "투표에 참여하면 포인트를 받습니다",
                feature: "Poll",
                requires_target: true,  // Poll SK 필요
                base_point: 10_000,
            },
            RewardType::CreatePost => RewardTypeMetadata {
                type_name: "CreatePost",
                label: "게시글 작성",
                description: "게시글을 작성하면 포인트를 받습니다",
                feature: "Post",
                requires_target: false,
                base_point: 50_000,
            },
            // ... 기타 types
        }
    }
}

pub struct RewardTypeMetadata {
    pub type_name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub feature: &'static str,
    pub requires_target: bool,
    pub base_point: i64,
}
```

### SpaceCommon (Extended)

**Location**: `packages/main-api/src/features/spaces/models/space_common.rs`

```rust
pub struct SpaceCommon {
    // ... existing fields

    /// Space 전체 Reward 배수 (Credit)
    /// - None: Reward 시스템 비활성화
    /// - Some(100): 모든 Reward에 100배 적용
    pub rewards: Option<i64>,  // ✅ Already exists at line 66
}
```

### SpaceReward (No Change)

기존 구조 유지:
```rust
pub struct SpaceReward {
    pub pk: CompositePartition,     // SPACE#{space_id}#REWARD
    pub sk: RewardType,             // RespondPoll, CreatePost, etc.

    pub created_at: i64,
    pub updated_at: i64,
    pub label: String,              // "투표 참여 보상"
    pub description: String,        // "투표에 참여하면 포인트를 받습니다"
    pub amount: i64,                // Cache: reward_type.point() × space_common.rewards
}
```

**의미**:
- **존재함**: Reward 활성화 (On)
- **존재하지 않음**: Reward 비활성화 (Off)

## New API Endpoints

### 1. Space 배수 설정

**Endpoint**: `PUT /v3/spaces/{space_pk}/rewards/multiplier`

**Request**:
```json
{
  "credit": 100
}
```

**Logic**:
1. Space Admin 권한 확인
2. `UserMembership.use_credits(100)` - Credit 차감
3. `SpaceCommon.rewards = Some(100)` - 배수 설정
4. 기존 SpaceReward들의 `amount` 재계산

**Response**:
```json
{
  "success": true,
  "space_pk": "SPACE#123",
  "multiplier": 100,
  "updated_rewards_count": 5
}
```

### 2. Feature별 Reward 목록 조회

**Endpoint**: `GET /v3/spaces/{space_pk}/features/{feature}/rewards/available`

**Example**: `GET /v3/spaces/SPACE#123/features/Poll/rewards/available`

**Response**:
```json
{
  "items": [
    {
      "reward_type": "RespondPoll",
      "label": "투표 참여",
      "description": "투표에 참여하면 포인트를 받습니다",
      "base_point": 10000,
      "version": 1,
      "feature": "Poll",
      "is_enabled": true,           // SpaceReward 존재 여부
      "user_participated": false     // UserReward 존재 여부 (optional user)
    }
  ]
}
```

**Use Case**:
- Frontend에서 Feature별 Reward 설정 UI 표시
- Admin이 각 Reward를 On/Off 토글 가능
- User가 자신의 참여 여부 확인 가능

### 3. Reward On/Off 토글 (Modified)

**Enable Reward**: `POST /v3/spaces/{space_pk}/rewards`

**Request**:
```json
{
  "reward_type": "RespondPoll"
}
```

**변경사항**:
- 기존: `credit` 파라미터로 개별 Reward에 배수 적용
- 신규: `SpaceCommon.rewards`의 배수를 사용하여 `amount` 계산
- `label`, `description`은 `metadata()`에서 자동 채움

**Disable Reward**: `DELETE /v3/spaces/{space_pk}/rewards`

**Request**:
```json
{
  "reward_sk": "RespondPoll"
}
```

## Key Flows

### Flow 1: Admin이 Reward 시스템 활성화

```
1. Admin → PUT /v3/spaces/{space_pk}/rewards/multiplier { credit: 100 }
2. Backend:
   - UserMembership.use_credits(100) ✓
   - SpaceCommon.rewards = Some(100)
   - 기존 SpaceReward들의 amount 재계산:
     - RespondPoll: 10,000 × 100 = 1,000,000
3. Response: Success
```

### Flow 2: Admin이 특정 Reward 비활성화

```
1. Admin → DELETE /v3/spaces/{space_pk}/rewards?reward_type=CreateComment
2. Backend:
   - SpaceReward 레코드 삭제
3. Response: Success
4. 이후 CreateComment 액션은 Point 지급 안됨 (SpaceReward 없음)
```

### Flow 3: User가 액션 수행 (투표 참여)

```
1. User → POST /v3/spaces/{space_pk}/polls/{poll_sk}/respond
2. Backend:
   - 투표 처리 완료
   - UserReward.has_been_awarded() 체크
     → 이미 있으면 Skip
   - SpaceReward 존재 확인
     → 없으면 Skip (비활성화됨)
   - amount = RespondPoll.point() × SpaceCommon.rewards
            = 10,000 × 100 = 1,000,000
   - Biyard.award_points(user, 1000000) → transaction_id
   - UserReward 생성 (transaction_id 저장)
3. Response: Success (+ point 지급 정보)
```

### Flow 4: Frontend에서 Feature별 Reward 설정 UI 표시

```
1. Frontend → GET /v3/spaces/{space_pk}/features/Poll/rewards/available
2. Backend:
   - RewardType::all_for_feature("Poll") → [RespondPoll]
   - SpaceReward 조회 → is_enabled 체크
   - UserReward 조회 → user_participated 체크 (optional)
3. Response: [
     {
       reward_type: "RespondPoll",
       label: "투표 참여",
       base_point: 10000,
       is_enabled: true,
       user_participated: false
     }
   ]
4. Frontend: Toggle UI 표시 (Admin), Badge UI 표시 (User)
```

## Implementation Phases

### Phase 1: Backend - RewardType 확장
- [ ] `feature()`, `version()`, `point_history()` 메서드 추가
- [ ] `all_for_feature()` - Feature별 RewardType 목록
- [ ] `metadata()` - UI 메타데이터
- [ ] `RewardTypeMetadata` 구조체 정의

### Phase 2: Backend - Space 배수 설정 API
- [ ] `set_multiplier_handler` 구현
  - Admin 권한 체크
  - Credit 차감
  - SpaceCommon.rewards 업데이트
  - SpaceReward amounts 재계산
- [ ] Route 등록: `PUT /v3/spaces/{space_pk}/rewards/multiplier`

### Phase 3: Backend - Feature별 Reward 목록 API
- [ ] `list_available_rewards_handler` 구현
- [ ] Route 등록: `GET /v3/spaces/{space_pk}/features/{feature}/rewards/available`

### Phase 4: Backend - Reward On/Off 토글
- [ ] `create_reward_handler` 수정
  - Credit 개별 차감 제거
  - Space 배수 사용
  - Metadata 자동 채우기
- [ ] `delete_reward_handler` 검증

### Phase 5: Backend - 중복 지급 방지 및 Point 지급
- [ ] `UserReward::has_been_awarded()` 메서드
- [ ] 각 액션 핸들러에 Point 지급 로직 추가
  - 중복 체크
  - SpaceReward 존재 확인
  - Biyard API 호출
  - UserReward 기록

### Phase 6: Frontend - Reward 설정 UI
- [ ] Space settings에 Rewards 탭 추가
  - 배수 설정 UI
  - Feature별 Reward 목록
  - On/Off 토글
  - Credit 잔액 표시

### Phase 7: Frontend - User Point UI
- [ ] Point 잔액 페이지
- [ ] Point 거래 내역 페이지

### Phase 8: Testing
- [ ] Unit tests
- [ ] Integration tests
- [ ] E2E tests (Playwright)

## Design Decisions

### 1. Feature 계층 구조
**장점**:
- Frontend에서 Feature별로 UI 구성 가능
- 새 Feature 추가 시 확장 용이
- 코드 레벨 명확한 그룹화

### 2. Space 단위 배수
**장점**:
- 모든 Reward에 일괄 적용
- Credit 1회 차감으로 전체 활성화
- 배수 변경 시 자동 재계산

### 3. SpaceReward 레코드 = On/Off
**장점**:
- enabled 필드 불필요
- 비활성화된 Reward는 DB 공간 절약
- 조회 로직 단순

### 4. 빌드 타임 Point 관리
**장점**:
- DB 스키마 변경 불필요
- Git으로 변경 이력 추적
- 컴파일 타임 최적화
- 디버깅/감사 가능

### 5. 중복 지급 방지
**Two-Layer**:
1. Application: UserReward 레코드 확인
2. Service: Biyard API idempotency key

### 6. Metadata 기반 UI
**장점**:
- Backend = Single Source of Truth
- Frontend는 API로 받아서 표시
- 일관성 보장

## Critical Files

### Backend - 수정 필요
- `packages/main-api/src/features/spaces/rewards/types/reward_type.rs`
- `packages/main-api/src/features/spaces/rewards/models/user_reward.rs`

### Backend - 새로 생성
- `packages/main-api/src/controllers/v3/spaces/rewards/set_multiplier.rs`
- `packages/main-api/src/controllers/v3/spaces/features/list_available_rewards.rs`

### Backend - 로직 추가
- `packages/main-api/src/controllers/v3/spaces/rewards/create_reward.rs`
- `packages/main-api/src/controllers/v3/spaces/polls/respond_poll.rs`

### Frontend - 새로 생성
- `ts-packages/web/src/app/spaces/[space_id]/settings/rewards/`
- `ts-packages/web/src/app/me/points/`

## Open Questions

1. **RewardType 확장 범위**: 어떤 액션들에 Point를 지급할 계획인가요?
   - CreatePost, CreateComment, LikePost?
   - SprintLeague 관련 액션?

2. **배수 변경 정책**:
   - 이미 지급된 Point는 소급 적용하지 않음?
   - 앞으로 지급될 Point만 새 배수 적용?

3. **Default Reward 생성 시점**:
   - Space 생성 시 모든 RewardType을 자동으로 SpaceReward로 생성?
   - Admin이 배수 설정 시 생성?

---

## References

- **SpaceReward Model**: `packages/main-api/src/features/spaces/rewards/models/space_reward.rs`
- **UserReward Model**: `packages/main-api/src/features/spaces/rewards/models/user_reward.rs`
- **RewardType**: `packages/main-api/src/features/spaces/rewards/types/reward_type.rs`
- **Biyard Service**: `packages/main-api/src/services/biyard/biyard.rs`
- **Biyard Config**: `packages/main-api/src/config/biyard_config.rs`
- **Reward APIs**: `packages/main-api/src/controllers/v3/spaces/rewards/`
- **Points APIs**: `packages/main-api/src/controllers/v3/me/points/`
- **Tests**: `packages/main-api/src/controllers/v3/spaces/rewards/tests.rs`
