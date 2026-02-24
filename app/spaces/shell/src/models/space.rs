use crate::dto::GetSpaceResponse;
use crate::*;
use space_common::ratel_post::types::{BoosterType, SpaceType};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Space {
    pub pk: Partition,
    pub sk: EntityType,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub urls: Vec<String>,
    pub space_type: SpaceType,
    pub features: Vec<String>,
    pub status: Option<SpaceStatus>,
    pub permissions: i64,
    pub author_type: UserType,
    pub author_display_name: String,
    pub author_username: String,
    pub author_profile_url: String,
    pub certified: bool,
    pub likes: i64,
    pub comments: i64,
    pub shares: i64,
    pub reports: i64,
    pub rewards: Option<i64>,
    pub visibility: SpaceVisibility,
    pub publish_state: SpacePublishState,
    pub booster: BoosterType,
    pub files: Option<Vec<File>>,
    pub anonymous_participation: bool,
    pub can_participate: bool,
    pub participated: bool,
    pub participant_display_name: Option<String>,
    pub participant_profile_url: Option<String>,
    pub participant_username: Option<String>,
    pub remains: i64,
    pub quota: i64,
    pub is_report: bool,
}

impl From<GetSpaceResponse> for Space {
    fn from(dto: GetSpaceResponse) -> Self {
        Self {
            pk: dto.pk,
            sk: dto.sk,
            title: dto.title,
            content: dto.content,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
            urls: dto.urls,
            space_type: dto.space_type,
            features: dto.features,
            status: dto.status,
            permissions: dto.permissions,
            author_type: dto.author_type,
            author_display_name: dto.author_display_name,
            author_username: dto.author_username,
            author_profile_url: dto.author_profile_url,
            certified: dto.certified,
            likes: dto.likes,
            comments: dto.comments,
            shares: dto.shares,
            reports: dto.reports,
            rewards: dto.rewards,
            visibility: dto.visibility,
            publish_state: dto.publish_state,
            booster: dto.booster,
            files: dto.files,
            anonymous_participation: dto.anonymous_participation,
            can_participate: dto.can_participate,
            participated: dto.participated,
            participant_display_name: dto.participant_display_name,
            participant_profile_url: dto.participant_profile_url,
            participant_username: dto.participant_username,
            remains: dto.remains,
            quota: dto.quota,
            is_report: dto.is_report,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct File {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub size: String,
    pub ext: FileExtension,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum FileExtension {
    #[default]
    #[serde(alias = "jpg", alias = "jpeg", alias = "JPEG")]
    JPG,
    #[serde(alias = "png", alias = "PNG")]
    PNG,
    #[serde(alias = "pdf", alias = "PDF")]
    PDF,
    #[serde(alias = "zip", alias = "ZIP")]
    ZIP,
    #[serde(alias = "doc", alias = "docx", alias = "word", alias = "WORD")]
    WORD,
    #[serde(alias = "ppt", alias = "pptx", alias = "PPTX")]
    PPTX,
    #[serde(alias = "xls", alias = "xlsx", alias = "excel", alias = "EXCEL")]
    EXCEL,
    #[serde(alias = "mp4", alias = "MP4")]
    MP4,
    #[serde(alias = "mov", alias = "MOV")]
    MOV,
    #[serde(alias = "mkv", alias = "MKV")]
    MKV,
}
