use super::common::LatestAction;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillInfo {
    /// 1: 국회 회기 번호
    pub congress: i64,

    /// 2: 최근 법안 활동 정보
    #[serde(rename = "latestAction")]
    pub latest_action: LatestAction,

    /// 3: 법안 번호
    pub number: i64,

    /// 4: 발의 원(상원/하원)
    #[serde(rename = "originChamber")]
    pub origin_chamber: String,

    /// 4: 발의 원 코드
    #[serde(rename = "originChamberCode")]
    pub origin_chamber_code: String,

    /// 5: 법안 제목
    pub title: String,

    /// 6: 법안 유형
    #[serde(rename = "type")]
    pub bill_type: String,

    /// 7: 업데이트 날짜
    #[serde(rename = "updateDate")]
    pub update_date: String,

    /// 8: 텍스트를 포함한 업데이트 날짜
    #[serde(rename = "updateDateIncludingText")]
    pub update_date_including_text: String,

    /// 9: 법안 URL
    pub url: String,
}

impl BillInfo {
    pub fn convert_bill_type(&self) -> dto::USBillType {
        match self.bill_type.as_str() {
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
        match self.origin_chamber.as_str() {
            "House" => dto::Chamber::House,
            "Senate" => dto::Chamber::Senate,
            _ => dto::Chamber::Unknown,
        }
    }
}
