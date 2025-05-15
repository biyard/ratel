use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillTexts {
    #[serde(rename = "textVersions")]
    pub text_versions: Vec<BillTextItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillTextItem {
    // 1: 날짜
    pub date: Option<String>,
    // 2: 데이터 형식 별 텍스트
    pub formats: Vec<BillTextFormat>,
    // 3: 법안 분류
    pub r#type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillTextFormat {
    pub r#type: String,
    pub url: String,
}

impl BillTexts {
    pub fn get_pdf_url(&self) -> Option<String> {
        if self.text_versions.is_empty() {
            return None;
        }
        self.text_versions[0]
            .formats
            .iter()
            .find(|format| format.r#type == "PDF")
            .map(|format| format.url.clone())
    }

    pub fn get_html_url(&self) -> Option<String> {
        if self.text_versions.is_empty() {
            return None;
        }
        self.text_versions[0]
            .formats
            .iter()
            .find(|format| format.r#type == "Formatted Text")
            .map(|format| format.url.clone())
    }

    pub fn get_xml_url(&self) -> Option<String> {
        if self.text_versions.is_empty() {
            return None;
        }
        self.text_versions[0]
            .formats
            .iter()
            .find(|format| format.r#type == "Formatted XML")
            .map(|format| format.url.clone())
    }
}
