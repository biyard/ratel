use std::{error::Error as StdError, fmt::Display};

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
    HMacInitError(String),
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidAction => write!(f, "Invalid action"),
            Error::MissingParam(param) => write!(f, "Missing parameter: {}", param),
            Error::InvalidPhoneNumberFormat => write!(f, "Invalid phone number format"),
            Error::UpdateNotAllowed => write!(f, "Update not allowed"),
            Error::Unknown(msg) => write!(f, "Unknown error: {}", msg),
            Error::Klaytn(msg) => write!(f, "Klaytn error: {}", msg),
            Error::InvalidUserQuery(msg) => write!(f, "Invalid user query: {}", msg),
            Error::DbPoolTimeout => write!(f, "Database pool timeout"),
            Error::NotFound => write!(f, "Not found"),
            Error::Unauthorized => write!(f, "Unauthorized"),
            Error::UserAlreadyExists => write!(f, "User already exists"),
            Error::InvalidUser => write!(f, "Invalid user"),
            Error::InvalidEmail => write!(f, "Invalid email"),
            Error::InvalidUsername => write!(f, "Invalid username"),
            Error::InvalidPhoneNumber => write!(f, "Invalid phone number"),
            Error::InvalidPrinciapl => write!(f, "Invalid principal"),
            Error::DuplicatedTeamName => write!(f, "Duplicated team name"),
            Error::HMacInitError(msg) => write!(f, "Initialize HMac error: {}", msg),
            Error::VerifyException(msg) => write!(f, "Verify exception: {}", msg),
            Error::SignException => write!(f, "Sign exception"),
            Error::DatabaseException(msg) => write!(f, "Database exception: {}", msg),
            Error::AwsChimeError(msg) => write!(f, "AWS Chime error: {}", msg),
            Error::AwsMediaPipelinesError(msg) => write!(f, "AWS Media Pipelines error: {}", msg),
            Error::AwsMediaConvertError(msg) => write!(f, "AWS Media Convert error: {}", msg),
            Error::AwsS3Error(msg) => write!(f, "AWS S3 error: {}", msg),
            Error::DuplicatedGroupName => write!(f, "Duplicated group name"),
            Error::InsertGroupMemberFailed => write!(f, "Insert group member failed"),
            Error::OpenApiResponseError(msg) => write!(f, "OpenAPI response error: {}", msg),
            Error::NaOpenApiResponseParsingError => write!(f, "NA OpenAPI response parsing error"),
            Error::NaOpenApiRequestError => write!(f, "NA OpenAPI request error"),
            Error::UsCongressApiError(msg) => write!(f, "US Congress API error: {}", msg),
            Error::UsCongressApiRequestError => write!(f, "US Congress API request error"),
            Error::HkOpenDataApiError(msg) => write!(f, "HK Open Data API error: {}", msg),
            Error::HkOpenDataApiResponseParsingError => {
                write!(f, "HK Open Data API response parsing error")
            }
            Error::HkOpenDataApiRequestError => write!(f, "HK Open Data API request error"),
            Error::ChOpenDataApiError(msg) => write!(f, "CH Open Data API error: {}", msg),
            Error::ChOpenDataApiResponseParsingError => {
                write!(f, "CH Open Data API response parsing error")
            }
            Error::ChOpenDataApiRequestError => write!(f, "CH Open Data API request error"),
            Error::EuOpenDataApiError(msg) => write!(f, "EU Open Data API error: {}", msg),
            Error::EuOpenDataApiResponseParsingError => {
                write!(f, "EU Open Data API response parsing error")
            }
            Error::EuOpenDataApiRequestError => write!(f, "EU Open Data API request error"),
            Error::EuOpenDataFetchError(errors) => {
                write!(f, "EU Open Data fetch error: {:?}", errors)
            }
            Error::HtmlParseError(msg) => write!(f, "HTML parse error: {}", msg),
            Error::FetchError(errors) => write!(f, "Fetch error: {:?}", errors),
            Error::ReqwestClientError(msg) => write!(f, "Reqwest client error: {}", msg),
            Error::ApiEmptyRow => write!(f, "API empty row"),
            Error::BadRequest => write!(f, "Bad request"),
            Error::JsonDeserializeError(msg) => write!(f, "JSON deserialize error: {}", msg),
            Error::WalletNotFound => write!(f, "Wallet not found"),
            Error::WalletError(msg) => write!(f, "Wallet error: {}", msg),
            Error::UniqueViolation(msg) => write!(f, "Unique violation: {}", msg),
            Error::EmptyInputValue => write!(f, "Empty input value"),
            Error::EmailAlreadySubscribed => write!(f, "Email already subscribed"),
            Error::InvalidInputValue => write!(f, "Invalid input value"),
            Error::AlreadyVoted => write!(f, "Already voted"),
            Error::AlreadyLiked => write!(f, "Already liked"),
            Error::SlackNotificationError(msg) => write!(f, "Slack notification error: {}", msg),
            Error::JWTGenerationFail(msg) => write!(f, "JWT generation fail: {}", msg),
            Error::FeedWritePostError => write!(f, "Feed write post error"),
            Error::FeedPublishError => write!(f, "Feed publish error"),
            Error::FeedWriteCommentError => write!(f, "Feed write comment error"),
            Error::FeedInvalidParentId => write!(f, "Feed invalid parent ID"),
            Error::FeedInvalidQuoteId => write!(f, "Feed invalid quote ID"),
            Error::FeedInvalidQuoteSpaceId => write!(f, "Feed invalid quote space ID"),
            Error::FeedExclusiveParentOrIndustry => write!(f, "Feed exclusive parent or industry"),
            Error::SpaceWritePostError => write!(f, "Space write post error"),
            Error::InvalidType => write!(f, "Invalid type"),
            Error::AssetError(msg) => write!(f, "Asset error: {}", msg),
            Error::UploadMetadataError(msg) => write!(f, "Upload metadata error: {}", msg),
            Error::InvalidQuizId => write!(f, "Invalid quiz ID"),
            Error::InvalidTeamname => write!(f, "Invalid team name"),
            Error::BadgeCreationFailure => write!(f, "Badge creation failure"),
            Error::AlreadyClaimed => write!(f, "Already claimed"),
            Error::NFTLimitedError => write!(f, "NFT limited error"),
            Error::RedeemCodeCreationFailure => write!(f, "Redeem code creation failure"),
            Error::InvalidRedeemCode => write!(f, "Invalid redeem code"),
            Error::RedeemCodeNotFound => write!(f, "Redeem code not found"),
            Error::DiscussionInsertFailed => write!(f, "Discussion insert failed"),
            Error::DiscussionNotFound => write!(f, "Discussion not found"),
            Error::DiscussionCreateUserFailed(msg) => {
                write!(f, "Discussion create user failed: {}", msg)
            }
            Error::UpdateDiscussionError(msg) => write!(f, "Update discussion error: {}", msg),
            Error::PipelineNotFound => write!(f, "Pipeline not found"),
            Error::AlreadyFollowing => write!(f, "Already following"),
            Error::SESServiceError(msg) => write!(f, "SES service error: {}", msg),
            Error::InvalidVerificationCode => write!(f, "Invalid verification code"),
            Error::ServerError(msg) => write!(f, "Server error: {}", msg),
            Error::InvalidPayload => write!(f, "Invalid payload"),
            Error::SprintLeagueCreationFailed => write!(f, "Sprint league creation failed"),
            Error::SprintLeagueUpdateFailed => write!(f, "Sprint league update failed"),
            Error::FailedReward => write!(f, "Failed reward"),
            Error::PassportVerificationFailed(msg) => {
                write!(f, "Passport verification failed: {}", msg)
            }
            Error::MedicalInfoExtractionFailed(msg) => {
                write!(f, "Medical info extraction failed: {}", msg)
            }
            Error::AwsRekognitionError(msg) => write!(f, "AWS Rekognition error: {}", msg),
            Error::AwsTextractError(msg) => write!(f, "AWS Textract error: {}", msg),
            Error::AwsBedrockError(msg) => write!(f, "AWS Bedrock error: {}", msg),
            Error::DynamoDbError(msg) => write!(f, "DynamoDB error: {}", msg),
            Error::DynamoDbSerializationError(msg) => {
                write!(f, "DynamoDB serialization error: {}", msg)
            }
            Error::DynamoDbTableNotFound(msg) => write!(f, "DynamoDB table not found: {}", msg),
        }
    }
}

impl Error {
    pub fn to_string(&self) -> String {
        format!("{}", self)
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
