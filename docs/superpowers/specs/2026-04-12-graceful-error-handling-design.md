# Graceful Error Handling Refactoring

## Goal

Remove 5 parameterized error variants from `common::Error` and replace all ~290 usages with feature-specific error enums following the project's error handling conventions:
- Unit variant errors only (no `String` parameters)
- `Translate` derive for user-friendly messages (EN + KO)
- `crate::error!` for server-side logging before returning unit errors

## Variants to Remove from `common::Error`

| Variant | Approx Usages |
|---------|--------------|
| `Unknown(String)` | ~70 |
| `InternalServerError(String)` | ~90 |
| `BadRequest(String)` | ~85 |
| `Unauthorized(String)` | ~35 |
| `NotSupported(String)` | ~10 |

## New Feature-Specific Error Enums

### `features/auth/types/error.rs` — `AuthError`

| Variant | Replaces | HTTP Status |
|---------|----------|-------------|
| `InvalidCredentials` | `Unauthorized("Invalid email or password")` | 401 |
| `InvalidSignature` | `Unauthorized("Invalid signature")`, `BadRequest("Invalid signature: ...")` | 401 |
| `NonceMismatch` | `Unauthorized("Nonce mismatch")` | 401 |
| `NonceNotFound` | `Unauthorized("No nonce found in session")` | 401 |
| `TokenRevoked` | `Unauthorized("Token revoked")` | 401 |
| `TokenExpired` | `Unauthorized("Token expired")` | 401 |
| `InvalidRefreshToken` | `Unauthorized("Invalid refresh token")` | 401 |
| `InvalidTelegramData` | `Unauthorized("Invalid telegram data")` | 401 |
| `UserNotFound` | `Unauthorized("User not found ...")` | 401 |
| `PhoneNotFound` | `Unauthorized("Phone number not found ...")` | 401 |
| `SessionFailed` | `Unknown("Session error: ...")` | 500 |
| `InvalidInput` | `BadRequest("Invalid input: ...")` | 400 |
| `InvalidSignatureHex` | `BadRequest("Invalid signature hex: ...")` | 400 |
| `InvalidRecoveryId` | `BadRequest("Invalid recovery id: ...")` | 400 |
| `PublicKeyRecoveryFailed` | `BadRequest("Failed to recover public key: ...")` | 400 |
| `SignatureLengthInvalid` | `BadRequest("Invalid signature length: ...")` | 400 |
| `WalletConnectFailed` | `Unknown("Sign message failed: ...")`, etc. | 500 |
| `UserInfoParseFailed` | `Unknown("Failed to parse UserInfo: ...")` | 500 |
| `EmailTemplateFailed` | `InternalServerError("Failed to serialize email template data")` | 500 |
| `TelegramBotTokenMissing` | `InternalServerError("TELEGRAM_BOT_TOKEN not set")` | 500 |

### `features/posts/types/error.rs` — `PostError`

| Variant | Replaces | HTTP Status |
|---------|----------|-------------|
| `InvalidAuthor` | `InternalServerError("Invalid post author")` | 500 |
| `LikeFailed` | `InternalServerError("Failed to like post")` | 500 |
| `UnlikeFailed` | `InternalServerError("Failed to unlike post")` | 500 |
| `CommentFailed` | `InternalServerError("Failed to add comment")` | 500 |
| `CommentLikeFailed` | `InternalServerError("Failed to like comment")` | 500 |
| `CommentUnlikeFailed` | `InternalServerError("Failed to unlike comment")` | 500 |
| `ReplyFailed` | `InternalServerError("Failed to reply")` | 500 |
| `InvalidCommentKey` | `BadRequest("comment_sk must be a PostComment")` | 400 |
| `InvalidPostContent` | `BadRequest("Content is too short ...")` | 400 |
| `HasDependencies` | `BadRequest("Has dependencies")` | 400 |
| `InvalidTeamContext` | `BadRequest("No team found ...")` | 400 |
| `TeamNotFound` | `BadRequest("Team not found ...")` | 400 |
| `CategoryNameRequired` | `BadRequest("Category name is required")` | 400 |
| `ListFailed` | `InternalServerError("{context}: {err}")` | 500 |
| `PostNotAccessible` | `Unauthorized("Post is not accessible ...")` | 401 |

### `features/social/types/error.rs` — `SocialError`

| Variant | Replaces | HTTP Status |
|---------|----------|-------------|
| `TeamDeleteFailed` | `InternalServerError("Failed to delete team")` | 500 |
| `GroupDeleteFailed` | `InternalServerError("Failed to delete group")` | 500 |
| `InvalidGender` | `BadRequest("Invalid gender")` | 400 |
| `InvalidMembershipTier` | `BadRequest("Invalid membership tier")` | 400 |
| `PasswordTooShort` | `BadRequest("Password must be at least ...")` | 400 |
| `PasswordMismatch` | `BadRequest("Password does not match")` | 400 |
| `IncorrectCurrentPassword` | `Unauthorized("Current password is incorrect")` | 401 |
| `InvalidTeamName` | `BadRequest("team name contains invalid chars")` | 400 |
| `TeamNameTaken` | `BadRequest("team name already taken")` | 400 |
| `PortOneRequestFailed` | `Unknown(e.to_string())` from portone | 500 |
| `PortOneBadStatus` | `BadRequest("PortOne ... status ...")` | 400 |
| `DaoRegistrationFailed` | `Unknown("Failed to register DAO: ...")` | 500 |
| `WalletConnectFailed` | `Unknown(format_js_error(e))` | 500 |
| `InvalidVerificationAttribute` | `BadRequest("Invalid attribute ...")` | 400 |

### `features/membership/types/error.rs` — `MembershipPaymentError`

| Variant | Replaces | HTTP Status |
|---------|----------|-------------|
| `InvalidCurrency` | `BadRequest("invalid currency: ...")` | 400 |
| `MissingCardInfo` | `BadRequest("Card info required")` | 400 |
| `MissingBillingKey` | `BadRequest("Missing billing key")` | 400 |
| `PortOneRequestFailed` | `BadRequest(err.to_string())` from portone | 400 |
| `PortOnePaymentFailed` | `BadRequest("PortOne payment ... failed")` | 400 |
| `PortOneScheduleFailed` | `BadRequest("PortOne schedule ... failed")` | 400 |
| `PortOneVerifyFailed` | `BadRequest("PortOne verify ... failed")` | 400 |
| `PortOneCancelFailed` | `BadRequest("PortOne cancel ... failed")` | 400 |
| `WebhookProcessingFailed` | `Unknown("Webhook ...")` | 500 |
| `AwsConversionFailed` | `Unknown("AWS error: ...")` | 500 |
| `SessionConversionFailed` | `Unknown("Session error: ...")` | 500 |

### `features/timeline/types/error.rs` — `TimelineError`

| Variant | Replaces | HTTP Status |
|---------|----------|-------------|
| `FanOutFailed` | `InternalServerError("Failed to write timeline entries")` | 500 |
| `InvalidUser` | `BadRequest("Invalid user")` | 400 |
| `InvalidBookmark` | `BadRequest(e)` from bookmark parse | 400 |

### `features/admin/types/error.rs` — `AdminError`

| Variant | Replaces | HTTP Status |
|---------|----------|-------------|
| `UsernameRequired` | `BadRequest("Username is required")` | 400 |
| `InvalidBookmark` | `BadRequest("Invalid bookmark: ...")` | 400 |

### `common/services/error.rs` — `ServiceError`

For `common/services/biyard/` and `common/services/icp/`:

| Variant | Replaces | HTTP Status |
|---------|----------|-------------|
| `BiyardApiRequestFailed` | `Unknown(e.to_string())` from HTTP | 500 |
| `BiyardApiBadStatus` | `BadRequest("Biyard API returned status ...")` | 400 |
| `BiyardApiEmptyResponse` | `Unknown("Biyard API returned empty response")` | 500 |
| `IcpAgentFailed` | `InternalServerError("IC agent error: ...")` | 500 |
| `IcpCallFailed` | `InternalServerError("IC call error: ...")`| 500 |
| `IcpQueryFailed` | `InternalServerError("IC query error: ...")` / `Unknown(...)` | 500 |
| `IcpCandidEncodeFailed` | `InternalServerError("Candid encode error: ...")` / `Unknown(...)` | 500 |
| `IcpCandidDecodeFailed` | `InternalServerError("Candid decode error: ...")` / `Unknown(...)` | 500 |
| `IcpRootKeyFailed` | `InternalServerError("IC root key error: ...")` | 500 |
| `InvalidCanisterId` | `BadRequest("Invalid RATEL_CANISTER_ID: ...")` | 400 |
| `OAuthRequestFailed` | `InternalServerError("OAuth request failed: ...")` | 500 |
| `OAuthParseFailed` | `InternalServerError("OAuth response parse failed: ...")` | 500 |

### `common/utils/error.rs` �� `InfraError`

For `common/utils/aws/s3.rs`, `common/utils/aws/bedrock_embeddings.rs`, `common/stream_handler.rs`:

| Variant | Replaces | HTTP Status |
|---------|----------|-------------|
| `S3OperationFailed` | `InternalServerError(e.to_string())` (all S3 ops) | 500 |
| `BedrockSerializeFailed` | `InternalServerError("Failed to serialize embedding request: ...")` | 500 |
| `BedrockInvokeFailed` | `InternalServerError("Bedrock invoke_model failed: ...")` | 500 |
| `BedrockParseFailed` | `InternalServerError("Failed to parse embedding response: ...")` | 500 |
| `BedrockNoEmbedding` | `InternalServerError("No embedding in response")` | 500 |
| `StreamDeserializeFailed` | `InternalServerError("stream deserialize: ...")` | 500 |
| `StreamMissingImage` | `InternalServerError("Missing image in stream event")` | 500 |
| `QdrantFailed` | `InternalServerError("Qdrant error: ...")` (From impl) | 500 |

### `common/components/file_uploader/error.rs` — `FileUploadError`

| Variant | Replaces | HTTP Status |
|---------|----------|-------------|
| `UnsupportedFileType` | `NotSupported("File type not supported ...")` | 400 |
| `FileSizeLimitExceeded` | `InternalServerError("File size exceeds ...")` | 400 |
| `UploadFailed` | `Unknown(js_error_to_string(e))` (multiple) | 500 |
| `InvalidUploadResponse` | `Unknown("Invalid upload response.")` | 500 |

### Expand Existing Enums

#### `features/my_follower/types/error.rs` — `FollowError`

Add:
- `FollowFailed` — replaces `Unknown("Failed to follow user")`
- `UnfollowFailed` — replaces `Unknown("Failed to unfollow user")`

(Existing `InvalidTarget` and `CannotUnfollowSelf` already cover the `BadRequest` usages)

#### `features/activity/types/error.rs` — `ActivityError`

Add:
- `ScoreLoadFailed` — replaces `InternalServerError("failed to load my score: ...")`
- `RankingLoadFailed` — replaces `InternalServerError("failed to load ranking scores: ...")`

#### `common/types/mcp_error.rs` — `McpServerError`

Add:
- `OneshotRoutingFailed` — replaces `InternalServerError("MCP oneshot ...")` (all 5 usages in oneshot.rs)

#### `features/spaces/types/error.rs` — `SpaceError`

Add for `space_common/`:
- `EmailVerificationFailed` — replaces `InternalServerError("Failed to update verifications")`
- `EmailSendFailed` — replaces `InternalServerError("Failed to send ...")`
- `RewardDistributionFailed` — replaces `Unknown("reward distribution ...")`
- `StartNowNotSupported` — replaces `BadRequest("it does not support start now")`
- `FinishNowNotSupported` — replaces `BadRequest("it does not support finish now")`
- `UpdateFailed` — replaces `InternalServerError("Failed to get post")`
- `InvalidPanelQuota` — replaces `BadRequest("Invalid panel quota")`

### New Sub-Module Error Enums Under Spaces

#### `features/spaces/pages/actions/types/error.rs` — `SpaceActionError`

| Variant | HTTP Status |
|---------|-------------|
| `ActionLoadFailed` | 500 |
| `ActionUpdateFailed` | 500 |
| `ActionDeleteFailed` | 500 |
| `InvalidTimeRange` | 400 |
| `MembershipCheckFailed` | 500 |
| `TransactionFailed` | 500 |
| `RewardTemplateFailed` | 500 |

#### `features/spaces/pages/actions/actions/poll/types/error.rs` — `SpacePollError`

| Variant | HTTP Status |
|---------|-------------|
| `PollNotInProgress` | 400 |
| `AnswerMismatch` | 400 |
| `EditNotAllowed` | 400 |
| `QuestionsEmpty` | 400 |
| `InvalidTimeRange` | 400 |
| `InvalidQuestionFormat` | 400 |
| `VoteVerificationFailed` | 500 |
| `CreateFailed` | 500 |
| `EncryptionFailed` | 500 |
| `DecryptionFailed` | 500 |

#### `features/spaces/pages/actions/actions/follow/types/error.rs` — `SpaceFollowError`

| Variant | HTTP Status |
|---------|-------------|
| `CannotFollowSelf` | 400 |
| `InvalidTarget` | 400 |
| `FollowFailed` | 500 |
| `UnfollowFailed` | 500 |
| `CreatorCannotBeRemoved` | 400 |
| `CreateFailed` | 500 |
| `InvalidFollowTarget` | 400 |

#### `features/spaces/pages/apps/types/error.rs` — `SpaceAppError`

| Variant | HTTP Status |
|---------|-------------|
| `InstallFailed` | 500 |
| `UninstallFailed` | 500 |
| `DeployFailed` | 500 |
| `InvalidEvmAddress` | 400 |
| `ArchiveProviderFailed` | 500 |
| `ArchiveBlockFailed` | 500 |
| `ArchiveLogsFailed` | 500 |
| `ExcelExportFailed` | 500 |
| `ChartRenderFailed` | 500 |
| `CopyTextFailed` | 500 |
| `PanelQuotaCreateFailed` | 500 |
| `PanelQuotaDeleteFailed` | 500 |
| `InvalidInvitationEmail` | 400 |
| `CreatorCannotBeRemoved` | 400 |
| `IncentiveAddressRequired` | 400 |
| `IncentiveChainRequired` | 400 |

#### `features/spaces/pages/report/types/error.rs` — `SpaceReportError`

| Variant | HTTP Status |
|---------|-------------|
| `AnalyzeLoadFailed` | 500 |
| `AnalyzeUpdateFailed` | 500 |

#### `features/spaces/pages/overview/types/error.rs` — `SpaceOverviewError`

| Variant | HTTP Status |
|---------|-------------|
| `ContentUpdateFailed` | 500 |

### Expand `SpaceActionDiscussionError`

Add:
- `CreateFailed` | 500
- `DeleteFailed` | 500
- `InvalidDiscussionId` | 400
- `CommentTooShort` | 400

### Expand `SpaceActionQuizError`

Add:
- `CreateFailed` | 500

## Changes to `common::Error`

### Remove variants
- `Unknown(String)`
- `NotSupported(String)`
- `InternalServerError(String)`
- `BadRequest(String)`
- `Unauthorized(String)`

### Add unit variants (cross-cutting)
- `Internal` — generic 500 for `From<String>`, `From<ServerFnError>`, `From<base64::DecodeError>`
- `UnsupportedOperation` — for entity_type.rs and user_setting not-supported-on-server
- `MissingSpaceId` — for space_common.rs model
- `InvalidFormat` — for reward history key parsing
- `InvalidTeamContext` — for contexts/team_context.rs team-related validation
- `UserNotFoundInContext` — for contexts/team_context.rs user lookup
- `SpaceUserRoleFailed` — for space_user_role.rs

### Add `#[from]` + `#[translate(from)]` for new enums
```rust
#[error("{0}")]
#[translate(from)]
Auth(#[from] AuthError),

#[error("{0}")]
#[translate(from)]
Post(#[from] PostError),

#[error("{0}")]
#[translate(from)]
Social(#[from] SocialError),

#[error("{0}")]
#[translate(from)]
MembershipPayment(#[from] MembershipPaymentError),

#[error("{0}")]
#[translate(from)]
Timeline(#[from] TimelineError),

#[error("{0}")]
#[translate(from)]
Admin(#[from] AdminError),

#[error("{0}")]
#[translate(from)]
Service(#[from] ServiceError),

#[error("{0}")]
#[translate(from)]
Infra(#[from] InfraError),

#[error("{0}")]
#[translate(from)]
FileUpload(#[from] FileUploadError),

#[error("{0}")]
#[translate(from)]
SpaceAction(#[from] SpaceActionError),

#[error("{0}")]
#[translate(from)]
SpacePoll(#[from] SpacePollError),

#[error("{0}")]
#[translate(from)]
SpaceFollow(#[from] SpaceFollowError),

#[error("{0}")]
#[translate(from)]
SpaceApp(#[from] SpaceAppError),

#[error("{0}")]
#[translate(from)]
SpaceReport(#[from] SpaceReportError),

#[error("{0}")]
#[translate(from)]
SpaceOverview(#[from] SpaceOverviewError),
```

### Update `From` impls

```rust
// From<String> — log + return Internal
impl From<String> for Error {
    fn from(s: String) -> Self {
        tracing::error!("Untyped string error: {s}");
        Error::Internal
    }
}

// From<ServerFnError>
impl From<ServerFnError> for Error {
    fn from(e: ServerFnError) -> Self {
        tracing::error!("Server function error: {e}");
        Error::Internal
    }
}

// From<base64::DecodeError>
impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        tracing::error!("Base64 decode error: {e}");
        Error::Internal
    }
}

// From<QdrantError> — delegate to InfraError
impl From<qdrant_client::QdrantError> for Error {
    fn from(e: qdrant_client::QdrantError) -> Self {
        tracing::error!("Qdrant error: {e}");
        Error::Infra(InfraError::QdrantFailed)
    }
}
```

### Update `IntoResponse` / `AsStatusCode` / `Into<rmcp::ErrorData>`

Remove match arms for deleted variants. Add match arms for new `#[from]` variants delegating to `e.status_code()`.

### Update client-side matching

`features/auth/components/login_modal/mod.rs:135` currently matches `Err(Error::Unauthorized(_))` — change to `Err(Error::Auth(..))` or a broader pattern.

## Call Site Pattern

```rust
// Before (leaking details):
.map_err(|e| Error::InternalServerError(format!("failed to do X: {e}")))?;

// After (log + unit error):
.map_err(|e| {
    crate::error!("failed to do X: {e}");
    MyFeatureError::XFailed
})?;
```

## Error Enum Template

Every new error enum follows this structure:

```rust
use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum MyFeatureError {
    #[error("internal description")]
    #[translate(en = "User-facing message", ko = "사용자 메시지")]
    VariantName,
}

#[cfg(feature = "server")]
impl MyFeatureError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            MyFeatureError::VariantName => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for MyFeatureError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for MyFeatureError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
```
