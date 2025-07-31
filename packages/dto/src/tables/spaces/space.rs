pub use bdk::prelude::*;

use crate::*;

use validator::Validate;

//TODO: action(like, comments, find_by_id, create_space), query_action
#[derive(Validate)]
#[api_model(base = "/v1/spaces", table = spaces, action = [create_space(user_ids = Vec<i64>)], action_by_id = [posting_space, update_space(discussions = Vec<DiscussionCreateRequest>, elearnings = Vec<ElearningCreateRequest>, surveys = Vec<SurveyCreateRequest>, drafts = Vec<SpaceDraftCreateRequest>, quiz = Option<Vec<NoticeQuestionWithAnswer>>), like(value = bool), share()])]
pub struct Space {
    #[api_model(summary, primary_key, read_action = [find_by_id])]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    //FIXME: remove this field when unused
    #[api_model(summary, nullable, action_by_id = [update_space])]
    pub title: Option<String>,
    //FIXME: remove this field when unused
    #[api_model(summary, action_by_id = [update_space])]
    pub html_contents: String,
    #[api_model(summary, type = INTEGER, action = [create_space])]
    #[serde(default)]
    pub space_type: SpaceType,
    #[api_model(summary, many_to_one = users)]
    pub owner_id: i64,
    #[api_model(summary, many_to_one = industries)]
    pub industry_id: i64,
    #[api_model(summary, action_by_id = [update_space], action = [create_space], version = v0.2)]
    #[serde(default)]
    pub started_at: Option<i64>,
    #[api_model(summary, action_by_id = [update_space], action = [create_space], version = v0.2)]
    #[serde(default)]
    pub ended_at: Option<i64>,

    #[api_model(summary, many_to_one = feeds, action = create_space)]
    pub feed_id: i64,

    #[api_model(summary, version = v0.1, type = INTEGER)]
    #[serde(default)]
    pub status: SpaceStatus,
    #[api_model(version = v0.1, summary, type = JSONB, action_by_id = [update_space])]
    #[serde(default)]
    pub files: Vec<File>,

    #[api_model(summary, one_to_many = space_contracts, foreign_key = space_id)]
    #[serde(default)]
    pub contracts: Vec<SpaceContract>,
    // FIXME: separate holders into a different table for joined table of users and spaces
    // #[api_model(summary, one_to_many = space_holders, foreign_key = space_id)]
    // #[serde(default)]
    // pub holders: Vec<SpaceHolder>,
    #[api_model(one_to_many = users, reference_key = owner_id, foreign_key = id)]
    #[serde(default)]
    pub author: Vec<Author>,

    #[api_model(one_to_many = industries, reference_key = industry_id, foreign_key = id)]
    #[serde(default)]
    pub industry: Vec<Industry>,
    #[api_model(many_to_many = space_badges, foreign_table_name = badges, foreign_primary_key = badge_id, foreign_reference_key = space_id)]
    #[serde(default)]
    pub badges: Vec<Badge>,
    #[api_model(many_to_many = space_members, foreign_reference_key = space_id, foreign_primary_key = user_id, foreign_table_name = users)]
    #[serde(default)]
    pub members: Vec<Member>,
    #[api_model(one_to_many = space_groups, foreign_key = space_id)]
    #[serde(default)]
    pub groups: Vec<SpaceGroup>,

    #[api_model(version = v0.1, action = create_space)]
    pub num_of_redeem_codes: i64,
    #[api_model(many_to_many = redeem_codes, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = meta_id)]
    #[serde(default)]
    pub codes: Vec<RedeemCode>,

    #[api_model(summary, one_to_many = feeds, foreign_key = parent_id, nested)]
    #[serde(default)]
    pub comments: Vec<SpaceComment>,

    #[api_model(summary, one_to_many = feeds, reference_key = feed_id, foreign_key = parent_id, nested)]
    #[serde(default)]
    pub feed_comments: Vec<SpaceComment>,
    #[api_model(summary, one_to_many = discussions, foreign_key = space_id, nested)]
    #[serde(default)]
    pub discussions: Vec<Discussion>,
    #[api_model(summary, one_to_many = elearnings, foreign_key = space_id)]
    #[serde(default)]
    pub elearnings: Vec<Elearning>,
    #[api_model(summary, one_to_many = surveys, foreign_key = space_id)]
    #[serde(default)]
    pub surveys: Vec<Survey>,
    #[api_model(skip)]
    #[serde(default)]
    pub responses: Vec<SurveyResponse>,
    #[api_model(skip)]
    #[serde(default)]
    pub user_responses: Vec<SurveyResponse>,
    #[api_model(summary, one_to_many = space_drafts, foreign_key = space_id)]
    #[serde(default)]
    pub drafts: Vec<SpaceDraft>,
    #[api_model(summary, many_to_many = space_like_users, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = space_id, aggregator = count)]
    #[serde(default)]
    pub likes: i64,
    #[api_model(summary, many_to_many = space_share_users, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = space_id, aggregator = count)]
    #[serde(default)]
    pub shares: i64,
    #[api_model(summary, many_to_many = space_like_users, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = space_id, aggregator = exist)]
    #[serde(default)]
    pub is_liked: bool,

    #[api_model(one_to_many = sprint_leagues, foreign_key = space_id, nested)]
    #[serde(default)]
    // Vec length should be 0 or 1.
    pub sprint_leagues: Vec<SprintLeague>,

    #[api_model(summary, type=JSONB, nullable,)]
    #[serde(default)]
    pub notice_quiz: Vec<NoticeQuestion>,

    // Notice Type
    #[api_model(summary, version = v0.1, type = INTEGER, action = [create_space], nullable)]
    #[serde(default)]
    pub booster_type: Option<BoosterType>,
    
    // The publishing scope of the space (Private or Public)
    #[api_model(summary, version = v0.1, type = INTEGER, action_by_id = [update_space])]
    #[serde(default)]
    pub publishing_scope: PublishingScope,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum SpaceStatus {
    #[default]
    Draft = 1,
    InProgress = 2,
    Finish = 3,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum SpaceType {
    #[default]
    Legislation = 1,
    Poll = 2,
    Deliberation = 3,
    Nft = 4,
    Commitee = 5,
    SprintLeague = 6,
    Notice = 7,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum PublishingScope {
    #[translate(ko = "Private", en = "Private")]
    #[default]
    Private = 1,
    #[translate(ko = "Public", en = "Public")]
    Public = 2,
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

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct File {
    pub name: String,
    pub size: String,
    pub ext: FileExtension,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum BoosterType {
    #[translate(ko = "No Boost", en = "No Boost")]
    #[default]
    NoBoost = 1,

    #[translate(ko = "X2", en = "X2")]
    X2 = 2,
    #[translate(ko = "X10", en = "X10")]
    X10 = 3,
    #[translate(ko = "X100", en = "X100")]
    X100 = 4,
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

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Validate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct NoticeQuestion {
    pub title: String,
    #[validate(custom(function = "validate_image_files"))]
    pub images: Vec<File>,
    pub options: Vec<NoticeOption>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct NoticeOption {
    pub content: String,
}

// Validation function to ensure only JPG and PNG files are allowed for images
fn validate_image_files(files: &[File]) -> std::result::Result<(), validator::ValidationError> {
    for file in files {
        match file.ext {
            FileExtension::JPG | FileExtension::PNG => continue,
            _ => {
                let mut error = validator::ValidationError::new("invalid_image_extension");
                error.message = Some("Only JPG and PNG files are allowed for images".into());
                return Err(error);
            }
        }
    }
    Ok(())
}