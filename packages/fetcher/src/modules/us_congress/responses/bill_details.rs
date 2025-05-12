use super::common::{LatestAction, PolicyArea};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillDetail {
    pub bill: BillDetailItem,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillDetailItem {
    /// 1: 법안 조치 정보
    pub actions: ResourceCount,

    /// 2: 법안 수정안 정보
    pub amendments: ResourceCount,

    /// 3: CBO 비용 추정
    #[serde(rename = "cboCostEstimates")]
    pub cbo_cost_estimates: Vec<CboEstimate>,

    /// 4: 위원회 보고서
    #[serde(rename = "committeeReports")]
    pub committee_reports: Vec<CommitteeReport>,

    /// 5: 위원회 정보
    pub committees: ResourceCount,

    /// 6: 의회 회기
    pub congress: i64,

    /// 7: 헌법적 권한 진술문
    #[serde(rename = "constitutionalAuthorityStatementText")]
    pub constitutional_authority_statement_text: Option<String>,

    /// 8: 공동 발의자 정보
    pub cosponsors: CosponsorInfo,

    /// 9: 발의 날짜
    #[serde(rename = "introducedDate")]
    pub introduced_date: String,

    /// 10: 최근 조치 정보
    #[serde(rename = "latestAction")]
    pub latest_action: LatestAction,

    /// 11: 법률 정보
    pub laws: Vec<LawInfo>,

    /// 12: 법안 번호
    pub number: String,

    /// 13: 발의 원
    #[serde(rename = "originChamber")]
    pub origin_chamber: String,

    /// 14: 정책 영역
    #[serde(rename = "policyArea")]
    pub policy_area: PolicyArea,

    /// 15: 관련 법안
    #[serde(rename = "relatedBills")]
    pub related_bills: ResourceCount,

    /// 16: 발의자 정보
    pub sponsors: Vec<Sponsor>,

    /// 17: 주제
    pub subjects: ResourceCount,

    /// 18: 요약
    pub summaries: ResourceCount,

    /// 19: 법안 텍스트 버전
    #[serde(rename = "textVersions")]
    pub text_versions: ResourceCount,

    /// 20: 법안 제목
    pub title: String,

    /// 21: 법안 제목 정보
    pub titles: ResourceCount,

    /// 22: 법안 유형
    #[serde(rename = "type")]
    pub bill_type: String,

    /// 23: 업데이트 날짜
    #[serde(rename = "updateDate")]
    pub update_date: String,

    /// 24: 텍스트 포함 업데이트 날짜
    #[serde(rename = "updateDateIncludingText")]
    pub update_date_including_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceCount {
    /// 1: 항목 수
    pub count: u32,

    /// 2: 리소스 URL
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CosponsorInfo {
    /// 1: 공동발의자 수
    pub count: u32,

    /// 2: 철회한 공동발의자를 포함한 수
    #[serde(rename = "countIncludingWithdrawnCosponsors")]
    pub count_including_withdrawn_cosponsors: u32,

    /// 3: 리소스 URL
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CboEstimate {
    /// 1: 설명
    pub description: String,

    /// 2: 발행 날짜
    #[serde(rename = "pubDate")]
    pub pub_date: String,

    /// 3: 제목
    pub title: String,

    /// 4: URL
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommitteeReport {
    /// 1: 인용
    pub citation: String,

    /// 2: URL
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LawInfo {
    /// 1: 법률 번호
    pub number: String,

    /// 2: 법률 유형
    #[serde(rename = "type")]
    pub law_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sponsor {
    /// 1: 바이오가이드 ID
    #[serde(rename = "bioguideId")]
    pub bioguide_id: String,

    /// 2: 지역구
    pub district: Option<u32>,

    /// 3: 이름
    #[serde(rename = "firstName")]
    pub first_name: String,

    /// 4: 전체 이름
    #[serde(rename = "fullName")]
    pub full_name: String,

    /// 5: 요청에 의한 것인지 여부
    #[serde(rename = "isByRequest")]
    pub is_by_request: String,

    /// 6: 라스트네임
    #[serde(rename = "lastName")]
    pub last_name: String,

    /// 7: 미들네임
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,

    /// 8: 정당
    pub party: String,

    /// 9: 주
    pub state: String,

    /// 10: URL
    pub url: String,
}

impl BillDetail {
    pub fn convert_bill_type(&self) -> dto::USBillType {
        match self.bill.bill_type.as_str() {
            "hr" => dto::USBillType::HouseBill,
            "s" => dto::USBillType::SenateBill,
            "hjres" => dto::USBillType::HouseJointResolution,
            "sjres" => dto::USBillType::SenateJointResolution,
            "hconres" => dto::USBillType::HouseConcurrentResolution,
            "sconres" => dto::USBillType::SenateConcurrentResolution,
            "hres" => dto::USBillType::HouseSimpleResolution,
            "sres" => dto::USBillType::SenateSimpleResolution,
            _ => dto::USBillType::Unknown,
        }
    }

    pub fn get_origin_chamber(&self) -> dto::Chamber {
        match self.bill.origin_chamber.as_str() {
            "House" => dto::Chamber::House,
            "Senate" => dto::Chamber::Senate,
            _ => dto::Chamber::Unknown,
        }
    }
}
