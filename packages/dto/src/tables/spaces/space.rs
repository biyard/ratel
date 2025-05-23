use crate::SpaceComment;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/spaces", table = spaces)]
pub struct Space {
    #[api_model(summary, primary_key, read_action = [find_by_id])]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = [create_space])]
    pub html_contents: String,
    #[api_model(summary, action = [create_space])]
    pub space_type: SpaceType,
    #[api_model(summary, many_to_one = users)]
    pub user_id: i64,
    #[api_model(summary, many_to_one = industries)]
    pub industry_id: i64,

    #[api_model(summary, nullable, indexed)]
    pub parent_id: Option<i64>,
    #[api_model(summary, nullable, action = [create_space])]
    pub title: Option<String>,
    #[api_model(summary, nullable, indexed)]
    pub part_id: Option<i64>,
    #[api_model(summary, nullable, indexed)]
    pub quote_space_id: Option<i64>,

    #[api_model(summary)]
    pub proposer_profile: Option<String>,
    #[api_model(summary)]
    pub proposer_nickname: Option<String>,
    #[api_model(summary)]
    pub is_saved: bool,
    #[api_model(summary, action = [create_space])]
    pub content_type: ContentType,

    #[api_model(summary)]
    pub accepters: i64, // 찬성 수
    #[api_model(summary)]
    pub rejecters: i64, // 반대 수
    #[api_model(summary)]
    pub likes: i64, //좋아요 수
    #[api_model(summary)]
    pub comments: i64, // 코멘트 수
    #[api_model(summary)]
    pub rewards: i64, //리워드 토큰 수?
    #[api_model(summary)]
    pub shares: i64, //공유자 수

    #[api_model(summary, action = create, type = JSONB, version = v0.1, action_by_id = update)]
    pub files: Vec<File>,
    #[api_model(summary, one_to_many = space_comments)]
    pub comment_items: Vec<SpaceComment>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum SpaceType {
    #[default]
    Post = 1,

    // Belows are kinds of comments
    Reply = 2,
    Repost = 3,
    DocReview = 4,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ContentType {
    #[translate(ko = "Crypto", en = "Crypto")]
    #[default]
    Crypto = 1,
    #[translate(ko = "Social", en = "Social")]
    Social = 2,
}

pub use bdk::prelude::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct File {
    pub name: String,
    pub size: String,
    pub ext: FileExtension,
    pub url: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum FileExtension {
    #[translate(ko = "JPG", en = "JPG")]
    JPG = 1,
    #[translate(ko = "PNG", en = "PNG")]
    PNG = 2,
    #[translate(ko = "PDF", en = "PDF")]
    PDF = 3,
    #[translate(ko = "ZIP", en = "ZIP")]
    ZIP = 4,
    #[translate(ko = "WORD", en = "WORD")]
    WORD = 5,
    #[translate(ko = "PPTX", en = "PPTX")]
    PPTX = 6,
    #[translate(ko = "EXCEL", en = "EXCEL")]
    EXCEL = 7,
}
