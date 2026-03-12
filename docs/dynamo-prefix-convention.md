# DynamoEntity GSI Prefix Convention

All models share a single DynamoDB table (`{DYNAMO_TABLE_PREFIX}-main`). The `#[dynamo(prefix = "...", index = "gsiN", pk/sk)]` attribute on `DynamoEntity` fields determines how GSI partition/sort keys are composed. Because every model writes to the same set of GSIs, **prefixes must be chosen carefully to avoid key collisions**.

## Rules

1. **Model-specific prefixes must be unique abbreviations** that identify the model, not generic domain terms.
2. **Cross-model prefixes** (e.g., `USER_PK`, `LIKE`, `TS`) are allowed only when multiple models intentionally share the same query pattern.
3. **Never reuse** a model-specific prefix that is already claimed by another model on the same GSI.
4. **Keep abbreviations short** but recognizable (2-4 characters for model-specific, full name for cross-model).

## Prefix Registry

### Model-Specific Prefixes

These prefixes are **owned by a single model** (or its mirror in `packages/main-api`). No other model may use them on the same GSI.

| Prefix | Model | GSI | Role | Generated Query |
|--------|-------|-----|------|-----------------|
| `SA` | `SpaceAction` | gsi1 | pk | `find_by_space` |
| `SP` | `SpaceParticipant` | gsi1 | pk | `find_by_user` |
| `SP` | `SpaceParticipant` | gsi2 | pk | `find_by_space` |
| `SP` | `SpaceParticipant` | gsi3 | pk | `search_users_by_space` |
| `UM` | `UserMembership` | gsi1 | pk | `find_by_membership` |
| `TM` | `TeamMembership` | gsi1 | pk | `find_by_membership` |
| `REL` | `UserRelationship` | gsi1 | sk | — |
| `REQ` | `SpaceRequirement` | gsi1 | pk | `find_by_order` |
| `QUIZ_USER` | `SpaceQuizAttempt` | gsi1 | pk | `find_by_quiz_user` |
| `POLL_PK` | `SpacePollUserAnswer` | gsi1 | pk | `find_by_space_pk` |
| `FILE_URL` | `FileLink` | gsi1 | pk | `find_by_file_url` |
| `SPACE_COMMON_VIS` | `SpaceCommon` | gsi6 | pk | `find_by_visibility` |
| `EMAIL` | `User` | gsi3 | pk | `find_by_email` |
| `USERNAME` | `User` | gsi2 | pk | `find_by_username` |
| `PHONE` | `User` | gsi5 | pk | `find_by_phone` |
| `USER_TYPE` | `User` | gsi4 | pk+sk | `find_by_user_type` |
| `TEAM` | `Team` | gsi2 | sk | — |
| `TEAM_PK` | `UserTeam` | gsi1 | pk | `find_by_team` |
| `INFO` | `VerifiedAttributes`, `Topic`, `ExampleData` | gsi1 | sk/pk | `find_by_data`, `find_by_info_prefix` |

### Cross-Model Prefixes (Shared by Design)

These prefixes are **intentionally reused** across multiple models to enable uniform query patterns.

| Prefix | Purpose | GSI | Models |
|--------|---------|-----|--------|
| `USER_PK` | Find all entities created by a user | gsi1 pk | `Post`, `PostComment`, `PostRepost`, `TeamOwner`, `SpaceCommon`, `SpacePostComment`, `SpaceDiscussionMember`, `SpaceDiscussionParticipant`, `SpacePanelParticipant` |
| `POST_PK` | Find all entities referencing a post | gsi2 pk | `PostRepost`, `SpaceCommon` |
| `LIKE` | Find all likes by a user | gsi1 pk | `PostLike`, `PostCommentLike`, `SpacePostCommentLike` |
| `TS` | Time-based sort key for range queries | various sk | `User`, `Post`, `SpaceAction`, `UserMembership`, `TeamMembership`, `EmailVerification`, `PhoneVerification`, `TeamPurchase`, `TopicArticle`, `TopicArticleReply` |
| `PAYMENT` | Payment-related queries | gsi1-3 pk/sk | `UserPurchase`, `TeamPurchase` |
| `USER_VISIBILITY` | Find posts by user + visibility | gsi3 pk | `Post` |
| `USER_STATUS` | Find posts by user + status | gsi5 pk | `Post` |

## How to Choose a Prefix for a New Model

1. **Check this registry** — is the prefix already taken on the same GSI?
2. If you are indexing a field that represents a **relationship to another entity** (e.g., `user_pk`, `post_pk`), use the corresponding cross-model prefix (`USER_PK`, `POST_PK`, etc.).
3. If you are creating a **model-specific index**, derive a short abbreviation from the model name:
   - `SpaceAction` → `SA`
   - `SpaceParticipant` → `SP`
   - `UserMembership` → `UM`
   - `TeamMembership` → `TM`
   - `SpaceRequirement` → `REQ`
4. **Avoid generic domain words** like `SPACE`, `USER`, `POST` as model-specific prefixes — they are too likely to collide.
5. **Update this document** when adding a new prefix.

## Bad vs Good Examples

```rust
// BAD: "SPACE" is too generic — SpaceCommon, SpaceParticipant, or any
// future space model could also need gsi1 with a space-based prefix.
#[dynamo(prefix = "SPACE", name = "find_by_space", index = "gsi1", pk)]
pub space_pk: Partition,

// GOOD: "SA" uniquely identifies SpaceAction.
#[dynamo(prefix = "SA", name = "find_by_space", index = "gsi1", pk)]
pub space_pk: Partition,
```

```rust
// BAD: Using a model-specific prefix for a cross-model pattern.
#[dynamo(prefix = "POST_AUTHOR", name = "find_by_user_pk", index = "gsi1", pk)]
pub user_pk: Partition,

// GOOD: Reuse the shared cross-model prefix.
#[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
pub user_pk: Partition,
```
