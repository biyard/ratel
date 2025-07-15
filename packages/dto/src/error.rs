use std::error::Error as StdError;

use serde::{Deserialize, Serialize};

use bdk::prelude::*;

#[derive(Debug, Serialize)]
pub struct ServiceException {
    pub inner: Error,
}

impl std::fmt::Display for ServiceException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl StdError for ServiceException {}

#[derive(Debug, Serialize, PartialEq, Eq, Deserialize, Translate)]
#[repr(u64)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Error {
    InvalidAction,
    UpdateNotAllowed,
    Unknown(String),
    Klaytn(String),

    #[translate(en = "Could not find any resource", ko = "리소스를 찾을 수 없습니다.")]
    NotFound,

    #[translate(
        en = "Invalid signature or unsupported authentication",
        ko = "유효하지 않은 서명입니다."
    )]
    Unauthorized,
    #[translate(
        en = "You might have already registered",
        ko = "이미 등록된 사용자입니다."
    )]
    UserAlreadyExists,
    #[translate(en = "Could not find a valid user", ko = "유효하지 않은 사용자입니다.")]
    InvalidUser,
    #[translate(
        en = "You must pass a valid email",
        ko = "유효한 이메일을 입력해야 합니다."
    )]
    InvalidEmail,
    #[translate(
        en = "You must pass a valid principal",
        ko = "유효한 계정 주소를 입력해야 합니다."
    )]
    InvalidPrinciapl,
    #[translate(en = "Please change team name.")]
    DuplicatedTeamName,

    VerifyException(String),
    SignException,
    DatabaseException(String),

    #[translate(en = "AWS Chime service error occurred. Please try again later.")]
    AwsChimeError(String),
    #[translate(en = "AWS Media Pipelines error occurred. Please try again later.")]
    AwsMediaPipelinesError(String),
    #[translate(en = "AWS Media Convert error occurred. Please try again later.")]
    AwsMediaConvertError(String),
    AwsS3Error(String),

    #[translate(en = "Please change group name.")]
    DuplicatedGroupName,
    #[translate(en = "Failed to insert group member")]
    InsertGroupMemberFailed,

    // NA OpenAPI
    OpenApiResponseError(String),
    #[translate(en = "Failed to parse response")]
    NaOpenApiResponseParsingError,
    #[translate(en = "Failed to call national assembly API")]
    NaOpenApiRequestError,

    // US Congress API
    #[translate(en = "Failed to call US Congress API")]
    UsCongressApiError(String),
    #[translate(en = "Failed to call US Congress API")]
    UsCongressApiRequestError,

    // HK OpenData API
    #[translate(en = "Failed to call HK OpenData API")]
    HkOpenDataApiError(String),
    #[translate(en = "Failed to parse response in HK OpenData API")]
    HkOpenDataApiResponseParsingError,
    #[translate(en = "Failed to call HK OpenData API")]
    HkOpenDataApiRequestError,

    // Swiss OpenAPI
    #[translate(en = "Failed to call Swiss OpenData API")]
    ChOpenDataApiError(String),
    #[translate(en = "Failed to parse response in Swiss OpenData API")]
    ChOpenDataApiResponseParsingError,
    #[translate(en = "Failed to call Swiss OpenData API")]
    ChOpenDataApiRequestError,

    // EU OpenAPI
    #[translate(en = "Failed to call EU OpenData API")]
    EuOpenDataApiError(String),
    #[translate(en = "Failed to parse response in EU OpenData API")]
    EuOpenDataApiResponseParsingError,
    #[translate(en = "Failed to call EU OpenData API")]
    EuOpenDataApiRequestError,
    EuOpenDataFetchError(Vec<(String, String)>),

    #[translate(en = "Failed to parse website")]
    HtmlParseError(String),
    FetchError(Vec<(i64, String)>),
    #[translate(en = "Failed to initialize reqwest client")]
    ReqwestClientError(String),
    #[translate(en = "Could not find any resource")]
    ApiEmptyRow,

    BadRequest,
    JsonDeserializeError(String),
    WalletNotFound,
    WalletError(String),
    UniqueViolation(String),

    #[translate(en = "Required input value is missing", ko = "필수 입력값이 없습니다.")]
    EmptyInputValue,
    #[translate(en = "Email is already subscribed", ko = "이미 구독된 이메일입니다.")]
    EmailAlreadySubscribed,
    #[translate(en = "Invalid input value", ko = "유효하지 않은 입력값입니다.")]
    InvalidInputValue,

    // Votes
    #[translate(en = "You've already voted", ko = "이미 투표했습니다.")]
    AlreadyVoted,

    #[translate(en = "You might have already liked", ko = "이미 좋아요를 눌렀습니다.")]
    AlreadyLiked,

    #[translate(
        en = "Failed to send Slack notification",
        ko = "슬랙 알림 전송에 실패했습니다."
    )]
    SlackNotificationError(String),
    #[translate(
        en = "Failed to generate JWT token",
        ko = "JWT 토큰 생성에 실패했습니다."
    )]
    JWTGenerationFail(String),

    // feeds
    #[translate(en = "Failed to write a post")]
    FeedWritePostError,
    #[translate(en = "Failed to publish a post")]
    FeedPublishError,
    #[translate(en = "Failed to write a comment")]
    FeedWriteCommentError,
    #[translate(en = "You must write a comment on a valid feed")]
    FeedInvalidParentId,
    #[translate(en = "You must quote a valid feed")]
    FeedInvalidQuoteId,
    #[translate(en = "You must quote a valid space")]
    FeedInvalidQuoteSpaceId,
    #[translate(en = "You should select industry or a parent feed")]
    FeedExclusiveParentOrIndustry,

    // spaces
    #[translate(en = "Failed to write a space")]
    SpaceWritePostError,

    // metadata
    #[translate(en = "Data already exists. Please enter different data.")]
    InvalidType,
    #[translate(en = "Failed to get URL for upload. Please try again.")]
    AssetError(String),
    #[translate(en = "Failed to upload file. Please try again.")]
    UploadMetadataError(String),

    // quizzes
    #[translate(en = "You must select a valid quiz")]
    InvalidQuizId,

    #[translate(en = "You must pass a valid team name")]
    InvalidTeamname,

    #[translate(en = "Failed to create a badge. Please try again.")]
    BadgeCreationFailure,
    #[translate(en = "Already Claimed")]
    AlreadyClaimed,
    #[translate(en = "Only 1 NFT can be minted.")]
    NFTLimitedError,

    // redeem codes
    #[translate(en = "Failed to create redeem codes")]
    RedeemCodeCreationFailure,
    #[translate(en = "Failed to use redeem code")]
    InvalidRedeemCode,
    #[translate(en = "Redeem Code is not exists")]
    RedeemCodeNotFound,

    // discussion
    #[translate(en = "Failed to create discussion")]
    DiscussionInsertFailed,
    #[translate(en = "Discussion not found")]
    DiscussionNotFound,
    #[translate(en = "Failed to create discussion user")]
    DiscussionCreateUserFailed(String),
    #[translate(en = "Failed to update discussion")]
    UpdateDiscussionError(String),
    PipelineNotFound,

    #[translate(en = "You are already following this user")]
    AlreadyFollowing,

    // Email verification
    SESServiceError(String),
    #[translate(ko = "인증코드가 잘못되었습니다.", en = "Invalid verification code.")]
    InvalidVerificationCode,

    ServerError(String),

    #[translate(en = "Invalid Payload")]
    InvalidPayload,

    #[translate(
        en = "Failed to create sprint league",
        ko = "스프린트 리그 생성에 실패했습니다."
    )]
    SprintLeagueCreationFailed,
    #[translate(
        en = "Failed to update sprint league",
        ko = "스프린트 리그 업데이트에 실패했습니다."
    )]
    SprintLeagueUpdateFailed,
}

impl<E: StdError + 'static> From<E> for Error {
    fn from(e: E) -> Self {
        Error::Unknown(e.to_string())
    }
}

impl Into<ServiceException> for Error {
    fn into(self) -> ServiceException {
        ServiceException { inner: self }
    }
}

impl Error {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

#[cfg(feature = "server")]
impl by_axum::axum::response::IntoResponse for Error {
    fn into_response(self) -> by_axum::axum::response::Response {
        (
            by_axum::axum::http::StatusCode::BAD_REQUEST,
            by_axum::axum::Json(self),
        )
            .into_response()
    }
}
