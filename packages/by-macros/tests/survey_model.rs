#![allow(unused)]
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct QueryResponse<T> {
    pub items: Vec<T>,
    pub total_count: i64,
}

impl<T> From<(Vec<T>, i64)> for QueryResponse<T> {
    fn from((items, total_count): (Vec<T>, i64)) -> Self {
        QueryResponse { items, total_count }
    }
}

pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[api_model(base = "/public-surveys/v1/questions", table = questions, iter_type=QueryResponse)]
pub struct PublicSurveyQuestion {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    pub title: String,
    pub description: String,
    pub image_url: Option<String>,

    //1~10 까지 그렇다~아니다일 경우 다음 필드 활용
    pub answer_start_range: Option<i64>,
    pub answer_end_range: Option<i64>,

    //체크박스, 드롭다운인 경우 선택지 입력, 평가척도의 경우 1, 10이 어느 구간에 해당하는지 입력
    pub options: Option<Vec<String>>,

    //복수 응답 유무
    pub multiple_choice_enable: Option<bool>,

    //필수 입력 유무
    pub necessary_answer_enable: Option<bool>,
}
