use crate::utils::iso_to_date;
use dto::CHBillWriter;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CHAffair {
    // 1: 법안 ID e.g., 20250001
    pub id: i64,

    // 2: 법안 번호 e.g., 25.001
    #[serde(rename = "formattedId")]
    pub formatted_id: String,

    // 3: 법안 제목
    pub title: String,

    // 4: 법안 상세 내용
    pub description: Option<String>,

    // 5: 법안 제안 당시 상태
    #[serde(rename = "initialSituation")]
    pub initial_situation: Option<String>,

    // 6: 법안 진행 상황
    pub proceedings: Option<String>,

    // 7: 법안 심사 과정
    // pub objectives: Vec<Objective>,

    // 8: 법안 업데이트 날짜
    pub updated: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Committee {
    // 1: 위원회 ID
    pub id: i64,

    // 2: 위원회 이름
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resolution {
    // 1: 결의안 날짜
    pub date: String,

    // 2: 위원회
    pub committee: Option<Committee>,

    // 3: 의회 대수
    pub council: Option<i64>,

    // 4: 상세 내용
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Objective {
    // 1: 심사 순서
    pub number: i64,

    // 2: 상세 내용
    pub text: String,

    // 3: 결의안
    pub resolutions: Vec<Resolution>,
}

impl CHAffair {
    pub fn get_year(&self) -> i64 {
        self.id / 10000
    }

    pub fn get_bill_no(&self) -> i64 {
        self.id % 10000
    }
}

impl Into<CHBillWriter> for CHAffair {
    fn into(self) -> CHBillWriter {
        CHBillWriter {
            bill_id: self.id,
            year: self.get_year(),
            bill_no: self.get_bill_no(),
            title: self.title.clone(),
            description: self.description.clone(),
            initial_situation: self.initial_situation.clone(),
            proceedings: self.proceedings.clone(),
            date: iso_to_date(self.updated),

            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CHAffairSummary {
    // 1: 법안 ID e.g., 20250001
    pub id: i64,

    // 2: 법안 업데이트 날짜
    pub updated: String,

    // 3: 마지막 페이지 여부
    #[serde(rename = "hasMorePages")]
    pub has_more_pages: Option<bool>,

    // 4: 법안 번호 e.g., 25.001
    #[serde(rename = "formattedId")]
    pub formatted_id: String,

    // 5: 법안 제목
    pub title: String,
}
