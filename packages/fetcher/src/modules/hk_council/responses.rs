use crate::utils::iso_to_date;
use dto::{HKBillStatus, HKBillWriter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HKBill {
    /// 1: 내부 키 (Primary Key)
    pub internal_key: String,

    /// 2: 조례 영문 제목
    pub ordinance_title_eng: Option<String>,

    /// 3: 조례 중문 제목
    pub ordinance_title_chi: Option<String>,

    /// 4: 조례 내용 영문 URL
    pub ordinance_content_url_eng: Option<String>,

    /// 5: 조례 내용 중문 URL
    pub ordinance_content_url_chi: Option<String>,

    /// 6: 법안 영문 제목
    pub bill_title_eng: Option<String>,

    /// 7: 법안 중문 제목
    pub bill_title_chi: Option<String>,

    /// 8: 제안자 영문명
    pub proposed_by_eng: Option<String>,

    /// 9: 제안자 중문명
    pub proposed_by_chi: Option<String>,

    /// 10: 법안 관보 게재일
    pub bill_gazette_date: Option<String>,

    /// 11: 법안 내용 영문 URL
    pub bill_content_url_eng: Option<String>,

    /// 12: 법안 내용 중문 URL
    pub bill_content_url_chi: Option<String>,

    /// 13: 법안 관보 게재일 (2차)
    pub bill_gazette_date_2: Option<String>,

    /// 14: 법안 내용 영문 URL (2차)
    pub bill_content_url_2_eng: Option<String>,

    /// 15: 법안 내용 중문 URL (2차)
    pub bill_content_url_2_chi: Option<String>,

    /// 16: 법안 관보 게재일 (3차)
    pub bill_gazette_date_3: Option<String>,

    /// 17: 법안 내용 영문 URL (3차)
    pub bill_content_url_3_eng: Option<String>,

    /// 18: 법안 내용 중문 URL (3차)
    pub bill_content_url_3_chi: Option<String>,

    /// 19: 조례 관보 게재일
    pub ordinance_gazette_date: Option<String>,

    /// 20: 조례 연도 및 번호 (영문)
    pub ordinance_year_number_eng: Option<String>,

    /// 21: 조례 연도 및 번호 (중문)
    pub ordinance_year_number_chi: Option<String>,

    /// 22: 조례 관보 내용 영문 URL
    pub ordinace_gazette_content_url_eng: Option<String>,

    /// 23: 조례 관보 내용 중문 URL
    pub ordinance_gazette_content_url_chi: Option<String>,

    /// 24: LegCo 브리핑 파일 참조번호
    pub legco_brief_file_reference: Option<String>,

    /// 25: LegCo 브리핑 영문 URL
    pub legco_brief_url_eng: Option<String>,

    /// 26: LegCo 브리핑 중문 URL
    pub legco_brief_url_chi: Option<String>,

    /// 27: 1차 독회 일자
    pub first_reading_date: Option<String>,

    /// 28: 1차 독회 회의록 영문 URL
    pub first_reading_date_hansard_url_eng: Option<String>,

    /// 29: 1차 독회 회의록 중문 URL
    pub first_reading_date_hansard_url_chi: Option<String>,

    /// 30: 1차 독회 일자 (2차)
    pub first_reading_date_2: Option<String>,

    /// 31: 1차 독회 회의록 영문 URL (2차)
    pub first_reading_date_2_hansard_url_eng: Option<String>,

    /// 32: 1차 독회 회의록 중문 URL (2차)
    pub first_reading_date_2_hansard_url_chi: Option<String>,

    /// 33: 법안위원회 영문 제목
    pub bills_committee_title_eng: Option<String>,

    /// 34: 법안위원회 중문 제목
    pub bills_committee_title_chi: Option<String>,

    /// 35: 법안위원회 영문 URL
    pub bills_committee_url_eng: Option<String>,

    /// 36: 법안위원회 중문 URL
    pub bills_committee_url_chi: Option<String>,

    /// 37: 법안위원회 구성 일자
    pub bills_committee_formation_date: Option<String>,

    /// 38: 법안위원회 보고서 영문 URL
    pub bills_committee_report_url_eng: Option<String>,

    /// 39: 법안위원회 보고서 중문 URL
    pub bills_committee_report_url_chi: Option<String>,

    /// 40: 2차 독회 일자
    pub second_reading_date: Option<String>,

    /// 41: 2차 독회 회의록 영문 URL
    pub second_reading_date_hansard_url_eng: Option<String>,

    /// 42: 2차 독회 회의록 중문 URL
    pub second_reading_date_hansard_url_chi: Option<String>,

    /// 43: 2차 독회 일자 (2차)
    pub second_reading_date_2: Option<String>,

    /// 44: 2차 독회 회의록 영문 URL (2차)
    pub second_reading_date_2_hansard_url_eng: Option<String>,

    /// 45: 2차 독회 회의록 중문 URL (2차)
    pub second_reading_date_2_hansard_url_chi: Option<String>,

    /// 46: 2차 독회 일자 (3차)
    pub second_reading_date_3: Option<String>,

    /// 47: 2차 독회 회의록 영문 URL (3차)
    pub second_reading_date_3_hansard_url_eng: Option<String>,

    /// 48: 2차 독회 회의록 중문 URL (3차)
    pub second_reading_date_3_hansard_url_chi: Option<String>,

    /// 49: 2차 독회 일자 (4차)
    pub second_reading_date_4: Option<String>,

    /// 50: 2차 독회 회의록 영문 URL (4차)
    pub second_reading_date_4_hansard_url_eng: Option<String>,

    /// 51: 2차 독회 회의록 중문 URL (4차)
    pub second_reading_date_4_hansard_url_chi: Option<String>,

    /// 52: 2차 독회 일자 (5차)
    pub second_reading_date_5: Option<String>,

    /// 53: 2차 독회 회의록 영문 URL (5차)
    pub second_reading_date_5_hansard_url_eng: Option<String>,

    /// 54: 2차 독회 회의록 중문 URL (5차)
    pub second_reading_date_5_hansard_url_chi: Option<String>,

    /// 55: 3차 독회 일자
    pub third_reading_date: Option<String>,

    /// 56: 3차 독회 회의록 영문 URL
    pub third_reading_date_hansard_url_eng: Option<String>,

    /// 57: 3차 독회 회의록 중문 URL
    pub third_reading_date_hansard_url_chi: Option<String>,

    /// 58: 추가 정보 (영문)
    pub additional_information_eng: Option<String>,

    /// 59: 추가 정보 (중문)
    pub additional_information_chi: Option<String>,

    /// 60: 비고 (영문)
    pub remarks_eng: Option<String>,

    /// 61: 비고 (중문)
    pub remarks_chi: Option<String>,
}

/// 법안 상태를 나타내는 열거형
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BillStatus {
    Introduced,
    FirstReading,
    SecondReading,
    ThirdReading,
    Passed,
}

impl From<HKBill> for HKBillWriter {
    fn from(bill: HKBill) -> Self {
        let parse_date = |opt_date: &Option<String>| -> Option<i32> {
            opt_date.as_ref().and_then(|s| {
                let date_part = s.split('T').next().unwrap_or(s);
                date_part.replace("-", "").parse::<i32>().ok()
            })
        };

        let bill_clone = bill.clone();

        HKBillWriter {
            bill_id: bill.get_id(),
            year: bill.get_year(),
            bill_no: bill.get_bill_number(),
            title: bill.bill_title_eng.unwrap_or_default(),
            proposer: bill.proposed_by_eng.unwrap_or_default(),
            content_url: bill.bill_content_url_eng.clone(),
            committee_name: bill.bills_committee_title_eng.clone(),

            proposed_date: bill.bill_gazette_date.map(iso_to_date).unwrap_or_default(),
            first_reading_date: bill.first_reading_date.map(iso_to_date),
            second_reading_date: bill.second_reading_date.map(iso_to_date),
            third_reading_date: bill.third_reading_date.map(iso_to_date),
            ordinance_date: bill.ordinance_gazette_date.map(iso_to_date),

            additional_information: bill.additional_information_eng.clone(),
            remarks: bill.remarks_eng.clone(),
            status: bill_clone.get_status(),

            ..Default::default()
        }
    }
}

impl HKBill {
    pub fn get_id(&self) -> String {
        self.internal_key.trim().to_string()
    }

    pub fn get_year(&self) -> i32 {
        self.bill_gazette_date
            .as_ref()
            .and_then(|s| s.split('-').next())
            .and_then(|year_str| year_str.parse::<i32>().ok())
            .unwrap_or(0)
    }

    // e.g., "Ord. No. 28 of 2004" -> "28"
    pub fn get_bill_number(&self) -> i32 {
        self.bill_title_eng
            .as_ref()
            .and_then(|s| s.split_whitespace().nth(2))
            .and_then(|num_str| num_str.parse::<i32>().ok())
            .unwrap_or(0)
    }

    pub fn get_status(&self) -> HKBillStatus {
        if self.ordinance_gazette_date.is_some() {
            HKBillStatus::Passed
        } else if self.third_reading_date.is_some() {
            HKBillStatus::ThirdReading
        } else if self.second_reading_date.is_some() {
            HKBillStatus::SecondReading
        } else if self.first_reading_date.is_some() {
            HKBillStatus::FirstReading
        } else if self.bill_gazette_date.is_some() {
            HKBillStatus::Introduced
        } else {
            HKBillStatus::Unknown
        }
    }
}
