use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LatestAction {
    /// 1: 현황 업데이트 날짜
    #[serde(rename = "actionDate")]
    pub action_date: String,

    /// 2: 현황 내용
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyArea {
    /// 1: 정책 영역 이름
    pub name: String,
}
