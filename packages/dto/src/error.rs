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

#[derive(Debug, Serialize, PartialEq, Eq, Deserialize)]
#[repr(u64)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Error {
    InvalidAction,
    MissingParam(String),
    InvalidPhoneNumberFormat,
    UpdateNotAllowed,
    Unknown(String),
    Klaytn(String),
    InvalidUserQuery(String),

    DbPoolTimeout,

    NotFound,

    Unauthorized,
    UserAlreadyExists,
    InvalidUser,
    InvalidEmail,
    InvalidUsername,
    InvalidPhoneNumber,
    InvalidPrinciapl,
    DuplicatedTeamName,

    VerifyException(String),
    SignException,
    DatabaseException(String),

    AwsChimeError(String),
    AwsMediaPipelinesError(String),
    AwsMediaConvertError(String),
    AwsS3Error(String),

    DuplicatedGroupName,
    InsertGroupMemberFailed,

    // NA OpenAPI
    OpenApiResponseError(String),
    NaOpenApiResponseParsingError,
    NaOpenApiRequestError,

    // US Congress API
    UsCongressApiError(String),

    UsCongressApiRequestError,

    // HK OpenData API
    HkOpenDataApiError(String),

    HkOpenDataApiResponseParsingError,

    HkOpenDataApiRequestError,

    // Swiss OpenAPI
    ChOpenDataApiError(String),

    ChOpenDataApiResponseParsingError,

    ChOpenDataApiRequestError,

    // EU OpenAPI
    EuOpenDataApiError(String),

    EuOpenDataApiResponseParsingError,

    EuOpenDataApiRequestError,
    EuOpenDataFetchError(Vec<(String, String)>),

    HtmlParseError(String),
    FetchError(Vec<(i64, String)>),

    ReqwestClientError(String),

    ApiEmptyRow,

    BadRequest,
    JsonDeserializeError(String),
    WalletNotFound,
    WalletError(String),
    UniqueViolation(String),

    EmptyInputValue,

    EmailAlreadySubscribed,

    InvalidInputValue,

    // Votes
    AlreadyVoted,

    AlreadyLiked,

    SlackNotificationError(String),

    JWTGenerationFail(String),

    // feeds
    FeedWritePostError,

    FeedPublishError,

    FeedWriteCommentError,

    FeedInvalidParentId,

    FeedInvalidQuoteId,

    FeedInvalidQuoteSpaceId,

    FeedExclusiveParentOrIndustry,

    // spaces
    SpaceWritePostError,

    // metadata
    InvalidType,

    AssetError(String),

    UploadMetadataError(String),

    // quizzes
    InvalidQuizId,

    InvalidTeamname,

    BadgeCreationFailure,

    AlreadyClaimed,

    NFTLimitedError,

    // redeem codes
    RedeemCodeCreationFailure,

    InvalidRedeemCode,

    RedeemCodeNotFound,

    // discussion
    DiscussionInsertFailed,

    DiscussionNotFound,

    DiscussionCreateUserFailed(String),

    UpdateDiscussionError(String),
    PipelineNotFound,

    AlreadyFollowing,

    // Email verification
    SESServiceError(String),

    InvalidVerificationCode,

    ServerError(String),

    InvalidPayload,

    SprintLeagueCreationFailed,

    SprintLeagueUpdateFailed,

    FailedReward,

    PassportVerificationFailed(String),
    MedicalInfoExtractionFailed(String),
    AwsRekognitionError(String),
    AwsTextractError(String),
    AwsBedrockError(String),
    DynamoDbError(String),
    DynamoDbSerializationError(String),
    DynamoDbTableNotFound(String),
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
        match self {
            Error::Unauthorized => (
                by_axum::axum::http::StatusCode::UNAUTHORIZED,
                by_axum::axum::Json(self),
            )
                .into_response(),
            _ => (
                by_axum::axum::http::StatusCode::BAD_REQUEST,
                by_axum::axum::Json(self),
            )
                .into_response(),
        }
    }
}
