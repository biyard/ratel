use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillTitles {
    pub titles: Vec<BillTitleItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillTitleItem {
    // 1: 법안 제목
    pub title: String,
    // 2: 법안 제목 분류
    pub title_type: String,
    // 3: 법안 제목 코드
    pub title_type_code: i64,
    // 4: 업데이트 날짜
    pub update_date: Option<String>,
    // 5: 법안 제목 버전 코드
    pub bill_text_version_code: Option<String>,
    // 6: 법안 제목 버전 이름
    pub bill_text_version_name: Option<String>,
    // 7: 발의원 코드
    pub chamber_code: Option<String>,
    // 8: 발의원 이름
    pub chamber_name: Option<String>,
}

impl BillTitles {
    pub fn get_display_title(&self) -> Option<String> {
        self.titles
            .iter()
            .find(|item| item.title_type == "Display Title")
            .map(|item| item.title.clone())
    }

    pub fn get_short_title(&self) -> Option<String> {
        self.titles
            .iter()
            .find(|item| {
                item.title_type == "Short Title(s) as Reported to House for portions of this bill"
            })
            .map(|item| item.title.clone())
    }
}
