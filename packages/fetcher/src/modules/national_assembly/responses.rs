use dto::BillWriter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Member {
    // 1: HG_NM (이름)
    #[serde(rename(deserialize = "HG_NM"))]
    pub name: String, // Korean Name

    // 2: HJ_NM (한자명) - Assuming optional
    #[serde(rename(deserialize = "HJ_NM"))]
    pub hanja_name: Option<String>, // Hanja Name

    // 3: ENG_NM (영문명칭) - Assuming optional
    #[serde(rename(deserialize = "ENG_NM"))]
    pub english_name: Option<String>, // English Name

    // 4: BTH_GBN_NM (음/양력)
    #[serde(rename(deserialize = "BTH_GBN_NM"))]
    pub birth_calendar_type: String, // e.g., "음력", "양력"

    // 5: BTH_DATE (생년월일) - Keeping as String, parsing can be done later if needed
    #[serde(rename(deserialize = "BTH_DATE"))]
    pub birth_date: String, // Birth Date (e.g., "YYYY-MM-DD")

    // 6: JOB_RES_NM (직책명)
    #[serde(rename(deserialize = "JOB_RES_NM"))]
    pub job_title: String, // Job/Position Name

    // 7: POLY_NM (정당명)
    #[serde(rename(deserialize = "POLY_NM"))]
    pub party: String, // Political Party Name

    // 8: ORIG_NM (선거구)
    #[serde(rename(deserialize = "ORIG_NM"))]
    pub district: String, // Electoral District Name

    // 9: ELECT_GBN_NM (선거구분)
    #[serde(rename(deserialize = "ELECT_GBN_NM"))]
    pub election_type: String, // Type of election (e.g., "지역구", "비례대표")

    // 10: CMIT_NM (대표 위원회)
    #[serde(rename(deserialize = "CMIT_NM"))]
    pub representative_committee: String, // Name of the representative committee

    // 11: CMITS (소속 위원회 목록) - Often a list, but image doesn't specify format, using String
    #[serde(rename(deserialize = "CMITS"))]
    pub committees: String, // Affiliated committees (might be comma-separated or similar)

    // 12: REELE_GBN_NM (재선)
    #[serde(rename(deserialize = "REELE_GBN_NM"))]
    pub election_term_desc: String, // Description of terms (e.g., "초선", "재선", "3선")

    // 13: UNITS (당선) - Number of terms, using String as type isn't specified
    #[serde(rename(deserialize = "UNITS"))]
    pub terms_served: String, // Number of terms served (e.g., "1", "2")

    // 14: SEX_GBN_NM (성별)
    #[serde(rename(deserialize = "SEX_GBN_NM"))]
    pub gender: String, // Gender (e.g., "남", "여")

    // 15: TEL_NO (전화번호) - Assuming optional
    #[serde(rename(deserialize = "TEL_NO"))]
    pub telephone: Option<String>, // Telephone Number

    // 16: E_MAIL (이메일) - Already present, confirmed optional
    #[serde(rename(deserialize = "E_MAIL"))]
    pub email: Option<String>, // Email

    // 17: HOMEPAGE (홈페이지) - Assuming optional
    #[serde(rename(deserialize = "HOMEPAGE"))]
    pub homepage: Option<String>, // Homepage URL

    // 18: STAFF (보좌관) - Assuming optional
    #[serde(rename(deserialize = "STAFF"))]
    pub staff: Option<String>, // Aide(s)

    // 19: SECRETARY (선임비서관) - Assuming optional
    #[serde(rename(deserialize = "SECRETARY"))]
    pub senior_secretary: Option<String>, // Senior Secretary

    // 20: SECRETARY2 (비서관) - Assuming optional
    #[serde(rename(deserialize = "SECRETARY2"))]
    pub secretary: Option<String>, // Secretary

    // 21: MONA_CD (국회의원코드) - Already present
    #[serde(rename(deserialize = "MONA_CD"))]
    pub code: String, // Unique Member Code

    // 22: MEM_TITLE (약력) - Assuming optional
    #[serde(rename(deserialize = "MEM_TITLE"))]
    pub profile_summary: Option<String>, // Profile Summary / Biography

    // 23: ASSEM_ADDR (사무실 호실) - Assuming optional
    #[serde(rename(deserialize = "ASSEM_ADDR"))]
    pub office_address: Option<String>, // Assembly Office Address/Room Number
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EnMember {
    // https://open.assembly.go.kr/portal/data/service/selectAPIServicePage.do/OOWY4R001216HX11447
    #[serde(rename(deserialize = "NAAS_EN_NM"))]
    pub name: String, // English Name
    #[serde(rename(deserialize = "PLPT_NM"))]
    pub party: String, // English Party Name
    #[serde(rename(deserialize = "ELECD_NM"))]
    pub district: Option<String>, // English District Name
    #[serde(rename(deserialize = "NAAS_EMAIL_ADDR"))]
    pub email: Option<String>, // Email
}

// Struct name suggestion: AssemblyMember (as NAAS likely refers to National Assembly)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssemblyMember {
    // 1: NAAS_CD (국회의원코드)
    #[serde(rename(deserialize = "NAAS_CD"))]
    pub code: String,

    // 2: NAAS_NM (국회의원명)
    #[serde(rename(deserialize = "NAAS_NM"))]
    pub name: String,

    // 3: NAAS_CH_NM (국회의원한자명) - Assuming optional
    #[serde(rename(deserialize = "NAAS_CH_NM"))]
    pub hanja_name: Option<String>,

    // 4: NAAS_EN_NM (국회의원영문명) - Assuming optional
    #[serde(rename(deserialize = "NAAS_EN_NM"))]
    pub english_name: Option<String>,

    // 5: BIRDY_DIV_CD (생일구분코드)
    #[serde(rename(deserialize = "BIRDY_DIV_CD"))]
    pub birth_date_type_code: String,

    // 6: BIRDY_DT (생년월일) - Keeping as String
    #[serde(rename(deserialize = "BIRDY_DT"))]
    pub birth_date: String,

    // 7: DTY_NM (직책명)
    #[serde(rename(deserialize = "DTY_NM"))]
    pub duty_name: String,

    // 8: PLPT_NM (정당명)
    #[serde(rename(deserialize = "PLPT_NM"))]
    pub party_name: String,

    // 9: ELECD_NM (선거구명)
    #[serde(rename(deserialize = "ELECD_NM"))]
    pub electoral_district_name: String,

    // 10: ELECD_DIV_NM (선거구구분명)
    #[serde(rename(deserialize = "ELECD_DIV_NM"))]
    pub electoral_district_type_name: String,

    // 11: CMIT_NM (위원회명)
    #[serde(rename(deserialize = "CMIT_NM"))]
    pub committee_name: String,

    // 12: BLNG_CMIT_NM (소속위원회명) - Using String, adjust if it's actually a list
    #[serde(rename(deserialize = "BLNG_CMIT_NM"))]
    pub affiliated_committee_name: String,

    // 13: RLCT_DIV_NM (재선구분명)
    #[serde(rename(deserialize = "RLCT_DIV_NM"))]
    pub reelection_type_name: String, // e.g., "초선", "재선"

    // 14: GTELT_ERACO (당선대수) - Using String as format is unclear (e.g., "21대")
    #[serde(rename(deserialize = "GTELT_ERACO"))]
    pub terms_served_eras: String,

    // 15: NTR_DIV (성별)
    #[serde(rename(deserialize = "NTR_DIV"))]
    pub gender: String,

    // 16: NAAS_TEL_NO (전화번호) - Assuming optional
    #[serde(rename(deserialize = "NAAS_TEL_NO"))]
    pub telephone: Option<String>,

    // 17: NAAS_EMAIL_ADDR (국회의원이메일주소) - Assuming optional
    #[serde(rename(deserialize = "NAAS_EMAIL_ADDR"))]
    pub email: Option<String>,

    // 18: NAAS_HP_URL (국회의원홈페이지URL) - Assuming optional
    #[serde(rename(deserialize = "NAAS_HP_URL"))]
    pub homepage_url: Option<String>,

    // 19: AIDE_NM (보좌관) - Assuming optional
    #[serde(rename(deserialize = "AIDE_NM"))]
    pub aide_name: Option<String>,

    // 20: CHF_SCRT_NM (비서관) - Assuming optional, likely Chief Secretary based on 'CHF'
    #[serde(rename(deserialize = "CHF_SCRT_NM"))]
    pub chief_secretary_name: Option<String>,

    // 21: SCRT_NM (비서) - Assuming optional
    #[serde(rename(deserialize = "SCRT_NM"))]
    pub secretary_name: Option<String>,

    // 22: BRF_HIST (약력) - Assuming optional
    #[serde(rename(deserialize = "BRF_HIST"))]
    pub brief_history: Option<String>, // Biography / Brief History

    // 23: OFFM_RNUM_NO (사무실 호실) - Assuming optional
    #[serde(rename(deserialize = "OFFM_RNUM_NO"))]
    pub office_room_number: Option<String>,

    // 24: NAAS_PIC (국회의원사진) - Assuming optional, likely a URL
    #[serde(rename(deserialize = "NAAS_PIC"))]
    pub picture_url: Option<String>,
}

// Represents the detailed information and progress of a legislative bill
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillDetail {
    // 1: ERACO (대수)
    #[serde(rename(deserialize = "ERACO"))]
    pub assembly_term: String, // Assembly term, e.g., "21"

    // 2: BILL_ID (의안ID)
    #[serde(rename(deserialize = "BILL_ID"))]
    pub bill_id: String, // Bill identifier

    // 3: BILL_NO (의안번호)
    #[serde(rename(deserialize = "BILL_NO"))]
    pub bill_no: String, // Bill number

    // 4: BILL_KND (의안종류)
    #[serde(rename(deserialize = "BILL_KND"))]
    pub bill_kind: String, // Kind of bill (e.g., "법률안")

    // 5: BILL_NM (의안명)
    #[serde(rename(deserialize = "BILL_NM"))]
    pub bill_name: String, // Name/Title of the bill

    // 6: PPSR_KIND (제안자구분)
    #[serde(rename(deserialize = "PPSR_KND"))]
    pub proposer_kind: String, // Type of proposer (e.g., "의원", "정부")

    // 7: PPSR_NM (제안자명) - Might contain multiple names
    #[serde(rename(deserialize = "PPSR_NM"))]
    pub proposer_name: String, // Name(s) of the proposer(s)

    // 8: PPSL_SESS (제안회기) - Assuming optional
    #[serde(rename(deserialize = "PPSL_SESS"))]
    pub proposal_session: Option<String>, // Assembly session when proposed

    // 9: PPSL_DT (제안일)
    #[serde(rename(deserialize = "PPSL_DT"))]
    pub proposal_date: String, // Date proposed (e.g., "YYYY-MM-DD")

    // 10: JRCMIT_NM (소관위원회명) - Assuming optional (might not be assigned yet)
    #[serde(rename(deserialize = "JRCMIT_NM"))]
    pub committee_name: Option<String>, // Name of the responsible committee

    // 11: JRCMIT_CMMT_DT (소관위원회 회부일) - Assuming optional
    #[serde(rename(deserialize = "JRCMIT_CMMT_DT"))]
    pub committee_referral_date: Option<String>, // Date referred to the committee

    // 12: JRCMIT_PRSNT_DT (소관위원회 상정일) - Assuming optional
    #[serde(rename(deserialize = "JRCMIT_PRSNT_DT"))]
    pub committee_presentation_date: Option<String>, // Date presented to the committee

    // 13: JRCMIT_PROC_DT (소관위원회 처리일) - Assuming optional
    #[serde(rename(deserialize = "JRCMIT_PROC_DT"))]
    pub committee_processing_date: Option<String>, // Date processed by the committee

    // 14: JRCMIT_PROC_RSLT (소관위원회 처리결과) - Assuming optional
    #[serde(rename(deserialize = "JRCMIT_PROC_RSLT"))]
    pub committee_processing_result: Option<String>, // Result from the committee (e.g., "원안가결")

    // 15: LAW_CMMT_DT (법사위 체계자구심사 회부일) - Assuming optional
    #[serde(rename(deserialize = "LAW_CMMT_DT"))]
    pub law_committee_referral_date: Option<String>, // Date referred to Legislation & Judiciary Committee

    // 16: LAW_PRSNT_DT (법사위 체계자구심사 상정일) - Assuming optional
    #[serde(rename(deserialize = "LAW_PRSNT_DT"))]
    pub law_committee_presentation_date: Option<String>, // Date presented to L&J Committee

    // 17: LAW_PROC_DT (법사위 체계자구심사 처리일) - Assuming optional
    #[serde(rename(deserialize = "LAW_PROC_DT"))]
    pub law_committee_processing_date: Option<String>, // Date processed by L&J Committee

    // 18: LAW_PROC_RSLT (법사위 체계자구심사 처리결과) - Assuming optional
    #[serde(rename(deserialize = "LAW_PROC_RSLT"))]
    pub law_committee_processing_result: Option<String>, // Result from L&J Committee

    // 19: RGS_PRSNT_DT (본회의 심의 상정일) - Assuming optional
    #[serde(rename(deserialize = "RGS_PRSNT_DT"))]
    pub plenary_presentation_date: Option<String>, // Date presented to plenary session

    // 20: RGS_RSLN_DT (본회의 심의 의결일) - Assuming optional
    #[serde(rename(deserialize = "RGS_RSLN_DT"))]
    pub plenary_resolution_date: Option<String>, // Date resolved (voted on) in plenary session

    // 21: RGS_CONF_NM (본회의 심의 회의명) - Assuming optional
    #[serde(rename(deserialize = "RGS_CONF_NM"))]
    pub plenary_conference_name: Option<String>, // Name/ID of the plenary session meeting

    // 22: RGS_CONF_RSLT (본회의 심의결과) - Assuming optional
    #[serde(rename(deserialize = "RGS_CONF_RSLT"))]
    pub plenary_conference_result: Option<String>, // Result from plenary session (e.g., "가결")

    // 23: GVRN_TRSF_DT (정부 이송일) - Assuming optional
    #[serde(rename(deserialize = "GVRN_TRSF_DT"))]
    pub government_transfer_date: Option<String>, // Date transferred to the government

    // 24: PROM_LAW_NM (공포 법률명) - Assuming optional
    #[serde(rename(deserialize = "PROM_LAW_NM"))]
    pub promulgated_law_name: Option<String>, // Official name of the promulgated law

    // 25: PROM_DT (공포일) - Assuming optional
    #[serde(rename(deserialize = "PROM_DT"))]
    pub promulgation_date: Option<String>, // Date promulgated

    // 26: PROM_NO (공포번호) - Assuming optional (could be number or string)
    #[serde(rename(deserialize = "PROM_NO"))]
    pub promulgation_number: Option<String>, // Promulgation number

    // 27: LINK_URL (링크URL)
    #[serde(rename(deserialize = "LINK_URL"))]
    pub link_url: String, // URL for details
}

impl Into<BillWriter> for BillDetail {
    fn into(self) -> BillWriter {
        let proposal_date = self
            .proposal_date
            .replace("-", "")
            .parse()
            .expect("Failed to parse proposal_date");

        BillWriter {
            bill_no: self.bill_no.parse().expect("Failed to parse bill_no"),
            bill_id: self.bill_id,

            title: self.bill_name,
            date: self.proposal_date,

            proposer_kind: self.proposer_kind.parse().unwrap_or_default(),
            proposer_name: self.proposer_name,
            proposal_session: self.proposal_session,
            proposal_date,

            committee: self
                .committee_name
                .clone()
                .map(|name| name.parse().unwrap_or_default()),
            committee_name: self.committee_name,
            committee_referral_date: self.committee_referral_date.map(to_date),
            committee_presentation_date: self.committee_presentation_date.map(to_date),
            committee_processing_date: self.committee_processing_date.map(to_date),
            committee_processing_result: self.committee_processing_result,

            law_committee_referral_date: self.law_committee_referral_date.map(to_date),
            law_committee_presentation_date: self.law_committee_presentation_date.map(to_date),
            law_committee_processing_date: self.law_committee_processing_date.map(to_date),
            law_committee_processing_result: self.law_committee_processing_result,

            plenary_presentation_date: self.plenary_presentation_date.map(to_date),
            plenary_resolution_date: self.plenary_resolution_date.map(to_date),
            plenary_conference_name: self.plenary_conference_name,
            plenary_conference_result: self.plenary_conference_result,
            government_transfer_date: self.government_transfer_date.map(to_date),
            promulgated_law_name: self.promulgated_law_name,
            promulgation_date: self.promulgation_date.map(to_date),
            promulgation_number: self.promulgation_number,
            link_url: self.link_url,

            ..Default::default()
        }
    }
}

fn to_date(date: String) -> i32 {
    if let Ok(res) = date.replace("-", "").parse() {
        res
    } else {
        tracing::error!("Failed to parse date: {}", date);
        0
    }
}

// Represents summary information and processing status for a legislative bill
// Based on image_29867a.png
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillInfo {
    // 1: BILL_ID (의안ID)
    #[serde(rename(deserialize = "BILL_ID"))]
    pub bill_id: String,

    // 2: BILL_NO (의안번호)
    #[serde(rename(deserialize = "BILL_NO"))]
    pub bill_no: String,

    // 3: BILL_NAME (법률안명)
    #[serde(rename(deserialize = "BILL_NAME"))]
    pub bill_name: String, // Title of the bill

    // 4: COMMITTEE (소관위원회) - Assuming optional
    #[serde(rename(deserialize = "COMMITTEE"))]
    pub committee_name: Option<String>, // Name of the responsible committee

    // 5: PROPOSE_DT (제안일)
    #[serde(rename(deserialize = "PROPOSE_DT"))]
    pub propose_date: String, // Date proposed

    // 6: PROC_RESULT (본회의심의결과) - Assuming optional
    #[serde(rename(deserialize = "PROC_RESULT"))]
    pub plenary_processing_result: Option<String>, // Result from plenary session

    // 7: AGE (대수)
    #[serde(rename(deserialize = "AGE"))]
    pub assembly_term: String, // Assembly term (e.g., "21")

    // 8: DETAIL_LINK (상세페이지)
    #[serde(rename(deserialize = "DETAIL_LINK"))]
    pub detail_link: String, // URL to detail page

    // 9: PROPOSER (제안자) - Could be names or description like "정부"
    #[serde(rename(deserialize = "PROPOSER"))]
    pub proposer_info: String, // Proposer information

    // 10: MEMBER_LIST (제안자목록링크)
    #[serde(rename(deserialize = "MEMBER_LIST"))]
    pub proposer_list_link: String, // URL to list of proposers

    // 11: LAW_PROC_DT (법사위처리일) - Assuming optional
    #[serde(rename(deserialize = "LAW_PROC_DT"))]
    pub law_committee_processing_date: Option<String>, // Date processed by Legislation & Judiciary Committee

    // 12: LAW_PRESENT_DT (법사위상정일) - Assuming optional
    #[serde(rename(deserialize = "LAW_PRESENT_DT"))]
    pub law_committee_presentation_date: Option<String>, // Date presented to L&J Committee

    // 13: LAW_SUBMIT_DT (법사위회부일) - Assuming optional
    #[serde(rename(deserialize = "LAW_SUBMIT_DT"))]
    pub law_committee_referral_date: Option<String>, // Date referred to L&J Committee

    // 14: CMT_PROC_RESULT_CD (소관위처리결과) - Assuming optional (Result code)
    #[serde(rename(deserialize = "CMT_PROC_RESULT_CD"))]
    pub committee_processing_result_code: Option<String>, // Result code from responsible committee

    // 15: CMT_PROC_DT (소관위처리일) - Assuming optional
    #[serde(rename(deserialize = "CMT_PROC_DT"))]
    pub committee_processing_date: Option<String>, // Date processed by responsible committee

    // 16: CMT_PRESENT_DT (소관위상정일) - Assuming optional
    #[serde(rename(deserialize = "CMT_PRESENT_DT"))]
    pub committee_presentation_date: Option<String>, // Date presented to responsible committee

    // 17: COMMITTEE_DT (소관위회부일) - Assuming optional
    #[serde(rename(deserialize = "COMMITTEE_DT"))]
    pub committee_referral_date: Option<String>, // Date referred to responsible committee

    // 18: PROC_DT (의결일) - Assuming optional (Likely plenary resolution date)
    #[serde(rename(deserialize = "PROC_DT"))]
    pub resolution_date: Option<String>, // Date resolved/voted on

    // 19: COMMITTEE_ID (소관위원회ID) - Assuming optional
    #[serde(rename(deserialize = "COMMITTEE_ID"))]
    pub committee_id: Option<String>, // ID of the responsible committee

    // 20: PUBL_PROPOSER (공동발의자) - Likely list of names
    #[serde(rename(deserialize = "PUBL_PROPOSER"))]
    pub public_proposers: String, // Co-sponsors / Public proposers

    // 21: LAW_PROC_RESULT_CD (법사위처리결과) - Assuming optional (Result code)
    #[serde(rename(deserialize = "LAW_PROC_RESULT_CD"))]
    pub law_committee_processing_result_code: Option<String>, // Result code from L&J Committee

    // 22: RST_PROPOSER (대표발의자)
    #[serde(rename(deserialize = "RST_PROPOSER"))]
    pub representative_proposer: String, // Representative proposer/sponsor
}

impl BillInfo {
    pub fn get_representative_proposers(&self) -> Vec<String> {
        self.representative_proposer
            .split(',')
            .map(|s| s.to_string())
            .collect()
    }

    pub fn get_co_proposers(&self) -> Vec<String> {
        self.public_proposers
            .split(',')
            .map(|s| s.to_string())
            .collect()
    }
}
