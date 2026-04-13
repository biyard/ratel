# Graceful Error Handling Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove 5 parameterized error variants (`Unknown(String)`, `NotSupported(String)`, `InternalServerError(String)`, `BadRequest(String)`, `Unauthorized(String)`) from `common::Error` and replace all ~290 usages with feature-specific error enums that use unit variants + `Translate` derive.

**Architecture:** Each feature module gets its own error enum (or expands an existing one). Call sites change from `Error::BadRequest("msg".into())` to `crate::error!("detail"); FeatureError::Variant`. The `common::Error` enum gains `#[from]` + `#[translate(from)]` wrappers for each feature error. `From<String>`, `From<ServerFnError>`, and `From<base64::DecodeError>` impls are updated to log + return a generic `Error::Internal` unit variant.

**Tech Stack:** Rust, thiserror, serde, dioxus-translate (`Translate` derive), bdk/axum (status codes)

**Spec:** `docs/superpowers/specs/2026-04-12-graceful-error-handling-design.md`

**Error enum boilerplate template** (referenced by all tasks as "the template"):
```rust
use crate::common::*;
pub use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum XxxError {
    #[error("internal description")]
    #[translate(en = "User-facing EN", ko = "사용자 KO")]
    VariantName,
}

#[cfg(feature = "server")]
impl XxxError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            XxxError::VariantName => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for XxxError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for XxxError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
```

---

### Task 1: Create `ServiceError` enum for `common/services/`

**Files:**
- Create: `app/ratel/src/common/services/error.rs`
- Modify: `app/ratel/src/common/services/mod.rs`
- Modify: `app/ratel/src/common/services/biyard/mod.rs`
- Modify: `app/ratel/src/common/services/icp/mod.rs`
- Modify: `app/ratel/src/common/types/oauth_provider.rs`

- [ ] **Step 1: Create `common/services/error.rs`**

Use the template. Define `ServiceError` with these variants:

```rust
#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum ServiceError {
    #[error("Biyard API request failed")]
    #[translate(en = "Service request failed", ko = "서비스 요청에 실패했습니다.")]
    BiyardApiRequestFailed,

    #[error("Biyard API returned bad status")]
    #[translate(en = "Service returned an error", ko = "서비스에서 오류가 반환되었습니다.")]
    BiyardApiBadStatus,

    #[error("Biyard API returned empty response")]
    #[translate(en = "Service returned empty response", ko = "서비스에서 빈 응답이 반환되었습니다.")]
    BiyardApiEmptyResponse,

    #[error("ICP agent initialization failed")]
    #[translate(en = "Blockchain service unavailable", ko = "블록체인 서비스를 사용할 수 없습니다.")]
    IcpAgentFailed,

    #[error("ICP call failed")]
    #[translate(en = "Blockchain call failed", ko = "블록체인 호출에 실패했습니다.")]
    IcpCallFailed,

    #[error("ICP query failed")]
    #[translate(en = "Blockchain query failed", ko = "블록체인 조회에 실패했습니다.")]
    IcpQueryFailed,

    #[error("Candid encode failed")]
    #[translate(en = "Data encoding failed", ko = "데이터 인코딩에 실패했습니다.")]
    IcpCandidEncodeFailed,

    #[error("Candid decode failed")]
    #[translate(en = "Data decoding failed", ko = "데이터 디코딩에 실패했습니다.")]
    IcpCandidDecodeFailed,

    #[error("ICP root key fetch failed")]
    #[translate(en = "Blockchain service unavailable", ko = "블록체인 서비스를 사용할 수 없습니다.")]
    IcpRootKeyFailed,

    #[error("Invalid canister ID")]
    #[translate(en = "Invalid blockchain configuration", ko = "잘못된 블록체인 설정입니다.")]
    InvalidCanisterId,

    #[error("OAuth request failed")]
    #[translate(en = "Authentication service failed", ko = "인증 서비스에 실패했습니다.")]
    OAuthRequestFailed,

    #[error("OAuth response parse failed")]
    #[translate(en = "Authentication service failed", ko = "인증 서비스에 실패했습니다.")]
    OAuthParseFailed,
}
```

Status codes: `BiyardApiBadStatus` and `InvalidCanisterId` → 400, all others → 500.

- [ ] **Step 2: Register in `common/services/mod.rs`**

Add `pub mod error;` and `pub use error::*;`.

- [ ] **Step 3: Update `common/services/biyard/mod.rs`**

Replace all `Error::Unknown(e.to_string())` with `{ crate::error!("Biyard API: {e}"); ServiceError::BiyardApiRequestFailed }`.
Replace all `Error::BadRequest(format!("Biyard ... status ..."))` with `{ crate::error!("Biyard API bad status: ..."); ServiceError::BiyardApiBadStatus }`.
Replace all `Error::Unknown("Biyard API returned empty response" ...)` with `ServiceError::BiyardApiEmptyResponse`.

- [ ] **Step 4: Update `common/services/icp/mod.rs`**

Replace all ICP-related `Error::Unknown(...)` and `Error::InternalServerError(...)` and `Error::BadRequest(...)` with the corresponding `ServiceError` variants. Add `crate::error!` before each for detail logging. Server-gated functions (`#[cfg(feature = "server")]`) and client functions must both be updated.

- [ ] **Step 5: Update `common/types/oauth_provider.rs`**

Replace `Error::InternalServerError("OAuth request failed: ...")` with `{ crate::error!("OAuth: {e}"); ServiceError::OAuthRequestFailed }` and similarly for parse.

- [ ] **Step 6: Compile check**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web`

- [ ] **Step 7: Commit**

---

### Task 2: Create `InfraError` enum for `common/utils/`

**Files:**
- Create: `app/ratel/src/common/utils/error.rs`
- Modify: `app/ratel/src/common/utils/mod.rs`
- Modify: `app/ratel/src/common/utils/aws/s3.rs`
- Modify: `app/ratel/src/common/utils/aws/bedrock_embeddings.rs`
- Modify: `app/ratel/src/common/stream_handler.rs`

- [ ] **Step 1: Create `common/utils/error.rs`**

Use the template. Define `InfraError`:

```rust
#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum InfraError {
    #[error("S3 operation failed")]
    #[translate(en = "File storage operation failed", ko = "파일 저장소 작업에 실패했습니다.")]
    S3OperationFailed,

    #[error("Bedrock serialization failed")]
    #[translate(en = "AI service failed", ko = "AI 서비스에 실패했습니다.")]
    BedrockSerializeFailed,

    #[error("Bedrock invocation failed")]
    #[translate(en = "AI service failed", ko = "AI 서비스에 실패했습니다.")]
    BedrockInvokeFailed,

    #[error("Bedrock response parse failed")]
    #[translate(en = "AI service failed", ko = "AI 서비스에 실패했습니다.")]
    BedrockParseFailed,

    #[error("Bedrock returned no embedding")]
    #[translate(en = "AI service failed", ko = "AI 서비스에 실패했습니다.")]
    BedrockNoEmbedding,

    #[error("Stream deserialization failed")]
    #[translate(en = "Data processing failed", ko = "데이터 처리에 실패했습니다.")]
    StreamDeserializeFailed,

    #[error("Stream missing image data")]
    #[translate(en = "Data processing failed", ko = "데이터 처리에 실패했습니다.")]
    StreamMissingImage,

    #[error("Qdrant operation failed")]
    #[translate(en = "Search service failed", ko = "검색 서비스에 실패했습니다.")]
    QdrantFailed,
}
```

All status codes → 500.

- [ ] **Step 2: Register in `common/utils/mod.rs`**

Add `pub mod error;` and `pub use error::*;`.

- [ ] **Step 3: Update `common/utils/aws/s3.rs`**

Replace all `Error::InternalServerError(e.to_string())` with `{ crate::error!("S3: {e}"); InfraError::S3OperationFailed }`.

- [ ] **Step 4: Update `common/utils/aws/bedrock_embeddings.rs`**

Replace each `Error::InternalServerError(...)` with the corresponding `InfraError::Bedrock*` variant + `crate::error!`.

- [ ] **Step 5: Update `common/stream_handler.rs`**

Replace `Error::InternalServerError("stream deserialize: ...")` → `{ crate::error!(...); InfraError::StreamDeserializeFailed }`.
Replace `Error::InternalServerError("Missing ... image ...")` → `InfraError::StreamMissingImage`.

- [ ] **Step 6: Compile check**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web`

- [ ] **Step 7: Commit**

---

### Task 3: Create `FileUploadError` enum for `common/components/file_uploader/`

**Files:**
- Create: `app/ratel/src/common/components/file_uploader/error.rs`
- Modify: `app/ratel/src/common/components/file_uploader/mod.rs`

- [ ] **Step 1: Create `common/components/file_uploader/error.rs`**

Use the template. Define `FileUploadError`:

```rust
#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum FileUploadError {
    #[error("Unsupported file type")]
    #[translate(en = "This file type is not supported", ko = "지원되지 않는 파일 형식입니다.")]
    UnsupportedFileType,

    #[error("File size limit exceeded")]
    #[translate(en = "File size exceeds the allowed limit", ko = "파일 크기가 허용 한도를 초과했습니다.")]
    FileSizeLimitExceeded,

    #[error("Upload failed")]
    #[translate(en = "File upload failed", ko = "파일 업로드에 실패했습니다.")]
    UploadFailed,

    #[error("Invalid upload response")]
    #[translate(en = "File upload failed", ko = "파일 업로드에 실패했습니다.")]
    InvalidUploadResponse,
}
```

All status codes → 400 for `UnsupportedFileType`/`FileSizeLimitExceeded`, 500 for `UploadFailed`/`InvalidUploadResponse`.

- [ ] **Step 2: Update `file_uploader/mod.rs`**

Add `mod error; pub use error::*;` at the top.
Replace `Error::NotSupported(...)` → `FileUploadError::UnsupportedFileType.into()`.
Replace `Error::Unknown(js_error_to_string(e))` → `{ crate::error!("upload: {}", js_error_to_string(e)); FileUploadError::UploadFailed.into() }`.
Replace `Error::Unknown("Invalid upload response.")` → `FileUploadError::InvalidUploadResponse.into()`.
Replace `Error::InternalServerError("File size exceeds ...")` → `FileUploadError::FileSizeLimitExceeded.into()`.

- [ ] **Step 3: Compile check**
- [ ] **Step 4: Commit**

---

### Task 4: Expand `McpServerError` with `OneshotRoutingFailed`

**Files:**
- Modify: `app/ratel/src/common/types/mcp_error.rs`
- Modify: `app/ratel/src/common/mcp/oneshot.rs`

- [ ] **Step 1: Add variant to `McpServerError`**

```rust
#[error("MCP oneshot routing failed")]
#[translate(en = "Internal routing failed", ko = "내부 라우팅에 실패했습니다.")]
OneshotRoutingFailed,
```

Add `McpServerError::OneshotRoutingFailed => StatusCode::INTERNAL_SERVER_ERROR` to `status_code()`.

- [ ] **Step 2: Update `common/mcp/oneshot.rs`**

Replace all 5 `Error::InternalServerError(...)` with `{ crate::error!("MCP oneshot: ..."); McpServerError::OneshotRoutingFailed }`. Use `.into()` if the return type is `common::Error`.

- [ ] **Step 3: Compile check**
- [ ] **Step 4: Commit**

---

### Task 5: Expand `FollowError` for `features/my_follower/`

**Files:**
- Modify: `app/ratel/src/features/my_follower/types/error.rs`
- Modify: `app/ratel/src/features/my_follower/controllers/follow_user.rs`
- Modify: `app/ratel/src/features/my_follower/controllers/unfollow_user.rs`

- [ ] **Step 1: Add variants to `FollowError`**

```rust
#[error("Follow operation failed")]
#[translate(en = "Failed to follow user", ko = "팔로우에 실패했습니다.")]
FollowFailed,

#[error("Unfollow operation failed")]
#[translate(en = "Failed to unfollow user", ko = "언팔로우에 실패했습니다.")]
UnfollowFailed,
```

Both → `StatusCode::INTERNAL_SERVER_ERROR`.

- [ ] **Step 2: Update `follow_user.rs`**

Replace `Error::BadRequest("Invalid target")` → `FollowError::InvalidTarget.into()` (already exists in enum).
Replace `Error::Unknown("Failed to follow user")` → `{ crate::error!("follow failed"); FollowError::FollowFailed.into() }`.

- [ ] **Step 3: Update `unfollow_user.rs`**

Replace `Error::BadRequest("Cannot unfollow yourself")` → `FollowError::CannotUnfollowSelf.into()` (already exists).
Replace `Error::BadRequest("Invalid target")` → `FollowError::InvalidTarget.into()`.
Replace `Error::Unknown("Failed to unfollow user")` → `{ crate::error!("unfollow failed"); FollowError::UnfollowFailed.into() }`.

- [ ] **Step 4: Compile check**
- [ ] **Step 5: Commit**

---

### Task 6: Expand `ActivityError` for `features/activity/`

**Files:**
- Modify: `app/ratel/src/features/activity/types/error.rs`
- Modify: `app/ratel/src/features/activity/controllers/get_my_score.rs`

- [ ] **Step 1: Add variants to `ActivityError`**

```rust
#[error("Score load failed")]
#[translate(en = "Failed to load score", ko = "점수 로드에 실패했습니다.")]
ScoreLoadFailed,

#[error("Ranking load failed")]
#[translate(en = "Failed to load ranking", ko = "랭킹 로드에 실패했습니다.")]
RankingLoadFailed,
```

Both → `StatusCode::INTERNAL_SERVER_ERROR`.

- [ ] **Step 2: Update `get_my_score.rs`**

Replace `Error::InternalServerError("failed to load my score: ...")` → `{ crate::error!("failed to load my score: {e:?}"); ActivityError::ScoreLoadFailed.into() }`.
Replace `Error::InternalServerError("failed to load ranking scores: ...")` → `{ crate::error!("failed to load ranking scores: {e:?}"); ActivityError::RankingLoadFailed.into() }`.

- [ ] **Step 3: Compile check**
- [ ] **Step 4: Commit**

---

### Task 7: Create `AuthError` enum for `features/auth/`

**Files:**
- Create: `app/ratel/src/features/auth/types/error.rs`
- Modify: `app/ratel/src/features/auth/types/mod.rs`
- Modify: `app/ratel/src/features/auth/controllers/login.rs`
- Modify: `app/ratel/src/features/auth/controllers/signup.rs`
- Modify: `app/ratel/src/features/auth/controllers/change_account.rs`
- Modify: `app/ratel/src/features/auth/utils/evm.rs`
- Modify: `app/ratel/src/features/auth/utils/telegram.rs`
- Modify: `app/ratel/src/features/auth/models/email_template.rs`
- Modify: `app/ratel/src/features/auth/interop/mod.rs`
- Modify: `app/ratel/src/features/auth/interop/wallet_connect.rs`
- Modify: `app/ratel/src/features/auth/components/login_modal/mod.rs`

- [ ] **Step 1: Create `features/auth/types/error.rs`**

Use the template. Define `AuthError`:

```rust
#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum AuthError {
    #[error("Invalid credentials")]
    #[translate(en = "Invalid email or password", ko = "이메일 또는 비밀번호가 올바르지 않습니다.")]
    InvalidCredentials,

    #[error("Invalid signature")]
    #[translate(en = "Invalid signature", ko = "유효하지 않은 서명입니다.")]
    InvalidSignature,

    #[error("Nonce mismatch")]
    #[translate(en = "Authentication failed. Please try again.", ko = "인증에 실패했습니다. 다시 시도해주세요.")]
    NonceMismatch,

    #[error("No nonce found in session")]
    #[translate(en = "Session expired. Please try again.", ko = "세션이 만료되었습니다. 다시 시도해주세요.")]
    NonceNotFound,

    #[error("Token revoked")]
    #[translate(en = "Your session has been revoked", ko = "세션이 취소되었습니다.")]
    TokenRevoked,

    #[error("Token expired")]
    #[translate(en = "Your session has expired", ko = "세션이 만료되었습니다.")]
    TokenExpired,

    #[error("Invalid refresh token")]
    #[translate(en = "Invalid session. Please sign in again.", ko = "유효하지 않은 세션입니다. 다시 로그인해주세요.")]
    InvalidRefreshToken,

    #[error("Invalid telegram data")]
    #[translate(en = "Telegram authentication failed", ko = "텔레그램 인증에 실패했습니다.")]
    InvalidTelegramData,

    #[error("User not found")]
    #[translate(en = "User not found", ko = "사용자를 찾을 수 없습니다.")]
    UserNotFound,

    #[error("Phone number not found")]
    #[translate(en = "Phone number not registered", ko = "등록되지 않은 전화번호입니다.")]
    PhoneNotFound,

    #[error("Session operation failed")]
    #[translate(en = "Session error. Please try again.", ko = "세션 오류가 발생했습니다. 다시 시도해주세요.")]
    SessionFailed,

    #[error("Invalid input data")]
    #[translate(en = "Invalid input", ko = "유효하지 않은 입력입니다.")]
    InvalidInput,

    #[error("Invalid signature hex")]
    #[translate(en = "Invalid signature format", ko = "유효하지 않은 서명 형식입니다.")]
    InvalidSignatureHex,

    #[error("Invalid recovery id")]
    #[translate(en = "Invalid signature", ko = "유효하지 않은 서명입니다.")]
    InvalidRecoveryId,

    #[error("Public key recovery failed")]
    #[translate(en = "Signature verification failed", ko = "서명 검증에 실패했습니다.")]
    PublicKeyRecoveryFailed,

    #[error("Signature length invalid")]
    #[translate(en = "Invalid signature length", ko = "유효하지 않은 서명 길이입니다.")]
    SignatureLengthInvalid,

    #[error("WalletConnect operation failed")]
    #[translate(en = "Wallet connection failed", ko = "지갑 연결에 실패했습니다.")]
    WalletConnectFailed,

    #[error("User info parse failed")]
    #[translate(en = "Failed to load user information", ko = "사용자 정보 로드에 실패했습니다.")]
    UserInfoParseFailed,

    #[error("Email template serialization failed")]
    #[translate(en = "Email service failed", ko = "이메일 서비스에 실패했습니다.")]
    EmailTemplateFailed,

    #[error("Telegram bot token missing")]
    #[translate(en = "Telegram service unavailable", ko = "텔레그램 서비스를 사용할 수 없습니다.")]
    TelegramBotTokenMissing,

    #[error("EVM address mismatch")]
    #[translate(en = "Wallet address does not match", ko = "지갑 주소가 일치하지 않습니다.")]
    EvmAddressMismatch,
}
```

Status codes: `InvalidCredentials`, `InvalidSignature`, `NonceMismatch`, `NonceNotFound`, `TokenRevoked`, `TokenExpired`, `InvalidRefreshToken`, `InvalidTelegramData`, `UserNotFound`, `PhoneNotFound`, `EvmAddressMismatch` → 401. `InvalidInput`, `InvalidSignatureHex`, `InvalidRecoveryId`, `PublicKeyRecoveryFailed`, `SignatureLengthInvalid` → 400. `SessionFailed`, `WalletConnectFailed`, `UserInfoParseFailed`, `EmailTemplateFailed`, `TelegramBotTokenMissing` → 500.

- [ ] **Step 2: Register in `auth/types/mod.rs`**

Add `pub mod error; pub use error::*;`.

- [ ] **Step 3: Update `auth/controllers/login.rs`**

Replace every `Error::Unauthorized(...)` with the appropriate `AuthError::*` variant. Replace `Error::Unknown("Session error: ...")` with `{ crate::error!("session: {e}"); AuthError::SessionFailed }`. Use `.into()` for conversion to `common::Error`.

- [ ] **Step 4: Update `auth/controllers/signup.rs`**

Replace `Error::BadRequest("Invalid input: ...")` → `{ crate::error!("signup validation: {e}"); AuthError::InvalidInput.into() }`.
Replace `Error::Unknown("Session error: ...")` → `AuthError::SessionFailed.into()`.
Replace `Error::Unauthorized("No nonce found ...")` → `AuthError::NonceNotFound.into()`.
Replace `Error::Unauthorized("Nonce mismatch")` → `AuthError::NonceMismatch.into()`.
Replace `Error::Unauthorized("Invalid wallet signature")` → `AuthError::InvalidSignature.into()`.

- [ ] **Step 5: Update `auth/controllers/change_account.rs`**

Replace all `Error::Unauthorized(...)` with `AuthError::TokenRevoked`, `AuthError::TokenExpired`, `AuthError::InvalidRefreshToken`, `AuthError::UserNotFound` as appropriate.

- [ ] **Step 6: Update `auth/utils/evm.rs`**

Replace `Error::BadRequest("Invalid signature hex: ...")` → `{ crate::error!("evm sig hex: {e}"); AuthError::InvalidSignatureHex.into() }`.
Replace `Error::BadRequest("Invalid signature length: ...")` → `AuthError::SignatureLengthInvalid.into()`.
Replace `Error::BadRequest("Invalid recovery id: ...")` → `{ crate::error!("evm recovery: {e}"); AuthError::InvalidRecoveryId.into() }`.
Replace `Error::BadRequest("Invalid signature: ...")` → `{ crate::error!("evm sig: {e}"); AuthError::InvalidSignature.into() }`.
Replace `Error::BadRequest("Failed to recover public key: ...")` → `{ crate::error!("evm pubkey: {e}"); AuthError::PublicKeyRecoveryFailed.into() }`.

- [ ] **Step 7: Update `auth/utils/telegram.rs`**

Replace `Error::InternalServerError("TELEGRAM_BOT_TOKEN ...")` → `AuthError::TelegramBotTokenMissing.into()`.

- [ ] **Step 8: Update `auth/models/email_template.rs`**

Replace `Error::InternalServerError("Failed to serialize email template data")` → `{ crate::error!("email template: {e}"); AuthError::EmailTemplateFailed.into() }`.

- [ ] **Step 9: Update `auth/interop/mod.rs` and `auth/interop/wallet_connect.rs`**

Replace all `Error::Unknown(...)` with `AuthError::WalletConnectFailed.into()` or `AuthError::UserInfoParseFailed.into()`. Add `crate::error!` for detail logging.

- [ ] **Step 10: Update `auth/components/login_modal/mod.rs`**

Change `Err(Error::Unauthorized(_))` match to `Err(Error::Auth(..))`.

- [ ] **Step 11: Compile check**
- [ ] **Step 12: Commit**

---

### Task 8: Create `PostError` enum for `features/posts/`

**Files:**
- Create: `app/ratel/src/features/posts/types/error.rs`
- Modify: `app/ratel/src/features/posts/types/mod.rs`
- Modify: `app/ratel/src/features/posts/models/post.rs`
- Modify: `app/ratel/src/features/posts/models/post_comment.rs`
- Modify: `app/ratel/src/features/posts/models/team.rs`
- Modify: `app/ratel/src/features/posts/utils/validator.rs`
- Modify: `app/ratel/src/features/posts/controllers/create_space.rs`
- Modify: `app/ratel/src/features/posts/controllers/get_post.rs`
- Modify: `app/ratel/src/features/posts/controllers/update_post.rs`
- Modify: `app/ratel/src/features/posts/controllers/delete_post.rs`
- Modify: `app/ratel/src/features/posts/controllers/list_posts.rs`
- Modify: `app/ratel/src/features/posts/controllers/create_category.rs`

- [ ] **Step 1: Create `features/posts/types/error.rs`**

Use the template. Define `PostError`:

```rust
#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum PostError {
    #[error("Invalid post author")]
    #[translate(en = "Invalid post author", ko = "유효하지 않은 게시물 작성자입니다.")]
    InvalidAuthor,

    #[error("Failed to like post")]
    #[translate(en = "Failed to like post", ko = "게시물 좋아요에 실패했습니다.")]
    LikeFailed,

    #[error("Failed to unlike post")]
    #[translate(en = "Failed to unlike post", ko = "게시물 좋아요 취소에 실패했습니다.")]
    UnlikeFailed,

    #[error("Failed to add comment")]
    #[translate(en = "Failed to add comment", ko = "댓글 추가에 실패했습니다.")]
    CommentFailed,

    #[error("Failed to like comment")]
    #[translate(en = "Failed to like comment", ko = "댓글 좋아요에 실패했습니다.")]
    CommentLikeFailed,

    #[error("Failed to unlike comment")]
    #[translate(en = "Failed to unlike comment", ko = "댓글 좋아요 취소에 실패했습니다.")]
    CommentUnlikeFailed,

    #[error("Failed to reply")]
    #[translate(en = "Failed to reply", ko = "답글 작성에 실패했습니다.")]
    ReplyFailed,

    #[error("Invalid comment key")]
    #[translate(en = "Invalid comment", ko = "유효하지 않은 댓글입니다.")]
    InvalidCommentKey,

    #[error("Post content too short")]
    #[translate(en = "Post content is too short", ko = "게시물 내용이 너무 짧습니다.")]
    ContentTooShort,

    #[error("Post has dependencies")]
    #[translate(en = "Cannot delete: post has dependencies", ko = "삭제할 수 없습니다: 게시물에 종속 항목이 있습니다.")]
    HasDependencies,

    #[error("No team found for user")]
    #[translate(en = "No team context found", ko = "팀 컨텍스트를 찾을 수 없습니다.")]
    InvalidTeamContext,

    #[error("Team not found")]
    #[translate(en = "Team not found", ko = "팀을 찾을 수 없습니다.")]
    TeamNotFound,

    #[error("Category name required")]
    #[translate(en = "Category name is required", ko = "카테고리 이름이 필요합니다.")]
    CategoryNameRequired,

    #[error("Post list failed")]
    #[translate(en = "Failed to load posts", ko = "게시물 로드에 실패했습니다.")]
    ListFailed,

    #[error("Post not accessible")]
    #[translate(en = "You don't have access to this post", ko = "이 게시물에 접근 권한이 없습니다.")]
    NotAccessible,
}
```

Status codes: `ContentTooShort`, `HasDependencies`, `InvalidCommentKey`, `InvalidTeamContext`, `TeamNotFound`, `CategoryNameRequired` → 400. `NotAccessible` → 401. All others → 500.

- [ ] **Step 2: Register in `posts/types/mod.rs`**

Add `pub mod error; pub use error::*;`.

- [ ] **Step 3: Update all post model and controller files**

Replace each `Error::InternalServerError(...)`, `Error::BadRequest(...)`, `Error::Unauthorized(...)` with the corresponding `PostError::*` variant + `.into()` + `crate::error!` where detail logging is needed.

- [ ] **Step 4: Compile check**
- [ ] **Step 5: Commit**

---

### Task 9: Create `SocialError` enum for `features/social/`

**Files:**
- Create: `app/ratel/src/features/social/types/mod.rs`
- Create: `app/ratel/src/features/social/types/error.rs`
- Modify: `app/ratel/src/features/social/mod.rs`
- Modify: `app/ratel/src/features/social/controllers/team.rs`
- Modify: `app/ratel/src/features/social/pages/setting/controllers/update_team.rs`
- Modify: `app/ratel/src/features/social/pages/setting/controllers/delete_team.rs`
- Modify: `app/ratel/src/features/social/pages/group/controllers/*.rs` (5 files)
- Modify: `app/ratel/src/features/social/pages/credentials/controllers/sign_attributes.rs`
- Modify: `app/ratel/src/features/social/pages/credentials/services/portone.rs`
- Modify: `app/ratel/src/features/social/pages/user_setting/controllers/change_password.rs`
- Modify: `app/ratel/src/features/social/pages/user_setting/controllers/update_user.rs`
- Modify: `app/ratel/src/features/social/pages/user_setting/views/mod.rs`
- Modify: `app/ratel/src/features/social/pages/user_membership/dto/transfer_type.rs`
- Modify: `app/ratel/src/features/social/pages/dao/views/admin_page.rs`
- Modify: `app/ratel/src/features/social/pages/reward/controllers/*.rs` (2 files)

- [ ] **Step 1: Create `features/social/types/error.rs`**

Use the template. Define `SocialError` with variants covering:
- Team ops: `TeamDeleteFailed`, `InvalidTeamName`, `TeamNameTaken`
- Group ops: `GroupDeleteFailed`
- Credentials: `InvalidGender`, `InvalidVerificationAttribute`, `PortOneRequestFailed`, `PortOneBadStatus`
- Password: `PasswordTooShort`, `PasswordMismatch`, `IncorrectCurrentPassword`
- Membership: `InvalidMembershipTier`
- Wallet: `WalletConnectFailed`
- DAO: `DaoRegistrationFailed`
- Auth context: `SessionNotFound`, `UserNotFound`

Status codes: auth-related → 401, validation → 400, internal failures → 500.

- [ ] **Step 2: Create `features/social/types/mod.rs`**

```rust
pub mod error;
pub use error::*;
```

- [ ] **Step 3: Register in `features/social/mod.rs`**

Add `pub mod types;` and `pub use types::*;`.

- [ ] **Step 4: Update all social controller and view files**

Replace every `Error::Unauthorized(...)`, `Error::BadRequest(...)`, `Error::Unknown(...)`, `Error::NotSupported(...)`, `Error::InternalServerError(...)` with the corresponding `SocialError::*` variant. Add `crate::error!` before 500-class errors.

Note: `controllers/team.rs` uses `crate::features::social::Error::Unauthorized(...)` — this was `crate::Error` via re-export. Change to `SocialError::SessionNotFound.into()` etc.

- [ ] **Step 5: Compile check**
- [ ] **Step 6: Commit**

---

### Task 10: Create `MembershipPaymentError` enum for `features/membership/`

**Files:**
- Create: `app/ratel/src/features/membership/types/mod.rs`
- Create: `app/ratel/src/features/membership/types/error.rs`
- Modify: `app/ratel/src/features/membership/mod.rs`
- Modify: `app/ratel/src/features/membership/services/portone/mod.rs`
- Modify: `app/ratel/src/features/membership/models/payment.rs`
- Modify: `app/ratel/src/features/membership/controllers/portone_hook.rs`
- Modify: `app/ratel/src/features/membership/controllers/mod.rs`

- [ ] **Step 1: Create `features/membership/types/error.rs`**

Use the template. Define `MembershipPaymentError` with variants:
- `InvalidCurrency`, `MissingCardInfo`, `MissingBillingKey` → 400
- `PortOneRequestFailed`, `PortOnePaymentFailed`, `PortOneScheduleFailed`, `PortOneVerifyFailed`, `PortOneCancelFailed` → 400
- `WebhookProcessingFailed`, `AwsConversionFailed`, `SessionConversionFailed` → 500

- [ ] **Step 2: Create `features/membership/types/mod.rs`**

```rust
pub mod error;
pub use error::*;
```

- [ ] **Step 3: Register in `features/membership/mod.rs`**

Add `pub mod types;` after existing modules. No need for `pub use types::*;` if the feature already has its own wildcard imports — check and add if needed.

- [ ] **Step 4: Update all membership files**

Replace every usage. Add `crate::error!` for 500-class errors.

- [ ] **Step 5: Compile check**
- [ ] **Step 6: Commit**

---

### Task 11: Create `TimelineError` for `features/timeline/`

**Files:**
- Create: `app/ratel/src/features/timeline/types/mod.rs`
- Create: `app/ratel/src/features/timeline/types/error.rs`
- Modify: `app/ratel/src/features/timeline/mod.rs`
- Modify: `app/ratel/src/features/timeline/services/fan_out/common.rs`
- Modify: `app/ratel/src/features/timeline/controllers/list_timeline.rs`

- [ ] **Step 1: Create `features/timeline/types/error.rs`**

Use the template. Variants: `FanOutFailed` → 500, `InvalidUser` → 400, `InvalidBookmark` → 400.

- [ ] **Step 2: Create `features/timeline/types/mod.rs`** and register in `timeline/mod.rs`
- [ ] **Step 3: Update call sites**
- [ ] **Step 4: Compile check**
- [ ] **Step 5: Commit**

---

### Task 12: Create `AdminError` for `features/admin/`

**Files:**
- Create: `app/ratel/src/features/admin/types/mod.rs`
- Create: `app/ratel/src/features/admin/types/error.rs`
- Modify: `app/ratel/src/features/admin/mod.rs`
- Modify: `app/ratel/src/features/admin/controllers/memberships/grant_enterprise_membership.rs`
- Modify: `app/ratel/src/features/admin/controllers/memberships/list_enterprise_memberships.rs`

- [ ] **Step 1: Create `features/admin/types/error.rs`**

Use the template. Variants: `UsernameRequired` → 400, `InvalidBookmark` → 400.

- [ ] **Step 2: Create `features/admin/types/mod.rs`** and register
- [ ] **Step 3: Update call sites**
- [ ] **Step 4: Compile check**
- [ ] **Step 5: Commit**

---

### Task 13: Expand `SpaceError` + create spaces sub-module error enums

This is the largest task. It covers `features/spaces/` which has ~80 usages across many sub-modules.

**Files to create:**
- `app/ratel/src/features/spaces/pages/actions/types/error.rs` — `SpaceActionError`
- `app/ratel/src/features/spaces/pages/actions/actions/poll/types/error.rs` — `SpacePollError`
- `app/ratel/src/features/spaces/pages/actions/actions/follow/types/mod.rs` + `error.rs` — `SpaceFollowError`
- `app/ratel/src/features/spaces/pages/apps/types/error.rs` — `SpaceAppError`
- `app/ratel/src/features/spaces/pages/report/types/mod.rs` + `error.rs` — `SpaceReportError`

**Files to modify:**
- `app/ratel/src/features/spaces/types/error.rs` — expand `SpaceError`
- `app/ratel/src/features/spaces/pages/actions/types/mod.rs`
- `app/ratel/src/features/spaces/pages/actions/actions/poll/types/mod.rs`
- `app/ratel/src/features/spaces/pages/actions/actions/follow/mod.rs`
- `app/ratel/src/features/spaces/pages/apps/types/mod.rs`
- `app/ratel/src/features/spaces/pages/report/mod.rs`
- All controller/model files under spaces/ that use the 5 removed variants (~30+ files)

- [ ] **Step 1: Expand `SpaceError`**

Add to `features/spaces/types/error.rs`:
- `EmailVerificationFailed` → 500
- `StartNowNotSupported` → 400
- `FinishNowNotSupported` → 400
- `UpdateFailed` → 500
- `InvalidPanelQuota` → 400
- `RewardDistributionFailed` → 500

Update `status_code()` match arm.

- [ ] **Step 2: Create `SpaceActionError`**

Create `features/spaces/pages/actions/types/error.rs`. Variants:
- `ActionLoadFailed`, `ActionUpdateFailed`, `ActionDeleteFailed`, `TransactionFailed`, `RewardTemplateFailed`, `MembershipCheckFailed` → 500
- `InvalidTimeRange` → 400

Register in `features/spaces/pages/actions/types/mod.rs`.

- [ ] **Step 3: Create `SpacePollError`**

Create `features/spaces/pages/actions/actions/poll/types/error.rs`. Variants:
- `PollNotInProgress`, `AnswerMismatch`, `EditNotAllowed`, `QuestionsEmpty`, `InvalidTimeRange`, `InvalidQuestionFormat` → 400
- `VoteVerificationFailed`, `CreateFailed`, `EncryptionFailed`, `DecryptionFailed` → 500

Register in `features/spaces/pages/actions/actions/poll/types/mod.rs`.

- [ ] **Step 4: Create `SpaceFollowError`**

Create `features/spaces/pages/actions/actions/follow/types/mod.rs` and `types/error.rs`. Variants:
- `CannotFollowSelf`, `InvalidTarget`, `InvalidFollowTarget`, `CreatorCannotBeRemoved` → 400
- `FollowFailed`, `UnfollowFailed`, `CreateFailed` → 500

Register in `features/spaces/pages/actions/actions/follow/mod.rs`.

- [ ] **Step 5: Create `SpaceAppError`**

Create `features/spaces/pages/apps/types/error.rs`. Variants:
- `InstallFailed`, `UninstallFailed`, `DeployFailed`, `ArchiveProviderFailed`, `ArchiveBlockFailed`, `ArchiveLogsFailed`, `ExcelExportFailed`, `ChartRenderFailed`, `CopyTextFailed`, `PanelQuotaCreateFailed`, `PanelQuotaDeleteFailed` → 500
- `InvalidEvmAddress`, `InvalidInvitationEmail`, `CreatorCannotBeRemoved`, `IncentiveAddressRequired`, `IncentiveChainRequired` → 400
- `UnsupportedOnServer` → 400

Register in `features/spaces/pages/apps/types/mod.rs`.

- [ ] **Step 6: Create `SpaceReportError`**

Create `features/spaces/pages/report/types/mod.rs` and `types/error.rs`. Variants:
- `AnalyzeLoadFailed`, `AnalyzeUpdateFailed` → 500

Register in `features/spaces/pages/report/mod.rs`.

- [ ] **Step 7: Expand `SpaceActionDiscussionError`**

Add to existing `features/spaces/pages/actions/actions/discussion/types/error.rs`:
- `CreateFailed`, `DeleteFailed` → 500
- `InvalidDiscussionId`, `CommentTooShort` → 400

- [ ] **Step 8: Expand `SpaceActionQuizError`**

Add to existing `features/spaces/pages/actions/actions/quiz/types/error.rs`:
- `CreateFailed` → 500

- [ ] **Step 9: Update ALL spaces call sites**

Go through every controller/model file under `features/spaces/` and replace all ~80 usages. Add `crate::error!` before each 500-class error. Use `.into()` for conversion to `common::Error`.

Key files: `space_common/controllers/update_space.rs`, `space_common/models/space_email_verification.rs`, `space_common/models/space_reward.rs`, `space_common/models/dashboard/aggregate.rs`, `pages/actions/controllers/*.rs`, `pages/actions/actions/poll/controllers/*.rs`, `pages/actions/actions/poll/models/space_poll.rs`, `pages/actions/actions/follow/controllers/*.rs`, `pages/actions/actions/discussion/controllers/**/*.rs`, `pages/actions/actions/discussion/models/*.rs`, `pages/actions/services/vote_crypto.rs`, `pages/apps/controllers/*.rs`, `pages/apps/apps/incentive_pool/**/*.rs`, `pages/apps/apps/analyzes/**/*.rs`, `pages/apps/apps/panels/controllers/*.rs`, `pages/report/controllers/*.rs`, `pages/overview/controllers/*.rs`.

- [ ] **Step 10: Compile check**
- [ ] **Step 11: Commit**

---

### Task 14: Update `common::Error` — remove variants, add `#[from]` wrappers, update impls

**Files:**
- Modify: `app/ratel/src/common/types/error.rs`
- Modify: `app/ratel/src/common/types/entity_type.rs`
- Modify: `app/ratel/src/common/types/space_user_role.rs`
- Modify: `app/ratel/src/common/types/reward/user_reward_history_key.rs`
- Modify: `app/ratel/src/common/models/space/space_common.rs`
- Modify: `app/ratel/src/common/dev_tools/toast_tools.rs`
- Modify: `app/ratel/src/contexts/team_context.rs`

- [ ] **Step 1: Add new unit variants to `common::Error`**

```rust
#[error("Internal error")]
#[translate(en = "An unexpected error occurred", ko = "예기치 않은 오류가 발생했습니다.")]
Internal,

#[error("Unsupported operation")]
#[translate(en = "This operation is not supported", ko = "지원되지 않는 작업입니다.")]
UnsupportedOperation,

#[error("Missing space ID")]
#[translate(en = "Space ID is required", ko = "스페이스 ID가 필요합니다.")]
MissingSpaceId,

#[error("Invalid format")]
#[translate(en = "Invalid data format", ko = "잘못된 데이터 형식입니다.")]
InvalidFormat,

#[error("Invalid team context")]
#[translate(en = "Invalid team context", ko = "유효하지 않은 팀 컨텍스트입니다.")]
InvalidTeamContext,

#[error("User not found")]
#[translate(en = "User not found", ko = "사용자를 찾을 수 없습니다.")]
UserNotFoundInContext,

#[error("Space role check failed")]
#[translate(en = "Authorization check failed", ko = "권한 확인에 실패했습니다.")]
SpaceUserRoleFailed,
```

- [ ] **Step 2: Add `#[from]` + `#[translate(from)]` for all new error enums**

Add these to the `Error` enum body (after the existing `#[from]` variants):

```rust
#[error("{0}")]
#[translate(from)]
Auth(#[from] crate::features::auth::types::error::AuthError),

#[error("{0}")]
#[translate(from)]
Post(#[from] crate::features::posts::types::error::PostError),

#[error("{0}")]
#[translate(from)]
Social(#[from] crate::features::social::types::error::SocialError),

#[error("{0}")]
#[translate(from)]
MembershipPayment(#[from] crate::features::membership::types::error::MembershipPaymentError),

#[error("{0}")]
#[translate(from)]
Timeline(#[from] crate::features::timeline::types::error::TimelineError),

#[error("{0}")]
#[translate(from)]
Admin(#[from] crate::features::admin::types::error::AdminError),

#[error("{0}")]
#[translate(from)]
Service(#[from] crate::common::services::error::ServiceError),

#[error("{0}")]
#[translate(from)]
Infra(#[from] crate::common::utils::error::InfraError),

#[error("{0}")]
#[translate(from)]
FileUpload(#[from] crate::common::components::file_uploader::error::FileUploadError),

#[error("{0}")]
#[translate(from)]
SpaceAction(#[from] crate::features::spaces::pages::actions::types::error::SpaceActionError),

#[error("{0}")]
#[translate(from)]
SpacePoll(#[from] crate::features::spaces::pages::actions::actions::poll::types::error::SpacePollError),

#[error("{0}")]
#[translate(from)]
SpaceFollow(#[from] crate::features::spaces::pages::actions::actions::follow::types::error::SpaceFollowError),

#[error("{0}")]
#[translate(from)]
SpaceApp(#[from] crate::features::spaces::pages::apps::types::error::SpaceAppError),

#[error("{0}")]
#[translate(from)]
SpaceReport(#[from] crate::features::spaces::pages::report::types::error::SpaceReportError),
```

- [ ] **Step 3: Remove the 5 parameterized variants**

Delete:
```rust
Unknown(String),
NotSupported(String),
InternalServerError(String),
BadRequest(String),
Unauthorized(String),
```

- [ ] **Step 4: Update `From` impls**

```rust
impl From<String> for Error {
    fn from(s: String) -> Self {
        tracing::error!("Untyped string error: {s}");
        Error::Internal
    }
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        tracing::error!("Base64 decode error: {e}");
        Error::Internal
    }
}

impl From<ServerFnError> for Error {
    fn from(e: ServerFnError) -> Self {
        tracing::error!("Server function error: {e}");
        Error::Internal
    }
}

// QdrantError → InfraError
#[cfg(feature = "server")]
impl From<qdrant_client::QdrantError> for Error {
    fn from(e: qdrant_client::QdrantError) -> Self {
        tracing::error!("Qdrant error: {e}");
        Error::Infra(crate::common::utils::error::InfraError::QdrantFailed)
    }
}
```

- [ ] **Step 5: Update `IntoResponse` impl**

Remove match arms for deleted variants. Add arms for new `#[from]` variants delegating to `e.status_code()`:

```rust
Error::Auth(e) => e.status_code(),
Error::Post(e) => e.status_code(),
Error::Social(e) => e.status_code(),
Error::MembershipPayment(e) => e.status_code(),
Error::Timeline(e) => e.status_code(),
Error::Admin(e) => e.status_code(),
Error::Service(e) => e.status_code(),
Error::Infra(e) => e.status_code(),
Error::FileUpload(e) => e.status_code(),
Error::SpaceAction(e) => e.status_code(),
Error::SpacePoll(e) => e.status_code(),
Error::SpaceFollow(e) => e.status_code(),
Error::SpaceApp(e) => e.status_code(),
Error::SpaceReport(e) => e.status_code(),
Error::Internal | Error::UnsupportedOperation | Error::MissingSpaceId
| Error::InvalidFormat | Error::InvalidTeamContext
| Error::UserNotFoundInContext | Error::SpaceUserRoleFailed => // appropriate status code
```

Do the same for `AsStatusCode` impl and `Into<rmcp::ErrorData>` impl.

- [ ] **Step 6: Update remaining common/ call sites**

- `common/types/entity_type.rs` — replace `Error::NotSupported(...)` → `Error::UnsupportedOperation`
- `common/types/space_user_role.rs` — replace `Error::InternalServerError(...)` → `{ crate::error!(...); Error::SpaceUserRoleFailed }`
- `common/types/reward/user_reward_history_key.rs` — replace `Error::BadRequest("Invalid format")` → `Error::InvalidFormat`
- `common/models/space/space_common.rs` — replace `Error::BadRequest("Missing space_id ...")` → `Error::MissingSpaceId`
- `common/dev_tools/toast_tools.rs` — replace `Error::Unknown(format!(...))` → `Error::Internal`
- `contexts/team_context.rs` — replace `Error::Unauthorized("no session")` → `Error::NoSessionFound`, `Error::BadRequest(...)` → `Error::InvalidTeamContext`, `Error::Unauthorized("User not found")` → `Error::UserNotFoundInContext`

- [ ] **Step 7: Compile check**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web`

This is the critical step — all previous tasks' changes must compile together. Fix any remaining issues.

- [ ] **Step 8: Commit**

---

### Task 15: Final verification and cleanup

- [ ] **Step 1: Verify no remaining usages**

```bash
cd app/ratel && grep -rn 'Error::Unknown\|Error::NotSupported\|Error::InternalServerError\|Error::BadRequest\|Error::Unauthorized' src/ --include='*.rs' | grep -v '// ' | grep -v 'test'
```

Should return zero results (excluding comments and test files).

- [ ] **Step 2: Full compile check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

- [ ] **Step 3: Run server function tests**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass"
```

- [ ] **Step 4: Final commit if any fixups needed**
