use crate::utils::to_date;
use dto::EUBillWriter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EUAdoptedText {
    // 1: 문서 ID e.g., eli/dl/doc/TA-9-2022-0201
    pub id: i64,

    // 2: 의회 대수 e.g., org/ep-9
    pub parliamentary_term: String,

    // 3: 날짜 e.g., 2022-05-05
    pub document_date: String,

    // 4. 식별자 e.g., TA-10-2024-0001
    pub identifier: String,

    // 5. 문서 실제 구현 정보
    #[serde(rename = "is_realized_by")]
    pub realizations: Vec<EUTextRealization>,

    // 6. 다국어 제목 정보
    #[serde(rename = "title_dcterms")]
    pub titles: HashMap<String, String>,

    // 7. EP 번호
    #[serde(rename = "epNumber")]
    pub ep_number: String,

    // 8. 공개 등록부 표기
    #[serde(rename = "notation_publicRegister")]
    pub notation_public_register: String,

    // 9. 라벨
    pub label: String,

    // 10. 디렉토리 코드 목록
    #[serde(rename = "isAboutDirectoryCode")]
    pub directory_codes: Vec<String>,

    // 11. 주제 분야 목록 e.g., http://publications.europa.eu/resource/authority/subject-matter/PESC
    #[serde(rename = "isAboutSubjectMatter")]
    pub subject_matters: Vec<String>,

    // 12. 관련 주제 URI 목록 e.g., http://eurovoc.europa.eu/584
    #[serde(rename = "is_about")]
    pub topics: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EUTextRealization {
    // ID e.g., eli/dl/doc/TA-9-2022-0201/en
    pub id: String,

    // 구현 형태 목록
    #[serde(rename = "is_embodied_by", default)]
    pub embodiments: Vec<EUTextEmbodiment>,

    // 제목 (다국어)
    pub title: HashMap<String, String>,

    // 대체 제목 (다국어)
    pub title_alternative: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EUTextEmbodiment {
    // ID e.g., eli/dl/doc/TA-9-2022-0201/en/docx
    pub id: String,

    // 실제 파일 경로
    #[serde(rename = "is_exemplified_by")]
    pub file_path: String,

    // 파일 형식 e.g., http://publications.europa.eu/resource/authority/file-type/DOCX
    pub format: String,

    // 발행 날짜 e.g., 2022-06-28T16:35:59+02:00
    pub issued: String,

    // 파일 크기 (바이트)
    #[serde(rename = "byteSize")]
    pub byte_size: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EUAdoptedTextSummary {
    // 1: 문서 ID e.g., eli/dl/doc/TA-9-2022-0201
    pub id: String,

    // 2: 식별자 e.g., TA-10-2024-0001
    pub identifier: String,

    // 3: 라벨 e.g., T10-0001/2024
    pub label: String,
}

impl Into<EUBillWriter> for EUAdoptedText {
    fn into(self) -> EUBillWriter {
        EUBillWriter {
            bill_id: self.identifier.clone(),
            year: self
                .identifier
                .split('-')
                .nth(2)
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or_default(),
            parliamentary_term: self
                .parliamentary_term
                .split('-')
                .last()
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or_default(),
            bill_no: self
                .identifier
                .split('-')
                .last()
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or_default(),
            date: to_date(self.document_date.clone()),
            label: self.label.clone(),
            ep_number: self.ep_number.clone(),

            title: self.titles.get("en").unwrap_or(&"".to_string()).clone(),
            alternative_title: self.titles.get("en").map(|s| s.clone()),
            pdf_url: self.realizations[0]
                .embodiments
                .iter()
                .find(|r| r.id.ends_with("/en/pdf"))
                .and_then(|e| Some(e.file_path.clone())),
            ..Default::default()
        }
    }
}
